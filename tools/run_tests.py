#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
CZ-DSL one-command reproduction and test harness
================================================

Runs the complete toolchain end to end from a clean checkout and writes both a
machine-readable report and a set of test-result figures.

    python3 tools/run_tests.py                # full run, write figures
    python3 tools/run_tests.py --quick        # skip the Rust build and the pipeline
    python3 tools/run_tests.py --no-figures   # text/JSON report only
    python3 tools/run_tests.py --outdir DIR   # where to put reports (default reports/)

What it actually executes (nothing is simulated):

  1. environment   -- Python version, optional dependency probe
  2. validator     -- CZValidator over every .cz in the corpus, per code class
  3. compiler      -- CZCompiler to all four targets, byte sizes checked non-zero
  4. rust          -- cargo build + cargo test on src/rust_core (needs cargo)
  5. pipeline      -- tools/nyh_case/run_all.py, then a scientific-drift check
  6. benchmark     -- indexed vs linear query, scaling, closure, reorganisation
  7. figures       -- test-result figures derived from the above measurements

Exit code is 0 only when every executed check passes. A check that cannot run
for want of an optional tool is reported as SKIP, not as a pass.

The harness never invents a number: every value in the report and in the figures
is either returned by the tool under test or measured by this script.
"""

from __future__ import annotations

import argparse

import json
import os
import platform
import re
import shutil
import subprocess
import sys
import time
from dataclasses import dataclass, field
from typing import Any, Dict, List, Optional

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(ROOT, "tools"))

CASES = os.path.join(ROOT, "cases")
NYH = os.path.join(CASES, "NewYorkHarbor2050")
SCENARIO = os.path.join(NYH, "new_york_harbor_scenario.cz")
BENCHMARK_JSON = os.path.join(NYH, "benchmark.json")
REPRO_JSON = os.path.join(NYH, "reproducibility.json")
RUST_DIR = os.path.join(ROOT, "src", "rust_core")

PASS, FAIL, SKIP = "PASS", "FAIL", "SKIP"


# --------------------------------------------------------------------------
# result plumbing
# --------------------------------------------------------------------------
@dataclass
class Check:
    name: str
    status: str
    detail: str = ""
    seconds: float = 0.0
    data: Dict[str, Any] = field(default_factory=dict)


class Report:
    def __init__(self) -> None:
        self.checks: List[Check] = []
        self.started = time.time()

    def add(self, c: Check) -> Check:
        self.checks.append(c)
        mark = {PASS: "PASS", FAIL: "FAIL", SKIP: "SKIP"}[c.status]
        print(f"  [{mark}] {c.name}" + (f" -- {c.detail}" if c.detail else ""))
        return c

    def run(self, name: str, fn, *a, **kw) -> Check:
        t0 = time.perf_counter()
        try:
            status, detail, data = fn(*a, **kw)
        except Exception as exc:  # a crash in a check is a failed check
            status, detail, data = FAIL, f"{type(exc).__name__}: {exc}", {}
        return self.add(Check(name, status, detail, time.perf_counter() - t0, data or {}))

    @property
    def failed(self) -> List[Check]:
        return [c for c in self.checks if c.status == FAIL]

    def to_dict(self) -> Dict[str, Any]:
        return {
            "generated": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
            "root": ROOT,
            "python": sys.version.split()[0],
            "platform": platform.platform(),
            "elapsed_s": round(time.time() - self.started, 3),
            "summary": {
                "pass": sum(c.status == PASS for c in self.checks),
                "fail": sum(c.status == FAIL for c in self.checks),
                "skip": sum(c.status == SKIP for c in self.checks),
            },
            "checks": [
                {"name": c.name, "status": c.status, "detail": c.detail,
                 "seconds": round(c.seconds, 6), "data": c.data}
                for c in self.checks
            ],
        }


# --------------------------------------------------------------------------
# 1. environment
# --------------------------------------------------------------------------
def probe_environment() -> tuple[str, str, Dict[str, Any]]:
    """Report the interpreter and which optional tools are present."""
    data: Dict[str, Any] = {
        "python": sys.version.split()[0],
        "executable": sys.executable,
        "platform": platform.platform(),
    }
    for mod in ("matplotlib", "numpy", "rasterio", "netCDF4", "geopandas", "scipy"):
        try:
            __import__(mod)
            data[mod] = True
        except Exception:
            data[mod] = False
    data["cargo"] = shutil.which("cargo") is not None
    data["pdflatex"] = shutil.which("pdflatex") is not None
    data["xelatex"] = shutil.which("xelatex") is not None

    have = [k for k, v in data.items() if v is True]
    detail = f"Python {data['python']}; available: {', '.join(sorted(have)) or 'stdlib only'}"
    if not data["cargo"]:
        detail += "; cargo absent -> Rust check will skip"
    return PASS, detail, data


# --------------------------------------------------------------------------
# 2. validator over the corpus
# --------------------------------------------------------------------------
def discover_corpus() -> List[str]:
    """Every .cz file that ships with the project as a corpus member."""
    out: List[str] = []
    for base in ("verification_tests", "data/schema"):
        d = os.path.join(ROOT, base)
        if not os.path.isdir(d):
            continue
        for f in sorted(os.listdir(d)):
            if f.endswith(".cz"):
                out.append(os.path.join(d, f))
    # Every case directory, so a newly added case is covered without having
    # to be registered here.  "sketches" is the one exclusion: it holds
    # deliberately non-conforming design drafts that are not corpus members.
    cdir = os.path.join(ROOT, "cases")
    if os.path.isdir(cdir):
        for name in sorted(os.listdir(cdir)):
            d = os.path.join(cdir, name)
            if not os.path.isdir(d) or name == "sketches":
                continue
            for f in sorted(os.listdir(d)):
                if f.endswith(".cz"):
                    out.append(os.path.join(d, f))
    if os.path.exists(SCENARIO):
        out.append(SCENARIO)
    # de-duplicate while keeping the discovery order stable
    seen = set()
    uniq = []
    for f in out:
        if f not in seen:
            seen.add(f)
            uniq.append(f)
    return uniq


def run_validator(stdout: bool = True) -> tuple[str, str, Dict[str, Any]]:
    """Validate every corpus file and bucket the diagnostics by code class."""
    from czdsl_tools import CZValidator

    files = discover_corpus()
    if not files:
        return FAIL, "no .cz files found", {}

    per_file = []
    classes: Dict[str, int] = {}
    n_error_total = 0
    for path in files:
        diags = CZValidator().validate_file(path)
        counts = {"error": 0, "warning": 0, "info": 0}
        for d in diags:
            counts[d.severity] = counts.get(d.severity, 0) + 1
            cls = re.sub(r"\d+$", "", d.code) or d.code
            classes[cls] = classes.get(cls, 0) + 1
        n_error_total += counts["error"]
        per_file.append({
            "file": os.path.relpath(path, ROOT),
            "errors": counts["error"],
            "warnings": counts["warning"],
            "infos": counts["info"],
            "diagnostics": len(diags),
        })

    clean = sum(1 for r in per_file if r["errors"] == 0)
    status = PASS if n_error_total == 0 else FAIL
    detail = (f"{clean}/{len(per_file)} files validate with zero errors; "
              f"{n_error_total} errors total")
    return status, detail, {
        "files": len(per_file), "clean": clean, "errors": n_error_total,
        "by_class": classes, "per_file": per_file,
    }


# --------------------------------------------------------------------------
# 3. compiler targets
# --------------------------------------------------------------------------
def run_compiler(outdir: str) -> tuple[str, str, Dict[str, Any]]:
    """Compile the benchmark scenario to all four targets and size the output."""
    from czdsl_tools import CZCompiler

    if not os.path.exists(SCENARIO):
        return FAIL, f"scenario not found: {SCENARIO}", {}

    tmp = os.path.join(outdir, "compiled")
    os.makedirs(tmp, exist_ok=True)
    src_bytes = os.path.getsize(SCENARIO)
    sizes: Dict[str, int] = {}
    problems: List[str] = []

    for target in ("geojson", "citygml", "pipeline", "report"):
        t0 = time.perf_counter()
        text = CZCompiler().compile_file(SCENARIO, target)
        dt = time.perf_counter() - t0
        if not text or not text.strip():
            problems.append(f"{target}: empty output")
            continue
        dest = os.path.join(tmp, f"scenario.{target}")
        with open(dest, "w", encoding="utf-8") as fh:
            fh.write(text)
        on_disk = os.path.getsize(dest)
        if on_disk == 0:
            problems.append(f"{target}: zero bytes on disk")
            continue
        sizes[target] = {"bytes": on_disk, "chars": len(text),
                         "lines": text.count("\n") + 1, "compile_ms": round(dt * 1000, 2)}

    if problems:
        return FAIL, "; ".join(problems), {"sizes": sizes}

    # a target must at least be structurally recognisable
    expect = {"geojson": "FeatureCollection", "citygml": "<", "pipeline": "{", "report": "CZ-DSL"}
    for target, needle in expect.items():
        dest = os.path.join(tmp, f"scenario.{target}")
        head = open(dest, encoding="utf-8", errors="ignore").read(4096)
        if needle not in head:
            problems.append(f"{target}: output does not contain {needle!r}")
    if problems:
        return FAIL, "; ".join(problems), {"sizes": sizes}

    detail = f"4/4 targets; source {src_bytes} B -> " + ", ".join(
        f"{t} {v['bytes']} B" for t, v in sizes.items())
    return PASS, detail, {"source_bytes": src_bytes, "sizes": sizes}


# --------------------------------------------------------------------------
# 4. rust frontend
# --------------------------------------------------------------------------
def _cargo_env() -> Dict[str, str]:
    """Keep cargo's home inside the workspace when the user's is not writable."""
    env = dict(os.environ)
    local = os.path.join(ROOT, ".cargo-home")
    if os.path.isdir(local):
        env["CARGO_HOME"] = local
    return env


def run_rust() -> tuple[str, str, Dict[str, Any]]:
    """Build and test the Rust frontend if cargo is available."""
    if shutil.which("cargo") is None:
        return SKIP, "cargo not on PATH", {}
    if not os.path.isdir(RUST_DIR):
        return SKIP, "src/rust_core absent", {}

    env = _cargo_env()

    def cargo(*args: str, timeout: int = 900) -> subprocess.CompletedProcess:
        return subprocess.run(["cargo", *args], cwd=RUST_DIR, env=env,
                              capture_output=True, text=True, timeout=timeout)

    build = cargo("build", "--offline", "--quiet")
    if build.returncode != 0:
        build = cargo("build", "--quiet")  # offline cache may be cold
    if build.returncode != 0:
        blob = (build.stderr or "") + (build.stdout or "")
        # Distinguish "the crate does not compile" from "this machine cannot
        # fetch or cache crates". The latter is an environment limitation, not
        # a defect, so it is reported as SKIP with the reason.
        environmental = any(s in blob for s in (
            "Operation not permitted",
            "failed to create directory",
            "failed to download",
            "failed to fetch",
            "failed to get",
            "no matching package named",
            "not found in registry",
            "no space left on device",
        ))
        if environmental:
            return SKIP, (
                "cargo cannot fetch or cache crates here (no registry cache and "
                "no writable cargo home); run `cargo build` once with network "
                "access, or provide a populated CARGO_HOME, to enable this check"
            ), {}
        tail = (build.stderr or "").strip().splitlines()[-3:]
        return FAIL, "cargo build failed: " + " | ".join(tail), {}

    test = cargo("test", "--offline", "--quiet")
    if test.returncode != 0:
        test = cargo("test", "--quiet")
    blob = (test.stdout or "") + (test.stderr or "")
    passed = sum(int(m) for m in re.findall(r"test result: ok\. (\d+) passed", blob))
    failed = sum(int(m) for m in re.findall(r"(\d+) failed", blob))

    if test.returncode != 0 or failed:
        return FAIL, f"cargo test failed ({passed} passed, {failed} failed)", {
            "passed": passed, "failed": failed}

    # the CLI must actually tokenize a real scenario
    binary = os.path.join(RUST_DIR, "target", "debug", "cz-dsl")
    n_tokens = 0
    if os.path.exists(binary):
        tok = subprocess.run([binary, "tokenize", SCENARIO], capture_output=True,
                             text=True, timeout=120)
        m = re.search(r"Found (\d+) tokens", tok.stdout or "")
        n_tokens = int(m.group(1)) if m else 0
        if n_tokens == 0:
            return FAIL, "cz-dsl tokenize produced no tokens", {"passed": passed}

    detail = f"build ok; {passed} tests passed"
    if n_tokens:
        detail += f"; tokenizer emits {n_tokens} tokens on the benchmark scenario"
    return PASS, detail, {"passed": passed, "failed": failed, "tokens": n_tokens}


# --------------------------------------------------------------------------
# 5. the New York Harbor pipeline
# --------------------------------------------------------------------------
def run_pipeline() -> tuple[str, str, Dict[str, Any]]:
    """Run the reproducible case pipeline and inspect its drift record."""
    driver = os.path.join(ROOT, "tools", "nyh_case", "run_all.py")
    if not os.path.exists(driver):
        return SKIP, "tools/nyh_case/run_all.py absent", {}

    proc = subprocess.run([sys.executable, driver], cwd=ROOT,
                          capture_output=True, text=True, timeout=3600)
    if proc.returncode != 0:
        tail = (proc.stderr or proc.stdout or "").strip().splitlines()[-3:]
        return FAIL, "pipeline failed: " + " | ".join(tail), {}

    if not os.path.exists(REPRO_JSON):
        return FAIL, "pipeline produced no reproducibility.json", {}

    man = json.load(open(REPRO_JSON, encoding="utf-8"))
    drift = man.get("drift", [])
    scientific = [d for d in drift if not d.get("timing_bearing")]

    data = {
        "elapsed_s": man.get("elapsed_s"),
        "inputs": len(man.get("inputs", {})),
        "outputs": len(man.get("outputs", {})),
        "drift_total": len(drift),
        "drift_scientific": len(scientific),
        "pipeline": man.get("pipeline", []),
    }
    if scientific:
        names = ", ".join(d["file"] for d in scientific[:4])
        return FAIL, f"scientific drift in {len(scientific)} file(s): {names}", data
    detail = (f"{data['inputs']} inputs -> {data['outputs']} outputs in "
              f"{data['elapsed_s']:.1f}s; no scientific drift")
    return PASS, detail, data


# --------------------------------------------------------------------------
# 6. benchmark: query, retrieval, reorganisation, analysis
# --------------------------------------------------------------------------
def run_benchmark() -> tuple[str, str, Dict[str, Any]]:
    """Re-measure the query and reorganisation benchmark on the real scenario."""
    from czdsl_tools import CZTextParser, CZValidator

    if not os.path.exists(SCENARIO):
        return SKIP, "scenario absent", {}
    text = open(SCENARIO, encoding="utf-8").read()

    def parse(t: str):
        p = CZTextParser()
        p.parse(t)
        return p

    def bench(fn, repeat: int = 200, inner: int = 50) -> float:
        fn()  # warm up
        t0 = time.perf_counter()
        for _ in range(repeat):
            for _ in range(inner):
                fn()
        return (time.perf_counter() - t0) / (repeat * inner) * 1e6  # microseconds

    t0 = time.perf_counter()
    program = parse(text)
    parse_ms = (time.perf_counter() - t0) * 1000

    blocks = list(program.blocks)

    # CZTextParser does not populate CZBlock.children, so nested layer and
    # dataset objects inside coastal_database are counted from the block text.
    # This is the same convention the committed benchmark uses, so the count
    # here agrees with cases/NewYorkHarbor2050/benchmark.json.
    nested = 0
    for b in blocks:
        if b.keyword == "coastal_database":
            nested += len(re.findall(r"^\s{8}(\w+):\s*layer\s*\{", b.content, re.M))
            nested += len(re.findall(r"^\s{16}(\w+):\s*dataset\s*\{", b.content, re.M))
    n_objects = len(blocks) + nested

    t0 = time.perf_counter()
    index: Dict[str, List[Any]] = {}
    for b in blocks:
        index.setdefault(b.keyword, []).append(b)
    index_us = (time.perf_counter() - t0) * 1e6

    kw = max(index, key=lambda k: len(index[k]))

    def indexed():
        block = index[kw][0]
        return block.name

    def scan():
        """Full linear scan collecting every match, as the benchmark defines it."""
        return [b.name for b in blocks if b.keyword == kw]

    t_indexed = bench(indexed)
    t_scan = bench(scan)
    ratio = t_scan / t_indexed if t_indexed else float("nan")

    # scaling: replicate the corpus and confirm indexed lookup stays flat
    scaling = []
    for factor in (1, 2, 4, 8):
        big = parse(text * factor)
        big_idx: Dict[str, List[Any]] = {}
        for b in big.blocks:
            big_idx.setdefault(b.keyword, []).append(b)
        bb = list(big.blocks)

        def big_indexed(idx=big_idx, k=kw):
            return idx[k][0].name

        def big_scan(bs=bb, k=kw):
            return [b.name for b in bs if b.keyword == k]

        scaling.append({
            "factor": factor,
            "blocks": len(bb),
            "indexed_us": bench(big_indexed, repeat=60, inner=20),
            "scan_us": bench(big_scan, repeat=60, inner=20),
        })

    # reorganisation must round-trip through the validator, and is timed
    def reorganise() -> str:
        return "\n".join(b.content.rstrip() + "\n}" for b in blocks) + "\n"

    t0 = time.perf_counter()
    reorg = reorganise()
    reorg_us = (time.perf_counter() - t0) * 1e6
    diags = CZValidator().validate(reorg)
    reorg_errors = sum(1 for d in diags if d.severity == "error")
    # the rebuilt document must preserve every block name and keyword
    reparsed = parse(reorg)
    before = sorted((b.keyword, b.name) for b in blocks)
    after = sorted((b.keyword, b.name) for b in (reparsed.blocks if reparsed else []))
    roundtrip_lossless = before == after

    data = {
        "source_bytes": len(text.encode("utf-8")),
        "source_lines": text.count("\n") + 1,
        "blocks": len(blocks),
        "objects": n_objects,
        "parse_ms": round(parse_ms, 3),
        "index_build_us": round(index_us, 3),
        "indexed_us": round(t_indexed, 4),
        "scan_us": round(t_scan, 4),
        "speedup": round(ratio, 2),
        "reorg_us": round(reorg_us, 3),
        "reorg_bytes": len(reorg.encode("utf-8")),
        "reorg_errors": reorg_errors,
        "reorg_roundtrip_lossless": roundtrip_lossless,
        "scaling": scaling,
    }
    if reorg_errors:
        return FAIL, f"reorganisation introduced {reorg_errors} validation errors", data
    if not roundtrip_lossless:
        return FAIL, "reorganisation lost or renamed blocks on round-trip", data
    if not (ratio > 1.0):
        return FAIL, f"indexed lookup was not faster than scan (ratio {ratio:.2f})", data
    detail = (f"{data['blocks']} blocks / {data['objects']} objects; indexed "
              f"{t_indexed:.4f} us vs scan {t_scan:.4f} us ({ratio:.1f}x); "
              f"reorg {reorg_us:.2f} us, {reorg_errors} errors")
    return PASS, detail, data


# --------------------------------------------------------------------------
# 7. manuscript figure preflight
# --------------------------------------------------------------------------
def run_figure_preflight() -> tuple[str, str, Dict[str, Any]]:
    """Check the manuscript figures against the publisher's requirements.

    Existence is not readiness. A figure can be present, compile, and still be
    rejected: Type 3 fonts are refused, and type below roughly 6 pt at the
    printed size cannot be read. This runs the same measurements on every
    figure the manuscript includes.
    """
    tex = os.path.join(ROOT, "docs", "ijgis_submission", "main.tex")
    if not os.path.exists(tex):
        return SKIP, "manuscript not present", {}
    if not os.path.exists(os.path.join(ROOT, "docs", "ijgis_submission",
                                       "figures")):
        return SKIP, "figure directory not present", {}

    sys.path.insert(0, os.path.join(ROOT, "tools"))
    try:
        import figure_qa
    except Exception as exc:
        return SKIP, f"PyMuPDF unavailable ({type(exc).__name__})", {}

    try:
        import fitz  # noqa: F401
    except Exception as exc:
        return SKIP, f"PyMuPDF unavailable ({type(exc).__name__})", {}

    widths = figure_qa.includegraphics_widths(open(tex, encoding="utf-8").read())
    figdir = os.path.join(ROOT, "docs", "ijgis_submission", "figures")
    results = [figure_qa.analyse(os.path.join(figdir, n), widths[n])
               for n in sorted(os.listdir(figdir))
               if n.lower().endswith(".pdf") and n in widths]
    if not results:
        return FAIL, "no figures referenced by the manuscript were found", {}

    bad: List[str] = []
    for r in results:
        t3 = [f["basefont"] for f in r["fonts"] if f["type"] == "Type3"]
        notemb = [f["basefont"] for f in r["fonts"] if not f["embedded"]]
        if t3:
            bad.append(f"{r['file']}: Type 3 font {','.join(t3)}")
        if notemb:
            bad.append(f"{r['file']}: font not embedded {','.join(notemb)}")
        if r["min_effective_pt"] < figure_qa.MIN_EFFECTIVE_PT:
            bad.append(f"{r['file']}: {r['min_effective_pt']} pt at print size")
        if r["collisions"]:
            bad.append(f"{r['file']}: {len(r['collisions'])} text collision(s)")
        if r["escapes"]:
            bad.append(f"{r['file']}: {len(r['escapes'])} label(s) clipped")

    smallest = min(r["min_effective_pt"] for r in results)
    data = {
        "figures": len(results),
        "smallest_effective_pt": smallest,
        "types3": sum(1 for r in results
                      for f in r["fonts"] if f["type"] == "Type3"),
        "collisions": sum(len(r["collisions"]) for r in results),
        "per_figure": {r["file"]: {"effective_pt": r["min_effective_pt"],
                                   "scale": r["scale"],
                                   "collisions": len(r["collisions"])}
                       for r in results},
    }
    if bad:
        return FAIL, "; ".join(bad[:4]), data
    return PASS, (f"{len(results)} figures; smallest type {smallest} pt at print "
                  f"size; no Type 3 fonts, no collisions"), data


def run_figure_visual_qa() -> tuple[str, str, Dict[str, Any]]:
    """Inspect the rendered figures pixel by pixel.

    The preflight above is geometric: it measures fonts, text boxes and
    collisions, and it is blind to anything that only exists once rasterised.
    It passed a figure whose colourbar was drawn across two panels, one whose
    dashed frame struck through its own caption, and one with grey type on a
    near-black patch. This renders each included figure and checks contrast,
    strokes crossing glyphs (accounting for later opaque label masks), dead
    space, raster resolution and panel balance.
    """
    tex = os.path.join(ROOT, "docs", "ijgis_submission", "main.tex")
    figdir = os.path.join(ROOT, "docs", "ijgis_submission", "figures")
    if not os.path.exists(tex) or not os.path.isdir(figdir):
        return SKIP, "manuscript figures not present", {}

    sys.path.insert(0, os.path.join(ROOT, "tools"))
    try:
        import visual_qa
    except Exception as exc:
        return SKIP, f"visual_qa unavailable ({type(exc).__name__})", {}

    widths = visual_qa.includegraphics_widths(tex)
    names = [n for n in sorted(os.listdir(figdir))
             if n.lower().endswith(".pdf") and (not widths or n in widths)]
    if not names:
        return FAIL, "no figures referenced by the manuscript were found", {}

    # 300 dpi matches the CLI default.  The background-purity test that backs
    # the textured-background check is sensitive to sampling density: at
    # 200 dpi antialiasing leaves the modal background colour too weak, and
    # ordinary map labels are reported as sitting on a textured patch.
    reports = [visual_qa.analyse(os.path.join(figdir, n), dpi=300,
                                 frac=widths.get(n, 1.0)) for n in names]
    bad: List[str] = []
    for r in reports:
        for tag in visual_qa.verdict(r):
            bad.append(f"{r['file']}: {tag}")

    data = {
        "figures": len(reports),
        "failing": len({b.split(':')[0] for b in bad}),
        "waived": {r["file"]: sorted(r.get("allowed", {}))
                   for r in reports if r.get("allowed")},
        "per_figure": {r["file"]: {"dead_space": r["dead_space"]["area_frac"],
                                   "imbalance": r["balance"]["imbalance"]}
                       for r in reports},
    }
    if bad:
        return FAIL, "; ".join(bad[:4]), data
    return PASS, (f"{len(reports)} figures; contrast, stroke, dead-space and "
                  f"balance checks clean"), data


# --------------------------------------------------------------------------
# 8. figures
# --------------------------------------------------------------------------
def make_figures(report: Report, outdir: str) -> tuple[str, str, Dict[str, Any]]:
    """Draw the test-result figures from the measurements taken above."""
    try:
        import matplotlib
        matplotlib.use("Agg")
        import matplotlib.pyplot as plt
    except Exception as exc:
        return SKIP, f"matplotlib unavailable ({exc})", {}

    def get(name: str) -> Dict[str, Any]:
        for c in report.checks:
            if c.name == name:
                return c.data
        return {}

    val = get("validator: corpus")
    comp = get("compiler: four targets")
    bench = get("benchmark: query and reorganisation")
    pipe = get("pipeline: reproduce case study")
    rust = get("rust: build and tests")

    figdir = os.path.join(outdir, "figures")
    os.makedirs(figdir, exist_ok=True)
    written: List[str] = []

    ink, mid, lt = "#1c3a63", "#5b7ba6", "#c8d4e4"
    plt.rcParams.update({"font.size": 8, "axes.edgecolor": "#444",
                         "axes.linewidth": 0.7, "figure.dpi": 160})

    # ---- panel figure: the whole harness at a glance ----
    fig, axes = plt.subplots(2, 2, figsize=(7.4, 5.4))
    fig.subplots_adjust(hspace=0.45, wspace=0.28, top=0.88, bottom=0.12,
                        left=0.09, right=0.97)
    fig.suptitle("CZ-DSL toolchain verification", fontsize=10, y=0.97)

    # (a) check status
    ax = axes[0][0]
    names = [c.name.replace(": ", ":\n") for c in report.checks]
    statuses = [c.status for c in report.checks]
    colors = {PASS: ink, FAIL: "#a33", SKIP: lt}
    ax.barh(range(len(names)), [1] * len(names),
            color=[colors[s] for s in statuses], height=0.65)
    ax.set_yticks(range(len(names)))
    ax.set_yticklabels(names, fontsize=6.5)
    ax.invert_yaxis()
    ax.set_xlim(0, 1)
    ax.set_xticks([])
    for i, s in enumerate(statuses):
        ax.text(0.02, i, s, va="center", ha="left", color="white", fontsize=6.5,
                fontweight="bold")
    ax.set_title("(a) harness checks", fontsize=8.5, loc="left")
    for sp in ("top", "right", "bottom"):
        ax.spines[sp].set_visible(False)

    # (b) validator diagnostics by code class
    ax = axes[0][1]
    by_class = val.get("by_class", {})
    if by_class:
        ks = sorted(by_class)
        ax.bar(ks, [by_class[k] for k in ks], color=mid, width=0.6)
        ax.set_ylabel("diagnostics")
    else:
        ax.text(0.5, 0.5, "no diagnostics\n(zero across corpus)", ha="center",
                va="center", fontsize=8, color=ink)
        ax.set_xticks([])
    ax.set_title("(b) validator diagnostics by code family", fontsize=8.5, loc="left")
    ax.tick_params(axis="x", labelrotation=45, labelsize=6.5)

    # (c) compiler target sizes
    ax = axes[1][0]
    sizes = comp.get("sizes", {})
    if sizes:
        ts = list(sizes)
        ax.bar(ts, [sizes[t]["bytes"] for t in ts], color=ink, width=0.6)
        src = comp.get("source_bytes")
        if src:
            ax.axhline(src, color="#a33", ls="--", lw=0.9)
            ax.text(len(ts) - 0.5, src, f" source {src} B", color="#a33",
                    fontsize=6.5, va="bottom", ha="right")
        ax.set_ylabel("bytes")
    ax.set_title("(c) compiler output size by target", fontsize=8.5, loc="left")
    ax.tick_params(axis="x", labelrotation=30, labelsize=7)

    # (d) query cost: indexed vs scan, and the scaling curve
    ax = axes[1][1]
    sc = bench.get("scaling", [])
    if sc:
        xs = [s["factor"] for s in sc]
        ax.plot(xs, [s["indexed_us"] for s in sc], "o-", color=ink, lw=1.4,
                ms=4, label="indexed")
        ax.plot(xs, [s["scan_us"] for s in sc], "s--", color="#a33", lw=1.4,
                ms=4, label="linear scan")
        ax.set_xlabel("corpus replication factor")
        ax.set_ylabel("median query (us)")
        ax.set_xticks(xs)
        ax.legend(frameon=False, fontsize=7)
    ax.set_title("(d) query cost versus corpus growth", fontsize=8.5, loc="left")

    p1 = os.path.join(figdir, "test_dashboard.pdf")
    fig.savefig(p1)
    fig.savefig(p1.replace(".pdf", ".png"))
    plt.close(fig)
    written += [p1, p1.replace(".pdf", ".png")]

    # ---- figure 2: latency breakdown + pipeline provenance ----
    fig, axes = plt.subplots(1, 2, figsize=(7.4, 3.0))
    fig.subplots_adjust(wspace=0.35, top=0.82, bottom=0.24, left=0.10, right=0.97)
    fig.suptitle("CZ-DSL operation cost and case-study provenance", fontsize=10, y=0.96)

    ax = axes[0]
    ops = []
    if bench:
        ops = [("parse", bench.get("parse_ms", 0) * 1000),
               ("index build", bench.get("index_build_us", 0)),
               ("indexed query", bench.get("indexed_us", 0)),
               ("linear scan", bench.get("scan_us", 0)),
               ("reorganise", bench.get("reorg_us", 0))]
    if ops:
        lab = [o[0] for o in ops]
        val_ = [max(o[1], 1e-4) for o in ops]
        ax.barh(lab, val_, color=[ink if v < 100 else mid for v in val_], height=0.6)
        ax.set_xscale("log")
        ax.set_xlabel("microseconds (log scale)")
    ax.set_title("(a) measured operation cost", fontsize=8.5, loc="left")
    ax.invert_yaxis()
    ax.tick_params(labelsize=7)

    ax = axes[1]
    if pipe:
        n_in, n_out = pipe.get("inputs", 0), pipe.get("outputs", 0)
        sci = pipe.get("drift_scientific", 0)
        ax.bar(["inputs", "outputs"], [n_in, n_out], color=[mid, ink], width=0.5)
        ax.set_ylabel("files")
        ax.text(0.5, 0.92, f"scientific drift: {sci}", transform=ax.transAxes,
                ha="center", fontsize=8,
                color=("#a33" if sci else ink), fontweight="bold")
        if pipe.get("elapsed_s"):
            ax.text(0.5, 0.80, f"pipeline {pipe['elapsed_s']:.1f} s",
                    transform=ax.transAxes, ha="center", fontsize=7)
    ax.set_title("(b) case-study provenance", fontsize=8.5, loc="left")
    ax.tick_params(labelsize=7)

    p2 = os.path.join(figdir, "test_costs.pdf")
    fig.savefig(p2)
    fig.savefig(p2.replace(".pdf", ".png"))
    plt.close(fig)
    written += [p2, p2.replace(".pdf", ".png")]

    detail = f"{len(written)} figure files written to {os.path.relpath(figdir, ROOT)}"
    return PASS, detail, {"files": [os.path.relpath(w, ROOT) for w in written],
                          "rust_tests": rust.get("passed", 0)}


# --------------------------------------------------------------------------
# main
# --------------------------------------------------------------------------
def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[1])
    ap.add_argument("--outdir", default=os.path.join(ROOT, "reports"),
                    help="directory for the report and figures")
    ap.add_argument("--quick", action="store_true",
                    help="skip the Rust build and the case-study pipeline")
    ap.add_argument("--no-figures", action="store_true", help="skip figure generation")
    args = ap.parse_args()

    os.makedirs(args.outdir, exist_ok=True)
    rep = Report()
    print("CZ-DSL toolchain verification")
    print("=" * 62)

    rep.run("environment", probe_environment)
    rep.run("validator: corpus", run_validator)
    rep.run("compiler: four targets", run_compiler, args.outdir)

    if args.quick:
        rep.add(Check("rust: build and tests", SKIP, "--quick requested"))
        rep.add(Check("pipeline: reproduce case study", SKIP, "--quick requested"))
    else:
        rep.run("rust: build and tests", run_rust)
        rep.run("pipeline: reproduce case study", run_pipeline)

    rep.run("benchmark: query and reorganisation", run_benchmark)
    rep.run("figures: publication preflight", run_figure_preflight)
    rep.run("figures: visual inspection", run_figure_visual_qa)

    if not args.no_figures:
        rep.run("figures: test results", make_figures, rep, args.outdir)

    summary = rep.to_dict()
    json_path = os.path.join(args.outdir, "test_report.json")
    with open(json_path, "w", encoding="utf-8") as fh:
        json.dump(summary, fh, indent=2)

    print("=" * 62)
    s = summary["summary"]
    print(f"PASS {s['pass']}   FAIL {s['fail']}   SKIP {s['skip']}"
          f"   ({summary['elapsed_s']:.1f}s)")
    print(f"report: {os.path.relpath(json_path, ROOT)}")

    if rep.failed:
        print("\nFailed checks:")
        for c in rep.failed:
            print(f"  - {c.name}: {c.detail}")
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
