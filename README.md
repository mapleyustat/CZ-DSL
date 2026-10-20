# CZ-DSL — a coastal-zone description language

CZ-DSL is a domain-specific language for describing **coastal-zone complex
systems**: one declarative file that carries heterogeneous coastal data
together with their datum, geometry, feature attributes, classification,
spatio-temporal processes and governing mechanisms, so that the description —
not the payload — becomes the object that can be validated, queried,
versioned and compiled.

This repository contains the **language and its implementation**: the grammar
and vocabulary specification, the XSD and JSON Schema, a dependency-free
Python toolchain (scene builder, validator, four-target compiler), a Rust
lexer core, the machine-readable vocabulary checks, and the example and
verification corpus. It contains **no scientific payload data and no
manuscript sources**; companion papers and the coastal data corpus are
released separately.

## Repository layout

```
docs/zh/    technical documents, Chinese (authoritative originals)
docs/en/    technical documents, English translations
  01_Language_Specification   grammar, 43 atoms in 7 groups, four-level classification
  02_Design_Principles        the design commitments and their trade-offs
  03_Formal_Verification      the five propositions and their proof sketches
  04_Engineering_Protocol     toolchain contract, validation, benchmark protocol
  06_Tool_Test_Report         functional, compatibility and stability testing
docs/examples/  one scenario in XML and in JSON (schema instances)
schema/     cz-dsl.xsd, cz-dsl.schema.json  (one conceptual model, two encodings)
tools/czdsl_tools/  scene_builder.py · validator.py · compiler.py   (pure Python)
tools/ijgis/       corpus expressiveness + description-layer benchmark scripts
tools/run_tests.py one-command verification of the whole toolchain
cases/      example scenario descriptions (.cz); no payload, paths are registry-style
verification_tests/  synthetic conformance cases, conforming and non-conforming
src/rust_core/  Rust lexer + recursive-descent parser for composite declarations
src/frontend/   small web front-end demo (HTML/JS)
```

## The language in one paragraph

A `.cz` file is a block-structured declarative description. Units are
literals (`-74.3 with unit degree`), classification codes are typed
identifiers (`CZ.CC.YZ-E-01`), constraints are first-class declarations
checked at validation time, and every object carries a seven-position grade
tuple (vertical datum, horizontal datum, geometry, attributes, relations,
temporal reference, governing mechanism) that makes the land–sea join an
explicit, checkable act. The vocabulary is 43 atoms in seven plug-in groups
(DataAccess, ModelAdapter, CrossSubjectLink, LLMDispatch, Granularity,
Classification, DigitalTwin); scenarios are compositions under declared
interface contracts. See `docs/en/01_Language_Specification.md`.

## Quick start

```bash
python3 -m pip install -r requirements.txt      # toolchain itself needs no third-party deps
python3 tools/run_tests.py                      # validate corpus, compile targets, run rust tests
```

```python
import sys; sys.path.insert(0, "tools")
from czdsl_tools.validator import CZValidator
from czdsl_tools.compiler import CZCompiler

diags = CZValidator().validate_file("cases/GeneralExamples/ocean_iso_metadata.cz")
print([d.message for d in diags if d.severity == "error"])   # -> []

CZCompiler().compile_file("cases/GeneralExamples/ocean_iso_metadata.cz", "geojson")
```

## Verification status

`python3 tools/run_tests.py` on a clean clone (Python 3.12, cargo optional):
all shipped `.cz` files validate with zero errors; the benchmark scenario
compiles to all four targets; the Rust core passes 72 unit tests. Checks that
require external payload report `SKIP` rather than failing.

## What this repository deliberately does not contain

* **Payload data.** Descriptions reference sources by registry-style paths;
  the coastal data corpus (CZ-Coastal-16) is a separate, citable release.
* **Manuscript sources and figures.** The companion language-and-systems
  paper (IJGIS), the scenario-integration paper (ISPRS) and the data paper
  (ESSD) are under review; their sources are not published here.
* **Compiled binaries.** Everything is built from source.

## License

Software: MIT (`LICENSE`). Documentation: CC BY 4.0 (`LICENSE-DOCS`).

## Citation

See `CITATION.cff`.
