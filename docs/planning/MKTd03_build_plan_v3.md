# MKTd03 Build Plan v3

**Date:** 2026-03-26  
**Status:** Draft for review  
**Scope:** Opening design / ADR plan for the MKTd03 protocol-library build  
**Operating assumption:** Prep is closed. Repo-boundary cleanup is closed. Standards uplift is complete. TinyPress remains a later reference target only and must not shape protocol truth. The plan now incorporates C’s control fixes, especially on sequencing, tree-structure ADRs, interface-definition gates, and explicit review/approval checkpoints. 

---

## 0. Purpose of this plan

This plan defines the **first bounded design sequence** for the MKTd03 build from the clean post-prep baseline.

It is intentionally **spec-first, ADR-first, and interface-gated**, not code-first.

The immediate goal is **not** to begin broad Tree-mode implementation. The immediate goal is to establish a trustworthy, dApp-agnostic protocol baseline so later implementation work is bounded, reviewable, and aligned with the playbook, design principles, and the .did / interface-definition discipline already elevated during prep. 

This plan assumes:

- **G** drives the work and owns protocol/design decisions.
- **Codex** is used only for bounded inspection, drafting, editing, and later tightly scoped coding.
- **C** acts as adversarial reviewer at explicit checkpoints.
- **Repo ground truth** overrides memory, earlier chat summaries, or stale design notes.
- **TinyPress** is a later reference target only and must not leak into normative MKTd03 protocol design. 

---

## 1. Core framing

### 1.1 What changed since the original MKTd03 spec

The original spec in the MKTd03 spreadsheet is now **out of date**.

MKTd02 taught a large amount that must carry into MKTd03, including:

- ADR-before-implementation discipline,
- interface-definition sequencing before testing or downstream validation,
- explicit failure semantics,
- diagnostics/status surfaces from the outset,
- compatibility/versioning questions needing early answers,
- the danger of letting implementation details silently define protocol truth,
- the value of bounded Codex tasks and adversarial review. 

Accordingly, the old spreadsheet tab must now be treated as an **audit input**, not as an authority.

### 1.2 Product-family framing

The intended high-level relationship remains:

- **MKTd02 = Leaf mode**
- **MKTd03 = Tree mode**

That means a substantial amount of MKTd02 may be reusable or generalisable for MKTd03.

However, Tree mode is not a trivial delta. In particular:

- state representation changes materially,
- proof structure changes materially,
- CVDR shape and verifier requirements change materially,
- new invariants and new failure modes appear around tree state and Merkle-path reasoning. 

So the correct stance is:

> Assume reuse is desirable, but require it to be demonstrated module by module.

### 1.3 Opening-phase non-goals

The following are explicitly out of scope for this opening phase:

- reopening repo-boundary cleanup,
- reopening prep or standards work except for concrete regressions,
- making TinyPress-specific design decisions,
- broad Tree-mode implementation before protocol decisions are written down,
- allowing Codex to infer architecture from app code or partial implementations,
- authorising any MKTd02 back-port work merely because a reuse audit identifies a candidate. Back-port decisions remain out of scope and require separate sign-off. :contentReference[oaicite:5]{index=5}

### 1.4 Operational test of dApp-agnosticism

The library’s core logic must be **unit-testable without a running ICP canister**. If a test requires `dfx` deployment or a live canister, it is an integration test, not a protocol unit test. This is the operational test of dApp-agnosticism. 

---

## 2. Conceptual dependency hierarchy

This section defines **conceptual dependencies**, not the exact temporal execution sequence. The temporal sequence is defined in §10.

- **ADR-00 evidentiary scope** depends on knowing what the stale spec claimed, but not on the final refreshed spec note.
- **Spec refresh note** depends on ADR-00, because evidentiary scope filters what counts as retained, revised, or rejected protocol meaning.
- **Tree-mode invariants note** depends on ADR-00, because scope constrains which invariants matter.
- **ADR-02 tree-structure choice** depends on the Tree-mode invariants note.
- **ADR-01 library vs adapter boundary** depends on ADR-00 plus Tree-mode invariants; it is informed by tree-structure choice but need not wait for every later artifact.
- **ADR-03 Tree-mode CVDR structure and verifier requirements** depends on ADR-01 plus ADR-02.
- **MKTd02 reuse audit** depends on ADR-01, because reuse can only be judged once the generic library boundary is known.
- **Diagnostics / versioning / security / privacy artifacts** depend on ADR-01, because the library boundary must be known before those surfaces can be fixed.
- **Protocol surface + formal interface definition files** depend on the earlier ADRs and design notes.
- **Golden vectors** depend especially on ADR-02 and ADR-03.
- **Code** depends on reviewed formal interface definition files plus golden vectors. 

---

## 3. Required opening design artifacts

The following artifacts should exist before substantive MKTd03 library coding begins.

### 3.1 ADR-00 — Evidentiary scope and claims

**Title:**  
`ADR-00: What MKTd03 Tree mode proves, and what it does not prove`

**Purpose:**  
To pin the cryptographic/evidentiary claim surface before CVDR shape, verifier requirements, or proof objects drift.

**This ADR must decide:**

- what an MKTd03 receipt/proof proves cryptographically,
- what remains an application assertion,
- what remains an integration assertion,
- what remains live-only vs archival/offline,
- what unit of proof is in scope,
- what negative claims are explicitly out of scope,
- what privacy assumptions and residue assumptions must be made explicit. 

**Authority boundary:**  
ADR-00 is authoritative for **claims and evidentiary scope**. The later spec refresh note is authoritative for **carry-over, deltas, and stale-assumption cleanup**.

---

### 3.2 Stale-spec inventory and spec refresh note

**Suggested files:**
- `docs/spec/MKTd03_stale_spec_inventory_v1.md`
- `docs/spec/MKTd03_protocol_refresh_v1.md`

**Purpose:**  
First inventory what the old spreadsheet spec claimed. Then, after ADR-00 and the early Tree-mode decisions, produce a refreshed protocol source-of-truth note.

**Inventory must contain:**

- keep / revise / drop / unresolved classification for each meaningful stale-spec assumption,
- notes on omitted failure semantics,
- missing compatibility/versioning handling,
- missing diagnostics,
- app-specific assumptions that do not belong in protocol truth.

**Refresh note must contain:**

#### A. What remains unchanged from MKTd02
- concepts that remain valid,
- lifecycle/state-management ideas that remain valid,
- diagnostics/status expectations that carry forward,
- compatibility/versioning expectations that carry forward,
- reusable or likely reusable protocol machinery.

#### B. What changes in MKTd03 because of Tree mode
- committed tree-state model,
- root / path / proof object implications,
- Tree-mode CVDR implications,
- new invariants,
- new failure modes,
- any new state-lifecycle transitions introduced by Tree mode.

#### C. What the old spreadsheet spec got wrong or underspecified
- stale assumptions,
- overbroad claims,
- omitted failure semantics,
- omitted verifier requirements,
- omitted diagnostics/versioning surfaces.

#### D. Open questions
- unresolved protocol questions,
- unresolved proof-shape questions,
- unresolved module-boundary questions,
- unresolved compatibility questions.

**Important rule:**  
The spreadsheet tab is audited first, then superseded. It is not updated blindly. 

---

### 3.3 Tree-mode invariants note

**Suggested file:**  
`docs/spec/MKTd03_tree_mode_invariants_v1.md`

**Purpose:**  
To isolate the genuinely new hard part of MKTd03 instead of burying it inside general prose.

**This note must define:**

- what the committed tree state represents,
- what pre-state means in Tree mode,
- what post-state means in Tree mode,
- what constitutes a valid state transition,
- what witness/proof object properties are invariant at the protocol level,
- how absence / tombstone / subject-scope reasoning is modelled at the protocol level,
- what invariants must hold across versions,
- what invariants are library-owned vs adapter-owned.

**Hard rule:**  
ICP-specific deployment mechanics such as canister initialization details, upgrade freeze procedure, and cycle-management concerns must **not** be encoded as generic protocol invariants. Those belong in a separate deployment/integration note. The invariants note covers only cryptographic and protocol-logical properties that hold regardless of deployment platform. 

---

### 3.4 ADR-02 — Cryptographic tree structure choice

**Title:**  
`ADR-02: Cryptographic tree structure for MKTd03 Tree mode`

**Purpose:**  
To decide the concrete tree structure before CVDR shape, verifier requirements, and vectors are defined.

**This ADR must decide:**

- which tree structure MKTd03 uses,
- why that structure is chosen,
- what the proof object / witness shape looks like,
- what absence-proof semantics look like,
- what the verification algorithm requires,
- what alternatives were considered and rejected. 

**Rationale:**  
This was missing in v2 and is now promoted to a first-class ADR because it is a prerequisite for CVDR structure and golden vectors.

---

### 3.5 ADR-01 — Library vs adapter boundary

**Title:**  
`ADR-01: Generic library vs application-adapter boundary for MKTd03 Tree mode`

**Purpose:**  
To define what the MKTd03 protocol library owns and what remains application-specific.

**This ADR must decide:**

#### Library-owned responsibilities
- generic Tree-mode state-transition logic,
- generic proof/receipt artifact logic,
- generic invariants enforcement,
- generic lifecycle/diagnostics/version surface,
- generic error taxonomy,
- generic stable identifiers and protocol-version behavior.

#### Adapter-owned responsibilities
- application-specific subject/record discovery,
- application-specific mutation execution,
- application-specific enumeration of affected data surfaces,
- application-specific traversal/mapping logic,
- app-specific orchestration across surfaces or canisters.

#### Boundary constraints
- what the adapter must provide,
- what the library must never assume,
- what information flows from adapter to library,
- what proof-bearing objects flow from library to caller/verifier.

#### Minimal diagnostics/status interface — library-owned
- the minimal diagnostics/status surface must appear here at least in sketch form,
- Phase 6 later formalises it, but ADR-01 establishes that it is a library responsibility. 

#### Failure semantics
- stale pre-state,
- unsupported adapter capability,
- incomplete mutation,
- unsupported protocol/schema version,
- invalid proof input,
- not-safe-to-rerun conditions,
- idempotent re-entry conditions.

#### Trust/privacy boundary
- what the library can legitimately assert,
- what remains external or app-specific,
- what data-minimisation expectations apply,
- what must not be leaked through diagnostics or query surfaces.

#### Conceptual adapter contract artifact
Phase 3 must also produce a named conceptual adapter contract specification, derived from ADR-01, which later becomes a formal interface definition artifact in Phase 6. :contentReference[oaicite:13]{index=13}

---

### 3.6 ADR-03 — Tree-mode CVDR structure and verifier requirements

**Title:**  
`ADR-03: Tree-mode CVDR structure and verifier requirements`

**Purpose:**  
To define the artifact-level continuation of ADR-00 once the library boundary and tree structure are known.

**This ADR must decide:**

- CVDR fields for Tree mode,
- what the verifier can independently check,
- what requires trust in the adapter,
- what requires trust in deployment/integration context,
- what the receipt does and does not prove,
- what verifier inputs are mandatory,
- what the verification algorithm requires from the proof object. 

**Rationale:**  
MKTd03 needs a dedicated verifier-requirements artifact early, not as a Phase 7 afterthought.

---

## 4. MKTd02 reuse and generalisation work

### 4.1 Goal

Treat MKTd02 reuse as a **design hypothesis to test**, not a slogan.

### 4.2 Required artifact: module taxonomy

**Suggested file:**  
`docs/analysis/MKTd02_module_taxonomy_for_MKTd03.md`

Before doing the reuse audit, classify MKTd02 modules into categories such as:

- receipt / artifact identity,
- hashing / tagging / deterministic encoding,
- sequencing / lifecycle state,
- finalization / orchestration,
- diagnostics / status surface,
- storage abstraction,
- verifier-facing shared semantics,
- Leaf-mode-specific semantics.

This helps separate “currently implemented in MKTd02” from “intrinsically Leaf-mode-specific.” :contentReference[oaicite:15]{index=15}

### 4.3 Required artifact: reuse audit

**Suggested file:**  
`docs/analysis/MKTd02_module_reuse_audit_v1.md`

For each meaningful MKTd02 module, classify it as:

- **Reusable as-is**
- **Reusable with parameterisation/generalisation**
- **Conceptually reusable but currently too hard-coded**
- **Leaf-mode-specific; do not reuse**
- **Unknown pending deeper review**

For each module, answer:

- Does it encode protocol truth or only MKTd02 integration truth?
- Does it assume a single-record Leaf model?
- Does it assume a Leaf-mode CVDR shape?
- Does it hard-code hash preimages, identifiers, sequencing, or storage assumptions that should be abstracted?
- Does it align with desired diagnostics/versioning/failure semantics?
- Does it belong in a shared reusable core?

### 4.4 Generalise now / back-port later list

A separate section or file must list modules that:

- **should** be reusable in principle,
- but currently are not, due to hard-coding or MKTd02-specific assumptions.

For each such module, record:

- current name/location,
- why it is too specific,
- what the reusable abstraction should look like,
- whether MKTd02 should later adopt the generalised version,
- the likely back-port cost/risk.

**Scope restriction:**  
Listing a module as a back-port candidate does **not** authorise any changes to MKTd02. Back-port decisions are out of scope for this phase, require separate analysis, and require explicit sign-off before action. :contentReference[oaicite:16]{index=16}

---

## 5. Plan-specific operating rules

The general role model comes from the playbook. For this plan, the additions that matter most are:

- Codex must be kept on bounded prompts; it must not infer architecture or “bring everything into line” across multiple repos. :contentReference[oaicite:17]{index=17}
- TinyPress is a reference target only; protocol truth must remain independent of app-local semantics and API choices. 
- Interface definition files must exist before implementation begins; spec prose alone is not a sufficient target. This sequencing rule was already elevated during prep and in the standards/playbook refresh. 

---

## 6. Phase-gated design notes before implementation

### 6.1 Diagnostics/status design note

Produce a named artifact, for example:

`docs/spec/MKTd03_diagnostics_status_v1.md`

This must define:

- minimal status query surface,
- lifecycle states,
- pending / in-progress / completed / failure states,
- counts and diagnostic counters where appropriate,
- protocol/schema/build identity exposure,
- what is intentionally excluded for privacy/security reasons. 

### 6.2 Compatibility/versioning design note

Produce a named artifact, for example:

`docs/spec/MKTd03_compatibility_versioning_v1.md`

This must answer:

- what version fields exist,
- which artifacts version independently,
- what unsupported-version behavior looks like,
- when internal changes remain allowed,
- how deprecation will be signalled,
- what fail-loud behavior applies to version mismatch.

### 6.3 Security design note

Produce a named artifact, for example:

`docs/spec/MKTd03_security_design_v1.md`

This must address:

- replay risk,
- fake-proof risk,
- stale-proof / stale-state risk,
- tampering risk,
- ambiguous subject-scope risk,
- cross-version confusion risk,
- unsafe rerun/re-entry scenarios,
- misuse of query surfaces. 

### 6.4 Privacy design note

Produce a named artifact, for example:

`docs/spec/MKTd03_privacy_design_v1.md`

This must address:

- whether proof objects reveal more than intended,
- whether diagnostics leak sensitive structure,
- whether adapter requirements encourage over-collection,
- whether verifier routes differ in privacy exposure,
- what the design assumes about identifiers and residue. 

---

## 7. Implementation-facing protocol surface and formal interface files

Only after the earlier ADRs and design notes are in place should the first implementation-facing protocol surface be written.

### 7.1 Protocol surface note

**Suggested file:**  
`docs/spec/MKTd03_protocol_surface_v1.md`

It should define, at minimum:

- core object vocabulary,
- stable identifiers,
- protocol-meaningful object boundaries,
- error categories,
- lifecycle states,
- required diagnostic/status queries,
- version fields,
- Tree-mode artifact vocabulary,
- which fields are normative and which are advisory.

### 7.2 Formal interface definition files

Before implementation begins, both of the following must exist in reviewed form:

- a formal interface definition file for the **library’s public surface**,
- a formal interface definition file for the **adapter contract**.

Use `.did` or an equivalent formal interface notation, but the contract must exist as a formal artifact, not just prose. This is now an explicit implementation gate. 

---

## 8. Golden vectors and negative cases before code

### 8.1 Golden vectors

Create a pre-code vector pack, for example:

`docs/test-vectors/MKTd03_golden_vectors_v1.md`

This should include examples for:

- leaf hash construction,
- path recomputation,
- root recomputation,
- transition object hashing,
- receipt/proof identity derivation,
- byte order and deterministic encoding rules,
- version-tag behavior where applicable.

The playbook already requires explicit encoding conventions and at least one golden vector per cryptographic formula. :contentReference[oaicite:24]{index=24}

### 8.2 Negative cases

The same stage should define negative cases such as:

- wrong sibling/path element,
- wrong root,
- wrong version,
- stale pre-state,
- replayed transition,
- malformed proof object,
- adapter-reported wrong scope,
- unsupported-version object.

---

## 9. TinyPress containment rule

TinyPress remains a later **reference target only**.

Accordingly, TinyPress must not define any of the following in normative MKTd03 design artifacts:

- protocol nouns,
- proof-object names,
- state-transition examples,
- adapter contract semantics,
- core library boundary decisions,
- CVDR semantics,
- reusable module names or abstractions.

If TinyPress examples are later used at all, they belong in a clearly separated **reference-target / worked-example** section, never in the normative protocol core. This is consistent with the repo boundary and AGENTS guidance already established for MKTd03 and TinyPress. 

---

## 10. Phased execution sequence

This section governs the **actual temporal sequence**.

### Phase 1a — Re-anchor and stale-spec inventory
**Outputs:**
- current repo-state anchor,
- keep / revise / drop / unresolved matrix for the spreadsheet tab,
- `MKTd03_stale_spec_inventory_v1.md`

**Exit gate:**
- stale spreadsheet assumptions are classified,
- no refreshed spec note is written yet,
- G reviews artifacts and approves phase close before Phase 2 begins. 

---

### Phase 2 — Evidentiary scope, Tree-mode invariants, and tree-structure choice
**Outputs:**
- `ADR-00: What MKTd03 proves, and what it does not prove`
- `MKTd03_tree_mode_invariants_v1.md`
- `ADR-02: Cryptographic tree structure for MKTd03 Tree mode`

**Exit gate:**
- proof scope is explicitly bounded,
- Tree-mode invariants are written down,
- tree structure and rejected alternatives are documented,
- G reviews artifacts and approves phase close before Phase 3 begins.

---

### Phase 3 — Spec refresh and library/adapter boundary
**Outputs:**
- `MKTd03_protocol_refresh_v1.md`
- `ADR-01: Generic library vs application-adapter boundary for MKTd03 Tree mode`
- conceptual adapter contract specification

**Exit gate:**
- spec refresh note is now informed by ADR-00 and ADR-02,
- responsibilities are split clearly,
- minimal diagnostics/status interface is sketched inside ADR-01,
- conceptual adapter contract exists,
- G reviews artifacts and approves phase close before Phase 4 begins. 

---

### Phase 4 — CVDR structure and MKTd02 reuse/generalisation audit
**Outputs:**
- `ADR-03: Tree-mode CVDR structure and verifier requirements`
- module taxonomy note,
- `MKTd02_module_reuse_audit_v1.md`,
- “generalise now / back-port later” list

**Exit gate:**
- Tree-mode CVDR structure is defined,
- verifier requirements are explicit,
- every meaningful MKTd02 module is classified,
- reusable/generalise/Leaf-specific distinctions are written down,
- G reviews artifacts and approves phase close before Phase 5 begins.

---

### Phase 5 — Diagnostics/versioning/security/privacy artifacts
**Outputs:**
- diagnostics/status design note,
- compatibility/versioning design note,
- security design note,
- privacy design note

**Exit gate:**
- all gate questions have explicit answers,
- fail-loud behavior is defined,
- named security/privacy artifacts exist,
- G reviews artifacts and approves phase close before Phase 6 begins. 

---

### Phase 6 — Protocol surface, formal interface files, and vectors
**Outputs:**
- `MKTd03_protocol_surface_v1.md`
- formal interface definition file for library surface
- formal interface definition file for adapter contract
- `MKTd03_golden_vectors_v1.md`

**Exit gate:**
- protocol surface exists,
- both formal interface definition files exist and are reviewed,
- positive and negative vector/test cases exist,
- implementation target is now bounded enough for Codex,
- G reviews artifacts and approves phase close before Phase 7 begins. 

---

### Phase 7 — Begin bounded code slices
Only after the above phases are complete should implementation begin.

Suggested implementation order:

1. reusable/generalised module extraction where justified,  
2. Tree-mode state scaffold,  
3. diagnostics/status surface from day 1,  
4. first proof/artifact path,  
5. later reference-target integration.

Each implementation slice should be narrow enough for isolated review and reversal.

---

## 11. Concrete session sequence and review triggers

### Session 1
- Re-anchor live repo state.
- Review the MKTd03 spreadsheet tab.
- Produce keep / revise / drop / unresolved matrix.
- Draft `MKTd03_stale_spec_inventory_v1.md`.

### Session 2
- Draft ADR-00.
- Draft Tree-mode invariants note.
- Draft ADR-02 tree-structure choice.
- **C review trigger:** first adversarial review occurs here, covering ADR-00 + invariants + ADR-02. 

### Session 3
- Draft `MKTd03_protocol_refresh_v1.md`.
- Draft ADR-01 library vs adapter boundary.
- Draft conceptual adapter contract spec.
- **C review trigger:** second adversarial review occurs here, focused on ADR-01 and the refreshed boundary design. 

### Session 4
- Draft ADR-03 Tree-mode CVDR structure and verifier requirements.
- Build MKTd02 module taxonomy.
- Run reuse/generalisation audit.
- Produce “generalise now / back-port later” list.

### Session 5
- Draft diagnostics/versioning/security/privacy notes.
- Lock gates.

### Session 6
- Draft implementation-facing protocol surface.
- Draft formal interface definition files.
- Draft golden vectors and negative cases.
- **C review trigger:** first action is the pre-implementation adversarial gate review before any coding starts. 

---

## 12. Summary judgment

The opening MKTd03 build should **not** begin with broad library coding.

It should begin by:

1. inventorying the stale spreadsheet spec,  
2. defining what MKTd03 actually claims to prove,  
3. isolating Tree-mode invariants,  
4. choosing the cryptographic tree structure,  
5. refreshing the protocol spec from those decisions,  
6. defining the generic library vs adapter boundary,  
7. defining Tree-mode CVDR structure and verifier requirements,  
8. auditing MKTd02 for reuse/generalisation opportunities,  
9. locking diagnostics/versioning/security/privacy artifacts,  
10. writing formal interface files plus golden vectors,  
11. and only then starting bounded implementation.

That is the cleanest route to a dApp-agnostic MKTd03 protocol-library build that learns from MKTd02 without allowing either MKTd02 implementation history or TinyPress integration details to silently define MKTd03.
