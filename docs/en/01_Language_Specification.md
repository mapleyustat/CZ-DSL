# CZ-DSL Language Specification

> Domain description language for coastal-zone complex systems (Coastal Zone Domain-Specific Language, Version 2)
> Formal language specification document

> *This file is a faithful English translation of `docs/zh/01_语言规范.md` (the authoritative original).*

---

## Table of contents

1. [Overview and design goals](#1-overview-and-design-goals)
2. [Seven fundamental component categories](#2-seven-fundamental-component-categories)
3. [Four-level hierarchical classification system](#3-four-level-hierarchical-classification-system)
4. [Minimal atomic grammar (EBNF)](#4-minimal-atomic-grammar-ebnf)
5. [Keyword and unit tables](#5-keyword-and-unit-tables)
6. [Compatibility mapping with existing .cz examples](#6-compatibility-mapping-with-existing-cz-examples)

---

## 1. Overview and design goals

### 1.1 Positioning

CZ-DSL is a domain description language for coastal-zone complex systems. A coastal-zone complex system is an open mega-system coupling the atmosphere, hydrosphere, lithosphere, biosphere, and anthroposphere, characterized by high nonlinearity, multi-scalarity, and cross-subject interaction. CZ-DSL aims to provide such systems with a unified formal description framework.

CZ-DSL simultaneously fulfills three language roles:

| Role ID | Language role | Functional positioning | Primary users |
|:---:|:---|:---|:---|
| R1 | **Representational Language** | Provides domain experts with precise means to describe coastal-zone system structure and processes, supporting cross-disciplinary semantic alignment | Oceanographers, ecologists, planners |
| R2 | **Computational Language** | Bridges numerical simulation models and artificial-intelligence models, providing executable model invocation and data-flow orchestration | Model developers, computational scientists |
| R3 | **Business Process Hub Language** | Serves as the hub of business chains across government departments, research institutions, and industrial subjects, supporting multi-subject collaboration | Managers and decision-makers, business coordinators |

### 1.2 Design goals

| Goal ID | Design goal | Language role(s) | Verification criterion |
|:---:|:---|:---:|:---:|
| G1 | Semantic completeness: covers all core business scenarios of coastal-zone complex systems | R1 | Completeness theorem (see the formal verification document) |
| G2 | Atomic indivisibility: every semantic unit carries a single semantic; complex scenarios are built by composing atoms | R1, R2 | Atomicity rules |
| G3 | Spatio-temporal reference independence: natively supports arbitrary resolutions, coordinate systems, and reference frames | R1, R2 | Coordinate-independence theorem |
| G4 | Reversibility: a digital description can be reverse-restored into the original business-scenario information | R1, R3 | Reversibility theorem |
| G5 | Composability: components compose through standardized semantic interfaces | R2, R3 | Composability theorem |
| G6 | Backward compatibility: the atom grammar remains compatible with existing .cz constructs | R1, R2 | Compatibility mapping table |
| G7 | Industry generalizability: extension mechanisms leave existing semantics unchanged | R1, R2, R3 | Extension homomorphism theorem |

### 1.3 Version evolution overview

CZ-DSL already has a working lexer that supports bilingual Chinese–English keywords, physical units as first-class citizens, and geometric-algebra types (Multivector/Scalar/Vector/Bivector/Trivector). Building on the existing implementation, this version introduces the seven fundamental component categories (atomic components) and the four-level hierarchical classification system, reorganizing the existing scattered `sphere`/`process`/`model`/`data`/`ai_model`/`digital_twin`/`coastal_database`/`scenario`/`rule`/`infrastructure`/`policy_package`/`spatial_reference_system`/`classification`/`workflow` constructs into a standardized atomized semantic system.

---

## 2. Seven fundamental component categories

The semantic system of CZ-DSL is supported by seven fundamental component categories. Each category consists of a number of atomic components; each atom carries a single semantic and composes through standardized semantic interface contracts.

### 2.1 Data access unit (DataAccessAtom)

#### 2.1.1 Semantic definition

The data access unit is responsible for the unified description of, and access to, multi-source spatio-temporal data and thematic-attribute data of the coastal zone. Its core design idea is to natively support, at the semantic substrate, data at any spatio-temporal resolution, in any spatial coordinate system, and under any spatio-temporal reference framework, eliminating information fragmentation at the data-description layer.

#### 2.1.2 Atom inventory

| Atom | Semantic role | Indivisible semantics | Interface contract |
|:---|:---|:---|:---|
| `DataSource` | Data-source identity | Identifies an independent physical or logical dataset | `identity() -> URI; schema() -> SchemaRef` |
| `CoordinateReference` | Spatial coordinate reference | Describes the spatial coordinate reference system of the data | `to_srs(target: SRS) -> Transform; datum() -> Datum` |
| `TemporalReference` | Temporal reference | Describes the temporal reference system and time zone of the data | `to_epoch() -> UnixTimestamp; calendar() -> Calendar` |
| `ResolutionSpec` | Resolution specification | Describes the spatial and temporal resolution of the data | `spatial_res() -> Length; temporal_res() -> Duration` |
| `QualityProfile` | Quality profile | Describes the data quality grade and quality-control flags | `qc_level() -> QCLevel; flags() -> Set<QCFlag>` |
| `UncertaintyModel` | Uncertainty model | Describes the mathematical model of the data's uncertainty | `distribution() -> Distribution; ci(level: Real) -> Range` |
| `AccessEndpoint` | Access endpoint | Describes the protocol and address for accessing the data | `protocol() -> Protocol; endpoint() -> URI` |

#### 2.1.3 Syntax form

```
data_source <identifier> {
    source: DataSource { ... },
    coordinate_ref: CoordinateReference { ... },
    temporal_ref: TemporalReference { ... },
    resolution: ResolutionSpec { ... },
    quality: QualityProfile { ... },
    uncertainty: UncertaintyModel { ... },
    endpoint: AccessEndpoint { ... }
}
```

#### 2.1.4 Mapping to existing .cz constructs

| Existing construct | Atom combination | Mapping notes |
|:---|:---|:---|
| `coastal_database` | `DataSource` + `CoordinateReference` + `TemporalReference` + `ResolutionSpec` | The database as a whole decomposes into a data access unit |
| `dataset` | `DataSource` + `ResolutionSpec` + `QualityProfile` + `UncertaintyModel` | The dataset is atomized |
| `layer` | `DataSource` + `CoordinateReference` | A layer maps to a data source with a coordinate reference |
| `spatial_reference_system` | `CoordinateReference` + `TemporalReference` | An SRS splits into spatial and temporal reference atoms |

---

### 2.2 Model adapter unit (ModelAdapterAtom)

#### 2.2.1 Semantic definition

The model adapter unit bridges numerical simulation models (e.g. ROMS, FVCOM, SWAN), machine-learning models (e.g. neural-network predictors), and business-process models (e.g. workflow engines), providing a unified model interface contract and invocation-binding mechanism.

#### 2.2.2 Atom inventory

| Atom | Semantic role | Indivisible semantics | Interface contract |
|:---|:---|:---|:---|
| `ModelInterface` | Model interface | Describes the input/output interface signature of a model | `inputs() -> [TypeExpr]; outputs() -> [TypeExpr]` |
| `ModelCapability` | Model capability | Describes the types of computational capability the model can provide | `capabilities() -> Set<Capability>; domain() -> Domain` |
| `ModelIOContract` | Input/output contract | Constrains the types and units of model inputs and outputs | `validate(input: Map) -> Result; output_schema() -> Schema` |
| `ModelVersion` | Model version | Identifies the concrete version and dependencies of a model | `version() -> SemVer; dependencies() -> [ModelRef]` |
| `ModelDeployment` | Model deployment | Describes the runtime environment and resource requirements of a model | `runtime() -> RuntimeSpec; resources() -> ResourceSpec` |
| `ModelInvokeBinding` | Invocation binding | Binds a model to a specific scenario and data | `bind(data: DataAccessAtom, scenario: ScenarioRef) -> BoundModel` |

#### 2.2.3 Syntax form

```
model_adapter <identifier> {
    interface: ModelInterface { ... },
    capability: ModelCapability { ... },
    io_contract: ModelIOContract { ... },
    version: ModelVersion { ... },
    deployment: ModelDeployment { ... },
    invoke_binding: ModelInvokeBinding { ... }
}
```

#### 2.2.4 Mapping to existing .cz constructs

| Existing construct | Atom combination | Mapping notes |
|:---|:---|:---|
| `model` | `ModelInterface` + `ModelCapability` + `ModelIOContract` | A numerical model decomposes into interface, capability, and contract |
| `ai_model` | `ModelInterface` + `ModelCapability` + `ModelVersion` | An AI model gains version and capability descriptions |
| `process` | `ModelInterface` + `ModelIOContract` | A process maps to interface and contract atoms |
| `workflow` | `ModelInvokeBinding` + `ModelDeployment` | A workflow maps to invocation binding and deployment |
| `recommend_model()` | `ModelCapability` + `ModelInvokeBinding` | Model recommendation maps to capability matching and binding |

---

### 2.3 Cross-subject link engine (CrossSubjectLinkEngine)

#### 2.3.1 Semantic definition

The cross-subject link engine establishes semantic links across government departments (e.g. the Ministry of Natural Resources, the Ministry of Ecology and Environment, the Ministry of Transport), research institutions (e.g. marine research institutes, university laboratories), and industrial subjects (e.g. port groups, fishery enterprises). It adopts SEDRIS EDCS (Environmental Data Coding Specification) and CMECS (Coastal and Marine Ecological Classification Standard) as classification anchors to ensure cross-subject semantic alignment.

#### 2.3.2 Atom inventory

| Atom | Semantic role | Indivisible semantics | Interface contract |
|:---|:---|:---|:---|
| `Subject` | Acting subject | Identifies an organization or individual participating in coastal-zone management | `id() -> SubjectID; type() -> SubjectType` |
| `Role` | Role | Describes the subject's role in a business chain | `permissions() -> Set<Permission>; scope() -> Scope` |
| `Jurisdiction` | Jurisdiction | Describes the subject's administrative and spatial jurisdictional boundaries | `boundary() -> Polygon; level() -> AdminLevel` |
| `SemanticLink` | Semantic link | Establishes a semantic link relation between subjects | `source() -> SubjectRef; target() -> SubjectRef; type() -> LinkType` |
| `Provenance` | Provenance | Records the provenance chain of data and decisions | `origin() -> SubjectRef; timestamp() -> DateTime; chain() -> [Provenance]` |
| `Authorization` | Authorization | Describes data and operation authorizations between subjects | `granted_by() -> SubjectRef; granted_to() -> SubjectRef; scope() -> AuthScope` |
| `Responsibility` | Responsibility | Describes the subject's duty in a specific business context | `duty() -> Duty; accountable_to() -> SubjectRef` |

#### 2.3.3 Classification anchors

The cross-subject link engine uses SEDRIS EDCS and CMECS as its standard classification anchors:

- **EDCS anchor**: classifies environmental objects uniformly through the `ECC_*` code system (Environmental Classification Codes), e.g. `ECC_TYPHOON`, `ECC_SALT_MARSH`, `ECC_RIVER_MOUTH`.
- **CMECS anchor**: classifies coastal-zone ecological units in a standardized way through the four CMECS components (Biotic, Geoform, Substrate, WaterColumn), corresponding to the `CmecsComponent` enum in the existing `cmecs.rs`.

#### 2.3.4 Syntax form

```
cross_subject_link <identifier> {
    subject: Subject { ... },
    role: Role { ... },
    jurisdiction: Jurisdiction { ... },
    semantic_links: [SemanticLink { ... }],
    provenance: Provenance { ... },
    authorization: Authorization { ... },
    responsibility: Responsibility { ... }
}
```

#### 2.3.5 Mapping to existing .cz constructs

| Existing construct | Atom combination | Mapping notes |
|:---|:---|:---|
| `classification` (EDCS) | `Subject` + `SemanticLink` | An EDCS classification maps to a semantic-link anchor |
| `classification` (CMECS) | `SemanticLink` + `Jurisdiction` | A CMECS classification maps to a jurisdiction link |

---

### 2.4 LLM dispatch interface (LLMDispatchInterface)

#### 2.4.1 Semantic definition

The LLM dispatch interface connects the logical-reasoning capability of general artificial intelligence (AGI) with the spatial-cognition capability of geospatial large models, providing atoms for task definition, prompt contract, output binding, grounding constraint, and hallucination guarding.

#### 2.4.2 Atom inventory

| Atom | Semantic role | Indivisible semantics | Interface contract |
|:---|:---|:---|:---|
| `LLMTask` | Task definition | Defines a reasoning or cognition task for an LLM | `input() -> PromptData; expected() -> OutputSchema` |
| `PromptContract` | Prompt contract | Constrains the structure, variables, and format of a prompt | `template() -> PromptTemplate; variables() -> Map<Var, Type>` |
| `SpatialCognitionCall` | Spatial-cognition call | Invokes a geospatial large model for spatial reasoning | `spatial_query() -> SpatialQuery; model() -> GeoLLMRef` |
| `LogicInferenceCall` | Logical-inference call | Invokes a general large model for logical reasoning | `premises() -> [Proposition]; query() -> InferenceQuery` |
| `LLMOutputBinding` | Output binding | Binds LLM output to domain semantics | `parse(raw: Text) -> DomainValue; schema() -> OutputSchema` |
| `GroundingConstraint` | Grounding constraint | Constrains LLM output to be grounded in factual data | `grounding_source() -> DataAccessAtom; tolerance() -> Real` |
| `HallucinationGuard` | Hallucination guard | Detects and blocks hallucinated LLM output | `validate(output: DomainValue) -> GuardResult; threshold() -> Real` |

#### 2.4.3 Syntax form

```
llm_dispatch <identifier> {
    task: LLMTask { ... },
    prompt: PromptContract { ... },
    spatial_cognition: SpatialCognitionCall { ... },
    logic_inference: LogicInferenceCall { ... },
    output_binding: LLMOutputBinding { ... },
    grounding: GroundingConstraint { ... },
    hallucination_guard: HallucinationGuard { ... }
}
```

#### 2.4.4 Mapping to existing .cz constructs

| Existing construct | Atom combination | Mapping notes |
|:---|:---|:---|
| `ai_model` (inference part) | `LLMTask` + `PromptContract` + `LLMOutputBinding` | AI-model inference maps to LLM dispatch |
| `rule` | `LogicInferenceCall` + `GroundingConstraint` | Rule-based inference maps to a logic-inference call plus a grounding constraint |

---

### 2.5 Granularity partition engine (GranularityPartitionEngine)

#### 2.5.1 Semantic definition

The granularity partition engine partitions the coastal-zone system into multi-granularity levels according to business characteristics such as territorial spatial planning, ecological protection and restoration, and disaster emergency response, and supports aggregation and disaggregation operations.

#### 2.5.2 Atom inventory

| Atom | Semantic role | Indivisible semantics | Interface contract |
|:---|:---|:---|:---|
| `GranularityLevel` | Granularity level | Defines a discrete granularity level | `level() -> UInt; scale_range() -> (Length, Length)` |
| `PartitionRule` | Partition rule | Defines the criterion for partitioning a parent level into child levels | `apply(parent: Unit) -> [Unit]; criterion() -> Criterion` |
| `AggregationOperator` | Aggregation operator | Defines the aggregation operation from child levels to the parent level | `aggregate(children: [Unit]) -> Unit; method() -> AggMethod` |
| `DisaggregationOperator` | Disaggregation operator | Defines the disaggregation operation from the parent level to child levels | `disaggregate(parent: Unit) -> [Unit]; method() -> DisaggMethod` |
| `BoundaryRule` | Boundary rule | Defines the criterion for determining the boundary of a granularity unit | `resolve(conflict: Boundary) -> Boundary; priority() -> UInt` |

#### 2.5.3 Syntax form

```
granularity <identifier> {
    levels: [GranularityLevel { ... }],
    partition_rules: [PartitionRule { ... }],
    aggregation: AggregationOperator { ... },
    disaggregation: DisaggregationOperator { ... },
    boundary_rules: [BoundaryRule { ... }]
}
```

#### 2.5.4 Mapping to existing .cz constructs

| Existing construct | Atom combination | Mapping notes |
|:---|:---|:---|
| `scale` | `GranularityLevel` + `PartitionRule` | A scale declaration maps to a granularity level and partition rules |
| `model.coupling` (Upscaling/Downscaling) | `AggregationOperator` + `DisaggregationOperator` | Cross-scale coupling maps to aggregation/disaggregation operators |

---

### 2.6 Layered classification manager (LayeredClassificationManager)

#### 2.6.1 Semantic definition

The layered classification manager builds a "WholeDomain — CoastalCell — CorePatch — BaseElement" four-level hierarchical classification system. Every level has a well-defined definition, discrimination criteria, and coding rule, and is aligned with the CMECS/EDCS standards.

#### 2.6.2 Definition of the four levels

**Level 1: Whole Domain (WholeDomain)**

- Definition: the maximum spatial-extent unit covering the complete coastal-zone study area.
- Discrimination criteria: its spatial extent covers the entire study-area boundary and does not overlap any unit at the same level.
- Coding rule: `CZ.WD.<region code>.<serial>`
- CMECS/EDCS alignment: corresponds to top-level EDCS environmental classifications (e.g. `EDCS_ESTUARY`, `EDCS_COASTAL_ZONE`).
- Code example: `CZ.WD.ECS.01` (East China Sea shelf, whole-domain unit 01)

**Level 2: Coastal Cell (CoastalCell)**

- Definition: a secondary functional unit within the whole domain, delineated by landform–hydrodynamic–ecological characteristics.
- Discrimination criteria: it has a relatively independent hydrodynamic circulation or ecological-process boundary, with an area on the order of 10²—10⁴ km².
- Coding rule: `CZ.CC.<landform code>-<ecology code>-<serial>`
- CMECS/EDCS alignment: corresponds to the CMECS Geoform component-level classification + EDCS sub-classification.
- Code example: `CZ.CC.YZ-E-01` (Yangtze Estuary – estuarine ecology – unit 01)

**Level 3: Core Patch (CorePatch)**

- Definition: a homogeneous parcel within a coastal cell with a specific ecological function or human-activity characteristic.
- Discrimination criteria: the patch interior has a consistent land-cover type or land-use type, with an area on the order of 10⁻¹—10² km².
- Coding rule: `CZ.CP.<patch type code>-<serial>`
- CMECS/EDCS alignment: corresponds to the CMECS Biotic/Substrate component-level classification.
- Code example: `CZ.CP.WTL-01` (wetland patch – 01)

**Level 4: Base Element (BaseElement)**

- Definition: a base observation or management element within a core patch that cannot be further subdivided.
- Discrimination criteria: an atomic element carrying a single physical quantity or a single management attribute, with a spatial scale < 10⁻¹ km².
- Coding rule: `CZ.BE.<element type code>-<sub-type code>-<serial>`
- CMECS/EDCS alignment: corresponds to the finest CMECS classification units + EDCS attribute codes.
- Code example: `CZ.BE.SED-MUD-01` (sediment – muddy – element 01)

#### 2.6.3 Coverage

The layered classification manager must cover the following three major categories of elements:

| Category | Subcategory | Level(s) | CMECS/EDCS alignment |
|:---|:---|:---:|:---|
| **Natural elements** | Atmosphere (air temperature, air pressure, wind field, precipitation) | WD—CC | EDCS_Atmosphere |
| | Hydrology (sea surface temperature, salinity, waves, tides, currents) | WD—CC—CP | CMECS_WaterColumn + EDCS_Hydro |
| | Terrain (water depth, coastline, elevation) | WD—CC | CMECS_Geoform + EDCS_Topography |
| | Geology (sediment type, substrate composition) | CC—CP—BE | CMECS_Substrate + EDCS_Geology |
| **Human activities** | Fishery production (fishing grounds, catch volume, aquaculture zones) | CC—CP | EDCS_Fishery |
| | Port operations (terminals, shipping channels, anchorages) | CC—CP | EDCS_Port |
| | Urban construction (construction land, protective works) | CP—BE | EDCS_Urban |
| **Ecological processes** | Carbon cycle (carbon sink, carbon flux, carbon storage) | WD—CC—CP | EDCS_Carbon |
| | Biological migration (bird migration corridors, fish migration routes) | WD—CC | EDCS_Migration |
| | Pollution dispersion (pollutant concentration, dispersion paths) | CC—CP—BE | EDCS_Pollution |

#### 2.6.4 Syntax form

```
classification_layer <identifier> {
    level: WholeDomain | CoastalCell | CorePatch | BaseElement,
    code: <code string>,
    parent: <parent code reference>,
    criteria: <discrimination criterion expression>,
    cmecs_alignment: <CMECS classification reference>,
    edcs_alignment: <EDCS classification reference>,
    elements: [ <element list> ]
}
```

#### 2.6.5 Mapping to existing .cz constructs

| Existing construct | Atom combination | Mapping notes |
|:---|:---|:---|
| `classification` | `classification_layer` (multi-level) | A classification declaration maps to layered classification levels |
| `layer` (in coastal_database) | `classification_layer` (CorePatch level) | A database layer maps to a core-patch-level classification |

---

### 2.7 Digital-twin mapping module (DigitalTwinMapping)

#### 2.7.1 Semantic definition

The digital-twin mapping module manages the mapping relations between physical assets and their digital replicas, including assimilation binding, simulation binding, feedback control, and state-variable management.

#### 2.7.2 Atom inventory

| Atom | Semantic role | Indivisible semantics | Interface contract |
|:---|:---|:---|:---|
| `PhysicalAsset` | Physical asset | Identifies a physical entity being mapped | `id() -> AssetID; geometry() -> Geometry` |
| `DigitalReplica` | Digital replica | Describes the digital model of a physical asset | `model() -> ModelRef; fidelity() -> FidelityLevel` |
| `AssimilationBinding` | Assimilation binding | Binds observation data to the state update of the digital model | `method() -> AssimilationMethod; frequency() -> Duration` |
| `SimulationBinding` | Simulation binding | Binds the digital model to simulation execution | `engine() -> SimEngine; horizon() -> Duration` |
| `FeedbackControl` | Feedback control | Describes the feedback-control chain from the digital model to the physical asset | `actuators() -> [Actuator]; policy() -> ControlPolicy` |
| `StateVariable` | State variable | Describes a state quantity of the digital-twin system | `name() -> Identifier; type() -> TypeExpr; unit() -> Unit` |
| `ObservationStream` | Observation stream | Describes a real-time observation data stream | `source() -> DataAccessAtom; rate() -> Frequency` |

#### 2.7.3 Syntax form

```
digital_twin_mapping <identifier> {
    physical_asset: PhysicalAsset { ... },
    digital_replica: DigitalReplica { ... },
    assimilation: AssimilationBinding { ... },
    simulation: SimulationBinding { ... },
    feedback_control: FeedbackControl { ... },
    state_variables: [StateVariable { ... }],
    observation_streams: [ObservationStream { ... }]
}
```

#### 2.7.4 Mapping to existing .cz constructs

| Existing construct | Atom combination | Mapping notes |
|:---|:---|:---|
| `digital_twin` | `PhysicalAsset` + `DigitalReplica` + `AssimilationBinding` + `SimulationBinding` + `FeedbackControl` | The digital-twin construct is atomically decomposed |
| `digital_twin.assimilation` | `AssimilationBinding` + `StateVariable` + `ObservationStream` | The assimilation module maps to these atoms |
| `digital_twin.simulation` | `SimulationBinding` | The simulation module maps to this atom |
| `digital_twin.control` | `FeedbackControl` | The control module maps to this atom |

---

## 3. Four-level hierarchical classification system

### 3.1 System overview table

| Level | Chinese name | English name | Level code | Spatial scale range | Typical resolution | Discrimination criteria |
|:---:|:---|:---|:---:|:---|:---|:---|
| Level 1 | 全域 | WholeDomain | WD | 10⁴—10⁶ km² | 1—10 km | Covers the complete study area |
| Level 2 | 海岸带单元 | CoastalCell | CC | 10²—10⁴ km² | 100—1000 m | Independent hydrodynamic/ecological boundary |
| Level 3 | 核心斑块 | CorePatch | CP | 10⁻¹—10² km² | 10—100 m | Consistent land cover/land use |
| Level 4 | 基础要素 | BaseElement | BE | < 10⁻¹ km² | < 10 m | Single physical quantity/management attribute |

### 3.2 Mapping to the territorial spatial planning system

| Classification level | "Three Zones and Three Lines" mapping | Ecological protection redline mapping | Disaster emergency zoning mapping |
|:---:|:---|:---|:---|
| WD (Whole Domain) | Corresponds to the full territory of the provincial territorial spatial plan | Corresponds to the overall pattern of ecological protection redlines | Corresponds to the overall zoning of regional disaster risk |
| CC (Coastal Cell) | Corresponds to the urban/agricultural/ecological three zones of municipal/county planning | Corresponds to specific blocks of ecological protection redlines | Corresponds to disaster emergency response zones |
| CP (Core Patch) | Corresponds to the land-use parcels of detailed planning | Corresponds to prohibited/restricted development patches within ecological protection redlines | Corresponds to disaster emergency evacuation/resettlement patches |
| BE (Base Element) | Corresponds to parcel/site-level elements | Corresponds to ecological protection redline boundary markers/signs | Corresponds to disaster emergency facilities/supply points |

### 3.3 Code examples

| Level | Code example | Meaning breakdown |
|:---:|:---|:---|
| WD | `CZ.WD.ECS.01` | Coastal zone . Whole domain . East China Sea shelf . Unit 01 |
| CC | `CZ.CC.YZ-E-01` | Coastal zone . Coastal cell . Yangtze Estuary - estuarine ecology - unit 01 |
| CP | `CZ.CP.WTL-01` | Coastal zone . Core patch . Wetland - patch 01 |
| BE | `CZ.BE.SED-MUD-01` | Coastal zone . Base element . Sediment - muddy - element 01 |

### 3.4 Cross-level aggregation/disaggregation rules

| Operation | Direction | Operator | Example |
|:---|:---|:---|:---|
| Aggregation | BE → CP → CC → WD | `AggregationOperator` | Multiple base elements aggregated into a core-patch statistic |
| Disaggregation | WD → CC → CP → BE | `DisaggregationOperator` | A whole-domain sea-level-rise scenario disaggregated to each base element |

---

## 4. Minimal atomic grammar (EBNF)

The following is the complete EBNF grammar of CZ-DSL. The grammar is backward compatible with existing .cz constructs and adds the keywords of the seven new component categories.

### 4.1 Top-level structure

```ebnf
(* CZ-DSL top-level program structure *)
program = { item } EOF ;

item = composite_decl
     | atom_decl
     | function_decl
     | type_decl
     | const_decl
     | let_decl ;

(* ===== Backward-compatible composite constructs ===== *)
composite_decl = sphere_decl
            | process_decl
            | model_decl
            | data_decl
            | ai_model_decl
            | digital_twin_decl
            | coastal_database_decl
            | scenario_decl
            | rule_decl
            | infrastructure_decl
            | policy_package_decl
            | spatial_reference_system_decl
            | classification_decl
            | workflow_decl ;

(* ===== New atom declarations for the seven component categories ===== *)
atom_decl = data_source_decl
                  | model_adapter_decl
                  | cross_subject_link_decl
                  | llm_dispatch_decl
                  | granularity_decl
                  | classification_layer_decl
                  | digital_twin_mapping_decl ;
```

### 4.2 Composite constructs

```ebnf
(* Sphere declaration *)
sphere_decl = ("sphere" | "圈层") identifier ["extends" type_expr]
              "{" { sphere_member } "}" ;
sphere_member = property_decl | process_decl | boundary_decl | constraint_decl ;
boundary_decl = "boundaries" ":" "{" { identifier ":" "Interface" "<" type_expr "," type_expr ">" "," } "}" ;

(* Process declaration *)
process_decl = ("process" | "过程") identifier [generic_params]
               "{" { "inputs" ":" "{" { io_member } "}"
                   | "outputs" ":" "{" { io_member } "}"
                   | "parameters" ":" "{" { parameter_member } "}"
                   | constraint_decl
                   | method_decl } "}" ;
io_member = identifier ":" type_expr "," ;

(* Model declaration *)
model_decl = ("model" | "模型") identifier [generic_params]
             "{" { "scales" ":" "{" { scale_member } "}"
                 | "coupling" ":" "{" { coupling_member } "}"
                 | "parameters" ":" "{" { parameter_member } "}"
                 | "variables" ":" "{" { variable_member } "}"
                 | "equations" ":" "{" { equation_member } "}"
                 | constraint_decl } "}" ;
coupling_member = identifier ":" "UpscalingOperator" "<" type_expr "," type_expr ">" ","
                | identifier ":" "DownscalingOperator" "<" type_expr "," type_expr ">" ","
                | identifier ":" "Coupling" "<" type_expr "," type_expr ">" "{" coupling_body "}" "," ;
coupling_body = "coupling_type" ":" ("OneWay" | "TwoWay" | "Feedback" | "BIDIRECTIONAL") ","
              { identifier ":" expression "," } ;

(* Data declaration *)
data_decl = ("data" | "数据") identifier [generic_params]
            "{" { field_decl | method_decl | constraint_decl } "}" ;

(* AI model declaration *)
ai_model_decl = "ai_model" identifier [generic_params]
                "{" { "architecture" ":" expression ","
                    | "training_data" ":" type_expr ","
                    | "physics_constraints" ":" "[" expression_list "]" ","
                    | method_decl } "}" ;

(* Digital twin declaration *)
digital_twin_decl = ("digital_twin" | "数字孪生") identifier
                    "{" { "physical_system" ":" type_expr ","
                        | "digital_replica" ":" type_expr ","
                        | "assimilation" ":" type_expr ","
                        | "simulation" ":" type_expr ","
                        | "control" ":" type_expr ","
                        | "data_sources" ":" "{" { identifier ":" string_literal "," } "}"
                        | "visualization" ":" "{" { identifier ":" expression "," } "}"
                        | "interactions" ":" "{" { identifier ":" expression "," } "}" } "}" ;

(* Coastal database declaration *)
coastal_database_decl = "coastal_database" identifier
                        "{" { "id" ":" string_literal ","
                            | "version" ":" string_literal ","
                            | "spatial" ":" identifier ","
                            | "temporal" ":" identifier ","
                            | "tiling" ":" identifier ","
                            | "cdb_profile" ":" identifier ","
                            | "metadata" ":" "{" { identifier ":" expression "," } "}"
                            | "layers" ":" "{" { layer_member } "}" } "}" ;
layer_member = identifier ":" "layer" "{" { "type" ":" identifier ","
                                          | "cdb_dataset" ":" identifier ","
                                          | "cdb_geometry_type" ":" identifier ","
                                          | "datasets" ":" "{" { dataset_member } "}" } "}" "," ;
dataset_member = identifier ":" "dataset" "{" { "feature_class" ":" identifier ","
                                              | "spatial_resolution" ":" expression ","
                                              | "temporal_resolution" ":" expression ","
                                              | "quality_profile" ":" identifier ","
                                              | "uncertainty_model" ":" identifier ","
                                              | "cmecs_unit" ":" identifier ","
                                              | "citygml_version" ":" identifier ","
                                              | "citygml_feature_class" ":" identifier ","
                                              | "citygml_lod" ":" expression ","
                                              | "citygml_ade" ":" identifier ","
                                              | "cdb_lod" ":" expression ","
                                              | "cdb_tile_size" ":" expression } "}" "," ;

(* Scenario declaration *)
scenario_decl = "scenario" identifier
                "{" { "spatial_ref" ":" identifier ","
                    | "time_horizon" ":" expression ","
                    | "drivers" ":" "{" { identifier ":" expression "," } "}"
                    | "outputs" ":" "{" { identifier ":" type_expr "," } "}"
                    | "parameters" ":" "{" { identifier ":" expression "," } "}"
                    | "uncertainty" ":" type_expr ","
                    | "assets" ":" expression ","
                    | "compute_pipeline" ":" "{" { identifier ":" expression "," } "}"
                    | "physical" ":" identifier ","
                    | "ecology" ":" "{" { identifier ":" identifier "," } "}"
                    | "socio_economics" ":" "{" { identifier ":" identifier "," } "}"
                    | "evaluation" ":" "{" { identifier ":" expression "," } "}"
                    | "domain" ":" expression ","
                    | "configuration" ":" "{" { identifier ":" expression "," } "}"
                    | "objectives" ":" "{" { identifier ":" expression "," } "}"
                    | "workflow" ":" "{" { identifier ":" expression "," } "}"
                    | "outcomes" ":" "{" { identifier ":" expression "," } "}" } "}" ;

(* Rule declaration *)
rule_decl = "rule" identifier
            "{" { "inputs" ":" "{" { identifier ":" type_expr "," } "}"
                | "function" identifier "(" [ parameter_list ")" ] "->" type_expr block_stmt } "}" ;

(* Infrastructure declaration *)
infrastructure_decl = "infrastructure" identifier
                      "{" { "type" ":" identifier ","
                          | "geometry" ":" string_literal ","
                          | "attributes" ":" "{" { identifier ":" expression "," } "}"
                          | "function" identifier "(" [ parameter_list ")" ] "->" type_expr block_stmt } "}" ;

(* Policy package declaration *)
policy_package_decl = "policy_package" identifier
                      "{" { "target_region" ":" expression ","
                          | "measures" ":" "{" { identifier ":" "{" { identifier ":" expression "," } "}" "," } "}"
                          | "objective" ":" expression } "}" ;

(* Spatial reference system declaration *)
spatial_reference_system_decl = "spatial_reference_system" identifier ["extends" type_expr]
                                "{" { "base_srs" ":" identifier ","
                                    | "origin" "(" expression "," expression "," expression ")" ","
                                    | "orientation" ":" identifier ","
                                    | "tidal_datum" ":" identifier ","
                                    | "temporal_reference" ":" identifier ","
                                    | "precision" ":" identifier ","
                                    | "coastal_parameters" ":" "{" { identifier ":" expression "," } "}" } "}" ;

(* Classification declaration *)
classification_decl = "classification" identifier ["extends" type_expr]
                      "{" { "edcs_code" ":" identifier ","
                          | "parent_classification" ":" identifier ","
                          | "biotic_component" ":" "{" { identifier ":" identifier "," } "}"
                          | "geoform_component" ":" identifier ","
                          | "substrate_component" ":" identifier ","
                          | "attributes" ":" "{" { identifier ":" expression "," } "}" } "}" ;

(* Workflow declaration *)
workflow_decl = "workflow" identifier
                "{" { "inputs" ":" "{" { identifier ":" expression "," } "}"
                    | "outputs" ":" "{" { identifier ":" expression "," } "}"
                    | "pipeline" ":" "{" { identifier ":" "{" { identifier ":" expression "," } "}" "," } "}"
                    | "dataflow" ":" "{" { expression "," } "}" } "}" ;
```

### 4.3 New atom declarations for the seven component categories

```ebnf
(* ===== Data access unit ===== *)
data_source_decl = "data_source" identifier
                   "{" { "source" ":" data_source_body ","
                       | "coordinate_ref" ":" coord_ref_body ","
                       | "temporal_ref" ":" temporal_ref_body ","
                       | "resolution" ":" resolution_body ","
                       | "quality" ":" quality_body ","
                       | "uncertainty" ":" uncertainty_body ","
                       | "endpoint" ":" endpoint_body } "}" ;
data_source_body = "DataSource" "{" { identifier ":" expression "," } "}" ;
coord_ref_body = "CoordinateReference" "{" { "base_srs" ":" identifier ","
                                           | "datum" ":" identifier ","
                                           | "projection" ":" identifier "," } "}" ;
temporal_ref_body = "TemporalReference" "{" { "calendar" ":" identifier ","
                                            | "timezone" ":" identifier ","
                                            | "epoch" ":" expression "," } "}" ;
resolution_body = "ResolutionSpec" "{" { "spatial" ":" expression ","
                                       | "temporal" ":" expression "," } "}" ;
quality_body = "QualityProfile" "{" { "qc_level" ":" identifier ","
                                    | "flags" ":" "[" expression_list "]" "," } "}" ;
uncertainty_body = "UncertaintyModel" "{" { "distribution" ":" identifier ","
                                          | "parameters" ":" "{" { identifier ":" expression "," } "}" } "}" ;
endpoint_body = "AccessEndpoint" "{" { "protocol" ":" identifier ","
                                     | "url" ":" string_literal "," } "}" ;

(* ===== Model adapter unit ===== *)
model_adapter_decl = "model_adapter" identifier
                     "{" { "interface" ":" "ModelInterface" "{" { "inputs" ":" "[" type_list "]" ","
                                                               | "outputs" ":" "[" type_list "]" "," } "}" ","
                         | "capability" ":" "ModelCapability" "{" { "type" ":" identifier ","
                                                                 | "domain" ":" identifier "," } "}" ","
                         | "io_contract" ":" "ModelIOContract" "{" { "input_schema" ":" type_expr ","
                                                                   | "output_schema" ":" type_expr ","
                                                                   | "units" ":" "[" unit_list "]" "," } "}" ","
                         | "version" ":" "ModelVersion" "{" { "semantic_version" ":" string_literal ","
                                                            | "dependencies" ":" "[" identifier_list "]" "," } "}" ","
                         | "deployment" ":" "ModelDeployment" "{" { "runtime" ":" identifier ","
                                                                 | "resources" ":" "{" { identifier ":" expression "," } "}" } "}" ","
                         | "invoke_binding" ":" "ModelInvokeBinding" "{" { "model" ":" identifier ","
                                                                         | "data" ":" identifier ","
                                                                         | "scenario" ":" identifier "," } "}" } "}" ;

(* ===== Cross-subject link engine ===== *)
cross_subject_link_decl = "cross_subject_link" identifier
                          "{" { "subject" ":" "Subject" "{" { "id" ":" string_literal ","
                                                             | "type" ":" identifier "," } "}" ","
                              | "role" ":" "Role" "{" { "permissions" ":" "[" identifier_list "]" ","
                                                      | "scope" ":" identifier "," } "}" ","
                              | "jurisdiction" ":" "Jurisdiction" "{" { "boundary" ":" expression ","
                                                                       | "level" ":" identifier "," } "}" ","
                              | "semantic_links" ":" "[" "SemanticLink" "{" { "source" ":" identifier ","
                                                                            | "target" ":" identifier ","
                                                                            | "type" ":" identifier "," } "}" { "," "SemanticLink" "{" { identifier ":" expression "," } "}" } "]" ","
                              | "provenance" ":" "Provenance" "{" { "origin" ":" identifier ","
                                                                  | "timestamp" ":" string_literal "," } "}" ","
                              | "authorization" ":" "Authorization" "{" { "granted_by" ":" identifier ","
                                                                        | "granted_to" ":" identifier ","
                                                                        | "scope" ":" identifier "," } "}" ","
                              | "responsibility" ":" "Responsibility" "{" { "duty" ":" identifier ","
                                                                           | "accountable_to" ":" identifier "," } "}" } "}" ;

(* ===== LLM dispatch interface ===== *)
llm_dispatch_decl = "llm_dispatch" identifier
                    "{" { "task" ":" "LLMTask" "{" { "input" ":" expression ""
                                                    | "expected" ":" type_expr "," } "}" ","
                        | "prompt" ":" "PromptContract" "{" { "template" ":" string_literal ""
                                                            | "variables" ":" "{" { identifier ":" type_expr "," } "}" } "}" ","
                        | "spatial_cognition" ":" "SpatialCognitionCall" "{" { "query" ":" expression ""
                                                                            | "model" ":" identifier "," } "}" ","
                        | "logic_inference" ":" "LogicInferenceCall" "{" { "premises" ":" "[" expression_list "]" ""
                                                                         | "query" ":" expression "," } "}" ","
                        | "output_binding" ":" "LLMOutputBinding" "{" { "parse" ":" expression ""
                                                                       | "schema" ":" type_expr "," } "}" ","
                        | "grounding" ":" "GroundingConstraint" "{" { "source" ":" identifier ""
                                                                    | "tolerance" ":" expression "," } "}" ","
                        | "hallucination_guard" ":" "HallucinationGuard" "{" { "validate" ":" expression ""
                                                                             | "threshold" ":" expression "," } "}" } "}" ;

(* ===== Granularity partition engine ===== *)
granularity_decl = "granularity" identifier
                   "{" { "levels" ":" "[" "GranularityLevel" "{" { "level" ":" integer_literal ""
                                                                 | "scale_range" ":" "(" expression "," expression ")" "," } "}" { "," "GranularityLevel" "{" { identifier ":" expression "," } "}" } "]" ","
                       | "partition_rules" ":" "[" "PartitionRule" "{" { "criterion" ":" expression "" } "}" { "," "PartitionRule" "{" { identifier ":" expression "," } "}" } "]" ","
                       | "aggregation" ":" "AggregationOperator" "{" { "method" ":" identifier "" } "}" ","
                       | "disaggregation" ":" "DisaggregationOperator" "{" { "method" ":" identifier "" } "}" ","
                       | "boundary_rules" ":" "[" "BoundaryRule" "{" { "priority" ":" integer_literal "" } "}" { "," "BoundaryRule" "{" { identifier ":" expression "," } "}" } "]" } "}" ;

(* ===== Layered classification manager ===== *)
classification_layer_decl = "classification_layer" identifier
                            "{" { "level" ":" ("WholeDomain" | "CoastalCell" | "CorePatch" | "BaseElement") ","
                                | "code" ":" string_literal ","
                                | "parent" ":" string_literal ","
                                | "criteria" ":" expression ","
                                | "cmecs_alignment" ":" identifier ","
                                | "edcs_alignment" ":" identifier ","
                                | "elements" ":" "[" identifier_list "]" } "}" ;

(* ===== Digital-twin mapping module ===== *)
digital_twin_mapping_decl = "digital_twin_mapping" identifier
                            "{" { "physical_asset" ":" "PhysicalAsset" "{" { "id" ":" string_literal ""
                                                                           | "geometry" ":" expression "," } "}" ","
                                | "digital_replica" ":" "DigitalReplica" "{" { "model" ":" identifier ""
                                                                             | "fidelity" ":" identifier "," } "}" ","
                                | "assimilation" ":" "AssimilationBinding" "{" { "method" ":" identifier ""
                                                                               | "frequency" ":" expression "," } "}" ","
                                | "simulation" ":" "SimulationBinding" "{" { "engine" ":" identifier ""
                                                                           | "horizon" ":" expression "," } "}" ","
                                | "feedback_control" ":" "FeedbackControl" "{" { "actuators" ":" "[" identifier_list "]" ""
                                                                                | "policy" ":" identifier "," } "}" ","
                                | "state_variables" ":" "[" "StateVariable" "{" { "name" ":" identifier ""
                                                                                | "type" ":" type_expr ","
                                                                                | "unit" ":" unit_expr "," } "}" { "," "StateVariable" "{" { identifier ":" expression "," } "}" } "]" ","
                                | "observation_streams" ":" "[" "ObservationStream" "{" { "source" ":" identifier ""
                                                                                        | "rate" ":" expression "," } "}" { "," "ObservationStream" "{" { identifier ":" expression "," } "}" } "]" } "}" ;
```

### 4.4 Shared grammar rules

```ebnf
(* Function declaration *)
function_decl = "fn" identifier [generic_params] "(" [ parameter_list ")" ] [ ":" type_expr ]
                ( block_stmt | ";" ) ;

(* Type declaration *)
type_decl = "type" identifier [generic_params] "=" type_expr ";" ;

(* Constant/variable declaration *)
const_decl = "const" identifier ":" type_expr "=" expression ";" ;
let_decl = "let" ["mut"] identifier [ ":" type_expr ] "=" expression ";" ;

(* Constraint declaration *)
constraint_decl = "constraint" identifier [generic_params] "{" constraint_body "}" ;
constraint_body = quantified_expr | expression ;
quantified_expr = ("∀" | "forall" | "∃" | "exists") identifier ":" type_expr "=>" expression ;

(* Field and parameter declarations *)
field_decl = identifier ":" type_expr [ "=" expression ] "," ;
property_decl = identifier ":" type_expr [ "=" expression ] "," ;
parameter_member = identifier ":" type_expr [ "=" expression ] "," ;
variable_member = identifier ":" type_expr [ "=" expression ] "," ;
equation_member = identifier ":" expression "," ;
scale_member = identifier ":" range_expr [ unit_expr ] "," ;
method_decl = ("method" | "fn") identifier [generic_params] "(" [ parameter_list ")" ] [ ":" type_expr ]
              ( block_stmt | ";" ) ;

(* Parameter list *)
parameter_list = parameter { "," parameter } ;
parameter = identifier ":" type_expr [ "=" expression ] ;

(* Type expressions *)
type_expr = basic_type | generic_type | function_type | tuple_type | array_type
          | field_type | uncertain_type | unit_type ;
basic_type = "Real" | "Integer" | "Complex" | "Boolean" | "String"
           | "Vector3D" | "Point2D" | "Point3D" | "GeoPoint"
           | "Multivector" | "Scalar" | "Vector" | "Bivector" | "Trivector"
           | "GridField" | "Length" | "Force" | "ForcePerArea" | "Duration"
           | "Salinity" | "QualityFlag" | "UncertaintyRange" | "RiskLevel"
           | "SuitabilityScore" | identifier ;
generic_type = identifier "<" type_list ">" ;
type_list = type_expr { "," type_expr } ;
function_type = "(" [ type_list ] ")" "->" type_expr ;
tuple_type = "(" type_list ")" ;
array_type = "[" type_expr "]" | "[" type_expr ";" expression "]" ;
field_type = ("ScalarField" | "VectorField" | "TensorField") "<" type_expr ">" ;
uncertain_type = "Uncertain" "<" type_expr ">" ;
unit_type = type_expr "with" "unit" unit_expr ;

(* Unit expressions *)
unit_expr = unit_term { ( "*" | "/" ) unit_term } ;
unit_term = unit_factor [ "^" integer_literal ] ;
unit_factor = unit_identifier | "(" unit_expr ")" ;
unit_list = unit_expr { "," unit_expr } ;

(* Range expressions *)
range_expr = expression ".." expression
           | expression "..=" expression
           | ".." expression
           | expression ".." ;

(* Generic parameters *)
generic_params = "<" generic_param { "," generic_param } ">" ;
generic_param = identifier [ ":" type_expr { "+" type_expr } ] ;

(* Identifiers and literals *)
identifier = ascii_identifier | unicode_identifier ;
ascii_identifier = ( letter | "_" ) { letter | digit | "_" } ;
unicode_identifier = xid_start { xid_continue } ;
identifier_list = identifier { "," identifier } ;
integer_literal = digit { digit } ;
float_literal = digit { digit } "." digit { digit } [ ("e" | "E") [ "+" | "-" ] digit { digit } ] ;
string_literal = "\"" { string_char } "\"" ;
expression_list = expression { "," expression } ;

(* Statements and expressions (kept consistent with composite constructs) *)
statement = expression_stmt | assignment_stmt | if_stmt | for_stmt | while_stmt
          | match_stmt | loop_stmt | break_stmt | continue_stmt | return_stmt | block_stmt ;
block_stmt = "{" { statement } "}" ;

expression = logical_or_expr ;
logical_or_expr = logical_and_expr { "||" logical_and_expr } ;
logical_and_expr = equality_expr { "&&" equality_expr } ;
equality_expr = relational_expr { ( "==" | "!=" | "≠" ) relational_expr } ;
relational_expr = additive_expr { ( "<" | "<=" | ">" | ">=" | "≤" | "≥" ) additive_expr } ;
additive_expr = multiplicative_expr { ( "+" | "-" ) multiplicative_expr } ;
multiplicative_expr = power_expr { ( "*" | "/" | "%" ) power_expr } ;
power_expr = unary_expr [ "**" power_expr ] ;
unary_expr = ( "+" | "-" | "!" | "∇" | "∂" ) unary_expr | postfix_expr ;
postfix_expr = primary_expr { "." identifier [ "(" [ expression_list ] ")" ]
                            | "[" expression "]"
                            | "(" [ expression_list ] ")" } ;
primary_expr = literal | identifier | "(" expression ")" | "[" [ expression_list ] "]"
             | "(" expression_list ")" | lambda_expr ;
lambda_expr = "|" [ parameter_list ] "|" ( expression | block_stmt ) ;
literal = integer_literal | float_literal | string_literal | "true" | "false" | "()" ;

(* Character classes *)
letter = "a" .. "z" | "A" .. "Z" ;
digit = "0" .. "9" ;
xid_start = (* Unicode XID_Start *) ;
xid_continue = (* Unicode XID_Continue *) ;
string_char = (* any char except " and \ *) | escape_sequence ;
escape_sequence = "\\" ( "n" | "t" | "r" | "\\" | "\"" | "'" | "0" ) ;
```

---

## 5. Keyword and unit tables

### 5.1 Reserved keywords of composite constructs (backward compatible)

| Keyword | Chinese equivalent | Semantics |
|:---|:---|:---|
| `sphere` | `圈层` | Sphere declaration |
| `process` | `过程` | Process declaration |
| `model` | `模型` | Model declaration |
| `data` | `数据` | Data declaration |
| `ai_model` | — | AI model declaration |
| `digital_twin` | `数字孪生` | Digital twin declaration |
| `coastal_database` | — | Coastal database declaration |
| `scenario` | — | Scenario declaration |
| `rule` | — | Rule declaration |
| `infrastructure` | — | Infrastructure declaration |
| `policy_package` | — | Policy package declaration |
| `spatial_reference_system` | — | Spatial reference system declaration |
| `classification` | — | Classification declaration |
| `workflow` | — | Workflow declaration |
| `scale` | `尺度` | Scale declaration |
| `interface` | — | Interface declaration |
| `coupling` | `耦合` | Coupling declaration |
| `layer` | — | Layer declaration |
| `dataset` | — | Dataset declaration |
| `constraint` | — | Constraint declaration |
| `fn` | — | Function declaration |
| `let` | — | Variable declaration |
| `const` | — | Constant declaration |
| `type` | — | Type declaration |
| `extends` | — | Inheritance |
| `with` | — | Unit annotation |
| `unit` | — | Unit keyword |
| `if` / `else` / `for` / `while` / `match` / `return` | — | Control flow |
| `true` / `false` | — | Boolean literals |
| `Uncertain` | — | Uncertainty type |
| `GridField` | — | Grid field type |
| `Multivector` / `Scalar` / `Vector` / `Bivector` / `Trivector` | — | Geometric algebra types |

### 5.2 New atom construct keywords

| Keyword | Chinese equivalent | Semantics | Component category |
|:---|:---|:---|:---|
| `data_source` | `数据源` | Data access unit declaration | DataAccessAtom |
| `model_adapter` | `模型适配器` | Model adapter unit declaration | ModelAdapterAtom |
| `cross_subject_link` | `跨主体关联` | Cross-subject link engine declaration | CrossSubjectLinkEngine |
| `llm_dispatch` | `大模型调度` | LLM dispatch interface declaration | LLMDispatchInterface |
| `granularity` | `颗粒度` | Granularity partition engine declaration | GranularityPartitionEngine |
| `classification_layer` | `分类层` | Layered classification manager declaration | LayeredClassificationManager |
| `digital_twin_mapping` | `数字孪生映射` | Digital-twin mapping module declaration | DigitalTwinMapping |
| `WholeDomain` | `全域` | Whole domain level | LayeredClassificationManager |
| `CoastalCell` | `海岸带单元` | Coastal cell level | LayeredClassificationManager |
| `CorePatch` | `核心斑块` | Core patch level | LayeredClassificationManager |
| `BaseElement` | `基础要素` | Base element level | LayeredClassificationManager |
| `DataSource` | — | Data source atom | DataAccessAtom |
| `CoordinateReference` | — | Coordinate reference atom | DataAccessAtom |
| `TemporalReference` | — | Temporal reference atom | DataAccessAtom |
| `ResolutionSpec` | — | Resolution specification atom | DataAccessAtom |
| `QualityProfile` | — | Quality profile atom | DataAccessAtom |
| `UncertaintyModel` | — | Uncertainty model atom | DataAccessAtom |
| `AccessEndpoint` | — | Access endpoint atom | DataAccessAtom |
| `ModelInterface` | — | Model interface atom | ModelAdapterAtom |
| `ModelCapability` | — | Model capability atom | ModelAdapterAtom |
| `ModelIOContract` | — | Input/output contract atom | ModelAdapterAtom |
| `ModelVersion` | — | Model version atom | ModelAdapterAtom |
| `ModelDeployment` | — | Model deployment atom | ModelAdapterAtom |
| `ModelInvokeBinding` | — | Invocation binding atom | ModelAdapterAtom |
| `Subject` | `主体` | Acting subject atom | CrossSubjectLinkEngine |
| `Role` | `角色` | Role atom | CrossSubjectLinkEngine |
| `Jurisdiction` | `管辖范围` | Jurisdiction atom | CrossSubjectLinkEngine |
| `SemanticLink` | `语义关联` | Semantic link atom | CrossSubjectLinkEngine |
| `Provenance` | `溯源` | Provenance information atom | CrossSubjectLinkEngine |
| `Authorization` | `授权` | Authorization atom | CrossSubjectLinkEngine |
| `Responsibility` | `责任` | Responsibility atom | CrossSubjectLinkEngine |
| `LLMTask` | — | LLM task atom | LLMDispatchInterface |
| `PromptContract` | — | Prompt contract atom | LLMDispatchInterface |
| `SpatialCognitionCall` | — | Spatial cognition call atom | LLMDispatchInterface |
| `LogicInferenceCall` | — | Logic inference call atom | LLMDispatchInterface |
| `LLMOutputBinding` | — | Output binding atom | LLMDispatchInterface |
| `GroundingConstraint` | — | Grounding constraint atom | LLMDispatchInterface |
| `HallucinationGuard` | — | Hallucination guard atom | LLMDispatchInterface |
| `GranularityLevel` | — | Granularity level atom | GranularityPartitionEngine |
| `PartitionRule` | — | Partition rule atom | GranularityPartitionEngine |
| `AggregationOperator` | — | Aggregation operator atom | GranularityPartitionEngine |
| `DisaggregationOperator` | — | Disaggregation operator atom | GranularityPartitionEngine |
| `BoundaryRule` | — | Boundary rule atom | GranularityPartitionEngine |
| `PhysicalAsset` | — | Physical asset atom | DigitalTwinMapping |
| `DigitalReplica` | — | Digital replica atom | DigitalTwinMapping |
| `AssimilationBinding` | — | Assimilation binding atom | DigitalTwinMapping |
| `SimulationBinding` | — | Simulation binding atom | DigitalTwinMapping |
| `FeedbackControl` | — | Feedback control atom | DigitalTwinMapping |
| `StateVariable` | — | State variable atom | DigitalTwinMapping |
| `ObservationStream` | — | Observation stream atom | DigitalTwinMapping |

### 5.3 Coastal-zone-specific physical unit table

| Unit name | Symbol | Physical quantity | Dimension | Notes |
|:---|:---:|:---|:---|:---|
| meter | m | Length | L | SI base unit |
| kilometer | km | Length | L | 10³ m |
| nautical_mile | nmi | Length | L | 1852 m, for navigation only |
| kilogram | kg | Mass | M | SI base unit |
| second | s | Time | T | SI base unit |
| minute | min | Time | T | 60 s |
| hour | h | Time | T | 3600 s |
| day | d | Time | T | 86400 s |
| year | yr | Time | T | 31557600 s (Julian year) |
| kelvin | K | Temperature | Θ | SI base unit |
| celsius | °C | Temperature | Θ | Offset 273.15 K |
| pascal | Pa | Pressure | ML⁻¹T⁻² | SI derived unit |
| hectopascal | hPa | Pressure | ML⁻¹T⁻² | 100 Pa, common in meteorology |
| decibar | dbar | Pressure | ML⁻¹T⁻² | 10⁴ Pa, common in oceanography |
| atmosphere | atm | Pressure | ML⁻¹T⁻² | 101325 Pa |
| newton | N | Force | MLT⁻² | SI derived unit |
| joule | J | Energy | ML²T⁻² | SI derived unit |
| watt | W | Power | ML²T⁻³ | SI derived unit |
| m/s | m/s | Speed | LT⁻¹ | Derived unit |
| psu | psu | Practical salinity | Dimensionless | Practical Salinity Unit |
| sverdrup | Sv | Volumetric flow | L³T⁻¹ | 10⁶ m³/s, oceanography-specific |
| degree | ° / deg | Angle | Dimensionless | π/180 rad |
| radian | rad | Angle | Dimensionless | SI derived unit |
| mg/L | mg/L | Concentration | ML⁻³ | Common in water quality |
| ppm | ppm | Concentration | Dimensionless | 10⁻⁶ |
| ppb | ppb | Concentration | Dimensionless | 10⁻⁹ |
| g/m² | g/m² | Areal density | ML⁻² | Common for biomass |
| kg/m³ | kg/m³ | Density | ML⁻³ | |
| W/m² | W/m² | Radiant flux density | MT⁻³ | Common for heat flux |
| N/m² | N/m² | Stress | ML⁻¹T⁻² | Common for momentum flux |
| m²/s | m²/s | Diffusion coefficient | L²T⁻¹ | Common for mixing coefficients |
| Hz | Hz | Frequency | T⁻¹ | |
| shannon_index | — | Biodiversity index | Dimensionless | Shannon diversity index |
| percent | % | Percentage | Dimensionless | |

---

## 6. Compatibility mapping with existing .cz examples

### 6.1 Mapping of paper1_2_core.cz

| Existing construct | Line range | Atom combination | Mapping notes |
|:---|:---:|:---|:---|
| `spatial_reference_system YangtzeEstuarySRS` | 1–15 | `CoordinateReference` + `TemporalReference` | The SRS splits into spatial and temporal reference atoms |
| `classification TyphoonHaikui (EDCS)` | 17–27 | `classification_layer` (CC level) + `SemanticLink` | An EDCS classification maps to a classification layer + semantic link |
| `scenario StormSurge_Test_01` | 29–42 | `data_source` + `model_adapter` + `granularity` | The scenario decomposes into data source, model adapter, and granularity |
| `classification ChongmingWetland (CMECS)` | 44–60 | `classification_layer` (CP level) + CMECS alignment | A CMECS classification maps to a core-patch-level classification layer |
| `rule SpartinaSuitability` | 62–73 | `LogicInferenceCall` + `GroundingConstraint` | A rule maps to a logic inference call + grounding constraint |
| `infrastructure Seawall_Section_A` | 75–94 | `PhysicalAsset` + `FeedbackControl` | Infrastructure maps to physical asset + feedback control |
| `layer LandUse_2023` | 96–103 | `classification_layer` (CP level) + `DataSource` | A layer maps to a core-patch classification layer + data source |
| `policy_package Hybrid_Adaptation_2050` | 105–120 | `cross_subject_link` + `Responsibility` | A policy package maps to a cross-subject link + responsibility atom |
| `scenario HangzhouBay_2050_SLR` | 122–142 | `llm_dispatch` + `model_adapter` + `data_source` | The scenario maps to LLM dispatch + model adapter + data source |
| `scenario HangzhouBay_2050_Integrated` | 144–162 | `digital_twin_mapping` + `granularity` | The integrated scenario maps to digital-twin mapping + granularity |

### 6.2 Mapping of coastal_model.cz

| Existing construct | Line range | Atom combination | Mapping notes |
|:---|:---:|:---|:---|
| `sphere 大气圈` | 5–24 | `classification_layer` (WD level) + `DataSource` | A sphere maps to a whole-domain classification layer + data source |
| `sphere Ocean` | 27–37 | `classification_layer` (WD level) + `DataSource` | The ocean sphere maps to a whole-domain classification layer |
| `process 潮汐过程` | 40–64 | `model_adapter` (ModelInterface + ModelIOContract) | A process maps to a model adapter unit |
| `data CoastalObservation` | 67–79 | `data_source` (all 7 atoms) | Observation data maps to a complete data access unit |
| `ai_model WaveHeightPredictor` | 82–111 | `model_adapter` + `llm_dispatch` | An AI model maps to model adapter + LLM dispatch |
| `digital_twin CoastalTwin` | 114–147 | `digital_twin_mapping` (all 7 atoms) | A digital twin maps to a complete digital-twin mapping module |
| `model CoastalSystemModel` | 150–183 | `granularity` + `model_adapter` | A multi-scale model maps to a granularity engine + model adapter |
| `scenario ClimateChangeImpact` | 186–205 | `data_source` + `granularity` + `model_adapter` | A climate-change scenario maps to data + granularity + model |

### 6.3 Mapping of cmecs_habitat.cz

| Existing construct | Line range | Atom combination | Mapping notes |
|:---|:---:|:---|:---|
| `coastal_database HangzhouBayCMECSDB` | 1–47 | `data_source` + `classification_layer` | A CMECS database maps to a data access unit + layered classification |
| `layer biotic` (tidal_flat_vegetation) | 20–31 | `classification_layer` (CP level) + `DataSource` + `ResolutionSpec` + `QualityProfile` + `UncertaintyModel` | The biotic-component layer fully atomized |
| `layer substrate` (sediment_type_map) | 34–45 | `classification_layer` (CP level) + `DataSource` + `ResolutionSpec` + `QualityProfile` + `UncertaintyModel` | The substrate-component layer fully atomized |
| `cmecs_unit: CMECS_Biotic_Tidal_Flat_Marsh` | 30 | `classification_layer.cmecs_alignment` | A CMECS unit maps to the classification layer's alignment field |
| `cmecs_unit: CMECS_Substrate_Mud_Sand_Mix` | 44 | `classification_layer.cmecs_alignment` | A CMECS unit maps to the classification layer's alignment field |

### 6.4 Compatibility guarantees

Atom constructs coexist with composite constructs through the following mechanisms:

1. **Syntax compatibility**: all composite keywords are retained in the atom grammar and can be parsed directly. The new atom construct keywords do not affect the syntax analysis of composite constructs.
2. **Semantic upgrading**: in the atomic semantic model, composite constructs are automatically mapped to the corresponding atom combinations, with no modification of source files required.
3. **Gradual migration**: composite-construct files can introduce atom constructs step by step; old and new constructs can be mixed.
4. **Toolchain compatibility**: the atom lexer is backward compatible with all token types of the existing lexer.

---

## Appendix A: Glossary

| Term | English | Definition |
|:---|:---|:---|
| 原子组件 | Atomic Component | An indivisible semantic unit carrying a single semantic |
| 构成分量 | Component Category | A semantic category composed of related atomic components |
| 语义接口契约 | Semantic Interface Contract | The standardized interface specification by which components compose |
| 几何积 | Geometric Product | The core binary operation of geometric algebra, used as the mathematical analogy for component composition |
| 多重向量 | Multivector | In geometric algebra, the collective term for scalars, vectors, bivectors, and trivectors |
| 可逆性 | Reversibility | The property that a digital description can be reverse-restored into the original business-scenario information |
| 坐标无关性 | Coordinate Independence | The property that a scenario description does not depend on any particular coordinate system |
| 语义同态 | Semantic Homomorphism | The algebraic property that extension mechanisms leave the semantic structure unchanged |

---

## Appendix B: Version history

| Version | Date | Main changes |
|:---|:---|:---|
| 1.0 | — | Initial version: basic constructs such as sphere/process/model/data/ai_model/digital_twin |
| 1.0 | 2026-08 | Introduced the seven fundamental component categories, the four-level hierarchical classification system, and the minimal atomic grammar |

---

*End of document*
