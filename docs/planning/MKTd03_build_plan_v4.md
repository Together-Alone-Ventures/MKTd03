# MKTd03 Build Plan v4

**Date:** 2026-03-26  
**Status:** Draft for review  
**Scope:** Opening design / ADR plan for the MKTd03 protocol-library build  
**Operating assumption:** Prep is closed. Repo-boundary cleanup is closed. Standards uplift is complete. TinyPress remains a later reference target only and must not shape protocol truth.

---

## 0. Purpose of this plan

This plan defines the **first bounded design sequence** for the MKTd03 build from the clean post-prep baseline.

It is intentionally **spec-first, ADR-first, and interface-gated**, not code-first.

The immediate goal is **not** to begin broad Tree-mode implementation. The immediate goal is to establish a trustworthy, dApp-agnostic protocol baseline so later implementation work is bounded, reviewable, and aligned with the playbook and design principles.

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
- authorising any MKTd02 back-port work merely because a reuse audit identifies a candidate.

Back-port decisions remain out of scope for this phase, require separate analysis, and require explicit sign-off before action.

### 1.4 Operational test of dApp-agnosticism

The library’s core logic must be **unit-testable without a running ICP canister**. If a test requires `dfx` deployment or a live canister, it is an integration test, not a protocol unit test. This is the operational test of dApp-agnosticism.

---

## 2. Conceptual dependency hierarchy

This section defines **conceptual dependencies**, not the exact temporal execution sequence. The temporal sequence is defined in §11.

- **ADR-00 evidentiary scope** depends on knowing what the stale spec claimed, but not on the final refreshed spec note.
- **Stale-spec inventory** depends only on reviewing the old spreadsheet material and related MKTd02 lessons.
- **Tree-mode invariants note** depends on ADR-00, because scope constrains which invariants matter.
- **ADR-02 tree-structure choice** depends on the Tree-mode invariants note.
- **ADR-01 library vs adapter boundary** depends on ADR-00 plus Tree-mode invariants; it is informed by ADR-02.
- **ADR-03 Tree-mode CVDR structure and verifier requirements** depends on ADR-01 plus ADR-02.
- **Spec refresh note** depends on ADR-00, ADR-02, and ADR-03. It must not define detailed CVDR semantics before ADR-03 exists.
- **MKTd02 reuse audit** depends on ADR-01, because reuse can only be judged once the generic library boundary is known.
- **Diagnostics / versioning / security / privacy artifacts** depend on ADR-01.
- **Protocol surface + formal interface definition files** depend on the earlier ADRs and design notes.
- **Golden vectors** depend especially on ADR-02 and ADR-03.
- **Code** depends on reviewed formal interface definition files, approved golden vectors, and approved negative cases.

### 2.1 ADR numbering note

ADR numbering is **thematic**, not temporal. Temporal execution order is governed only by §11.

---

## 3. Authority map

A short authority map must be created early and kept current.

**Path:**  
`docs/planning/MKTd03_authority_map_v1.md`

Its purpose is to state, in one place, which artifact is authoritative for which subject.

At minimum it must assign authority for:

- evidentiary claims,
- Tree-mode invariants,
- tree-structure choice,
- library vs adapter boundary,
- Tree-mode CVDR structure,
- diagnostics/status behavior,
- compatibility/versioning behavior,
- security constraints,
- privacy constraints,
- public library interface,
- adapter contract interface,
- test vectors and negative cases.

This exists to prevent later agents from “reconciling” overlapping prose on their own.

---

## 4. Required opening design artifacts

The following artifacts should exist before substantive MKTd03 library coding begins.

### 4.1 ADR-00 — Evidentiary scope and claims

**Path:**  
`docs/adr/ADR-00-evidentiary-scope.md`

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
ADR-00 is authoritative for **claims and evidentiary scope**.

---

### 4.2 Stale-spec inventory

**Path:**  
`docs/spec/MKTd03_stale_spec_inventory_v1.md`

**Purpose:**  
To inventory what the old spreadsheet spec claimed before it is superseded.

**This note must contain:**

- keep / revise / drop / unresolved classification for each meaningful stale-spec assumption,
- notes on omitted failure semantics,
- missing compatibility/versioning handling,
- missing diagnostics,
- app-specific assumptions that do not belong in protocol truth.

**Authority boundary:**  
This artifact is authoritative only for the **inventory of stale assumptions**, not for current protocol truth.

---

### 4.3 Tree-mode invariants note

**Path:**  
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
ICP-specific deployment mechanics such as canister initialization details, upgrade freeze procedure, and cycle-management concerns must **not** be encoded as generic protocol invariants. Those belong in a separate deployment/integration note if later needed.

**Authority boundary:**  
This note is authoritative for **protocol-logical invariants**, not deployment-specific procedures.

---

### 4.4 ADR-02 — Cryptographic tree structure choice

**Path:**  
`docs/adr/ADR-02-tree-structure-choice.md`

**Purpose:**  
To decide the concrete tree structure before CVDR shape, verifier requirements, and vectors are defined.

**This ADR must decide:**

- which tree structure MKTd03 uses,
- why that structure is chosen,
- what the proof object / witness shape looks like,
- what absence-proof semantics look like,
- what the verification algorithm requires,
- what alternatives were considered and rejected.

**Authority boundary:**  
ADR-02 is authoritative for **tree-structure choice and proof-object shape at the tree level**.

---

### 4.5 ADR-01 — Library vs adapter boundary

**Path:**  
`docs/adr/ADR-01-library-vs-adapter-boundary.md`

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
- later formal interface files will define the final surface.

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
Phase 3 must also produce a named conceptual adapter contract specification derived from ADR-01.

**Related conceptual artifact path:**  
`docs/spec/MKTd03_adapter_contract_concept_v1.md`

**Authority boundary:**  
ADR-01 is authoritative for **ownership split and boundary semantics**. The conceptual contract note is explanatory and preparatory, not the final formal interface.

---

### 4.6 ADR-03 — Tree-mode CVDR structure and verifier requirements

**Path:**  
`docs/adr/ADR-03-tree-mode-cvdr-structure.md`

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

**Authority boundary:**  
ADR-03 is authoritative for **Tree-mode CVDR structure and verifier requirements**.

---

### 4.7 Spec refresh note

**Path:**  
`docs/spec/MKTd03_protocol_refresh_v1.md`

**Purpose:**  
To replace the stale spreadsheet tab with a current protocol source-of-truth note after the key ADRs are in place.

**This note must contain:**

#### A. What remains unchanged from MKTd02
- concepts that remain valid,
- lifecycle/state-management ideas that remain valid,
- diagnostics/status expectations that carry forward,
- compatibility/versioning expectations that carry forward,
- reusable or likely reusable protocol machinery.

#### B. What changes in MKTd03 because of Tree mode
- committed tree-state model,
- root / path / proof object implications,
- high-level CVDR implications only, consistent with ADR-03,
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

**Authority boundary:**  
This note is authoritative for the **refreshed overall protocol narrative**, but not where a more specific ADR or formal interface file governs.

---

## 5. MKTd02 reuse and generalisation work

### 5.1 Goal

Treat MKTd02 reuse as a **design hypothesis to test**, not a slogan.

### 5.2 Required artifact: module taxonomy

**Path:**  
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

This helps separate “currently implemented in MKTd02” from “intrinsically Leaf-mode-specific.”

### 5.3 Required artifact: reuse audit

**Path:**  
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

### 5.4 Generalise now / back-port later candidate list

**Path:**  
`docs/analysis/MKTd02_generalise_now_backport_later_candidates_v1.md`

This file lists modules that:

- **should** be reusable in principle,
- but currently are not, due to hard-coding or MKTd02-specific assumptions.

For each such module, record:

- current name/location,
- why it is too specific,
- what the reusable abstraction should look like,
- whether MKTd02 should later adopt the generalised version,
- the likely back-port cost/risk.

**Scope restriction:**  
This artifact is a **candidate list only**. It does not authorise edits to MKTd02, creation of MKTd02 tasks, or parallel back-port work during this phase.

---

## 6. Plan-specific operating rules

- Codex must be kept on bounded prompts; it must not infer architecture or “bring everything into line” across multiple repos.
- TinyPress is a reference target only; protocol truth must remain independent of app-local semantics and API choices.
- Interface definition files must exist before implementation begins; spec prose alone is not a sufficient target.
- Example names, examples, and worked scenarios in normative protocol artifacts must be generic. TinyPress-specific worked examples, if later useful, belong only in explicitly separated reference-target materials.
- MKTd02 historical implementation details may be consulted for reuse analysis, but they are not authoritative for MKTd03 design.

---

## 7. Phase-gated design notes before implementation

### 7.1 Diagnostics/status design note

**Path:**  
`docs/spec/MKTd03_diagnostics_status_v1.md`

This must define:

- minimal status query surface,
- lifecycle states,
- pending / in-progress / completed / failure states,
- counts and diagnostic counters where appropriate,
- protocol/schema/build identity exposure,
- what is intentionally excluded for privacy/security reasons.

### 7.2 Compatibility/versioning design note

**Path:**  
`docs/spec/MKTd03_compatibility_versioning_v1.md`

This must answer:

- what version fields exist,
- which artifacts version independently,
- what unsupported-version behavior looks like,
- when internal changes remain allowed,
- how deprecation will be signalled,
- what fail-loud behavior applies to version mismatch.

### 7.3 Security design note

**Path:**  
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

### 7.4 Privacy design note

**Path:**  
`docs/spec/MKTd03_privacy_design_v1.md`

This must address:

- whether proof objects reveal more than intended,
- whether diagnostics leak sensitive structure,
- whether adapter requirements encourage over-collection,
- whether verifier routes differ in privacy exposure,
- what the design assumes about identifiers and residue.

---

## 8. Implementation-facing protocol surface and formal interface files

Only after the earlier ADRs and design notes are in place should the first implementation-facing protocol surface be written.

### 8.1 Protocol surface note

**Path:**  
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

### 8.2 Formal interface definition files

Before implementation begins, the following must exist in reviewed form:

- **library public interface definition**  
  Path: `interfaces/mktd03_library.did`
- **adapter contract interface definition**  
  Path: `interfaces/mktd03_adapter_contract.did`

If `.did` proves insufficient for some semantics, those semantics must be captured in adjacent normative companion notes, not left implicit.

**Companion note paths:**
- `interfaces/mktd03_library_interface_rules.md`
- `interfaces/mktd03_adapter_contract_rules.md`

### 8.3 What must be formalized before code begins

The formal interface layer plus companion rules must define, at minimum:

#### Library public interface
- operation names,
- operation inputs and outputs,
- error/result shapes,
- status/diagnostic query surface,
- version-reporting surface.

#### Adapter contract
- required adapter-provided inputs,
- required callback or invocation surface if any,
- required object shapes crossing the boundary,
- error propagation expectations,
- retry / idempotency semantics at the boundary.

#### Companion-rule material where interface notation is insufficient
- encoding rules,
- hashing domain-separation requirements,
- deterministic ordering rules,
- proof-object validation rules,
- any invariant that must be enforced across interface boundaries.

**Authority boundary:**  
The `.did` files plus companion-rule notes are authoritative for the **public callable interface and contract semantics at code start**.

---

## 9. Golden vectors and negative cases before code

### 9.1 Human-readable vector note

**Path:**  
`docs/test-vectors/MKTd03_golden_vectors_v1.md`

This should include examples for:

- leaf hash construction,
- path recomputation,
- root recomputation,
- transition object hashing,
- receipt/proof identity derivation,
- byte order and deterministic encoding rules,
- version-tag behavior where applicable.

### 9.2 Machine-readable vector fixtures

**Path:**  
`docs/test-vectors/fixtures/`

This directory must contain machine-readable vector fixtures for the first implementation slice.

Preferred formats:
- `.json`
- `.yaml`

### 9.3 Negative cases

**Path:**  
`docs/test-vectors/MKTd03_negative_cases_v1.md`

This must define negative cases such as:

- wrong sibling/path element,
- wrong root,
- wrong version,
- stale pre-state,
- replayed transition,
- malformed proof object,
- adapter-reported wrong scope,
- unsupported-version object.

**Machine-readable negative fixtures path:**  
`docs/test-vectors/fixtures/negative/`

---

## 10. TinyPress containment rule

TinyPress remains a later **reference target only**.

Accordingly, TinyPress must not define any of the following in normative MKTd03 design artifacts:

- protocol nouns,
- proof-object names,
- state-transition examples,
- adapter contract semantics,
- core library boundary decisions,
- CVDR semantics,
- reusable module names or abstractions.

If TinyPress examples are later used at all, they belong in a clearly separated **reference-target / worked-example** section, never in the normative protocol core.

---

## 11. Phased execution sequence

This section governs the **actual temporal sequence**.

### Phase 1a — Re-anchor and stale-spec inventory
**Outputs:**
- current repo-state anchor,
- keep / revise / drop / unresolved matrix for the spreadsheet tab,
- `docs/spec/MKTd03_stale_spec_inventory_v1.md`,
- `docs/planning/MKTd03_authority_map_v1.md` initial stub.

**Exit gate:**
- stale spreadsheet assumptions are classified,
- no refreshed spec note is written yet,
- authority map exists at least as a stub,
- G reviews artifacts and approves phase close before Phase 2 begins.

---

### Phase 2 — Evidentiary scope, Tree-mode invariants, and tree-structure choice
**Outputs:**
- `docs/adr/ADR-00-evidentiary-scope.md`
- `docs/spec/MKTd03_tree_mode_invariants_v1.md`
- `docs/adr/ADR-02-tree-structure-choice.md`

**Exit gate:**
- ADR-00 exists and is reviewed,
- Tree-mode invariants note exists and is reviewed,
- ADR-02 exists and includes rejected alternatives,
- authority map is updated,
- G reviews artifacts and approves phase close before Phase 3 begins.

---

### Phase 3 — Library/adapter boundary and Tree-mode CVDR
**Outputs:**
- `docs/adr/ADR-01-library-vs-adapter-boundary.md`
- `docs/spec/MKTd03_adapter_contract_concept_v1.md`
- `docs/adr/ADR-03-tree-mode-cvdr-structure.md`

**Exit gate:**
- ADR-01 exists and is reviewed,
- conceptual adapter contract note exists,
- ADR-03 exists and is reviewed,
- authority map is updated,
- G reviews artifacts and approves phase close before Phase 4 begins.

---

### Phase 4 — Spec refresh and MKTd02 reuse/generalisation audit
**Outputs:**
- `docs/spec/MKTd03_protocol_refresh_v1.md`
- `docs/analysis/MKTd02_module_taxonomy_for_MKTd03.md`
- `docs/analysis/MKTd02_module_reuse_audit_v1.md`
- `docs/analysis/MKTd02_generalise_now_backport_later_candidates_v1.md`

**Exit gate:**
- refresh note is consistent with ADR-00, ADR-02, and ADR-03,
- every meaningful MKTd02 module is classified,
- candidate list is explicitly marked non-authorising,
- authority map is updated,
- G reviews artifacts and approves phase close before Phase 5 begins.

---

### Phase 5 — Diagnostics/versioning/security/privacy artifacts
**Outputs:**
- `docs/spec/MKTd03_diagnostics_status_v1.md`
- `docs/spec/MKTd03_compatibility_versioning_v1.md`
- `docs/spec/MKTd03_security_design_v1.md`
- `docs/spec/MKTd03_privacy_design_v1.md`

**Exit gate:**
- all four named artifacts exist,
- each artifact states explicit normative decisions, not only questions,
- fail-loud behavior is defined in the compatibility/versioning note,
- status surface is defined in the diagnostics note,
- security note addresses replay/fake-proof/stale-state/tampering/cross-version confusion,
- privacy note addresses proof leakage/diagnostic leakage/identifier minimisation,
- authority map is updated,
- G reviews artifacts and approves phase close before Phase 6 begins.

---

### Phase 6 — Protocol surface, formal interface files, vectors, and pre-code review gate
**Outputs:**
- `docs/spec/MKTd03_protocol_surface_v1.md`
- `interfaces/mktd03_library.did`
- `interfaces/mktd03_adapter_contract.did`
- `interfaces/mktd03_library_interface_rules.md`
- `interfaces/mktd03_adapter_contract_rules.md`
- `docs/test-vectors/MKTd03_golden_vectors_v1.md`
- `docs/test-vectors/MKTd03_negative_cases_v1.md`
- machine-readable fixtures under `docs/test-vectors/fixtures/`

**Exit gate:**
- both `.did` files exist and are reviewed,
- both companion-rule notes exist and are reviewed,
- protocol surface note exists and is reviewed,
- human-readable golden vectors exist,
- human-readable negative cases exist,
- machine-readable positive fixtures exist,
- machine-readable negative fixtures exist,
- authority map is updated,
- C completes the pre-implementation adversarial review,
- G explicitly approves coding start.

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

## 12. Concrete session sequence and review triggers

### Session 1
- Re-anchor live repo state.
- Review the MKTd03 spreadsheet tab.
- Produce keep / revise / drop / unresolved matrix.
- Draft `docs/spec/MKTd03_stale_spec_inventory_v1.md`.
- Draft initial `docs/planning/MKTd03_authority_map_v1.md`.

### Session 2
- Draft ADR-00.
- Draft Tree-mode invariants note.
- Draft ADR-02 tree-structure choice.
- **C review trigger:** adversarial review of ADR-00 + invariants + ADR-02.

### Session 3
- Draft ADR-01 library vs adapter boundary.
- Draft conceptual adapter contract spec.
- Draft ADR-03 Tree-mode CVDR structure and verifier requirements.
- **C review trigger:** adversarial review of ADR-01 + ADR-03.

### Session 4
- Draft `docs/spec/MKTd03_protocol_refresh_v1.md`.
- Build MKTd02 module taxonomy.
- Run reuse/generalisation audit.
- Produce candidate list.

### Session 5
- Draft diagnostics/versioning/security/privacy notes.
- Update authority map.
- Lock Phase 5 gates.

### Session 6
- Draft protocol surface note.
- Draft formal interface definition files.
- Draft companion-rule notes.
- Draft human-readable vectors and negative cases.
- Create machine-readable fixtures.
- **C review trigger:** pre-implementation adversarial gate review.

### Session 7
- Only if Phase 6 passes, begin first bounded coding slice.

---

## 13. Summary judgment

The opening MKTd03 build should **not** begin with broad library coding.

It should begin by:

1. inventorying the stale spreadsheet spec,  
2. defining what MKTd03 actually claims to prove,  
3. isolating Tree-mode invariants,  
4. choosing the cryptographic tree structure,  
5. defining the library vs adapter boundary,  
6. defining Tree-mode CVDR structure and verifier requirements,  
7. refreshing the protocol spec from those decisions,  
8. auditing MKTd02 for reuse/generalisation opportunities,  
9. locking diagnostics/versioning/security/privacy artifacts,  
10. writing formal interface files plus companion rules plus vectors,  
11. passing a pre-implementation adversarial review gate,  
12. and only then starting bounded implementation.

That is the cleanest route to a dApp-agnostic MKTd03 protocol-library build that learns from MKTd02 without allowing either MKTd02 implementation history or TinyPress integration details to silently define MKTd03.
