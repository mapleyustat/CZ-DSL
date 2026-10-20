# cases

CZ-DSL scenario files and their derived artefacts.

| Directory | Contents |
|---|---|
| `NewYorkHarbor2050/` | The benchmark case: one scenario built from heterogeneous real coastal data, plus its compiled artefacts, benchmark record and reproducibility manifest. |
| `GeneralExamples/` | Small, focused single-topic scenarios (CMECS habitat, COAWST workflow, ocean ISO metadata). Used as compatibility and regression inputs. |
| `HangzhouBay2050/` | An earlier regional scenario, retained as a second regional instance. |
| `BalticSea2050/` | Regional validation: sixty years of measured hypoxia against a modelled reanalysis, plus shipping pressure and a relative sea-level trend that is really post-glacial rebound. |
| `SoutheastAsia2050/` | Regional validation: forty years of mangrove extent over four states whose trajectories disagree in sign, plus coral reef geometry. |
| `GreatBarrierReef2050/` | Regional validation: cyclone tracks and coral bleaching stress over the reef, with land and marine datums kept distinct. |
| `YangtzeDelta2050/` | Regional validation: surface-water change and a gauge record whose trend is dominated by subsidence rather than by the ocean. |
| `CycloneIdai2019/` | Regional validation: one cyclone described simultaneously by its track, a modelled flood depth, a satellite flood extent, a distant gauge and the displacement record. |

The five regional cases above were added to test that the description model is
not specific to New York Harbor. Each is built end to end from openly licensed
sources: `tools/regional/fetch.py` retrieves and hashes them,
`tools/regional/build_baltic.py` and `tools/regional/build_regions.py` derive
the numbers, and `tools/regional/emit_cz.py` writes the `.cz` file. Nothing in
those scenarios is typed by hand, so they regenerate from the registry alone.
`data/regional/PROVENANCE.md` records every source with its provider, licence,
byte count and SHA-256, and traces each consumed number to the construct that
uses it.

Every `.cz` file in this directory validates with zero errors. That is enforced by
`tools/run_tests.py`, which validates the whole corpus on every run.

Non-conforming design sketches are deliberately kept out of this directory; see
`sketches/`.
