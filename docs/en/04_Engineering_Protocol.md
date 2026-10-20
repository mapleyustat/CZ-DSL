# CZ-DSL 1.0 Engineering Protocol

> **Document number**: 04  
> **Protocol version**: 1.0.0  
> **Status**: Official release (Stable)  
> **Date**: 2026-08-14  

> *This file is a faithful English translation of `docs/zh/04_工程化协议.md` (the authoritative original).*

---

## Table of contents

1. [Protocol overview and version governance](#1-protocol-overview-and-version-governance)
2. [Syntax rules: three equivalent representations](#2-syntax-rules-three-equivalent-representations)
3. [Semantic schema constraints](#3-semantic-schema-constraints)
4. [Data-interaction interfaces](#4-data-interaction-interfaces)
5. [Non-invasive extension mechanism (core)](#5-non-invasive-extension-mechanism-core)
6. [Interoperability mapping tables](#6-interoperability-mapping-tables)
7. [Version evolution and lifecycle strategy](#7-version-evolution-and-lifecycle-strategy)

---

## 1. Protocol overview and version governance

### 1.1 Design goals

CZ-DSL 1.0 is the second-generation engineering protocol of the coastal-zone domain description language. Building on the v1 text syntax (`.cz` files), it adds two serialization forms, XML and JSON, forming **three equivalent representations**. The protocol design draws on the schema-design paradigms of KML, CityGML, and GML, aligns its classification system with CMECS (Coastal and Marine Ecological Classification Standard) and EDCS (Environmental Data Coding Specification), and is compatible with SEDRIS SRM (Spatial Reference Model) for coordinate referencing.

The core architecture of this version consists of the **seven constituent components**:

| No. | Component | Responsibility |
|------|--------|------|
| 1 | DataAccess | data source, coordinate reference, temporal reference, resolution, quality, uncertainty, access endpoint |
| 2 | ModelAdapter | model interface, capability, IO contract, version, deployment, invocation binding |
| 3 | CrossSubjectLink | subject, role, jurisdiction, semantic link, provenance, authorization, responsibility |
| 4 | LLMDispatch | LLM task, prompt contract, spatial-cognition invocation, logical-reasoning invocation, output binding, grounding constraint, hallucination guard |
| 5 | GranularityPartition | granularity level, partition rule, aggregation/disaggregation operators, boundary rule |
| 6 | LayeredClassification | four-level classification layers (WholeDomain / CoastalCell / CorePatch / BaseElement) |
| 7 | DigitalTwinMapping | physical asset, digital replica, assimilation binding, simulation binding, feedback control, state variable, observation stream |

Each component consists of several **atoms (Atom)**; an atom is the smallest pluggable unit of the protocol.

### 1.2 Version governance

- **Current version**: 1.0.0 (Semantic Versioning, MAJOR.MINOR.PATCH)
- **Backward compatibility**: fully compatible with the existing `.cz` text syntax for composite constructs. Existing scenario files can be parsed by the compiler front end and then mapped onto the seven-component architecture; the original top-level keywords `sphere`, `process`, `model`, `data`, `ai_model`, `digital_twin`, `coastal_database`, `scenario`, `rule`, `infrastructure`, `policy_package`, `spatial_reference_system`, `classification`, `workflow`, etc. are each assigned to the corresponding component group.
- **Namespace**: the XML namespace is `http://cz-dsl.org/schema/1.0`, prefix `cz`.
- **Schema files**:
  - XSD: `cz-dsl.xsd`
  - JSON Schema: `cz-dsl.schema.json` (Draft 2020-12)

### 1.3 Keyword mapping from composite constructs to atomic constructs

| Composite keyword | Target atomic component | Description |
|-----------|------------|------|
| `spatial_reference_system` | DataAccess / CoordinateReference | atomized coordinate reference |
| `coastal_database` | DataAccess / DataSource | a database is a data source |
| `data` | DataAccess / DataSource | a data declaration maps to a DataSource atom |
| `model` | ModelAdapter / ModelInterface | a model declaration maps to a ModelInterface atom |
| `ai_model` | ModelAdapter / ModelInterface + LLMDispatch | an AI model maps across components |
| `process` | ModelAdapter / ModelIOContract | the inputs and outputs of a process map to an IO contract |
| `digital_twin` | DigitalTwinMapping | direct mapping |
| `scenario` | CZScenario root element | the scenario becomes the root container |
| `rule` | CrossSubjectLink / Responsibility | a rule maps to a responsibility constraint |
| `infrastructure` | DigitalTwinMapping / PhysicalAsset | infrastructure maps to a physical asset |
| `policy_package` | CrossSubjectLink / Jurisdiction | a policy package maps to jurisdiction and responsibility |
| `classification` | LayeredClassification / ClassificationLayer | a classification maps to the four-level classification layers |
| `workflow` | GranularityPartition / PartitionRule | a workflow maps to a partition rule |

---

## 2. Syntax rules: three equivalent representations

### 2.1 Text syntax (EBNF summary)

The following is an EBNF summary of the text syntax; the complete grammar specification is given in `01_语言规范.md`.

```ebnf
scenario       ::= "scenario" identifier "{" scenario_body "}"
scenario_body  ::= metadata spatial_ref temporal_ref granularity component_group*
metadata       ::= "metadata" "{" "title" ":" string ("description" ":" string)? "}"
spatial_ref    ::= "spatial_reference" "{" crs_body "}"
temporal_ref   ::= "temporal_reference" "{" temporal_body "}"
granularity    ::= "granularity" ":" level
level          ::= "WholeDomain" | "CoastalCell" | "CorePatch" | "BaseElement"
component_group::= data_access | model_adapter | cross_subject | llm_dispatch
                | granularity_partition | layered_classification | digital_twin
atom           ::= atom_name identifier "{" atom_body "}"
atom_body      ::= (key ":" value (unit)?) ("," key ":" value)*
```

### 2.2 XML serialization

XML serialization follows the W3C XML Schema definition (`cz-dsl.xsd`); the root element is `cz:CZScenario`.

### 2.3 JSON serialization

JSON serialization follows JSON Schema Draft 2020-12 (`cz-dsl.schema.json`); the root object is `CZScenario`.

### 2.4 Comparison of the three representations

| Concept | .cz text | XML | JSON |
|------|---------|-----|------|
| Scenario root | `scenario X { }` | `<cz:CZScenario id="X">` | `{"id":"X",...}` |
| Metadata | `metadata { title: "T" }` | `<metadata><Title>T</Title></metadata>` | `"metadata":{"title":"T"}` |
| Coordinate reference | `spatial_reference { epsg: 32651 }` | `<spatialReference><EPSG>32651</EPSG></spatialReference>` | `"spatialReference":{"epsg":32651}` |
| Temporal reference | `temporal_reference { epoch: "2025-01-01" }` | `<temporalReference><Epoch>2025-01-01</Epoch></temporalReference>` | `"temporalReference":{"epoch":"2025-01-01"}` |
| Granularity | `granularity: CoastalCell` | `<granularity><Level>CoastalCell</Level></granularity>` | `"granularity":{"level":"CoastalCell"}` |
| DataSource atom | `DataSource ds1 { protocol: OPeNDAP }` | `<DataSource id="ds1"><Protocol>OPeNDAP</Protocol></DataSource>` | `{"id":"ds1","protocol":"OPeNDAP"}` |
| ModelInterface atom | `ModelInterface m1 { modelName: FVCOM }` | `<ModelInterface id="m1"><ModelName>FVCOM</ModelName></ModelInterface>` | `{"id":"m1","modelName":"FVCOM"}` |
| Subject atom | `Subject s1 { subjectName: "Water Resources Bureau" }` | `<Subject id="s1"><SubjectName>Water Resources Bureau</SubjectName></Subject>` | `{"id":"s1","subjectName":"Water Resources Bureau"}` |
| LLM task | `LLMTask t1 { taskType: "SLR_projection" }` | `<LLMTask id="t1"><TaskType>SLR_projection</TaskType></LLMTask>` | `{"id":"t1","taskType":"SLR_projection"}` |
| Classification layer | `ClassificationLayer { level: CoastalCell, code: "CZ.CC.HZB-01" }` | `<ClassificationLayer id="..."><Level>CoastalCell</Level><Code>CZ.CC.HZB-01</Code></ClassificationLayer>` | `{"id":"...","level":"CoastalCell","code":"CZ.CC.HZB-01"}` |
| Physical asset | `PhysicalAsset a1 { assetType: Seawall }` | `<PhysicalAsset id="a1"><AssetType>Seawall</AssetType></PhysicalAsset>` | `{"id":"a1","assetType":"Seawall"}` |
| Extension slot | `x-blue-carbon { ... }` | `<ExtensionSlot><bc:BlueCarbon .../></ExtensionSlot>` | `"x-blue-carbon": {...}` |

---

## 3. Semantic schema constraints

### 3.1 Schema file references

- XML constraints: `cz-dsl.xsd` (W3C XML Schema)
- JSON constraints: `cz-dsl.schema.json` (JSON Schema Draft 2020-12)

### 3.2 Cardinality constraints

| Component | Atom | Min | Max | Notes |
|------|------|------|------|------|
| DataAccess | DataSource | 0 | unbounded | at least one data source recommended |
| DataAccess | CoordinateReference | 0 | unbounded | the scenario-level CRS is defined on the root element |
| ModelAdapter | ModelInterface | 0 | unbounded | one per model instance |
| ModelAdapter | ModelIOContract | 0 | unbounded | paired with a ModelInterface |
| CrossSubjectLink | Authorization | 0 | unbounded | cross-subject authorization must be validated |
| LLMDispatch | GroundingConstraint | 0 | unbounded | ≥1 recommended to guard against hallucination |
| GranularityPartition | AggregationOperator | 0 | unbounded | aggregation operators work together with partition rules |
| LayeredClassification | ClassificationLayer | 0 | unbounded | at least one layer recommended at each of the four levels |
| DigitalTwinMapping | PhysicalAsset | 0 | unbounded | paired with a DigitalReplica |

### 3.3 Unit-consistency validation

`UnitType` (XSD) / `Unit` (JSON Schema) carries seven SI dimension attributes (`siLength`, `siMass`, `siTime`, `siCurrent`, `siTemperature`, `siAmount`, `siLuminosity`). Runtime validation rules:

1. **Model IO-contract validation**: the input units of a `ModelIOContract` are dimension-matched against the expected units declared by the `ModelInterface`; a dimension mismatch causes the invocation to be rejected.
2. **Data-source resolution validation**: the `spatialResolution` of a `ResolutionSpec` must carry `siLength=1`, and its `temporalResolution` must carry `siTime=1`.
3. **State-variable validation**: the `unit` of a `StateVariable` must match the unit of the observation data in the `AssimilationBinding`.

Example (JSON):
```json
{
  "variableName": "sea_surface_height",
  "unit": { "value": "meter", "siLength": 1 }
}
```

### 3.4 Coordinate-consistency validation

`CoordinateReferenceType` (XSD) / `CoordinateReference` (JSON Schema) supports arbitrary CRSs through EPSG codes, WKT definitions, and dynamic parameters.

Validation rules:
1. The scenario-root `spatialReference` defines the global CRS. If a `CoordinateReference` atom inside a component declares a different CRS, it must provide a transformation path to the global CRS.
2. The `srmMapping` attribute may reference a SEDRIS SRM coordinate-system identifier (e.g. `SRM_GEODETIC_WGS_1984`); consistency is verified at runtime through SRM transformers.
3. The dynamic parameters `epoch`, `tidalDatum`, and `geoidModel` must be explicitly declared in scenarios involving tidal datums; otherwise a warning is raised.

### 3.5 Four-level classification code validation

`ClassificationCodeType` enforces the code pattern:

```
CZ.(WD|CC|CP|BE).[A-Z0-9-]+
```

| Prefix | Level | Meaning |
|------|------|------|
| `CZ.WD.*` | WholeDomain | whole-domain level |
| `CZ.CC.*` | CoastalCell | coastal-cell level |
| `CZ.CP.*` | CorePatch | core-patch level |
| `CZ.BE.*` | BaseElement | base-element level |

Validation rules:
1. The `level` enum value of a `ClassificationLayer` and the prefix of its `code` must agree (e.g. when `level=CoastalCell`, the `code` must start with `CZ.CC.`).
2. If `parentCode` is present, it must point to a classification code exactly one level above the current level.
3. `cmecsRef` and `edcsRef` are optional and are used to cross-reference CMECS/EDCS classification codes.

### 3.6 Cross-subject authorization validation

The `Authorization` atom in the `CrossSubjectLink` component declares `(principal, action, resource)` triples. Validation rules:

1. When an LLM invocation in `LLMDispatch` involves data access, a corresponding `Authorization` grant entry must exist.
2. The `assignee` of a `Responsibility` must reference a declared `Subject`.
3. The `scope` of a `Jurisdiction` must be compatible with the spatial extent of the scenario.

---

## 4. Data-interaction interfaces

### 4.1 Interface overview

The protocol defines four classes of core interfaces:

| Interface | Method | Purpose |
|------|------|------|
| Atom registration | `registerAtom` | register a new atom type with the runtime |
| Scenario load/export | `loadScenario` / `exportScenario` | load or export a complete scenario |
| Model-binding invocation | `invokeModel` | invoke a bound numerical/AI model |
| LLM dispatch | `dispatchLLM` | dispatch an LLM to execute a reasoning task |

### 4.2 Atom registration interface (registerAtom)

**Interface signature (IDL)**:
```
interface AtomRegistry {
  RegisterAtomResponse registerAtom(
    AtomDefinition definition,
    ExtensionRegistry registry
  ) raises(AtomConflictError, SchemaValidationError);
}

struct AtomDefinition {
  string atomName;
  string extendsAtom;     // parent atom being extended; must be an AtomType subclass
  string xsdFragment;     // XSD extension fragment (optional)
  string jsonSchemaFragment; // JSON Schema extension fragment (optional)
  string czGrammarRule;   // .cz text-syntax extension rule (optional)
};
```

**Request example (JSON)**:
```json
{
  "method": "registerAtom",
  "params": {
    "atomName": "BlueCarbonAccountingAtom",
    "extendsAtom": "cz:AtomType",
    "jsonSchemaFragment": "{\"properties\":{\"carbonFlux\":{\"type\":\"number\"}}}"
  }
}
```

**Response example (JSON)**:
```json
{
  "status": "ok",
  "atomId": "urn:cz-dsl:atom:BlueCarbonAccountingAtom",
  "version": "1.0.0",
  "registeredAt": "2026-08-14T10:00:00Z"
}
```

### 4.3 Scenario load/export interface (loadScenario / exportScenario)

**Interface signature (IDL)**:
```
interface ScenarioManager {
  CZScenario loadScenario(
    string uri,
    SerializationFormat format  // TEXT | XML | JSON
  ) raises(ParseError, SchemaValidationError);

  string exportScenario(
    CZScenario scenario,
    SerializationFormat format
  ) raises(SerializationError);
};
```

**loadScenario request example (JSON)**:
```json
{
  "method": "loadScenario",
  "params": {
    "uri": "file:///scenarios/hangzhou_bay_2050.xml",
    "format": "XML"
  }
}
```

**loadScenario response example (JSON)**:
```json
{
  "status": "ok",
  "scenario": {
    "id": "HZB-2050-SLR",
    "version": "2.0",
    "metadata": { "title": "Hangzhou Bay 2050 sea-level rise scenario" }
  }
}
```

**exportScenario request example (JSON)**:
```json
{
  "method": "exportScenario",
  "params": {
    "scenarioId": "HZB-2050-SLR",
    "format": "JSON"
  }
}
```

### 4.4 Model-binding invocation interface (invokeModel)

**Interface signature (IDL)**:
```
interface ModelInvoker {
  ModelResult invokeModel(
    string modelInterfaceId,
    map<string, Value> inputs,
    InvocationOptions options
  ) raises(ModelNotFoundError, InputValidationError, TimeoutError);
};

struct InvocationOptions {
  TimeDuration timeout;
  boolean dryRun;
  string versionTag;  // specifies the model version
};
```

**Request example (JSON)**:
```json
{
  "method": "invokeModel",
  "params": {
    "modelInterfaceId": "mi-fvcom-hzb",
    "inputs": {
      "windField": "data://meteo/wrf_2050_summer.nc",
      "boundary": "data://tide/tpxo9_hzb.nc"
    },
    "options": {
      "timeout": "3600s",
      "versionTag": "v4.1.2"
    }
  }
}
```

**Response example (JSON)**:
```json
{
  "status": "ok",
  "modelResult": {
    "outputs": {
      "seaSurfaceHeight": "data://results/hzb_ssh_2050.nc",
      "currentField": "data://results/hzb_uv_2050.nc"
    },
    "metrics": { "runtimeSec": 1842, "exitCode": 0 }
  }
}
```

### 4.5 LLM dispatch interface (dispatchLLM)

**Interface signature (IDL)**:
```
interface LLMDispatcher {
  LLMResult dispatchLLM(
    string llmTaskId,
    map<string, Value> context,
    GroundingConfig grounding
  ) raises(GroundingViolation, HallucinationDetected);
};

struct GroundingConfig {
  boolean enforceGrounding;
  double hallucinationThreshold;
  list<string> allowedDataSourceRefs;
};
```

**Request example (JSON)**:
```json
{
  "method": "dispatchLLM",
  "params": {
    "llmTaskId": "llm-slr-projection",
    "context": {
      "region": "Hangzhou Bay",
      "timeHorizon": "2050",
      "historicalData": "data://tide/hzb_1990_2025.nc"
    },
    "grounding": {
      "enforceGrounding": true,
      "hallucinationThreshold": 0.15,
      "allowedDataSourceRefs": ["ds-tide-gauge", "ds-satellite-altimetry"]
    }
  }
}
```

**Response example (JSON)**:
```json
{
  "status": "ok",
  "llmResult": {
    "projection": {
      "seaLevelRise2050": { "value": 0.52, "unit": "meter", "confidence": 0.88 },
      "narrative": "Based on 1990-2025 tide-gauge and satellite altimetry data..."
    },
    "groundingReport": {
      "passed": true,
      "hallucinationScore": 0.07
    }
  }
}
```

---

## 5. Non-invasive extension mechanism (core)

### 5.1 Dual extension mechanism

CZ-DSL 1.0 provides two layers of non-invasive extension mechanism, ensuring that the core schema never needs to be modified to support new requirements:

#### Mechanism 1: the AtomType abstract base class + inheritance

All atom types inherit from the abstract base class `AtomType`, which provides four common attributes:

| Attribute | Type | Description |
|------|------|------|
| `id` | `xs:ID` / `string` | unique atom identifier (required) |
| `version` | `string` | atom version |
| `extends` | `string` | identifier of the parent atom being extended |
| `source` | `string` | data/model source |

A new atom inherits from an existing atom through the `extends` attribute, without modifying any type definition in the core schema.

XSD fragment:
```xml
<xs:complexType name="AtomType" abstract="true">
  <xs:sequence>
    <xs:element name="ExtensionSlot" type="cz:ExtensionSlotType"
                minOccurs="0" maxOccurs="unbounded"/>
  </xs:sequence>
  <xs:attribute name="id" type="xs:ID" use="required"/>
  <xs:attribute name="version" type="xs:string" default="1.0.0"/>
  <xs:attribute name="extends" type="xs:string" use="optional"/>
  <xs:attribute name="source" type="xs:string" use="optional"/>
  <xs:anyAttribute namespace="##any" processContents="lax"/>
</xs:complexType>
```

JSON Schema fragment:
```json
{
  "Atom": {
    "type": "object",
    "required": ["id"],
    "properties": {
      "id": { "type": "string" },
      "version": { "type": "string", "default": "1.0.0" },
      "extends": { "type": "string" },
      "source": { "type": "string" }
    },
    "patternProperties": { "^x-": {} }
  }
}
```

#### Mechanism 2: ExtensionSlot (`x-` prefix / `xs:any` lax)

`ExtensionSlot` provides wildcard extension points:

- **XML**: uses `xs:any namespace="##any" processContents="lax"` and `xs:anyAttribute namespace="##any" processContents="lax"`, allowing elements and attributes from arbitrary namespaces to be inserted; the validator takes a lenient policy towards unknown content (validated if a schema exists, passed through otherwise).
- **JSON**: uses `patternProperties: { "^x-": {} }`, allowing any attribute whose name begins with the `x-` prefix.

### 5.2 Extension Registry specification

The extension registry is a global catalogue recording all registered extension atoms, data types, and model interfaces.

**Registry entry structure (JSON)**:
```json
{
  "extensionId": "urn:cz-dsl:ext:blue-carbon:1.0.0",
  "name": "BlueCarbonAccountingAtom",
  "extends": "cz:AtomType",
  "component": "DigitalTwinMapping",
  "version": "1.0.0",
  "status": "active",
  "schema": {
    "xsd": "extensions/blue-carbon.xsd",
    "json": "extensions/blue-carbon.schema.json"
  },
  "czGrammar": "extensions/blue-carbon.grammar.ebnf"
}
```

**Registry lifecycle**:
1. **Propose**: the extension developer submits a registry entry.
2. **Review**: the core team reviews compatibility and naming conventions.
3. **Register**: a URN is assigned and the entry is written to the registry.
4. **Deprecate**: marked as deprecated; backward compatibility is retained.
5. **Remove**: removed in the next MAJOR version.

### 5.3 Full extension example: adding the blue-carbon accounting atom BlueCarbonAccountingAtom

#### Requirement

Coastal-zone blue-carbon accounting needs to record carbon-flux, carbon-stock, and carbon-trading attributes; the core schema has no such atom.

#### XSD extension fragment

New file `extensions/blue-carbon.xsd`:
```xml
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"
           xmlns:cz="http://cz-dsl.org/schema/1.0"
           xmlns:bc="http://cz-dsl.org/schema/1.0/ext/blue-carbon"
           targetNamespace="http://cz-dsl.org/schema/1.0/ext/blue-carbon">

  <xs:import namespace="http://cz-dsl.org/schema/1.0"
             schemaLocation="cz-dsl.xsd"/>

  <!-- Blue-carbon accounting atom, extends AtomType -->
  <xs:complexType name="BlueCarbonAccountingAtomType">
    <xs:complexContent>
      <xs:extension base="cz:AtomType">
        <xs:sequence>
          <xs:element name="CarbonFlux" type="xs:double"/>
          <xs:element name="CarbonStock" type="xs:double"/>
          <xs:element name="HabitatType" type="xs:string"/>
          <xs:element name="Unit" type="cz:UnitType"/>
        </xs:sequence>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>

  <xs:element name="BlueCarbonAccounting"
              type="bc:BlueCarbonAccountingAtomType"
              substitutionGroup="cz:ExtensionSlot"/>
</xs:schema>
```

#### JSON Schema extension fragment

New file `extensions/blue-carbon.schema.json`:
```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "http://cz-dsl.org/schema/1.0/ext/blue-carbon.schema.json",
  "title": "BlueCarbonAccountingAtom",
  "allOf": [
    { "$ref": "http://cz-dsl.org/schema/1.0/cz-dsl.schema.json#/$defs/Atom" },
    {
      "type": "object",
      "required": ["carbonFlux", "carbonStock"],
      "properties": {
        "carbonFlux": { "type": "number", "description": "Carbon flux (kgC/m²/yr)" },
        "carbonStock": { "type": "number", "description": "Carbon stock (kgC/m²)" },
        "habitatType": { "type": "string", "enum": ["SaltMarsh", "Mangrove", "Seagrass"] },
        "unit": { "$ref": "#/$defs/Unit" }
      }
    }
  ]
}
```

#### .cz text fragment

```
scenario HangzhouBay_BlueCarbon {
    metadata { title: "Hangzhou Bay blue-carbon accounting" }
    spatial_reference { epsg: 32651 }
    temporal_reference { epoch: "2025-01-01" }
    granularity: CorePatch

    DigitalTwinMapping {
        // Extension atom: BlueCarbonAccountingAtom
        x-blue-carbon BlueCarbon_HZB_01 {
            carbonFlux: 2.34 with unit kgC/(m^2*yr),
            carbonStock: 185.6 with unit kgC/m^2,
            habitatType: SaltMarsh,
            extends: BlueCarbonAccountingAtom
        }
    }
}
```

#### XML instance fragment

```xml
<cz:DigitalTwinMapping>
  <cz:ExtensionSlot>
    <bc:BlueCarbonAccounting id="bc-hzb-01" version="1.0.0"
                              xmlns:bc="http://cz-dsl.org/schema/1.0/ext/blue-carbon">
      <bc:CarbonFlux>2.34</bc:CarbonFlux>
      <bc:CarbonStock>185.6</bc:CarbonStock>
      <bc:HabitatType>SaltMarsh</bc:HabitatType>
      <bc:Unit siLength="0" siMass="1" siTime="-2">kgC/(m^2*yr)</bc:Unit>
    </bc:BlueCarbonAccounting>
  </cz:ExtensionSlot>
</cz:DigitalTwinMapping>
```

#### JSON instance fragment

```json
{
  "DigitalTwinMapping": {
    "x-blue-carbon": {
      "id": "bc-hzb-01",
      "version": "1.0.0",
      "carbonFlux": 2.34,
      "carbonStock": 185.6,
      "habitatType": "SaltMarsh",
      "unit": { "value": "kgC/(m^2*yr)", "siMass": 1, "siTime": -2 }
    }
  }
}
```

### 5.4 Argument: extensions require no modification of the core schema

The complete implementation of the blue-carbon accounting extension above involves only:

1. **Creating** the extension schema files (`extensions/blue-carbon.xsd`, `extensions/blue-carbon.schema.json`).
2. Using `ExtensionSlot` (XML) or an `x-` prefixed attribute (JSON) in the instance to insert the extension content.
3. Registering it in the extension registry.

**The core schema `cz-dsl.xsd` / `cz-dsl.schema.json` was not modified**. This is because:

- `AtomType`'s `xs:anyAttribute processContents="lax"` permits arbitrary additional attributes.
- `ExtensionSlotType`'s `xs:any processContents="lax"` permits arbitrary child elements.
- JSON Schema's `patternProperties: {"^x-": {}}` permits arbitrary `x-` prefixed attributes.
- `substitutionGroup="cz:ExtensionSlot"` allows an extension element to substitute for the extension-slot position.

Therefore the protocol satisfies the **open–closed principle**: open for extension, closed for modification.

---

## 6. Interoperability mapping tables

### 6.1 Interoperability with KML/CityGML/GML

| CZ-DSL concept | KML counterpart | CityGML counterpart | GML counterpart |
|----------------|----------|-------------|----------|
| CZScenario | Document | CityModel | FeatureCollection |
| CoordinateReference | Region/LatLonAltBox | CRS (gml:SRS) | gml:Envelope+srsName |
| SpatialReference (Point3D) | Point/Coordinates | gml:Point | gml:Point |
| ClassificationLayer | Folder/Document | CityObjectGroup | gml:featureMember |
| PhysicalAsset | Placemark | Bridge/Building/Tunnel | AbstractFeature |
| TemporalReference | TimeSpan/TimeStamp | validity | gml:validTime |
| DataSource | NetworkLink | appearance/texture | observationProvider |
| ExtensionSlot | ExtendedData/Schema | generics | _GenericApplicationPropertyOf* |

**Mapping strategy**:
- **Export to KML**: map `PhysicalAsset` to `Placemark`, map `ClassificationLayer` to a `Folder` hierarchy, and write extension data into `ExtendedData`.
- **Import from CityGML**: map `CityObjectGroup` to `ClassificationLayer`, map buildings/bridges/tunnels to `PhysicalAsset`, and map generic attributes to `ExtensionSlot`.
- **GML interoperability**: all geometric objects are exchanged through GML syntax, and `srsName` maps to the EPSG code or WKT of a `CoordinateReference`.

### 6.2 Interoperability with SEDRIS / CDB

| CZ-DSL concept | SEDRIS SRM counterpart | SEDRIS EDCS counterpart | CDB counterpart |
|----------------|-----------------|------------------|----------|
| CoordinateReference | SRM SRF (Spatial Reference Frame) | — | CDB Coordinate System |
| ClassificationLayer | — | EDCS Classification Code (ECC) | CDB Dataset Code (Dxxx) |
| TemporalReference | SRM temporal | EDCS Time | CDB Time Step |
| DataSource | DRM Transmittal Root | EDCS Attribute Value | CDB Tile |
| PhysicalAsset | DRM Feature Geometry | EDCS Object | CDB Model |
| UncertaintyModel | DRM uncertainty | EDCS uncertainty | CDB LOD |
| ResolutionSpec | DRM resolution | EDCS resolution | CDB LOD Level |
| ExtensionSlot | DRM Property | EDCS Custom Attribute | CDB Extended Dataset |

**Mapping strategy**:
- **Export to SEDRIS STF**: `CoordinateReference` maps to an SRM SRF (e.g. `SRM_GEODETIC_WGS_1984`), and the `edcsRef` of a `ClassificationLayer` maps to an EDCS ECC.
- **Import from CDB**: CDB tile directories (`Dxxx_Sxxx_Txxx`) map to `DataSource`, and LOD levels map to `ResolutionSpec`.

### 6.3 CMECS / EDCS classification cross-references

| CZ-DSL four-level classification | CMECS counterpart | EDCS counterpart |
|----------------|-----------|----------|
| WholeDomain (CZ.WD.*) | CMECS System | EDCS Environment |
| CoastalCell (CZ.CC.*) | CMECS Subsystem | EDCS Region |
| CorePatch (CZ.CP.*) | CMECS Tidal_Flat / Salt_Marsh | EDCS Salt_Marsh |
| BaseElement (CZ.BE.*) | CMECS Biotic Unit / Substrate Unit | EDCS Feature |

---

## 7. Version evolution and lifecycle strategy

### 7.1 Semantic versioning

CZ-DSL adopts Semantic Versioning:

| Version change | Meaning | Compatibility |
|----------|------|--------|
| MAJOR (x.0.0) | incompatible architectural change | migration guide required |
| MINOR (2.x.0) | new components/atoms added | backward compatible |
| PATCH (2.0.x) | bug fixes and documentation updates | fully compatible |

### 7.2 Deprecation process

1. **Announce**: mark as `@deprecated` in the release notes and provide a replacement.
2. **Grace period**: retain compatibility support for at least 2 MINOR versions.
3. **Remove**: remove in the next MAJOR version.

Example deprecation annotation:
```xml
<xs:element name="LegacyDataSource" type="cz:DataSourceType" deprecated="true">
  <xs:annotation>
    <xs:documentation xml:lang="en">
      @deprecated From a subsequent version, please use the DataSource atom instead.
    </xs:documentation>
  </xs:annotation>
</xs:element>
```

### 7.3 Migration guide (composite constructs → atomic constructs)

#### Step 1: text-syntax migration

`.cz` files with composite constructs can be parsed directly by the compiler front end. The compiler automatically maps composite keywords to atomic components at the AST level (see the mapping table in §1.3).

#### Step 2: serialization-format selection

- **Keep text**: suited to human authoring and version control (Git diff friendly).
- **Convert to XML**: suited to enterprise SOA architectures and XML toolchain integration.
- **Convert to JSON**: suited to Web APIs and microservice architectures.

#### Step 3: classification-code migration

Free-text classifications must be mapped onto the four-level coding scheme:

```
v1: classification SaltMarsh extends EDCS_Tidal_Wetland
1.0: ClassificationLayer {
       level: CorePatch,
       code: "CZ.CP.SALT-MARSH",
       edcsRef: "ECC_SALT_MARSH"
     }
```

#### Step 4: coordinate-reference migration

`spatial_reference_system` blocks must be mapped to the `CoordinateReference` type, with an EPSG code or WKT definition added.

#### Step 5: extension migration

Extension attributes implemented by convention (non-standard) should be migrated to `ExtensionSlot` (XML) or `x-` prefixed attributes (JSON), and registered in the extension registry.

### 7.4 Lifecycle states

| State | Meaning | Support |
|------|------|------|
| Experimental | experimental, may change at any time | community feedback |
| Draft | draft, open for public comment | limited |
| Stable | stable release | full support |
| Deprecated | deprecated, to be removed soon | security fixes only |
| Retired | retired, no longer supported | none |

---

## Appendix A: File inventory

| File | Description |
|------|------|
| `cz-dsl.xsd` | W3C XML Schema definition |
| `cz-dsl.schema.json` | JSON Schema (Draft 2020-12) definition |
| `04_工程化协议.md` | this document |
| `examples/sample_scenario.xml` | XML instance example |
| `examples/sample_scenario.json` | JSON instance example |

## Appendix B: Glossary

| Term | English | Description |
|------|------|------|
| 原子 | Atom | the smallest pluggable unit of the protocol |
| 组件 | Component | a logical grouping of atoms |
| 扩展槽 | ExtensionSlot | non-invasive extension point |
| 四级分类 | 4-Level Classification | WholeDomain/CoastalCell/CorePatch/BaseElement |
| 接地约束 | GroundingConstraint | data-provenance constraint on LLM output |
| 幻觉防护 | HallucinationGuard | factuality verification mechanism for LLM output |

---

*End of document*
