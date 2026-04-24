# MKTd02 Generalise-Now / Back-Port-Later Candidates v1

**Status:** Approved analysis (G review, 2026-04-24, §5 reuse-audit close-out).
**Path:** `docs/analysis/MKTd02_generalise_now_backport_later_candidates_v1.md`
**Build plan reference:** §5.4
**Drafted by:** C (Claude), at G's explicit request per session role-swap allowance.
**Draft history:** v1 (`MKTd02_generalise_now_backport_later_candidates_v1_DRAFT.md`) → v2 (`MKTd02_generalise_now_backport_later_candidates_v1_DRAFT_v2.md`) → approved.

**v2 draft changes (vs v1):**
- Anchor restructured into two structurally distinct sub-anchors (MKTd02 and `zombie-core`) per G's ruling.
- **New Candidate 7 — Receipt-ID / deterministic-derivation discipline** added to absorb the content previously reflected as a split classification on `zombie-core::receipt` in the reuse audit v1.
- Candidates 1–6 unchanged from v1 except for anchor-reference cleanups.

**Approval changes (v2 draft → approved):** anchor pins supplied (§2.1, §2.2); §6 converted from open questions to resolved G rulings. Candidate content itself substantively unchanged from v2 draft.

---

## 1. Purpose and scope

Per build plan §5.4, this artifact lists modules and disciplines that **should** be reusable in principle but currently are not — with the reusable abstraction shape proposed for each.

**Scope restriction (from build plan §5.4, verbatim):** *"This artifact is a candidate list only. It does not authorise edits to MKTd02, creation of MKTd02 tasks, or parallel back-port work during this phase."*

---

## 2. Anchors

Two separate repositories; each independently anchored.

### 2.1 Primary anchor — MKTd02

- **Repository:** `Together-Alone-Ventures/ICP-Delete-Leaf` (formerly `MKTd02`).
- **Repository URL:** `https://github.com/Together-Alone-Ventures/ICP-Delete-Leaf`.
- **Protocol line:** v0.2.x (Leaf mode).
- **Workspace crate version:** `0.1.0`.
- **Commit hash:** `54f1e2dc24dd0b79705a66894b2f25138e28a9ad`. (Same SHA as the Phase-4 reuse-generalisation audit v1; resolves under the renamed repo per G's ruling 2026-04-24.)

### 2.2 Secondary anchor — `zombie-core`

- **Repository:** `Together-Alone-Ventures/zombie-core`.
- **Repository URL:** `https://github.com/Together-Alone-Ventures/zombie-core`.
- **Dependency pin observed:** tag `zombie-core-v0.3.1`.
- **Commit hash at tag:** `508f2f8bb88f4395293168c6ef25c92a67dee894`. (Tag-to-commit resolution per G's ruling 2026-04-24; dependency pin confirmed in `mktd02/Cargo.toml`.)

---

## 3. Candidates

### Candidate 1 — A→B→C atomic-issuance pattern as a generic abstraction

**Current name / location:** `mktd02/src/finalization.rs` in combination with `mktd02/src/engine.rs::execute_deletion` and `mktd02/src/certified.rs::publish_certified_commitment` (all anchor §2.1). The pattern is implicit in their composition; it is not named.

**Why it is too specific now:**
- Hard-wired to `execute_deletion` / `get_pending_certificate` / `finalize_receipt` at the public-API layer.
- Pending-identity storage is a single `pending_receipt_id` cell.
- Finalization lock is a single boolean — only one atomic issuance can be pending at a time.
- Error types (`DeletionError`, `FinalizationError`) are issuance-domain-specific.

**What the reusable abstraction should look like:**
- A named architectural pattern in MKTd03 documentation, not necessarily a shared trait or module.
- Pattern statement: "Evidence-producing transitions on ICP follow a three-phase issuance: (A) update-context preimage composition, certified-data publication, and pending-state capture; (B) query-context BLS-certificate retrieval; (C) update-context embedding of certificate material into the persisted pending artifact, with pending-state release."
- Explicit invariant: "Pending-state identity must be persisted in Phase A; Phase B and Phase C must not recompute the identity from mutable runtime values."
- Explicit invariant: "Certified-data must not change while any pending-issuance state is outstanding."

**Future separate MKTd02 evaluation warranted?** Yes — back-porting the pattern into MKTd02 as a named abstraction would tighten MKTd02's own Phase B/C error-handling discipline. Low priority; not in MKTd03's critical path.

**Likely back-port cost/risk:** Low cost. Low risk. MKTd02 is stable at v0.2.x; back-port would be a refactor, not a behavioural change.

---

### Candidate 2 — Generic certified-commitment publish/read wrapper around ICP platform calls

**Current name / location:** `mktd02/src/certified.rs` (anchor §2.1) — `publish_certified_commitment`, `read_certified_commitment`, `get_certified_state_hash`.

**Why it is too specific now:**
- `publish_certified_commitment` takes `state_hash` and `deletion_event_hash` as typed parameters — Leaf-specific shape.
- The preimage `TAG_CERTIFIED || state_hash || deletion_event_hash` is hard-coded.
- `get_certified_state_hash` returns a `(state_hash, certificate)` tuple — Leaf-specific shape.

**What the reusable abstraction should look like:**
- `publish_commitment(commitment: [u8; 32])` that wraps `ic_cdk::api::set_certified_data` with finalization-lock guard. Preimage composition is caller's responsibility.
- Matching `read_published_commitment() -> [u8; 32]`.
- Phase-B-context `get_pending_certificate_material() -> Option<(commitment, certificate)>` returning commitment + runtime certificate for caller embedding.
- All tag-domain composition belongs in the caller.

**Future separate MKTd02 evaluation warranted?** Moderate. Refactoring MKTd02's certified layer to use a generic commitment wrapper would make it easier to prove MKTd02 and MKTd03 share identical certified-data discipline. Not critical.

**Likely back-port cost/risk:** Low cost. Medium risk of test churn if the refactor changes exposed symbols; behavioural surface unchanged.

---

### Candidate 3 — Domain-tag discipline as a protocol-neutral library

**Current name / location:** `zombie-core::hashing` (anchor §2.2) — `hash_with_tag` function plus the `TAG_*` constants suffixed `MKTD02_*_V1`.

**Why it is too specific now:**
- Function is already generic — the issue is the Leaf-suffixed tag constants.
- Tag names like `MKTD02_TOMBSTONE_HASH_V1` cannot be consumed by MKTd03 directly without polluting MKTd03 receipt preimages with `MKTD02_*_V1` strings.
- Tag constants are exported publicly; consumers cannot easily shadow them without confusion.

**What the reusable abstraction should look like:**
- A `hashing-core` module (or split crate) exposing `hash_with_tag` and the encoding discipline but declaring **no** tag constants — only the function and the discipline document.
- Tag constants live in each protocol's own crate: `mktd02::tags::*` (Leaf), `mktd03::tags::*` (Tree).
- Discipline document travels with the function: ASCII tag bytes, no null terminator, fixed-width big-endian integers, raw principal bytes, no Unicode normalisation, no length-prefix by default.

**Future separate MKTd02 evaluation warranted?** Yes. MKTd02 should migrate to owning tag constants in `mktd02::tags` rather than `zombie-core`. Low priority hygiene.

**Likely back-port cost/risk:** Low cost. Minor binary-compat risk if `zombie-core` is re-tagged as v0.4.0 and MKTd02 consumers need to rebuild; mitigated by keeping v0.3.x tag available.

---

### Candidate 4 — Module-hash provenance as a stable-memory-persisted, deployer-supplied input pattern

**Current name / location:** `mktd02/src/storage.rs::meta` cell (`module_hash`); `mktd02/src/lib.rs::init` / `on_post_upgrade` accept `module_hash: [u8; 32]`; `mktd02/src/engine.rs` reads it into the `deletion_event_hash` preimage. All anchor §2.1.

**Why it is too specific now:**
- Storage slot is part of Leaf-specific 8-slot layout.
- Placement of `module_hash` into the `deletion_event_hash` preimage is Leaf-specific.
- Pattern is not documented as an abstraction.

**What the reusable abstraction should look like:**
- Named pattern in the MKTd03 security / provenance note: "Shipped-WASM module-hash provenance is a deployer-supplied input, persisted in stable memory, and bound into evidence preimages as a build-provenance component."
- Concrete recommendation: MKTd03 allocates its own module-hash cell in its own storage schema; the `init(..., module_hash: [u8; 32])` API shape carries.
- Explicit treatment of upgrade semantics: module hash updated unconditionally on `on_post_upgrade` — the upgrade implies new WASM.

**Future separate MKTd02 evaluation warranted?** No. MKTd02 handles this correctly; the pattern is documentation/back-port of naming, not code change.

**Likely back-port cost/risk:** Documentation only.

---

### Candidate 5 — Pending-identity-persisted-across-phases discipline

**Current name / location:** `mktd02/src/storage.rs::pending_receipt_id` cell; `mktd02/src/finalization.rs::read_pending_receipt_id`; test `pending_identity_helper_uses_persisted_value_not_deletion_seq` specifically enforces the discipline. All anchor §2.1.

**Why it is too specific now:**
- Single pending-receipt-id (boolean-lock-equivalent).
- Receipt-ID derivation is Leaf-shaped.
- "Read persisted value, never recompute from mutable runtime state" invariant is implicit in the code pattern, made explicit only by a test — not by a documented rule.

**What the reusable abstraction should look like:**
- Named invariant in MKTd03 companion rules: "Across a multi-phase issuance, pending-issuance identifiers must be resolved by reading the stable-memory-persisted value from Phase A, not by recomputation from runtime counters, timestamps, or caller context."
- Generalisation from single-pending to set-of-pending: Tree-mode may have multiple concurrently-pending issuances; discipline generalises to *each* pending identifier being persisted once at Phase A and read-only thereafter.

**Future separate MKTd02 evaluation warranted?** Low-priority doc back-port. No code change.

**Likely back-port cost/risk:** Documentation only.

---

### Candidate 6 — Finalization-lock as a set, not a boolean

**Current name / location:** `mktd02/src/storage.rs` finalization-lock cell (Boolean); `mktd02/src/finalization.rs::is_pending_finalization()`; guard in `mktd02/src/certified.rs::publish_certified_commitment`. All anchor §2.1.

**Why it is too specific now:**
- Boolean lock assumes one pending issuance at a time.
- Tree mode may have many concurrently-pending leaf deletions.
- Generalising after the fact requires rewriting the lock-guard and every site that sets/releases it.

**What the reusable abstraction should look like:**
- Typed lock abstraction: `PendingIssuanceSet<K>` where `K` is issuance-identifier type. Boolean-lock case = `PendingIssuanceSet<()>` with cardinality ≤ 1.
- Guard semantics generalise: "certified-data may not change while the pending-issuance set is non-empty" — same rule, different cardinality.
- MKTd02 keeps boolean semantics at public API; underlying abstraction is one-element-max set.

**Future separate MKTd02 evaluation warranted?** Medium-priority. If `zombie-core` grows a `PendingIssuanceSet` type, MKTd02 would benefit from consuming it.

**Likely back-port cost/risk:** Moderate cost (touches storage schema). Low behavioural risk if boolean case preserved in MKTd02.

**Disposition per G's ruling 2026-04-24 (§6 Q1):** Raised as Phase-7 planning input. Not a reopening candidate for ADR-03.

---

### Candidate 7 — Receipt-ID / deterministic-derivation discipline (new in v2)

**Current name / location:** `zombie-core::receipt::compute_receipt_id` and the field-ordering conventions in `zombie-core::receipt::DeletionReceipt` (anchor §2.2). The discipline is implicit in the implementation; it is not named or separated from the Leaf-specific struct shape.

**Why it is too specific now:**
- The derivation function's inputs are the Leaf-shaped `DeletionReceipt` fields (`pre_state_hash_hex`, `post_state_hash_hex`, `tombstone_hash_hex`, `deletion_event_hash_hex`, `canister_module_hash_hex`, etc.).
- Field ordering is specified by convention within the struct definition; it is not declared as an explicit ordered list.
- No tag/version surface on the derivation itself (the tag/version of the *receipt type* is implicit in `ProtocolVersion`, not applied to the derivation step).
- No fail-loud on field-set drift — if a field is added or removed from `DeletionReceipt` in a future version, `compute_receipt_id` silently produces a different (but still well-formed) receipt-ID with no schema-mismatch signal.

**What the reusable abstraction should look like:**
- Named discipline in MKTd03 companion rules / security note: "Receipt-ID derivation is a deterministic hash over an explicitly-ordered, tagged, versioned field set. The derivation step has its own domain tag distinct from any protocol tag. Adding, removing, or reordering fields constitutes a receipt-version change that must bump `receipt_version` and must fail-loud on cross-version consumption."
- Protocol-level statement: the Tree-mode `Receipt` in `interfaces/mktd03_library.did` is MKTd03's native equivalent to what `DeletionReceipt` was for MKTd02; the derivation discipline applies to it, not a reused derivation function.
- Explicit fail-loud rule: if `receipt_version` on a returned receipt doesn't match the current library's receipt-version constant, verifier paths must reject, not coerce.

**Relationship to Candidate 5 (pending-identity discipline) and Candidate 3 (tag discipline):**
- Candidate 5 is about *which identifier is used* across phases.
- Candidate 7 is about *how that identifier is derived* from receipt content.
- Candidate 3 is about *which tags* participate in hashing.
All three are distinct but mutually supportive; Candidate 7 consumes the output of Candidate 3.

**Future separate MKTd02 evaluation warranted?** Low-priority doc back-port: promote the implicit field-ordering convention in `zombie-core::receipt` to an explicit derivation rule, with fail-loud on field-set drift. No code change behaviourally.

**Likely back-port cost/risk:** Documentation only.

**Disposition per G's ruling 2026-04-24 (§6 Q3):** Left analytical. Do not promote to normative companion-rule / security-note text now. Promotion deferred until a later slice actually exercises receipt issuance / receipt-ID derivation.

---

## 4. Candidates explicitly NOT included, and why

The following were considered and rejected as generalise-now-back-port-later candidates:

- **`engine.rs` deletion flow.** Leaf-mode-specific; no meaningful generalisation path. Captured as "lessons only" in the reuse audit.
- **`trait_def.rs` adapter trait.** Superseded by ADR-01's narrower Tree-mode adapter seam.
- **`export.rs` receipt export.** Shape of MKTd02's `DeletionReceipt` is Leaf-specific; no export-layer abstraction is meaningfully generic.
- **`guard.rs` single-canister tombstone guard.** Guard semantics are Leaf-shaped by design.
- **`nonce.rs` monotonic counter.** Tree mode's sequencing semantics are per-subject/per-record; no abstraction path unifies.
- **`state.rs` whole-canister state hash.** Tree mode commits tree-root material, not whole-canister state.
- **`zombie-core::receipt` module itself** (the struct and the function — as opposed to the derivation *discipline* in Candidate 7). The struct is Leaf-specific per the reuse audit v2 reclassification; only the abstract discipline carries, and it is now Candidate 7.

---

## 5. Scope restriction restatement

Per build plan §5.4, this list is a **candidate list only**. It does not authorise:

- Any edits to MKTd02 source.
- Any new MKTd02 tasks or tickets.
- Any parallel back-port work in MKTd02 during this MKTd03 phase.

Back-port work implied by candidates above must be opened as its own work packet. MKTd03's forward motion does not depend on any back-port.

---

## 6. Resolved questions — G rulings (2026-04-24)

1. **Candidate 6 (Finalization-lock as a set).** Raise as Phase-7 planning input. Not an ADR-03 reopening candidate. Candidate 6 annotated accordingly.
2. **Candidate 3 (tag-discipline extraction).** Not a precondition for S7-1 (S7-1 does not use hashing). **Precondition before any hashing/preimage slice** — becomes material at S7-3 or later.
3. **Candidate 7 (receipt-ID derivation discipline).** Leave analytical. Do not promote to normative companion-rule text now. Promote when a later slice actually issues receipts / derives receipt-IDs. Candidate 7 annotated accordingly.
4. **Missing candidates.** None worth adding now.
