# Toolchain

* `czdsl_tools/scene_builder.py` — emit a description from a source table or template.
* `czdsl_tools/validator.py`     — three passes over the text; named diagnostics
  (SEMANTIC, UNIT, DATA001–003, CLASS001); text or JSON reports.
* `czdsl_tools/compiler.py`      — four lowerings of one description:
  GeoJSON, CityGML-style XML, JSON pipeline descriptor, human-readable report.
* `ijgis/expressiveness.py`      — vocabulary coverage check over the case corpus.
* `ijgis/iso_benchmark.py`       — description-layer cost measurements on a shipped case.
* `run_tests.py`                 — one-command verification (environment, corpus
  validation, compilation, Rust tests, benchmark). Payload-dependent checks SKIP.

The Python toolchain uses the standard library only.
