# MKTd02 Module Reuse Audit v1

**Status:** Approved analysis (G review, 2026-04-24, §5 reuse-audit close-out).
**Path:** `docs/analysis/MKTd02_module_reuse_audit_v1.md`
**Build plan reference:** §5.3
**Drafted by:** C (Claude), at G's explicit request per session role-swap allowance.
**Draft history:** v1 (`MKTd02_module_reuse_audit_v1_DRAFT.md`) → v2 (`MKTd02_module_reuse_audit_v1_DRAFT_v2.md`) → approved.

**v2 draft changes (vs v1):**
- Anchor restructured into two structurally distinct sub-anchors (MKTd02 and `zombie-core`) per G's ruling.
- `zombie-core::receipt` reclassified from split judgment to single bucket: **Leaf-mode-specific; do not reuse**. Receipt-ID / deterministic-derivation discipline moved to §4 narrative and to candidate-list §5.
- `zombie-core::nns_keys` row annotated with explicit carry-over assumption per G's ruling.
- §6 summary counts updated to reflect the reclassification.

**Approval changes (v2 draft → approved):** anchor pins supplied (§1.1, §1.2); §5.4 note records tag-namespace option (c) as preferred default per G ruling Q4; §7 converted from open questions to resolved G rulings.

---

## 1. Anchors

Two separate repositories are in scope. Each is independently anchored.

### 1.1 Primary anchor — MKTd02 (audit target)

- **Repository:** `Together-Alone-Ventures/ICP-Delete-Leaf` (formerly `Together-Alone-Ventures/MKTd02`; internal source name still `mktd02`).
- **Repository URL:** `https://github.com/Together-Alone-Ventures/ICP-Delete-Leaf`.
- **Protocol line:** v0.2.x (Leaf mode) per `RELEASES.md`.
- **Workspace crate version:** `0.1.0`.
- **Commit hash:** `54f1e2dc24dd0b79705a66894b2f25138e28a9ad`. (Same SHA as the Phase-4 reuse-generalisation audit v1 at `docs/spec/MKTd03_mktd02_reuse_generalisation_audit_v1.md`; resolves under the renamed repo per G's ruling 2026-04-24.)

### 1.2 Secondary anchor — `zombie-core`

- **Repository:** `Together-Alone-Ventures/zombie-core` — distinct from MKTd02.
- **Repository URL:** `https://github.com/Together-Alone-Ventures/zombie-core`.
- **Dependency pin observed:** tag `zombie-core-v0.3.1` in `mktd02/Cargo.toml`.
- **Commit hash at tag:** `508f2f8bb88f4395293168c6ef25c92a67dee894`. (Tag-to-commit resolution per G's ruling 2026-04-24; dependency pin confirmed in `mktd02/Cargo.toml`.)
- **Audit treatment:** `zombie-core` modules are audited here for their role as MKTd02's dependency; classifications are attributable to the `zombie-core` repo, not to MKTd02.

Scope inclusions and exclusions: per companion taxonomy §1.3.

---

## 2. Classification scheme (build plan §5.3, verbatim)

Each module receives one of:

- **Reusable as-is**
- **Reusable with parameterisation or generalisation**
- **Conceptually reusable but currently too hard-coded**
- **Leaf-mode-specific; do not reuse**
- **Unknown pending deeper review**

Six per-module questions:

1. Does it encode protocol truth or only MKTd02 integration truth?
2. Does it assume a single-record Leaf model?
3. Does it assume a Leaf-mode CVDR shape?
4. Does it hard-code hash preimages, identifiers, sequencing, or storage assumptions that should be abstracted?
5. Does it align with desired diagnostics/versioning/failure semantics?
6. Does it belong in a shared reusable core?

---

## 3. Per-module audit

Sub-questions abbreviated as **Q1–Q6** in tables. Narrative justifications in §4.

### 3.1 `mktd02` crate modules (anchor §1.1)

| Module | Classification | Q1 protocol/integration | Q2 Leaf single-record? | Q3 Leaf CVDR? | Q4 hard-codes? | Q5 diagnostics/versioning alignment? | Q6 shared core candidate? |
|---|---|---|---|---|---|---|---|
| `lib.rs` public-API composition | Conceptually reusable but currently too hard-coded | Integration | Yes | Yes | Yes (composition order, receipt finality semantics) | No (no StatusSurface; no version-surface plumbing) | Only after rewrite |
| `engine.rs` deletion flow | Leaf-mode-specific; do not reuse | Mixed | Yes | Yes | Yes (`TAG_TOMBSTONE_HASH` preimage, `TAG_EVENT` preimage, single `deletion_seq`) | Partial (fail-loud traps, yes; status surface, no) | No as-is; lessons, yes |
| `certified.rs` certified-commitment publish | Reusable with parameterisation or generalisation | Protocol (certified-data surface) mixed with MKTd02 integration (commitment preimage) | No | Partially (Leaf-mode `deletion_event_hash` is a preimage input) | Yes (`TAG_CERTIFIED` preimage) | Fail-loud yes; versioning no | Yes, with generalised preimage shape |
| `finalization.rs` Phase B + C | Reusable with parameterisation or generalisation | Protocol (A→B→C pattern) mixed with MKTd02 integration (pending-receipt shape) | No (A→B→C is platform-shaped, not subject-shaped) | Yes at embedding layer (`DeletionReceipt` Leaf shape) | Partial (pending-identity persistence yes; receipt embedding Leaf-shaped) | Partial (error taxonomy yes; StatusSurface no) | Yes for the pattern; no for the embedding |
| `guard.rs` init/tombstone guards | Conceptually reusable but currently too hard-coded | Integration (with Leaf-specific meaning) | Yes | Yes | Yes (single-canister tombstone semantics) | No | Not as-is |
| `nonce.rs` deletion sequence | Conceptually reusable but currently too hard-coded | Integration | Yes (single monotonic counter) | Yes (tied to Leaf receipt identity) | Yes | Fail-loud yes; no otherwise | Yes, generalised |
| `state.rs` canister state hash | Conceptually reusable but currently too hard-coded | Protocol discipline (canonicalised commitment) with Leaf-scoped input | Yes | No (state hash is an input to, not the shape of, the CVDR) | Yes (`TAG_STATE` preimage; whole-canister scope) | No | Yes, generalised |
| `storage.rs` stable-memory layout | Conceptually reusable but currently too hard-coded | Integration | Yes | Yes | Yes (8-slot schema, Leaf-cell contents) | Partial (persistence discipline yes; status surface no) | Discipline yes; schema no |
| `trait_def.rs` adapter trait | Leaf-mode-specific; do not reuse | Integration | Yes | Yes | Yes (trait methods presume single-subject state, single tombstone) | No | No — superseded by ADR-01 narrow seam |
| `export.rs` receipt export | Leaf-mode-specific; do not reuse | Integration | Yes | Yes | Yes | No | No |

### 3.2 `mktd02-macros` crate (anchor §1.1)

| Module | Classification | Q1 | Q2 | Q3 | Q4 | Q5 | Q6 |
|---|---|---|---|---|---|---|---|
| `mktd02-macros` `#[mktd_guard]` | Leaf-mode-specific; do not reuse | Integration | Yes | Yes | Yes (emits Leaf-shaped guard calls) | No | No |

### 3.3 `zombie-core` external crate (anchor §1.2)

| Module | Classification | Q1 | Q2 | Q3 | Q4 | Q5 | Q6 |
|---|---|---|---|---|---|---|---|
| `zombie-core::hashing` (domain-tag discipline, `hash_with_tag`) | Reusable with parameterisation or generalisation | Protocol (domain-separation discipline) | No | No | Partial (Leaf-suffixed tag names `MKTD02_*_V1` are hard-coded in identifiers; `hash_with_tag` itself is generic) | Versioning yes via tag-v-suffix convention; fail-loud yes | Yes — strong shared-core candidate |
| `zombie-core::receipt` (`DeletionReceipt` + `compute_receipt_id`) | **Leaf-mode-specific; do not reuse** *(see §4.6; receipt-ID derivation discipline is a separate candidate-list item, not a reuse of this module)* | Integration | Yes | Yes | Yes | Partial (versioning via `ProtocolVersion` field, but version space is Leaf-shaped) | No — discipline carried separately |
| `zombie-core::tombstone` (`tombstone_constant`) | Leaf-mode-specific; do not reuse | Integration | Yes | Yes | Yes (single constant for canister-scope tombstone) | No | No |
| `zombie-core::nns_keys` (`active_key_id`, feature-gated mainnet / local-dev) | **Reusable as-is — assuming MKTd03 preserves the same ICP/NNS trust-root model and build-gating posture** | Protocol (ICP-platform trust-root identification) | No | No | Intentional (key ID strings are keyed to real NNS root-keys) | Versioning via build-config, yes; fail-loud via build-gate, yes | Yes — direct carry if assumption holds |

---

## 4. Narrative justifications

### 4.1 Why the A→B→C finalization pattern is reusable-with-generalisation, not Leaf-specific

G's guidance: *"the ICP platform mechanics around certificate capture and module-hash provenance are real carry-over candidates unless the MKTd03 design explicitly replaces them."*

The A→B→C shape exists because `ic0.data_certificate()` is only callable from query context. This is ICP-platform truth, not MKTd02 protocol truth. Any MKTd03 library that wants to produce a BLS-certified artifact on ICP will face the same constraint. Therefore:

- **Pattern (Phase A update → Phase B query-read → Phase C update-embed) = reusable-with-generalisation.**
- **Pending-identity-persisted-across-phases discipline = reusable-with-generalisation.**
- **Finalization-lock discipline = reusable-with-generalisation in concept, but the specific lock (single pending at a time) is Leaf-shaped.** Tree mode may have multiple pending deletions concurrently; lock model generalises to a set of pending identifiers, not a boolean.
- **The BLS-certificate-embedding-into-receipt mechanic = generalisable**, but only if the Tree-mode receipt structure explicitly allocates a field for the certificate payload (which `CertificationProvenanceBlock` in the frozen-draft library interface already does via `certification_material`).

### 4.2 Why `certified.rs` is reusable-with-generalisation

Two concerns in one module:
1. Composes `certified_commitment = H(TAG_CERTIFIED || state_hash || deletion_event_hash)` — Leaf-specific preimage.
2. Calls `ic_cdk::api::set_certified_data(commitment)` and exposes `ic_cdk::api::data_certificate()` in a query path — platform-specific mechanic.

The **platform mechanic (publish via certified-data, read via data_certificate)** generalises cleanly. The **preimage composition** does not — it is Leaf-shaped. Tree mode will publish a commitment over tree-root material, not a Leaf `state_hash`/`deletion_event_hash` pair. Module orchestration is reusable; cryptographic composition is not.

### 4.3 Why `engine.rs` is Leaf-mode-specific; do not reuse

Hard-codes three Leaf-mode preimages:
- `tombstone_hash = H(TAG_TOMBSTONE_HASH || canister_id || tombstone_constant || timestamp || deletion_seq)` — single-canister, single-tombstone.
- `deletion_event_hash = H(TAG_EVENT || pre_state_hash || post_state_hash || timestamp || module_hash || deletion_seq)` — single state-hash pair, single monotonic counter.
- `certified_commitment` composition of the above via `certified.rs`.

Tree-mode evidence semantics per ADR-03 differ materially: per-leaf transitions, tree-root material, per-subject/per-record sequencing, tree-proof as first-class evidence. The engine's structure cannot be lifted without rewriting every preimage and re-selecting inputs. Only reusable content is the **discipline** of atomic-execution-before-certified-publish — better captured as a principle in ADR-03 / companion rules than as reused code.

### 4.4 Why `zombie-core::hashing` is reusable-with-generalisation (strong candidate)

`hash_with_tag(tag_bytes, &[input_slices...])` is generic. The discipline — ASCII tag bytes, no null terminator; fixed-width big-endian integers; raw principal bytes; domain-separation by tag — is protocol-shaped and already aligned with MKTd03's companion-rules direction. Only Leaf-specific coupling is the set of declared tag constants (`TAG_TOMBSTONE_HASH = MKTD02_TOMBSTONE_HASH_V1`, etc.), which would not carry forward under any reuse path.

Recommended generalisation shape: hoist `hash_with_tag` and the tagging discipline into a shared module; declare MKTd03-specific tag constants (`MKTD03_*_V1`) in an MKTd03-owned module.

### 4.5 Why `zombie-core::nns_keys` is reusable as-is — with explicit carry-over assumption

The only module classified **Reusable as-is** in this audit, and reclassified in v2 only in how the assumption is flagged. The module:
- Declares the ICP mainnet NNS root-key identifier.
- Provides a `local-replica` feature flag for the local-dev root key, with CI enforcement that `local-replica` is not enabled on release builds.
- Is called at Phase C to stamp the trust-root-key-id onto the receipt, preventing integrators from supplying a wrong or fabricated key.

**Carry-over assumption (explicit):** "Reusable as-is" is valid **only if MKTd03 preserves the same ICP/NNS trust-root model and the same build-gating posture.** Specifically:
- MKTd03 trusts the same NNS subnet root key material as MKTd02.
- MKTd03 retains the mainnet / local-dev build-gate distinction for test versus production trust-root use.
- MKTd03 does not elect to bind to a different trust anchor (e.g., a delegated subnet key, an external BLS root, or a forked NNS topology) that would require a different key-ID taxonomy.

If any of these assumptions is broken at design time, `zombie-core::nns_keys` loses its as-is reusability and should be reclassified as "Reusable with parameterisation or generalisation" under the new trust-root model.

Per G's ruling 2026-04-24 (§7 Q2), this carry-over assumption remains audit-local; it is not promoted to normative companion-rule / security-note text until a certification/trust-root slice actually touches the material.

### 4.6 Why `zombie-core::receipt` is reclassified from split to single bucket

**v1 draft placed `zombie-core::receipt` in a split "Leaf-specific as-is / conceptually reusable as discipline" bucket.** G ruled this split out: the audit table needs one bucket per meaningful module, and the discipline-level observation belongs in narrative + candidate list, not in the classification column.

**v2 classification:** Leaf-mode-specific; do not reuse.

**Reasoning for the single bucket:** the module as it stands in `zombie-core` consists of:
- `DeletionReceipt` struct — Leaf-shaped: field set is tied to `pre_state_hash_hex`, `post_state_hash_hex`, `tombstone_hash_hex`, `deletion_event_hash_hex`, `canister_module_hash_hex`, and other Leaf-specific surface.
- `compute_receipt_id` — derivation function whose inputs are the Leaf-shaped `DeletionReceipt` fields.
- `ProtocolVersion`, `FieldDescriptor`, `ReceiptSummary` — supporting types tied to the Leaf-mode receipt model.

None of these sub-elements can be lifted into MKTd03 without rewriting the inputs; the Tree-mode equivalent is the already-defined `Receipt` with `core_transition_evidence` (including `transition_derivation_version`) and `certification_provenance` in `interfaces/mktd03_library.did`. That type is MKTd03-native and has no architectural relationship to `DeletionReceipt`.

**The generic observation that both Leaf and Tree modes need deterministic, tagged, versioned receipt-ID derivation with stable field ordering is a protocol-level discipline, not a reusable module.** That discipline is captured as **Candidate 7** in the companion candidate-list artifact (see §5 below), and is also honoured natively by the frozen-draft library interface (`receipt_version`, the `ProtocolVersion` / `SemanticVersion` surface, `transition_derivation_version`). Nothing is lost by this reclassification; the discipline is still carried. What changes is only where it's recorded — in the candidate list, not in a split audit-table bucket.

### 4.7 Why `trait_def.rs` and `export.rs` are Leaf-mode-specific; do not reuse

Both are wrapped tightly around Leaf semantics. `MKTdDataSource::get_state_bytes()` presumes one canonical state blob; `tombstone_state()` is a single-subject operation; `is_tombstoned()` returns a single boolean. ADR-01 has already chosen a narrower Tree-mode adapter seam, which makes `MKTdDataSource` superseded by design. `export.rs` is shaped around the Leaf-mode `DeletionReceipt`.

### 4.8 Why no module is classified **Unknown pending deeper review**

C's preference is explicit classification wherever possible. If G disagrees with any classification, the correct move is for G to push the module into "Unknown pending deeper review" during review pass. Currently zero modules are Unknown.

---

## 5. Cross-cutting findings

### 5.1 Module-hash provenance is a real carry-over, but small

Per G's guidance. The MKTd02 mechanic:
- Deployer computes SHA-256 of shipped WASM.
- Deployer passes the hash to `init()` / `on_post_upgrade()`.
- MKTd02 stores it in `storage::meta.module_hash`.
- `engine.rs` includes the stored module hash in the `deletion_event_hash` preimage.

The *pattern* (deployer-supplied hash; persisted in stable memory; bound to receipts) is reusable-with-generalisation. The *specific storage slot and preimage placement* are Leaf-shaped. The pattern belongs in MKTd03 but the code does not lift directly.

### 5.2 MKTd02 has no status-surface prior art usable by MKTd03

MKTd03's frozen-draft `StatusSurface` has no direct predecessor in MKTd02. MKTd02 offers `is_tombstoned()`, `is_initialised()`, `tombstoned_at`, and receipt-count queries — an informal, scattered status surface. `StatusSurface`/`LifecycleState`/`BlockedReason`/`Compatibility` discipline is MKTd03-native. Any first implementation slice that exercises status-surface semantics must be built fresh, not imported.

### 5.3 Finalization-lock discipline generalises, but semantics must be redrawn

MKTd02's finalization-lock is a single boolean. Tree mode will either:
- **Option L1:** serialize one deletion at a time (boolean lock, same shape). Simple but artificially sequential.
- **Option L2:** track a set of pending-finalization identifiers. More complex; aligns with tree-scale throughput.

Not settled in ADR-03 as far as I can see. **Per G's ruling 2026-04-24 (§7 Q3), this finding is held as Phase-7 planning input only; it does not reopen ADR-03.**

### 5.4 Domain-tag naming will drift if zombie-core is reused directly

If MKTd03 consumes `zombie-core` at tag `zombie-core-v0.3.1` as-is, any receipts or hashes it produces will contain `MKTD02_*_V1` in preimages. Wrong for MKTd03 and a source of confusion. Reuse path must either (a) rename tags to `MKTD03_*_V1` in a forked/vendored copy, (b) upstream tag-namespace parameterisation to `zombie-core` before consuming it, or (c) define MKTd03-owned tag constants and never use the `zombie-core`-declared constants directly. Option (c) is the lightest coupling. **Preferred default per G's ruling 2026-04-24 (§7 Q4); final decision deferred to the first-slice scope proposal.**

### 5.5 Receipt-ID derivation discipline is carried forward as a candidate-list item (new cross-cutting finding in v2)

Introduced in v2 to absorb the content previously reflected as a split classification on `zombie-core::receipt`. See Candidate 7 in the companion candidate-list artifact. Summary: "Deterministic receipt-ID derivation over an explicitly-ordered, tagged, versioned field set, with fail-loud on field-set drift." Discipline, not code reuse.

---

## 6. Summary

| Classification | Count | Modules |
|---|---:|---|
| Reusable as-is | 1 | `zombie-core::nns_keys` (under the §4.5 carry-over assumption) |
| Reusable with parameterisation or generalisation | 3 | `certified.rs`, `finalization.rs`, `zombie-core::hashing` |
| Conceptually reusable but currently too hard-coded | 5 | `lib.rs` public-API composition, `guard.rs`, `nonce.rs`, `state.rs`, `storage.rs` |
| Leaf-mode-specific; do not reuse | 6 | `engine.rs`, `trait_def.rs`, `export.rs`, `mktd02-macros`, `zombie-core::tombstone`, `zombie-core::receipt` |
| Unknown pending deeper review | 0 | — |

Total meaningful modules in anchored scope: 15.

**v2 counts vs v1:** Conceptually-reusable count drops from 6 to 5; Leaf-specific count rises from 5 to 6; `zombie-core::receipt` moved between the two, per G's reclassification ruling.

---

## 7. Resolved questions — G rulings (2026-04-24)

1. **Anchor pins for §1.1 and §1.2.** Supplied and applied: MKTd02 commit `54f1e2dc24dd0b79705a66894b2f25138e28a9ad`; `zombie-core-v0.3.1` commit `508f2f8bb88f4395293168c6ef25c92a67dee894`.
2. **`zombie-core::nns_keys` carry-over assumption (§4.5).** Leave audit-local for now. Do not lift to normative text (security/privacy note or companion rules) until the certification/trust-root slice. §4.5 annotated accordingly.
3. **Finalization-lock lock-shape cross-cutting finding (§5.3).** Phase-7 planning input only. Do not reopen ADR-03. §5.3 annotated accordingly.
4. **Tag-namespace reuse strategy (§5.4, Options a/b/c).** Decision deferred to the first-slice scope proposal. Option (c) — MKTd03-owned tag constants, never using `zombie-core`-declared constants directly — recorded as preferred default. §5.4 annotated accordingly.
