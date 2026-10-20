# Hangzhou Bay 2050 Case Study — Reproducibility Notes

This directory documents the reproducibility status of the Hangzhou Bay 2050
case study reported in Section 6.2 of the IJGIS manuscript.

## What is reproducible from this repository

The CZ-DSL language-level description of the scenario is fully reproducible:

```bash
# Generate the scenario definition (2954 bytes, 120 lines)
python3 tools/czdsl_tools/scene_builder.py \
    --template hangzhou_slr --out /tmp/hangzhou_2050.cz

# Validate the existing Hangzhou Bay database declaration
python3 tools/czdsl_tools/validator.py cases/HangzhouBay2050/hangzhou_bay_db.cz

# Compile the database to four interchange formats
python3 tools/czdsl_tools/compiler.py cases/HangzhouBay2050/hangzhou_bay_db.cz \
    --target geojson --out /tmp/hzb.geojson
python3 tools/czdsl_tools/compiler.py cases/HangzhouBay2050/hangzhou_bay_db.cz \
    --target citygml --out /tmp/hzb.xml
python3 tools/czdsl_tools/compiler.py cases/HangzhouBay2050/hangzhou_bay_db.cz \
    --target pipeline --out /tmp/hzb_pipeline.json
python3 tools/czdsl_tools/compiler.py cases/HangzhouBay2050/hangzhou_bay_db.cz \
    --target report  --out /tmp/hzb_report.txt
```

Expected outputs (current tool version 1.0.0, re-measured):

| Target | Bytes |
|:---|---:|
| geojson | 1534 |
| citygml | 1891 |
| pipeline | 224 |
| report | 1643 |

The validator reports zero diagnostics on `hangzhou_bay_db.cz`.

## What is NOT reproducible from this repository

The quantitative simulation results shown in Figures 4–7 of the manuscript
(peak water level 4.8 m, inundated area ~120 km², 35% high-marsh habitat loss,
adaptation-pathway cost-benefit comparison) are produced by external numerical
solvers (FVCOM for hydrodynamics, SLAMM for marsh migration) driven by boundary
conditions and forcing data that are **not bundled** in this repository.

Specifically, the following artifacts are required to fully reproduce
Figures 4–7 and are documented here as the reproducibility boundary:

- High-resolution bathymetry and DEM/DSM for Hangzhou Bay (source: regional
  marine surveys; not redistributable in this repository).
- Typhoon wind and pressure forcing fields for the 1-in-100-year event.
- FVCOM model executable and unstructured-grid configuration.
- SLAMM model executable and habitat input layers.
- Post-processing scripts that aggregate model output into the inundation,
  habitat-transition, and economic-risk maps.

The repository provides the **scenario definition** (the `.cz` file and the
`coastal_database` declaration that bind the data sources, model adapters,
cross-subject links, and classification layers), but the **numerical execution**
is delegated to the external solvers. Figures 4–7 should therefore be read as
illustrative outputs of the integrated modelling pipeline configured through
CZ-DSL, not as outputs of the CZ-DSL toolchain alone.

## Reproducibility roadmap (future work)

To close this gap, we plan to release:

1. A minimal synthetic bathymetry and forcing dataset under an open licence,
   sufficient to re-run a reduced-resolution FVCOM+SLAMM chain.
2. The post-processing scripts that convert model output into the figure
   formats, together with a `make_figures.py` script analogous to
   `docs/figures/make_fig1.py`.
3. A container image bundling FVCOM and SLAMM with the scenario configuration,
   enabling end-to-end reproduction of Figures 4–7.

Until these are released, the case study figures are reproducible only at the
language/schema level (scenario generation, validation, compilation), not at
the numerical-simulation level.
