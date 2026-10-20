#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Language expressiveness and formal-property checks for the CZ-DSL paper.

Four experiments over the 43-atom vocabulary and the .cz corpus:

  A  construct coverage     which grammar keywords and which atoms the corpus exercises
  B  grade well-formedness  the atom-to-grade map must be a total function
  C  semantic injectivity   atom names and interface contracts must not collide
  D  granularity consistency each declared level against the resolution it is used with

Writes data/ijgis/EXPRESSIVENESS.json and EXPRESSIVENESS.md.
"""
import collections
import glob
import json
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
OUT = os.path.join(ROOT, "data", "ijgis")

# ---------------------------------------------------------------- vocabulary
# The 43 atoms, as declared in the quick-reference table of the paper:
# (group, atom, grade, interface contract)
ATOMS = [
    ("DataAccessAtom", "DataSource", "1-vec", "identity() -> URI; schema() -> SchemaRef"),
    ("DataAccessAtom", "CoordinateReference", "1-vec", "to_srs(target) -> Transform"),
    ("DataAccessAtom", "TemporalReference", "1-vec", "to_epoch() -> UnixTimestamp"),
    ("DataAccessAtom", "ResolutionSpec", "1-vec", "spatial_res() -> Length"),
    ("DataAccessAtom", "QualityProfile", "1-vec", "qc_level() -> QCLevel"),
    ("DataAccessAtom", "UncertaintyModel", "1-vec", "distribution() -> Distribution"),
    ("DataAccessAtom", "AccessEndpoint", "1-vec", "protocol() -> Protocol"),
    ("ModelAdapterAtom", "ModelInterface", "2-vec", "inputs(); outputs()"),
    ("ModelAdapterAtom", "ModelCapability", "2-vec", "capabilities() -> Set"),
    ("ModelAdapterAtom", "ModelIOContract", "2-vec", "validate(input) -> Result"),
    ("ModelAdapterAtom", "ModelVersion", "2-vec", "version() -> SemVer"),
    ("ModelAdapterAtom", "ModelDeployment", "2-vec", "runtime() -> RuntimeSpec"),
    ("ModelAdapterAtom", "ModelInvokeBinding", "2-vec", "bind(data, scenario) -> BoundModel"),
    ("CrossSubjectLinkEngine", "Subject", "1+2-vec", "id() -> SubjectID"),
    ("CrossSubjectLinkEngine", "Role", "1+2-vec", "permissions() -> Set"),
    ("CrossSubjectLinkEngine", "Jurisdiction", "1+2-vec", "boundary() -> Polygon"),
    ("CrossSubjectLinkEngine", "SemanticLink", "1+2-vec", "source(); target(); type()"),
    ("CrossSubjectLinkEngine", "Provenance", "1+2-vec", "origin(); timestamp()"),
    ("CrossSubjectLinkEngine", "Authorization", "1+2-vec", "granted_by(); granted_to()"),
    ("CrossSubjectLinkEngine", "Responsibility", "1+2-vec", "duty(); accountable_to()"),
    ("LLMDispatchInterface", "LLMTask", "3-vec", "input(); expected()"),
    ("LLMDispatchInterface", "PromptContract", "3-vec", "template(); variables()"),
    ("LLMDispatchInterface", "SpatialCognitionCall", "3-vec", "spatial_query(); model()"),
    ("LLMDispatchInterface", "LogicInferenceCall", "3-vec", "premises(); query()"),
    ("LLMDispatchInterface", "LLMOutputBinding", "3-vec", "parse(raw); schema()"),
    ("LLMDispatchInterface", "GroundingConstraint", "3-vec", "grounding_source(); tolerance()"),
    ("LLMDispatchInterface", "HallucinationGuard", "3-vec", "validate(output); threshold()"),
    ("GranularityPartitionEngine", "GranularityLevel", "s+1-vec", "level(); scale_range()"),
    ("GranularityPartitionEngine", "PartitionRule", "s+1-vec", "apply(parent)"),
    ("GranularityPartitionEngine", "AggregationOperator", "s+1-vec", "aggregate(children)"),
    ("GranularityPartitionEngine", "DisaggregationOperator", "s+1-vec", "disaggregate(parent)"),
    ("GranularityPartitionEngine", "BoundaryRule", "s+1-vec", "resolve(conflict)"),
    ("LayeredClassificationManager", "WholeDomain", "scalar", "L0: 1e4--1e6 km2"),
    ("LayeredClassificationManager", "CoastalCell", "scalar", "L1: 1e2--1e4 km2"),
    ("LayeredClassificationManager", "CorePatch", "scalar", "L2: 1e-1--1e2 km2"),
    ("LayeredClassificationManager", "BaseElement", "scalar", "L3: <1e-1 km2"),
    ("DigitalTwinMapping", "PhysicalAsset", "pseudo", "id(); geometry()"),
    ("DigitalTwinMapping", "DigitalReplica", "pseudo", "model(); fidelity()"),
    ("DigitalTwinMapping", "AssimilationBinding", "pseudo", "method(); frequency()"),
    ("DigitalTwinMapping", "SimulationBinding", "pseudo", "engine(); horizon()"),
    ("DigitalTwinMapping", "FeedbackControl", "pseudo", "actuators(); policy()"),
    ("DigitalTwinMapping", "StateVariable", "pseudo", "name(); type(); unit()"),
    ("DigitalTwinMapping", "ObservationStream", "pseudo", "source(); rate()"),
]

# Which atom a declared slot in the grammar instantiates. First match wins.
SLOT_ATOMS = [
    (("coordinate_ref", "crs", "coordinate_reference", "geographic_crs", "projection"),
     "CoordinateReference"),
    (("temporal_ref", "temporal_reference", "time_ref", "epoch", "period"),
     "TemporalReference"),
    (("resolution", "spatial_res", "grid_res", "cell_size"), "ResolutionSpec"),
    (("quality", "qc", "quality_level", "accuracy"), "QualityProfile"),
    (("uncertainty", "error_model", "distribution"), "UncertaintyModel"),
    (("url", "endpoint", "access", "service", "protocol"), "AccessEndpoint"),
    (("source_type", "format", "data_type"), "DataSource"),
    (("input", "inputs", "output", "outputs"), "ModelInterface"),
    (("capability", "capabilities"), "ModelCapability"),
    (("contract", "validate"), "ModelIOContract"),
    (("version",), "ModelVersion"),
    (("runtime", "deployment", "engine"), "ModelDeployment"),
    (("bind", "binding", "invoke"), "ModelInvokeBinding"),
    (("source", "from"), "SemanticLink"),
    (("target", "to"), "SemanticLink"),
    (("link_type", "relation", "rel"), "SemanticLink"),
    (("role", "permissions"), "Role"),
    (("jurisdiction", "boundary"), "Jurisdiction"),
    (("provenance", "origin", "citation", "license"), "Provenance"),
    (("granted_by", "granted_to", "authorization"), "Authorization"),
    (("duty", "accountable", "responsibility", "agency"), "Responsibility"),
    (("subject", "stakeholder"), "Subject"),
    (("prompt", "template"), "PromptContract"),
    (("task", "expected"), "LLMTask"),
    (("spatial_query", "cognition"), "SpatialCognitionCall"),
    (("premises", "inference"), "LogicInferenceCall"),
    (("parse", "raw_output", "output_schema"), "LLMOutputBinding"),
    (("grounding", "tolerance"), "GroundingConstraint"),
    (("guard", "threshold", "hallucination"), "HallucinationGuard"),
    (("level", "scale_range"), "GranularityLevel"),
    (("partition", "apply"), "PartitionRule"),
    (("aggregate", "aggregation"), "AggregationOperator"),
    (("disaggregate", "disaggregation"), "DisaggregationOperator"),
    (("conflict", "boundary_rule"), "BoundaryRule"),
    (("asset", "physical"), "PhysicalAsset"),
    (("replica", "fidelity"), "DigitalReplica"),
    (("assimilation", "frequency"), "AssimilationBinding"),
    (("simulation", "horizon"), "SimulationBinding"),
    (("actuator", "feedback", "policy"), "FeedbackControl"),
    (("state_variable", "unit"), "StateVariable"),
    (("observation", "rate", "stream"), "ObservationStream"),
]

# The four classification levels and the anchors the paper declares for them
LEVELS = {
    "L0": ("WholeDomain", 1000.0, "year"),
    "L1": ("CoastalCell", 270.0, "month"),
    "L2": ("CorePatch", 90.0, "day"),
    "L3": ("BaseElement", 30.0, "hour"),
}


def corpus():
    return sorted(p for p in glob.glob(os.path.join(ROOT, "cases", "*", "*.cz"))
                  if "sketch" not in p.lower())


def slots_of(text):
    """Yield (keyword, slot_name) for every declared slot in the description."""
    for m in re.finditer(r"(?m)^\s*([A-Za-z_][A-Za-z0-9_]*)\s*:", text):
        yield m.group(1)


def experiment_a(files):
    kw = collections.Counter()
    slot = collections.Counter()
    per_file = {}
    for p in files:
        t = open(p, encoding="utf-8").read()
        used = set()
        for m in re.finditer(r"(?m)^(\w+)\s+\w+\s*\{", t):
            kw[m.group(1)] += 1
            used.add(m.group(1))
        for s in slots_of(t):
            slot[s] += 1
        per_file[os.path.relpath(p, os.path.join(ROOT, "cases"))] = sorted(used)

    # atom coverage: an atom is exercised when any of its slots appears
    atom_hits = collections.Counter()
    for s, n in slot.items():
        for names, atom in SLOT_ATOMS:
            if s in names:
                atom_hits[atom] += n
                break
    # the four classification levels are written as namespace codes rather
    # than as slots, so they are counted separately
    LEVEL_CODE = {"WD": "WholeDomain", "CC": "CoastalCell",
                  "CP": "CorePatch", "BE": "BaseElement"}
    level_codes = collections.Counter()
    for p in files:
        t = open(p, encoding="utf-8").read()
        for m in re.finditer(r"CZ\.(WD|CC|CP|BE)\.", t):
            level_codes[m.group(1)] += 1
            atom_hits[LEVEL_CODE[m.group(1)]] += 1
    covered, uncovered = [], []
    for group, atom, grade, _ in ATOMS:
        (covered if atom_hits.get(atom) else uncovered).append(
            dict(group=group, atom=atom, grade=grade,
                 hits=atom_hits.get(atom, 0)))
    return dict(keywords=dict(kw.most_common()), slots=dict(slot.most_common()),
                level_codes=dict(level_codes.most_common()),
                covered=covered, uncovered=uncovered,
                n_atoms=len(ATOMS), n_covered=len(covered),
                per_file=per_file)


def experiment_b():
    """The atom-to-grade map must be a function: one grade per atom."""
    by_atom = collections.defaultdict(set)
    for group, atom, grade, _ in ATOMS:
        by_atom[atom].add(grade)
    multi = {a: sorted(g) for a, g in by_atom.items() if len(g) > 1}
    # and the group-to-grade map should be a function too, except that the
    # paper declares two groups with composite grades
    by_group = collections.defaultdict(set)
    for group, atom, grade, _ in ATOMS:
        by_group[group].add(grade)
    group_multi = {g: sorted(v) for g, v in by_group.items() if len(v) > 1}
    dup = [a for a, c in collections.Counter(a for _, a, _, _ in ATOMS).items()
           if c > 1]
    return dict(n_atoms=len(ATOMS), n_distinct_atoms=len(by_atom),
                atoms_with_two_grades=multi, groups_with_two_grades=group_multi,
                duplicate_names=dup,
                grades=sorted({g for _, _, g, _ in ATOMS}),
                groups=sorted({g for g, _, _, _ in ATOMS}))


def _sig(contract):
    """The interface signature: the method names, stripped of prose."""
    return tuple(sorted(set(re.findall(r"[a-z_]+(?=\()", contract))))


def experiment_c():
    """Distinct atoms must not carry the same interface signature."""
    by_sig = collections.defaultdict(list)
    for _, atom, _, contract in ATOMS:
        by_sig[_sig(contract)].append(atom)
    collide = {",".join(k): v for k, v in by_sig.items() if len(v) > 1}
    # name-level: is any atom name a strict prefix of another atom name?
    names = [a for _, a, _, _ in ATOMS]
    prefix = [(a, b) for a in names for b in names
              if a != b and b.startswith(a)]
    return dict(n_signatures=len(by_sig),
                colliding_signatures=collide,
                prefix_pairs=[f"{a} < {b}" for a, b in prefix])


def experiment_d(files):
    """Each declared granularity level against the resolutions used with it.

    Spatial and temporal resolutions are read separately, and a resolution
    declared in degrees is converted before it is compared with a metre
    anchor; treating 0.008333 degree as 0.008333 metre would make every
    arc-second product look like the finest one in the corpus.
    """
    DEGREE_M = 111_320.0
    TEMPORAL_TO_DAYS = {"hour": 1 / 24, "day": 1.0, "month": 30.44,
                        "year": 365.25, "minute": 1 / 1440}
    rows = []
    for p in files:
        t = open(p, encoding="utf-8").read()
        m = re.search(r"(?m)^\s*(?:level|granularity_level)\s*:\s*(L[0-3])", t)
        lv = m.group(1) if m else None

        spatial, temporal, unconverted = [], [], []
        for r in re.finditer(
                r"(?m)^\s*(?:spatial_)?resolution\s*:\s*"          # spatial only when named
                r"([0-9.]+)\s*(?:\*\s*)?(kilometer|km|meter|m\b|degree)?", t):
            v = float(r.group(1))
            u = r.group(2) or "meter"
            if u in ("kilometer", "km"):
                spatial.append(v * 1000.0)
            elif u == "degree":
                spatial.append(v * DEGREE_M)
            else:
                spatial.append(v)
        for r in re.finditer(
                r"(?m)^\s*temporal_resolution\s*:\s*([0-9.]+)\s*(?:\*\s*)?"
                r"(hour|day|month|year|minute)", t):
            temporal.append(float(r.group(1)) * TEMPORAL_TO_DAYS[r.group(2)])

        anchor = LEVELS[lv][1] if lv else None
        finest = min(spatial) if spatial else None
        if lv and finest is not None:
            if finest <= anchor:
                verdict = "consistent"
            elif finest <= anchor * 3:
                verdict = "coarser than the anchor"
            else:
                verdict = "INCONSISTENT"
        elif lv:
            verdict = "level declared, no spatial resolution to check"
        else:
            verdict = "no level declared"
        rows.append(dict(file=os.path.relpath(p, os.path.join(ROOT, "cases")),
                         declared_level=lv, anchor_m=anchor,
                         spatial_m=sorted(set(round(v, 4) for v in spatial)),
                         finest_m=finest,
                         n_spatial=len(spatial),
                         temporal_days=sorted(set(round(v, 2) for v in temporal)),
                         verdict=verdict))
    return rows


def main():
    os.makedirs(OUT, exist_ok=True)
    files = corpus()
    print(f"corpus: {len(files)} files")
    a = experiment_a(files)
    b = experiment_b()
    c = experiment_c()
    d = experiment_d(files)
    print(f"  atoms exercised: {a['n_covered']}/{a['n_atoms']}")
    print(f"  atoms with two grades: {len(b['atoms_with_two_grades'])}")
    print(f"  colliding signatures: {len(c['colliding_signatures'])}")
    print(f"  scenarios declaring a level: "
          f"{sum(1 for r in d if r['declared_level'])}/{len(d)}")

    rep = dict(constructs=a, grades=b, injectivity=c, granularity=d)
    json.dump(rep, open(os.path.join(OUT, "EXPRESSIVENESS.json"), "w",
                        encoding="utf-8"), indent=1)

    L = ["# Language expressiveness and formal-property checks", "",
         "Generated by `tools/ijgis/expressiveness.py`.", ""]

    L += ["## A. Construct and atom coverage", "",
          f"The corpus exercises {a['n_covered']} of the {a['n_atoms']} declared "
          "atoms.", "",
          "| group | atom | grade | slot occurrences |",
          "|---|---|---|---:|"]
    for r in sorted(a["covered"], key=lambda r: -r["hits"]):
        L.append(f"| {r['group']} | `{r['atom']}` | {r['grade']} | {r['hits']} |")
    for r in a["uncovered"]:
        L.append(f"| {r['group']} | `{r['atom']}` | {r['grade']} | 0 |")
    L += ["", "Grammar keywords used:", "",
          "| keyword | occurrences |", "|---|---:|"]
    for k, v in a["keywords"].items():
        L.append(f"| `{k}` | {v} |")
    L.append("")

    L += ["## B. Grade well-formedness", "",
          f"Atoms declared: {b['n_atoms']}; distinct atom names: "
          f"{b['n_distinct_atoms']}; duplicate names: "
          f"{len(b['duplicate_names'])}.", "",
          f"Atoms carrying more than one grade: "
          f"{len(b['atoms_with_two_grades'])}.", "",
          "| group | grades declared |", "|---|---|"]
    for g in b["groups"]:
        gs = sorted({gr for gg, _, gr, _ in ATOMS if gg == g})
        L.append(f"| {g} | {', '.join(gs)} |")
    L.append("")

    L += ["## C. Semantic injectivity", "",
          f"Distinct interface signatures: {c['n_signatures']} for "
          f"{len(ATOMS)} atoms.", "",
          "Signatures shared by more than one atom:", "",
          "| signature | atoms |", "|---|---|"]
    for sig, atoms in sorted(c["colliding_signatures"].items()):
        L.append(f"| `{sig}` | {', '.join(atoms)} |")
    if not c["colliding_signatures"]:
        L.append("| (none) | |")
    L += ["", f"Atom names that are a strict prefix of another atom name: "
          f"{len(c['prefix_pairs'])}.", ""]
    L.append("")

    L += ["## D. Granularity declarations", "",
          "Each level declares a spatial anchor (L0 1000 m, L1 270 m, L2 90 m, "
          "L3 30 m) and a temporal anchor. A scenario is consistent when the "
          "finest resolution it declares is at or below its level's spatial "
          "anchor.", "",
          "| scenario | level | anchor (m) | distinct spatial (m) | finest | temporal (d) | consistency |",
          "|---|---|---:|---|---:|---|---|"]
    for r in d:
        lv = r["declared_level"] or "---"
        an = f"{r['anchor_m']:g}" if r["anchor_m"] else "---"
        rs = ", ".join(f"{v:g}" for v in r["spatial_m"]) or "---"
        fi = f"{r['finest_m']:g}" if r["finest_m"] else "---"
        ts = ", ".join(f"{v:g}" for v in r["temporal_days"]) or "---"
        L.append(f"| {r['file']} | {lv} | {an} | {rs} | {fi} | {ts} | "
                 f"{r['verdict']} |")
    L += ["", "Level codes used in classification namespaces: "
          + (", ".join(f"`{k}` ({v})" for k, v in a["level_codes"].items())
             or "none") + ".", ""]

    open(os.path.join(OUT, "EXPRESSIVENESS.md"), "w",
         encoding="utf-8").write("\n".join(L) + "\n")
    print("\nwrote data/ijgis/EXPRESSIVENESS.json and EXPRESSIVENESS.md")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
