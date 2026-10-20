# verification_tests

A small corpus of `.cz` files used to check that the language constructs
introduced in the paper parse and validate. Each file exercises a distinct part
of the grammar:

| File | Covers |
|---|---|
| `paper1_2_core.cz` | Core declarations: data sources, spatial reference systems, layers. |
| `paper3_metadata.cz` | Metadata, provenance and classification layers. |
| `paper4_coawst.cz` | Model-adapter declarations for the COAWST/ROMS coupling. |

These files are part of the corpus that `tools/run_tests.py` validates. They
validate with zero errors and are expected to keep doing so; a regression here
means a grammar or validator change broke backward compatibility.
