# CZ-DSL Design Principles

> Design-principles document for the coastal-zone complex-system domain description language.
> This document sets out the core design principles of CZ-DSL and their correspondence to the seven foundational constituent components.

> *This file is a faithful English translation of `docs/zh/02_设计原则.md` (the authoritative original).*

---

## Table of contents

1. [Plug-in logic](#1-plug-in-logic)
2. [Representational reversibility](#2-representational-reversibility)
3. [Descriptive completeness](#3-descriptive-completeness)
4. [Minimal atomicity framework](#4-minimal-atomicity-framework)
5. [Dynamic attachment of spatio-temporal references](#5-dynamic-attachment-of-spatio-temporal-references)
6. [Correspondence matrix of principles and seven component classes](#6-correspondence-matrix-of-principles-and-seven-component-classes)

---

## 1. Plug-in logic

### 1.1 Core idea

Plug-in logic is the foremost design principle of CZ-DSL. Its core idea is to decompose the description of a coastal-zone complex system into independent components defined by standardized semantic interfaces, where components are composed through semantic interface contracts rather than coupled through hard-coded dependencies.

The idea borrows directly from the geometric-product composition mechanism of geometric algebra. In geometric algebra, any multivector can be composed through the single binary operation of the geometric product, and the result remains within the algebra (closure). Similarly, in CZ-DSL any atomic component is composed with others through the standardized composition operator $\oplus$, and the result is again a legal semantic unit.

### 1.2 Semantic interface contracts

The semantic interface contract is the core mechanism of plug-in logic. Each atomic component declares the interfaces it provides (*provides*) and the interfaces it depends on (*requires*); composition matches interfaces to achieve loose coupling.

**Definition 1.1 (semantic interface contract)**: A semantic interface contract $\mathcal{C}$ is a pair:

$$
\mathcal{C} = (\mathcal{P}, \mathcal{R})
$$

where $\mathcal{P}$ is the set of interfaces the component provides (Provides) and $\mathcal{R}$ is the set of interfaces it requires (Requires).

**Composition rule**: two components $A$ and $B$ may be composed by $\oplus$ if and only if every interface $B$ requires can be provided by $A$, or every interface $A$ requires can be provided by $B$:

$$
A \oplus B \text{ is legal} \iff \mathcal{R}_A \subseteq \mathcal{P}_B \lor \mathcal{R}_B \subseteq \mathcal{P}_A
$$

### 1.3 Analogy with the geometric product

In the geometric algebra $\mathrm{Cl}(p,q)$ the geometric product satisfies the following properties:

| Geometric-algebra property | CZ-DSL analogue |
|:---|:---|
| Associativity: $(AB)C = A(BC)$ | Component composition is associative: $(A \oplus B) \oplus C = A \oplus (B \oplus C)$ |
| Bilinearity: $A(\alpha B + \beta C) = \alpha AB + \beta AC$ | Composition is linear in interface parameters |
| Non-commutativity: $AB \neq BA$ (in general) | Composition order affects semantics (data source → model adapter → LLM dispatch) |
| Multivector decomposition: $M = \sum_k \langle M \rangle_k$ | A complex scenario decomposes into a combination of graded atomic components |
| Existence of inverses: if $A \tilde{A} \neq 0$, then $A^{-1} = \tilde{A}/(A\tilde{A})$ | The reversibility principle (see §2) |

### 1.4 A plug-in composition example

For a coastal storm-surge assessment scenario, the plug-in composition chain is:

```
DataAccessAtom ──⊕──▶ ModelAdapterAtom ──⊕──▶ GranularityPartitionEngine
   (provides data        (provides model          (provides scale-partition
    interface)            interface)                interface)
                                                    │
                                                    ▼
                              LLMDispatchInterface ──⊕──▶ DigitalTwinMapping
                              (provides inference          (provides mapping
                               interface)                   interface)
```

Each arrow is one $\oplus$ composition; every composition is matched through semantic interface contracts, with no knowledge of the other side's internal implementation required.

### 1.5 Design corollaries

Plug-in logic yields the following corollaries:

1. **Interface stability**: once defined, an atomic component's interface contract never changes, guaranteeing composition compatibility.
2. **Implementation replaceability**: any implementation satisfying the same interface contract can be substituted freely without affecting composed chains.
3. **Composition extensibility**: a new component can participate in composition by declaring its interface contract alone; existing components need no modification.
4. **Test isolation**: each component can be tested independently by mocking only the interfaces it requires.

---

## 2. Representational reversibility

### 2.1 Core idea

Reversibility requires that a CZ-DSL digital description can be reverse-restored into the original business-scenario information. That is, there exists an inverse mapping $R$ such that for any business scenario $S$, applying the digital description $D$ and then the reverse restoration $R$ recovers the original information without loss:

$$
R(D(S)) = S
$$

This principle ensures that digitization loses no business semantics, allowing a domain expert to fully reconstruct the original business intent from the digital description.

### 2.2 Formal definitions

**Definition 2.1 (digitization mapping)**: The digitization mapping $D: \mathcal{S} \to \mathcal{D}$ maps the business-scenario space $\mathcal{S}$ to the digital-description space $\mathcal{D}$.

**Definition 2.2 (reverse-restoration mapping)**: The reverse-restoration mapping $R: \mathcal{D} \to \mathcal{S}$ maps the digital-description space $\mathcal{D}$ back to the business-scenario space $\mathcal{S}$.

**Definition 2.3 (reversibility)**: CZ-DSL satisfies reversibility if and only if

$$
\forall S \in \mathcal{S}: \quad R(D(S)) = S
$$

i.e. $R = D^{-1}$ and $D$ is a bijection.

### 2.3 Conditions preserving reversibility

Reversibility holds under the following conditions:

**Condition C1 (information completeness)**: the digital description must contain the full semantic information of the business scenario, with no loss.

$$
\forall S_1, S_2 \in \mathcal{S}: \quad S_1 \neq S_2 \implies D(S_1) \neq D(S_2)
$$

That is, $D$ is injective, which requires the atomic-component set to cover every business-semantic dimension.

**Condition C2 (semantic shape preservation)**: the structure of the digital description must preserve the semantic structural relations of the business scenario.

$$
\forall S_1, S_2 \in \mathcal{S}: \quad S_1 \prec S_2 \implies D(S_1) \prec_D D(S_2)
$$

where $\prec$ denotes semantic dependency in the business scenario and $\prec_D$ the corresponding relation in the digital description.

**Condition C3 (atomic identifiability)**: every atomic component has a unique semantic identity in the digital description, identifiable through its interface contract.

$$
\forall a_1, a_2 \in \mathcal{A}: \quad a_1 \neq a_2 \implies \text{contract}(a_1) \neq \text{contract}(a_2)
$$

where $\mathcal{A}$ is the set of atomic components.

### 2.4 Reversibility from the geometric-algebra viewpoint

Within geometric algebra, reversibility corresponds to the invertibility of multivectors. In a non-degenerate metric space $\mathrm{Cl}(p,q)$ ($p+q > 0$), a multivector $A$ is invertible iff its scalar part $A\tilde{A} \neq 0$, where $\tilde{A}$ is the reverse of $A$.

The analogy to CZ-DSL:

- **Non-degeneracy**: each atomic component carries non-zero semantic information (does not degenerate), equivalent to $A\tilde{A} \neq 0$.
- **Reverse ↔ inverse mapping**: the reverse-restoration mapping $R$ corresponds to the reversion operation $\tilde{\cdot}$, satisfying $\tilde{\tilde{A}} = A$, i.e. $R(R(x)) = x$.
- **Compositional invertibility**: if atomic components $A$ and $B$ are each invertible, their geometric product $AB$ is invertible with $(AB)^{-1} = B^{-1}A^{-1}$; correspondingly, reversibility is preserved under composition.

### 2.5 Reversibility verification criteria

| Criterion | What is verified | Method |
|:---:|:---|:---|
| V-REV-1 | information completeness | check that the digital description contains every business-semantic field |
| V-REV-2 | semantic shape preservation | check that the description structure preserves scenario dependencies |
| V-REV-3 | atomic identifiability | check that every atom has a unique interface contract |
| V-REV-4 | compositional reversibility | check whether $R(A \oplus B) = R(B) \oplus R(A)$ holds |

---

## 3. Descriptive completeness

### 3.1 Core idea

Completeness requires CZ-DSL to cover the expressive needs of all core business scenarios of coastal-zone complex systems: for any business scenario in the domain, there exists a combination of atomic components that describes it exactly.

### 3.2 Formal criterion

**Definition 3.1 (business-scenario space)**: let $\mathcal{S}_{CZ}$ be the set of all core business scenarios of coastal-zone complex systems.

**Definition 3.2 (descriptive-capability space)**: let $\mathcal{D}_{CZ}$ be the set of all digital descriptions that CZ-DSL atomic components can generate through the composition operator $\oplus$.

**Definition 3.3 (descriptive completeness)**: CZ-DSL satisfies descriptive completeness if and only if

$$
\forall S \in \mathcal{S}_{CZ}: \quad \exists D \in \mathcal{D}_{CZ}: \quad D \text{ describes } S \text{ exactly}
$$

### 3.3 Verification method

Completeness is verified by a three-layer method:

**Layer 1: business-scenario class coverage**

Coastal-zone complex-system business scenarios fall into six classes; each is verified as expressible by atomic combinations:

| Business-scenario class | Covering atomic components | Complete |
|:---|:---|:---:|
| Multi-source data access and fusion | all 7 DataAccessAtom atoms | ✓ |
| Numerical/AI model coupling and dispatch | ModelAdapterAtom + LLMDispatchInterface | ✓ |
| Cross-department collaboration and accountability tracing | all 7 CrossSubjectLinkEngine atoms | ✓ |
| Multi-scale analysis and scale transfer | all 5 GranularityPartitionEngine atoms | ✓ |
| Ecological / geological / human-activity classification | the four-level LayeredClassificationManager | ✓ |
| Digital twin and real-time feedback | all 7 DigitalTwinMapping atoms | ✓ |

**Layer 2: semantic-dimension coverage**

Verify that the seven constituent components cover every semantic dimension of the domain:

1. data semantics (what data) → DataAccessAtom
2. computation semantics (how to compute) → ModelAdapterAtom + LLMDispatchInterface
3. subject semantics (who) → CrossSubjectLinkEngine
4. scale semantics (at what scale) → GranularityPartitionEngine
5. classification semantics (how to classify) → LayeredClassificationManager
6. mapping semantics (how to twin) → DigitalTwinMapping

**Layer 3: generative power of the composition algebra**

Verify that atomic components generate all business-scenario descriptions through $\oplus$. By closure of the composition algebra, if the atomic set $\mathcal{A}$ satisfies

$$
\mathcal{D}_{CZ} = \text{closure}_{\oplus}(\mathcal{A})
$$

and $\mathcal{D}_{CZ} \supseteq D(\mathcal{S}_{CZ})$, completeness holds.

### 3.4 Conditions preserving completeness

Completeness is preserved under:

1. **Atomic-set stability**: existing atomic components are never deleted.
2. **Composition-operator closure**: $\oplus$ is closed over the atomic set; a composition is again a legal semantic unit.
3. **Monotone extension**: adding atomic components only enlarges $\mathcal{D}_{CZ}$, never shrinks it.

---

## 4. Minimal atomicity framework

### 4.1 Core idea

Minimal atomicity requires that each atomic component carries a single semantic, and complex scenarios are built by composing atoms. The principle ensures semantic minimality, orthogonality, and composability.

### 4.2 Atomicity rules

**Rule A1 (single semantics)**: a component $a$ is atomic iff it carries exactly one indivisible semantic unit.

$$
\text{Atomic}(a) \iff \exists! \sigma \in \Sigma: \quad \text{semantics}(a) = \{\sigma\}
$$

where $\Sigma$ is the set of all indivisible semantic units.

**Rule A2 (indecomposability)**: a component $a$ is atomic iff there is no non-trivial decomposition $a = b \oplus c$ with $b$ and $c$ both legal components.

$$
\text{Atomic}(a) \iff \nexists b, c \in \mathcal{A} \setminus \{a\}: \quad a = b \oplus c
$$

**Rule A3 (semantic orthogonality)**: the semantics of any two distinct atomic components are disjoint.

$$
\forall a_1 \neq a_2 \in \mathcal{A}: \quad \text{semantics}(a_1) \cap \text{semantics}(a_2) = \emptyset
$$

### 4.3 Composition-algebra rules

**Rule C1 (closure)**: a composition of atomic components is a legal semantic unit.

$$
\forall a, b \in \mathcal{A}: \quad a \oplus b \in \mathcal{D}_{CZ}
$$

**Rule C2 (associativity)**: composition is associative.

$$
\forall a, b, c \in \mathcal{A}: \quad (a \oplus b) \oplus c = a \oplus (b \oplus c)
$$

**Rule C3 (non-commutativity)**: composition is generally not commutative; order affects semantics.

$$
\exists a, b \in \mathcal{A}: \quad a \oplus b \neq b \oplus a
$$

For example, `DataAccessAtom ⊕ ModelAdapterAtom` (access data first, then adapt the model) differs semantically from `ModelAdapterAtom ⊕ DataAccessAtom` (adapt the model first, then access data).

**Rule C4 (identity)**: the empty description $\epsilon$ is the identity of composition.

$$
\forall a \in \mathcal{A}: \quad a \oplus \epsilon = \epsilon \oplus a = a
$$

**Rule C5 (reversibility)**: under the reversibility conditions of §2, composition is reversible.

$$
\forall a, b \in \mathcal{A}: \quad R(a \oplus b) = R(b) \oplus R(a)
$$

### 4.4 Measuring atomicity

Define the atomicity index $\eta_{atomic}$:

$$
\eta_{atomic} = \frac{|\mathcal{A}|}{|\mathcal{A}| + |\mathcal{N}|}
$$

where $|\mathcal{A}|$ is the number of atomic components and $|\mathcal{N}|$ the number of non-atomic (further-decomposable) components. The CZ-DSL design target is $\eta_{atomic} = 1$ — every component atomic.

### 4.5 Verification of the atom partition

The atom partition of the seven constituent components is verified as follows:

| Constituent component | Atoms | Single semantics | Indecomposable | Orthogonal |
|:---|:---:|:---:|:---:|:---:|
| DataAccessAtom | 7 | ✓ one data semantic each | ✓ no further decomposition | ✓ semantics disjoint |
| ModelAdapterAtom | 6 | ✓ one model semantic each | ✓ | ✓ |
| CrossSubjectLinkEngine | 7 | ✓ one subject semantic each | ✓ | ✓ |
| LLMDispatchInterface | 7 | ✓ one dispatch semantic each | ✓ | ✓ |
| GranularityPartitionEngine | 5 | ✓ one scale semantic each | ✓ | ✓ |
| LayeredClassificationManager | 4 levels | ✓ one classification semantic per level | ✓ | ✓ |
| DigitalTwinMapping | 7 | ✓ one mapping semantic each | ✓ | ✓ |

---

## 5. Dynamic attachment of spatio-temporal references

### 5.1 Core idea

The dynamic-attachment principle requires CZ-DSL to natively support, at the semantic substrate, multi-source data at any spatio-temporal resolution, in any spatial coordinate system, and under any spatio-temporal reference framework. Cross-domain rules are unified at the semantic substrate, resolving the data-description conflicts and information fragmentation of traditional GIS description languages.

### 5.2 Limitations of traditional GIS description languages

Traditional GIS description languages (KML, CityGML, GML) have the following limitations for coastal-zone complex-system description:

| Dimension | KML | CityGML | GML | CZ-DSL |
|:---|:---|:---|:---|:---|
| Coordinate systems | fixed WGS84 | fixed projected CRS | external declaration required | any CRS, natively supported |
| Temporal reference | none native | none native | none native | TemporalReference atom |
| Resolution description | none | 5 fixed LOD levels | none | ResolutionSpec, arbitrary specs |
| Uncertainty modelling | none | none | none | UncertaintyModel atom |
| Quality description | none | none | none | QualityProfile atom |
| Multi-source fusion | weak | weak | medium | strong (DataSource + AccessEndpoint) |
| Coastal-specific units | none | none | none | psu / dbar / Sv / hPa / nmi, etc. |
| Cross-subject semantics | none | none | none | CrossSubjectLinkEngine |
| Semantic-layer unification | presentation layer | presentation layer | presentation layer | unified at the semantic substrate |

### 5.3 The unification mechanism

CZ-DSL achieves semantic-substrate unification of spatio-temporal references through these mechanisms:

**Mechanism M1: atomized coordinate reference**

The spatial coordinate reference is separated from the data and exists as the standalone `CoordinateReference` atomic component. Any data source declares its coordinate reference by referencing a `CoordinateReference` atom; conversion between different references happens automatically through the interface contract `to_srs(target) -> Transform`.

**Mechanism M2: atomized temporal reference**

The temporal reference is separated from the data as the standalone `TemporalReference` atom, supporting different calendar systems (Gregorian, lunar), time zones, and time precisions.

**Mechanism M3: atomized resolution specification**

Spatial and temporal resolution are unified into the `ResolutionSpec` atom, supporting arbitrary resolution specifications rather than fixed LOD levels.

**Mechanism M4: coordinate independence**

Scenario descriptions are independent of any concrete coordinate system at the semantic level: semantic content does not change when the coordinate system changes — the coordinate system is merely a presentation form of the data. This is guaranteed by the covariance of geometric algebra: under different coordinate systems a multivector's semantics is invariant; only its component representation changes.

### 5.4 Formal statement of coordinate independence

**Definition 5.1 (coordinate-transformation group)**: let $G$ be the coordinate-transformation group, with $g \in G$ a coordinate transformation.

**Definition 5.2 (coordinate independence)**: a scenario description $D$ satisfies coordinate independence iff for every transformation $g \in G$:

$$
\llbracket g(D) \rrbracket = \llbracket D \rrbracket
$$

where $\llbracket \cdot \rrbracket$ is the semantic interpretation function — a coordinate transformation does not change the scenario's semantic interpretation.

In geometric algebra this corresponds to the covariance of multivectors under rotation/transformation: with rotation $R$, $M' = RMR^{-1}$ preserves the geometric semantics of $M$.

### 5.5 Integration with SEDRIS SRM

The dynamic-attachment principle integrates deeply with the SEDRIS SRM (Spatial Reference Model):

- The `base_srs` field of `CoordinateReference` atoms directly references coordinate reference systems defined by SEDRIS SRM (e.g. `WGS84_UTM_Zone_51N`, `WGS84_Geographic`).
- Coordinate transformation is performed automatically by SEDRIS SRM transformers, corresponding to the existing `transformer sedris_transformer` construct in .cz files.
- Existing `spatial_reference_system` constructs (e.g. the Yangtze-estuary coordinate system) map directly onto a `CoordinateReference` + `TemporalReference` atom combination.

---

## 6. Correspondence matrix of principles and seven component classes

The matrix below shows how the design principles support the seven foundational constituent components.

### 6.1 The matrix

|  | P1 plug-in logic | P2 reversibility | P3 completeness | P4 minimal atomicity | P5 spatio-temporal reference |
|:---|:---:|:---:|:---:|:---:|:---:|
| **C1 DataAccessAtom** | ● | ● | ● | ● | ● |
| **C2 ModelAdapterAtom** | ● | ● | ● | ● | ○ |
| **C3 CrossSubjectLinkEngine** | ● | ● | ● | ● | ○ |
| **C4 LLMDispatchInterface** | ● | ○ | ● | ● | ○ |
| **C5 GranularityPartitionEngine** | ● | ● | ● | ● | ● |
| **C6 LayeredClassificationManager** | ● | ○ | ● | ● | ● |
| **C7 DigitalTwinMapping** | ● | ● | ● | ● | ○ |

> **Legend**: ● = directly supported (the principle is directly embodied in the component); ○ = indirectly supported (the principle is embodied through composition).

### 6.2 Reading the matrix

**P1 plug-in logic**: directly supported by all seven components — each participates in plug-in composition via its semantic interface contract.

**P2 reversibility**: directly supported by all except LLMDispatchInterface and LayeredClassificationManager. LLM dispatch outputs carry intrinsic uncertainty, so reversibility is a conditional conclusion there; the layered classification manager's mapping may lose information (many-to-one mapping).

**P3 completeness**: directly supported by all seven — their union covers every core semantic dimension of the domain.

**P4 minimal atomicity**: directly supported by all seven — each satisfies the atomicity rules (single semantics, indecomposability, orthogonality).

**P5 dynamic attachment of spatio-temporal references**: directly supported by DataAccessAtom, GranularityPartitionEngine and LayeredClassificationManager; the remaining components acquire spatio-temporal reference capability indirectly by referencing DataAccessAtom.

### 6.3 Principle priority

| Priority | Principle | Reason |
|:---:|:---|:---|
| 1 | P4 minimal atomicity | the foundation of every other principle — without atomicity there is neither composability nor reversibility |
| 2 | P1 plug-in logic | defines composition rules; the core of language extensibility |
| 3 | P3 completeness | ensures the language covers domain needs; the precondition of practical usefulness |
| 4 | P2 reversibility | ensures digitization loses no semantics; the guarantee of trustworthiness |
| 5 | P5 dynamic attachment of spatio-temporal references | solves the domain-specific problem of unifying multi-source spatio-temporal data; the language's domain adaptability |

---

## Appendix: logical dependencies among the principles

```
P4 minimal atomicity
    │
    ├──▶ P1 plug-in logic (atomized interfaces are what plug-in composition needs)
    │       │
    │       └──▶ P3 completeness (the generative power of atomic combinations supports completeness)
    │
    ├──▶ P2 reversibility (atomic identifiability underpins the reversible mapping)
    │
    └──▶ P5 dynamic attachment of spatio-temporal references (atomized coordinate reference is the basis of spatio-temporal unification)
```

P4 is the foundation of all the principles: atomicity ensures the independence and identifiability of every semantic unit, making plug-in composition (P1), reversible mapping (P2), complete coverage (P3), and spatio-temporal unification (P5) possible.

---

*End of document*
