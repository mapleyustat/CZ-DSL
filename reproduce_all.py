#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Top-level reproduction entry point for CZ-DSL.

Runs the verification harness, then builds the manuscript and the slides with
the correct engine for each.

    python3 reproduce_all.py               # harness + paper + slides
    python3 reproduce_all.py --quick       # harness only, skipping Rust and the pipeline
    python3 reproduce_all.py --paper-only  # just rebuild the manuscript
    python3 reproduce_all.py --slides-only # just rebuild the slides

Engine note: the manuscript is English and uses `pdflatex` with a manual
`thebibliography`, so it needs no BibTeX run. The slides are a Chinese-language
Beamer deck and need `xelatex`, because its CTeX fontset is unavailable under
`pdflatex`. An earlier version of this script used `xelatex`+`bibtex` for the
paper and pointed at directories that do not exist; that is fixed here.
"""

from __future__ import annotations

import argparse
import os
import shutil
import subprocess
import sys

ROOT = os.path.dirname(os.path.abspath(__file__))
PAPER_DIR = os.path.join(ROOT, "docs", "ijgis_submission")
PAPER_TEX = "main.tex"
SLIDES_DIR = os.path.join(ROOT, "slides")
SLIDES_TEX = "cz_dsl_beamer.tex"


def run(cmd: list, cwd: str, label: str) -> bool:
    print(f"  -> {label}")
    proc = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True)
    if proc.returncode != 0:
        tail = (proc.stderr or proc.stdout or "").strip().splitlines()[-5:]
        print(f"     FAILED (exit {proc.returncode})")
        for line in tail:
            print(f"     | {line}")
        return False
    return True


def build_paper() -> bool:
    if not os.path.exists(os.path.join(PAPER_DIR, PAPER_TEX)):
        print("  manuscript not found; skipping")
        return False
    print("\n[paper] pdflatex, run twice (manual thebibliography, no BibTeX)")
    for f in ("main.aux", "main.log", "main.out"):
        p = os.path.join(PAPER_DIR, f)
        if os.path.exists(p):
            os.remove(p)
    for i in (1, 2):
        if not run(["pdflatex", "-interaction=nonstopmode", "-halt-on-error", PAPER_TEX],
                   PAPER_DIR, f"pass {i}/2"):
            return False
    log = os.path.join(PAPER_DIR, "main.log")
    errs = undef = 0
    if os.path.exists(log):
        with open(log, encoding="utf-8", errors="ignore") as fh:
            text = fh.read()
        errs = sum(1 for line in text.splitlines() if line.startswith("!"))
        undef = text.count("undefined")
    pdf = os.path.join(PAPER_DIR, "main.pdf")
    ok = errs == 0 and undef == 0 and os.path.exists(pdf)
    print(f"     errors={errs} undefined={undef} pdf={'yes' if os.path.exists(pdf) else 'NO'}")
    return ok


def build_slides() -> bool:
    if not os.path.exists(os.path.join(SLIDES_DIR, SLIDES_TEX)):
        print("  slides not found; skipping")
        return False
    if shutil.which("xelatex") is None:
        print("  xelatex not available; skipping")
        return False
    print("\n[slides] xelatex, run twice (Chinese CTeX deck)")
    for f in ("cz_dsl_beamer.aux", "cz_dsl_beamer.log", "cz_dsl_beamer.out"):
        p = os.path.join(SLIDES_DIR, f)
        if os.path.exists(p):
            os.remove(p)
    for i in (1, 2):
        if not run(["xelatex", "-interaction=nonstopmode", "-halt-on-error", SLIDES_TEX],
                   SLIDES_DIR, f"pass {i}/2"):
            return False
    log = os.path.join(SLIDES_DIR, "cz_dsl_beamer.log")
    errs = 0
    if os.path.exists(log):
        with open(log, encoding="utf-8", errors="ignore") as fh:
            errs = sum(1 for line in fh if line.startswith("!"))
    pdf = os.path.join(SLIDES_DIR, "cz_dsl_beamer.pdf")
    print(f"     errors={errs} pdf={'yes' if os.path.exists(pdf) else 'NO'}")
    return errs == 0 and os.path.exists(pdf)


def main() -> int:
    ap = argparse.ArgumentParser(description="Reproduce CZ-DSL end to end.")
    ap.add_argument("--quick", action="store_true", help="harness only, skipping Rust and the pipeline")
    ap.add_argument("--paper-only", action="store_true")
    ap.add_argument("--slides-only", action="store_true")
    args = ap.parse_args()

    print("=" * 62)
    print("CZ-DSL reproduction")
    print("=" * 62)
    results = {}

    if not (args.paper_only or args.slides_only):
        harness = os.path.join(ROOT, "tools", "run_tests.py")
        if not os.path.exists(harness):
            print("tools/run_tests.py missing")
            return 1
        cmd = [sys.executable, harness] + (["--quick"] if args.quick else [])
        print("\n[harness] tools/run_tests.py")
        proc = subprocess.run(cmd, cwd=ROOT)
        results["harness"] = proc.returncode == 0

    if not args.slides_only:
        results["paper"] = build_paper()
    if not args.paper_only:
        results["slides"] = build_slides()

    print("\n" + "=" * 62)
    for k, v in results.items():
        print(f"  {k:10s} {'PASS' if v else 'FAIL'}")
    print("=" * 62)
    return 0 if all(results.values()) else 1


if __name__ == "__main__":
    sys.exit(main())
