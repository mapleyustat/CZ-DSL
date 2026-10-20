# data/schema — language artefacts, not payload data

The `.cz` files in this directory are **language artefacts** used as test
fixtures by the toolchain (see `tools/run_tests.py` and the compatibility
matrix in `docs/*/06_*Tool_Test_Report*` / `06_工具集测试报告.md`). They carry
no scientific payload; data sources are referenced by registry-style paths
only.

| File | Role |
|:---|:---|
| `coastal_model.cz` | coastal complex-system model with bilingual (Chinese/English) annotations — compatibility file F4 in the tool test report |

Everything else that would normally live under `data/` (payload rasters,
grids, derived fields) is deliberately excluded from this repository.
