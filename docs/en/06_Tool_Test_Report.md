# CZ-DSL Toolchain Compatibility and Stability Test Report

> Test date: 2026-08-14
> Tool version under test: czdsl_tools 1.0.0
> Execution environment: macOS / Python 3.8+

> *This file is a faithful English translation of `docs/zh/06_工具集测试报告.md` (the authoritative original).*

---

## 1 Test environment

| Item | Value |
|:---|:---|
| Operating system | macOS (Darwin) |
| Python version | Python 3.8+ |
| Dependencies | Python standard library only (json, argparse, re, dataclasses, xml.sax.saxutils) |
| Tool path | `tools/czdsl_tools/` |
| Tools under test | scene_builder.py, validator.py, compiler.py |

## 2 Tool versions under test

| Tool | File | Version | Function |
|:---|:---|:---:|:---|
| Scene builder | scene_builder.py | 2.0.0 | Constructs CZ-DSL scenarios programmatically; emits .cz / XML / JSON |
| Content validator | validator.py | 2.0.0 | Parses .cz files and performs syntax, semantic and data-adaptation checks |
| System compiler | compiler.py | 2.0.0 | Compiles .cz scenarios to GeoJSON / CityGML / pipeline / report |

## 3 Test method

### 3.1 Functional testing

For each tool, real .cz files from `cases/` and `verification_tests/` are used as input; the CLI command is executed and the exit code, output and diagnostics are recorded. Criteria:

- **Pass**: the tool runs normally with exit code 0 or 1 (exit code 1 when the validator finds errors is normal), with no crash or exception.
- **Fail**: the tool crashes, raises an unhandled exception, or exits with code 2.

### 3.2 Compatibility testing

All three tools are run over the following six .cz files to verify backward compatibility:

| ID | File path | Description |
|:---|:---|:---|
| F1 | `verification_tests/paper1_2_core.cz` | Core: coastal reference system, typhoon classification, storm-surge scenario |
| F2 | `verification_tests/paper3_metadata.cz` | Metadata: ISO 19115 / ACDD / CF conventions |
| F3 | `verification_tests/paper4_coawst.cz` | COAWST coupled-model workflow |
| F4 | `data/schema/coastal_model.cz` | Coastal complex-system model (bilingual annotations) |
| F5 | `cases/GeneralExamples/cmecs_habitat.cz` | CMECS ecological-unit database |
| F6 | `cases/HangzhouBay2050/hangzhou_bay_db.cz` | Hangzhou Bay CDB-style database |

### 3.3 Stability testing

Each tool is run ten times on a fixed input; per-run wall time is recorded with `time.perf_counter()` and the mean, minimum, maximum and standard deviation are computed.

---

## 4 Functional test matrix

### 4.1 Scene builder (scene_builder.py)

| Case | Command | Input | Expected | Actual | Pass/Fail |
|:---|:---|:---|:---|:---|:---:|
| SB-01 | `python3 scene_builder.py --template hangzhou_slr --out /tmp/hz_slr.cz` | hangzhou_slr template | emits a .cz file containing atomic-construct keywords | .cz file of 2,884 bytes, 120 lines, with data_source/model_adapter etc. | ✅ Pass |
| SB-02 | `python3 scene_builder.py --template yangtze_typhoon --out /tmp/yt.xml` | yangtze_typhoon template | emits an XML file | XML file of 5,929 bytes, 158 lines | ✅ Pass |
| SB-03 | `python3 scene_builder.py --template wetland_restoration --out /tmp/wt.json` | wetland_restoration template | emits a JSON file | JSON file of 4,715 bytes, 196 lines | ✅ Pass |
| SB-04 | Programmatic API `SceneBuilder.from_template("hangzhou_slr").to_cz()` | — | returns a non-empty string | .cz text containing atomic-construct keywords | ✅ Pass |

### 4.2 Content validator (validator.py)

| Case | Input file | Expected | Actual | Pass/Fail |
|:---|:---|:---|:---|:---:|
| VAL-01 | `paper1_2_core.cz` | parses; exit code 0 or 1 | 0 diagnostics, exit 0 | ✅ Pass |
| VAL-02 | `paper3_metadata.cz` | parses | 1 info (scenario lacks spatial reference), exit 0 | ✅ Pass |
| VAL-03 | `paper4_coawst.cz` | parses | 1 warning (undefined reference coarse_grid), 1 info, exit 0 | ✅ Pass |
| VAL-04 | `coastal_model.cz` | parses | 1 info (scenario lacks spatial reference), exit 0 | ✅ Pass |
| VAL-05 | `cmecs_habitat.cz` | parses | 0 diagnostics, exit 0 | ✅ Pass |
| VAL-06 | `hangzhou_bay_db.cz` | parses | 0 diagnostics, exit 0 | ✅ Pass |
| VAL-07 | `paper1_2_core.cz --format json` | emits a JSON report | valid JSON with summary and diagnostics fields | ✅ Pass |

### 4.3 System compiler (compiler.py)

| Case | Input file | Target | Expected | Actual | Pass/Fail |
|:---|:---|:---:|:---|:---|:---:|
| CMP-01 | `paper1_2_core.cz` | geojson | GeoJSON FeatureCollection | 5,684 B, 190 lines | ✅ Pass |
| CMP-02 | `paper1_2_core.cz` | citygml | CityGML-style XML | 6,447 B, 125 lines | ✅ Pass |
| CMP-03 | `paper1_2_core.cz` | pipeline | JSON pipeline descriptor | 1,513 B, 46 lines | ✅ Pass |
| CMP-04 | `paper1_2_core.cz` | report | human-readable report | 4,425 B, 134 lines | ✅ Pass |
| CMP-05 | `paper3_metadata.cz` | geojson | GeoJSON | 6,768 B | ✅ Pass |
| CMP-06 | `paper4_coawst.cz` | pipeline | pipeline descriptor | 4,695 B | ✅ Pass |
| CMP-07 | `hangzhou_bay_db.cz` | citygml | CityGML XML | 1,833 B | ✅ Pass |
| CMP-08 | `cmecs_habitat.cz` | report | report | 1,421 B | ✅ Pass |

---

## 5 Compatibility testing

### 5.1 Three-tool × six-file compatibility matrix

| File | scene_builder | validator | compiler (geojson) | compiler (citygml) | compiler (pipeline) | compiler (report) |
|:---|:---:|:---:|:---:|:---:|:---:|:---:|
| paper1_2_core.cz | N/A | ✅ Pass | ✅ Pass | ✅ Pass | ✅ Pass | ✅ Pass |
| paper3_metadata.cz | N/A | ✅ Pass | ✅ Pass | ✅ Pass | ✅ Pass | ✅ Pass |
| paper4_coawst.cz | N/A | ✅ Pass | ✅ Pass | ✅ Pass | ✅ Pass | ✅ Pass |
| coastal_model.cz | N/A | ✅ Pass | ✅ Pass | ✅ Pass | ✅ Pass | ✅ Pass |
| cmecs_habitat.cz | N/A | ✅ Pass | ✅ Pass | ✅ Pass | ✅ Pass | ✅ Pass |
| hangzhou_bay_db.cz | N/A | ✅ Pass | ✅ Pass | ✅ Pass | ✅ Pass | ✅ Pass |

> Note: scene_builder does not accept .cz input (it generates .cz files), hence N/A; its functional cases are in §4.1.

### 5.2 Compatibility analysis

All six test files are processed normally by the validator and the compiler, with no crash or abnormal exit. The validator correctly recognises the v1-syntax constructs (`spatial_reference_system`, `classification`, `scenario`, `rule`, `infrastructure`, `layer`, `policy_package`, `coastal_database`, `data`, `process`, `model`, `workflow`, `ai_model`, `digital_twin`, `sphere`) as known top-level keywords and produces no false positives.

The validator produces reasonable diagnostics (not crashes) in the following cases:

| File | Diagnostics | Severity | Description |
|:---|:---:|:---|:---|
| paper3_metadata.cz | 1 | info | scenario lacks a spatial-reference declaration |
| paper4_coawst.cz | 2 | warning + info | reference to undefined identifier coarse_grid; scenario lacks spatial reference |
| coastal_model.cz | 1 | info | scenario lacks a spatial-reference declaration |
| paper1_2_core.cz | 0 | — | no diagnostics |
| cmecs_habitat.cz | 0 | — | no diagnostics |
| hangzhou_bay_db.cz | 0 | — | no diagnostics |

### 5.3 Compiler output sizes

> Note: the byte counts below were re-measured with the current tool version (reproduction check, 2026-05) and supersede the initial report where they differ; this table is authoritative.

| File | geojson | citygml | pipeline | report |
|:---|:---:|:---:|:---:|:---:|
| paper1_2_core.cz | 5,678 B | 6,447 B | 1,507 B | 4,637 B |
| paper3_metadata.cz | 6,762 B | 8,685 B | 3,394 B | 5,542 B |
| paper4_coawst.cz | 7,712 B | 9,066 B | 4,689 B | 6,379 B |
| coastal_model.cz | 5,208 B | 6,310 B | 1,949 B | 4,520 B |
| cmecs_habitat.cz | 1,362 B | 1,649 B | 212 B | 1,505 B |
| hangzhou_bay_db.cz | 1,534 B | 1,891 B | 224 B | 1,643 B |

---

## 6 Stability testing

### 6.1 Scene builder (10 repeated runs)

| Run | Time (s) |
|:---:|:---:|
| 1 | 0.0719 |
| 2 | 0.0474 |
| 3 | 0.0477 |
| 4 | 0.0474 |
| 5 | 0.0470 |
| 6 | 0.0469 |
| 7 | 0.0461 |
| 8 | 0.0465 |
| 9 | 0.0464 |
| 10 | 0.0467 |

| Statistic | Value |
|:---|:---:|
| Mean | 0.0494 s |
| Min | 0.0461 s |
| Max | 0.0719 s |
| Std dev | 0.0079 s |

> The first run is slightly slower due to Python interpreter cold start; subsequent runs are stable within 0.046–0.048 s.

### 6.2 Content validator (10 repeated runs)

| Input file | Mean (s) | Min (s) | Max (s) | Std dev (s) |
|:---|:---:|:---:|:---:|:---:|
| paper1_2_core.cz | 0.0307 | 0.0297 | 0.0332 | 0.0010 |
| paper3_metadata.cz | 0.0351 | 0.0337 | 0.0369 | 0.0011 |
| paper4_coawst.cz | 0.0338 | 0.0323 | 0.0360 | 0.0014 |
| coastal_model.cz | 0.0308 | 0.0302 | 0.0313 | 0.0004 |
| cmecs_habitat.cz | 0.0297 | 0.0288 | 0.0315 | 0.0008 |
| hangzhou_bay_db.cz | 0.0297 | 0.0293 | 0.0299 | 0.0002 |

### 6.3 System compiler (10 repeated runs, target=geojson)

| Input file | Mean (s) | Min (s) | Max (s) | Std dev (s) |
|:---|:---:|:---:|:---:|:---:|
| paper1_2_core.cz | 0.0399 | 0.0391 | 0.0404 | 0.0005 |
| paper3_metadata.cz | 0.0413 | 0.0403 | 0.0434 | 0.0009 |
| paper4_coawst.cz | 0.0413 | 0.0405 | 0.0421 | 0.0005 |
| coastal_model.cz | 0.0406 | 0.0396 | 0.0426 | 0.0009 |
| cmecs_habitat.cz | 0.0399 | 0.0391 | 0.0426 | 0.0010 |
| hangzhou_bay_db.cz | 0.0397 | 0.0389 | 0.0408 | 0.0005 |

### 6.4 Stability analysis

- No failures in any of the ten repeated runs; outputs identical across runs.
- Standard deviations are all below 0.002 s (excluding cold start), indicating highly stable execution times.
- Mean times: validator 0.030–0.035 s, compiler 0.040–0.041 s, scene builder 0.049 s (including file writing).
- Times correlate mildly with file size but the spread is small (smallest file cmecs_habitat.cz 0.0297 s vs largest paper4_coawst.cz 0.0338 s), showing that the dominant cost is Python interpreter startup rather than file processing.

---

## 7 Defects and limitations

### 7.1 Known limitations

| ID | Tool | Limitation | Impact |
|:---|:---|:---|:---:|
| L-01 | validator | the line-level parser does not support cross-line expressions (e.g. multi-line string concatenation) and may miss some semantics | low |
| L-02 | validator | unit checking is basic: warnings are raised only for simple identifier units; compound units (e.g. m^3/s) are not validated | low |
| L-03 | validator | reference checking cannot distinguish type names from instance names; identifiers starting with an upper-case letter are treated as type names and not warned as undefined | low |
| L-04 | compiler | GeoJSON geometry relies on regex coordinate extraction from text; some files may yield no geometry | medium |
| L-05 | compiler | CityGML output is a style stub; the full CityGML 3.0 Application Schema is not yet included | medium |
| L-06 | compiler | the pipeline descriptor is a custom format, not yet mapped to a concrete coupling-framework standard (OMS, CSDMS) | low |
| L-07 | scene_builder | templates are hard-coded; reverse import from an existing .cz file into a template is not supported | low |

### 7.2 Defects not found

Within the scope of this test, no defect was found that crashes a tool or produces wrong output. All tools behaved correctly on the six test files.

---

## 8 Improvement proposals

| Priority | Proposal | Tool |
|:---:|:---|:---|
| High | add a CRS declaration field to the GeoJSON output to improve interoperability with GIS software | compiler |
| High | extend the validator's cross-line expression parsing: multi-line strings and nested structures | validator |
| Medium | complete the CityGML Application Schema with LOD 0–4 | compiler |
| Medium | map the pipeline output to the CSDMS BMI interface standard | compiler |
| Medium | add reverse .cz import to the scene builder, generating editable templates from existing files | scene_builder |
| Low | add a SEDRIS/CDB compatibility-check mode to the validator | validator |
| Low | add a --verbose debug option to all tools | all |

---

## 9 Test conclusion

This report covers functional, compatibility and stability testing of the three CZ-DSL tools (scene builder, content validator, system compiler). The results show:

1. **Functional completeness**: all three tools work as designed — the scene builder generates .cz/XML/JSON files containing atomic-construct keywords; the validator correctly identifies syntax and semantic issues; the compiler converts .cz files into the four target formats.

2. **Backward compatibility**: all six test files (covering every major v1 construct family) are processed normally by the validator and the compiler, with no crashes or abnormalities, meeting the backward-compatibility design goal.

3. **Runtime stability**: results are identical across ten repeated runs, with standard deviations below 0.002 s (excluding cold start); the tools are stable and reliable at the current test scale.

4. **Performance**: single-file processing takes 0.03–0.05 s, which is adequate for interactive use.

**Overall verdict: the toolchain passes every test item and is fit for delivery.**

---

## Appendix A: Test file inventory

| File path | Lines | Size | Main construct types |
|:---|:---:|:---:|:---|
| verification_tests/paper1_2_core.cz | 162 | ~5.5 KB | spatial_reference_system, classification, scenario, rule, infrastructure, layer, policy_package |
| verification_tests/paper3_metadata.cz | 332 | ~9.0 KB | data, process, workflow, scenario |
| verification_tests/paper4_coawst.cz | 385 | ~11 KB | data, process, workflow, model, digital_twin, scenario |
| data/schema/coastal_model.cz | 222 | ~7.0 KB | sphere, process, data, ai_model, digital_twin, model, scenario, fn |
| cases/GeneralExamples/cmecs_habitat.cz | 48 | ~1.5 KB | coastal_database |
| cases/HangzhouBay2050/hangzhou_bay_db.cz | 68 | ~2.0 KB | coastal_database |

## Appendix B: Validation of scene-builder output

The .cz files emitted by the scene builder were validated by the validator and confirmed to use the following atomic-construct keywords:

- `data_source` — data source declaration
- `model_adapter` — model adapter declaration
- `cross_subject_link` — cross-subject link declaration
- `llm_dispatch` — LLM dispatch declaration
- `granularity` — granularity control declaration
- `classification_layer` — classification layer declaration
- `digital_twin_mapping` — digital twin mapping declaration

The generated files pass validation with no syntax errors.

---

*End of report*
