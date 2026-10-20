# CZ-DSL scenario benchmark - New York Harbor

Single scenario file: `cases/NewYorkHarbor2050/new_york_harbor_scenario.cz`

## Size
- `.cz` file: **22,836 bytes** (686 lines, 78 described objects)
- referenced source payload: 0.652 GB (15 inputs resolved)
- **metadata-to-payload ratio: 1 : 28,554** (the description stores paths and structured metadata, not payload bytes, so this measures catalogue compactness and not data compression)
- for transparency, the whole download tree holds 3.594 GB in 380 files; only the referenced inputs above are consumed by the pipeline
- bytes per described object: 293
- compiled GeoJSON 23,193 B | pipeline 15,841 B | report 16,742 B

## Query efficiency (per operation)

| operation | mean (us) | median (us) | p95 (us) |
|---|---:|---:|---:|
| keyword_lookup | 0.6 | 0.6 | 0.6 |
| indexed_keyword_lookup | 0.0 | 0.0 | 0.0 |
| name_exact | 0.0 | 0.0 | 0.0 |
| name_regex | 8.8 | 8.8 | 9.2 |
| property_value | 0.2 | 0.2 | 0.2 |
| classification_codes | 10.0 | 9.9 | 11.0 |

## Retrieval, reorganisation, analysis
- forward closure from `ROMS_NYOFS_Hydro`: 0.5 us -> ['ConnectivityFlood', 'HabitatExposure', 'PopulationExposure', 'ROMS_NYOFS_Hydro']
- backward impact into `PopulationExposure`: 0.8 us -> ['ConnectivityFlood', 'Gumbel_ExtremeValue', 'HabitatExposure', 'PopulationExposure', 'ROMS_NYOFS_Hydro', 'SHAPEDTWValidation', 'SeaLevelProjection', 'WaveWindForcing']
- regroup + re-emit the whole file: 0.01 ms, 20,889 bytes, 0 validation errors
- dependency analysis: 0.00 ms, 8 nodes, 8 edges, max depth 4

## Scaling

| x | blocks | .cz bytes | parse (ms) | index (ms) | scan query (us) | indexed query (us) |
|---:|---:|---:|---:|---:|---:|---:|
| 1 | 48 | 22,836 | 3.10 | 0.023 | 0.6 | 0.0 |
| 2 | 96 | 45,672 | 6.39 | 0.041 | 1.1 | 0.0 |
| 4 | 192 | 91,344 | 12.05 | 0.073 | 2.1 | 0.0 |
| 8 | 384 | 182,688 | 25.45 | 0.146 | 4.7 | 0.0 |

## Completeness

| attribute | present | of | % |
|---|---:|---:|---:|
| coordinate_ref | 16 | 16 | 100.0 |
| temporal_ref | 16 | 16 | 100.0 |
| resolution | 12 | 16 | 75.0 |
| source_type | 16 | 16 | 100.0 |
| format | 16 | 16 | 100.0 |
| url | 16 | 16 | 100.0 |
| classification | 16 | 16 | 100.0 |
| quality_profile | 16 | 30 | 53.3 |
| uncertainty_model | 16 | 30 | 53.3 |
| cmecs_unit | 16 | 30 | 53.3 |
| spatial_resolution | 10 | 30 | 33.3 |
| cdb_dataset | 14 | 30 | 46.7 |

## Stability
- cz_sha256: `838611b372febc01230d3e3d138725c0a57c9cc3994b47208ef22dd5b5e10723`
- reorganised_sha256: `35f9d8a47ad7781eeaf094b3e983264496a366f3a35a87955b6d1d3670fe288a`
- round_trip_block_names_identical: `True`
- round_trip_properties_identical: `True`
- repeated_query_identical: `True`
- validator_diagnostics_on_source: `0`
- validator_diagnostics_on_reorganised: `0`
- deterministic_emit: `True`
