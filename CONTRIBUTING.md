# Contributing to CZ-DSL

Thanks for considering a contribution. This project is intended to grow as a
collaborative, open research infrastructure for coastal-zone data integration,
so contributions of all sizes are welcome — not only code.

## Ways to contribute

| Contribution | Where it goes |
|---|---|
| A new scenario `.cz` for a coastal region | `cases/<RegionName>/` with a README and, where possible, a reproduction script |
| A new plug-in atom or component group | `docs/01_语言规范.md`, `docs/cz-dsl.xsd`, `docs/cz-dsl.schema.json` |
| A new classification binding (CMECS / EDCS / CDB / S-100) | `docs/04_工程化协议.md` plus a worked example |
| Toolchain fixes and tests | `tools/czdsl_tools/` |
| Benchmark pipeline improvements | `tools/nyh_case/` |
| Documentation, tutorials, translations | `docs/` |

## Ground rules

1. **Every number must be reproducible.** If you add a quantitative claim, add
   the script that regenerates it and the SHA-256 of its inputs. No value may be
   hand-entered or tuned to a target.
2. **Scenarios must validate cleanly.** A contributed `.cz` must pass
   `validator.py` with zero errors. Warnings should be justified in the PR.
3. **Cite real data.** State the provider, the version and the licence for every
   source layer. Do not commit third-party data to this repository; commit the
   access script and a manifest instead.
4. **Do not break the corpus.** `verification_tests/` and `cases/` are the
   compatibility corpus; changes to the grammar must keep them parsing.

## Development setup

The toolchain has no third-party dependencies for its core function:

```bash
git clone <repo>
cd cz-dsl
python3 --version        # 3.9 or newer

# smoke test
python3 tools/czdsl_tools/validator.py cases/NewYorkHarbor2050/new_york_harbor_scenario.cz
python3 tools/nyh_case/run_all.py
```

The benchmark pipeline additionally needs `numpy`, `rasterio`, `netCDF4`,
`geopandas`, `pandas` and `scipy`; the case figures need `matplotlib`.

The Rust frontend lives in `src/rust_core/` and builds with `cargo build`.
Build output is not committed.

## Before opening a pull request

```bash
# 1. the compatibility corpus still validates
for f in verification_tests/*.cz cases/*/*.cz; do
  python3 tools/czdsl_tools/validator.py "$f" || echo "FAILED: $f"
done

# 2. the benchmark pipeline still reproduces with no scientific drift
python3 tools/nyh_case/run_all.py

# 3. the paper still builds cleanly (if you touched it)
cd docs/ijgis_submission
pdflatex -interaction=nonstopmode main.tex && pdflatex -interaction=nonstopmode main.tex
```

A pull request is ready when the validator reports zero errors on the corpus,
`run_all.py` reports `scientific drift: none`, and the manuscript builds with
zero errors, zero undefined references and zero overfull hboxes.

## Commit style

Conventional-commit prefixes are used: `feat:`, `fix:`, `docs:`, `data:`,
`fig:`, `test:`, `chore:`. Keep the subject under 72 characters and explain the
*why* in the body.

## Code of conduct

Be civil and assume good faith. Disagreement about technical design is expected
and welcome; personal criticism is not. Report unacceptable behaviour to the
maintainer at the address in `CITATION.cff`.

## Licence of contributions

By contributing you agree that code contributions are licensed under the MIT
License and that documentation, schema, figure and data contributions are
licensed under CC BY 4.0, matching the existing terms in `LICENSE` and
`LICENSE-DOCS`.
