# MKTd02 Module Taxonomy for MKTd03

**Status:** Approved analysis (G review, 2026-04-24, §5 reuse-audit close-out).
**Path:** `docs/analysis/MKTd02_module_taxonomy_for_MKTd03.md`
**Build plan reference:** §5.2
**Drafted by:** C (Claude), at G's explicit request per session role-swap allowance.
**Draft history:** v1 (`MKTd02_module_taxonomy_for_MKTd03_DRAFT.md`) → v2 (`MKTd02_module_taxonomy_for_MKTd03_DRAFT_v2.md`) → approved.
**v2 draft changes (vs v1):** anchor restructured into two structurally distinct sub-anchors (MKTd02 and `zombie-core`) per G's ruling; remainder of content substantively unchanged.
**Approval changes (v2 draft → approved):** anchor pins supplied (§1.1, §1.2); explicit Category E "absent in MKTd02" row added at §3.4 per G ruling Q1; §6 converted from open questions to resolved G rulings.

---

## 1. Anchors

Two separate repositories are in scope. Each is independently anchored.

### 1.1 Primary anchor — MKTd02 (audit target)

- **Repository:** `Together-Alone-Ventures/ICP-Delete-Leaf` (formerly `Together-Alone-Ventures/MKTd02`; still named `mktd02` throughout the source tree).
- **Repository URL:** `https://github.com/Together-Alone-Ventures/ICP-Delete-Leaf`.
- **Protocol line:** v0.2.x (Leaf mode) per `RELEASES.md`.
- **Workspace crate version:** `0.1.0` per workspace `Cargo.toml`.
- **Commit hash:** `54f1e2dc24dd0b79705a66894b2f25138e28a9ad`. (Same SHA as the Phase-4 reuse-generalisation audit v1 at `docs/spec/MKTd03_mktd02_reuse_generalisation_audit_v1.md`; resolves under the renamed repo per G's ruling 2026-04-24.)

### 1.2 Secondary anchor — `zombie-core` (external dependency of MKTd02)

- **Repository:** `Together-Alone-Ventures/zombie-core` — a distinct git repository from MKTd02.
- **Repository URL:** `https://github.com/Together-Alone-Ventures/zombie-core`.
- **Dependency pin observed in MKTd02:** tag `zombie-core-v0.3.1` per `mktd02/Cargo.toml`.
- **Commit hash at that tag:** `508f2f8bb88f4395293168c6ef25c92a67dee894`. (Tag-to-commit resolution per G's ruling 2026-04-24.)
- **Scope treatment:** `zombie-core` is a dependency of MKTd02, not part of it. Its modules are audited here because MKTd02 re-exports and consumes them, but classifications are separately attributable to the `zombie-core` repo, not to MKTd02.

### 1.3 Scope inclusions and exclusions

Scope included:
- All Rust source under `mktd02/src/` (primary anchor).
- The `mktd02-macros` crate (primary anchor, same workspace).
- The `zombie-core` external dependency at its pinned tag (secondary anchor), specifically the modules referenced by `mktd02` source.

Scope excluded:
- Application-layer canisters that *consume* MKTd02.
- Test-vector-only harnesses and verifier scripts (treated as validation surface, not protocol module).
- `MKTd02_Integration_Guide.md` and similar prose artifacts, except insofar as they document the behaviour of scoped modules.

---

## 2. Taxonomy categories (from build plan §5.2)

Each MKTd02 module is assigned a **primary** category and, where applicable, one or more **secondary** categories.

| Code | Category |
|------|----------|
| A | Receipt / artifact identity |
| B | Hashing / tagging / deterministic encoding |
| C | Sequencing / lifecycle state |
| D | Finalization / orchestration |
| E | Diagnostics / status surface |
| F | Storage abstraction |
| G | Verifier-facing shared semantics |
| H | Leaf-mode-specific semantics |

A module may also carry an out-of-band note **P** (platform integration) where its reason for existence is an ICP-platform constraint rather than a protocol concern; **P** is noted separately to preserve taxonomy clarity.

---

## 3. Module inventory and classification

### 3.1 `mktd02` crate — modules under `mktd02/src/` (anchor §1.1)

| Module (source file) | Primary | Secondary | Notes on classification |
|---|---|---|---|
| `lib.rs` (public API surface, `MktdConfig`, `init`, `on_post_upgrade`, public API re-exports) | D | C, P | The public-API surface composes the Phase A/B/C flow and is tightly Leaf-shaped at its composition layer; the shape is orchestration, not protocol predicate. |
| `engine.rs` (`execute_deletion`, `first_init`, `upgrade_cascade`, tombstone / deletion-event hash composition) | D | B, C, H | Central orchestrator for Phase A. Contains hard-coded Leaf-mode hash preimages (`TAG_TOMBSTONE_HASH` inputs, `TAG_EVENT` inputs) and single-subject assumptions. |
| `certified.rs` (certified commitment compute/publish/read; finalization-lock guard on publish) | D | B, G, P | Platform-mechanic module: composes `certified_commitment = H(TAG_CERTIFIED \|\| state_hash \|\| deletion_event_hash)` and calls `ic_cdk::api::set_certified_data`. The pattern (publish-then-query-read) is ICP-platform truth. |
| `finalization.rs` (Phase B `get_pending_certificate`; Phase C `finalize_receipt`; lock release) | D | C, P | A→B→C pattern exists because `ic0.data_certificate()` is query-only on ICP. The A→B→C shape itself is platform truth; specific receipt-shape embedding is Leaf-mode. |
| `guard.rs` (`is_initialised`, `is_tombstoned`, `assert_can_write`) | C | H | Guard semantics are "is **the** canister tombstoned?" — single-subject by construction. |
| `nonce.rs` (`increment_deletion_seq`, deletion-sequence counter) | C | B | Single monotonic counter is Leaf-shaped; Tree mode requires per-leaf or per-subject sequencing. |
| `state.rs` (`compute_state_hash`, `refresh_state_hash`, `read_state_hash`) | B | H | Computes a single state hash over the canister's PII bytes — Leaf-scoped by design. Replaced under Tree mode by tree-root semantics, though the *discipline* of `state_hash` as a canonicalised commitment carries. |
| `storage.rs` (stable-memory slot layout: `certified_commitment` cell, `meta` cell, `receipts` map, `tombstoned_at`, `deletion_event_hash`, finalization-lock cell, `pending_receipt_id` cell, etc.) | F | C, H, P | Concrete slot map (8 slots from `base_memory_id=100`) is Leaf-specific. The *discipline* of explicit slot allocation and `ic-stable-structures` integration carries. |
| `trait_def.rs` (`MKTdDataSource` trait; `CommitMode`, `GuardError`) | — (integration seam) | H | Adapter trait defines MKTd02's library/host seam. ADR-01 has since chosen a narrower Tree-mode seam, so the exact shape is superseded. |
| `export.rs` (receipt export helpers) | A | G | Receipt-shape export; downstream of `zombie-core::receipt` definitions. |

### 3.2 `mktd02-macros` crate (anchor §1.1, same workspace)

| Module | Primary | Secondary | Notes |
|---|---|---|---|
| `mktd02-macros` (`#[mktd_guard]` proc-macro) | — (integration helper) | H | Ergonomic guard installation helper; not protocol truth. |

### 3.3 `zombie-core` external crate (anchor §1.2)

| Module | Primary | Secondary | Notes |
|---|---|---|---|
| `zombie-core::hashing` (`hash_with_tag`, `TAG_EVENT`, `TAG_TOMBSTONE_HASH`, `TAG_CERTIFIED`, `TAG_STATE`, `ZERO_HASH`) | B | G | Domain-tag discipline (ASCII tag bytes, no null terminator; fixed-width big-endian integers; raw principal bytes). Tag namespaces are Leaf-suffixed (`MKTD02_*_V1`). |
| `zombie-core::receipt` (`DeletionReceipt` struct, `compute_receipt_id`, `ProtocolVersion`, `FieldDescriptor`, `ReceiptSummary`) | A | G, H | Leaf-mode receipt struct. Receipt-ID-derivation discipline sits conceptually above the struct shape; treated separately in the reuse audit narrative and candidate list. |
| `zombie-core::tombstone` (`tombstone_constant()`) | H | — | Single canister-scope tombstone constant. Tree mode needs per-position tombstone semantics. |
| `zombie-core::nns_keys` (`active_key_id`, build-configured; `local-replica` feature gate) | G | P | ICP NNS root-key identification; relevant to BLS-certificate verification framing. Mainnet vs local-dev distinction is platform-shaped. |

### 3.4 Category E (Diagnostics / status surface) — absent in MKTd02

Recorded as an explicit inventory entry per G's ruling 2026-04-24 (Q1), so that later readers do not infer Category E was accidentally skipped during classification.

| Surface | Primary | Secondary | Notes |
|---|---|---|---|
| Diagnostics / status surface (no MKTd02 or `zombie-core` module owns this) | E (absent) | — | MKTd02 exposes partial accessors — `guard::is_tombstoned()`, `guard::is_initialised()`, `storage::tombstoned_at`, receipt-count queries — but there is no `StatusSurface`-equivalent module. Diagnostics/status surface is not instantiated at module level in either anchor. See §4 observation 4 for the bounded observation. |

---

## 4. Bounded observations (non-authoritative; flagged for reuse-audit inputs)

1. MKTd02's strongest carry-over surface sits in **category D (finalization/orchestration) where also tagged P (platform integration)** — the A→B→C pattern exists because of `ic0.data_certificate()` query-only semantics, which is ICP-platform truth, not MKTd02 protocol truth.
2. MKTd02's most Leaf-scoped surface sits in **categories B (hashing) and C (sequencing)** — tombstone and event preimages assume one tombstone per canister and one monotonic `deletion_seq`.
3. **Category F (storage abstraction)** contains both reusable discipline (explicit slot allocation, stable-structures integration) and Leaf-specific schema (the 8-slot layout with Leaf-specific cell contents).
4. **Category E (diagnostics/status surface)** is effectively *absent* from MKTd02. MKTd02 has `is_tombstoned()` / `is_initialised()` accessors and a receipt-count query but no `StatusSurface`-equivalent. MKTd03 introduces this as new surface; MKTd02 is not a source of usable prior art here. Inventory record at §3.4.
5. The `trait_def::MKTdDataSource` trait's Leaf-mode assumptions (single-subject tombstoning, `get_state_bytes()` over the whole canister) make it unsuitable as a direct progenitor for the Tree-mode adapter seam that ADR-01 approved.

---

## 5. Out of scope for this taxonomy

- Whether any module should be reused — that decision belongs to the reuse audit (§5.3).
- Whether any module needs back-port / generalisation work — that decision belongs to the candidate list (§5.4).
- Code-level review of MKTd02 correctness. This taxonomy classifies modules; it does not assess them.

---

## 6. Resolved questions — G rulings (2026-04-24)

1. **Anchor pins for §1.1 and §1.2.** Supplied and applied: MKTd02 commit `54f1e2dc24dd0b79705a66894b2f25138e28a9ad` (same SHA as Phase-4 reuse-generalisation audit v1; resolves under the renamed `ICP-Delete-Leaf` repo); `zombie-core-v0.3.1` commit `508f2f8bb88f4395293168c6ef25c92a67dee894`.
2. **Category-E representation.** Add explicit "absent in MKTd02" row to §3. Applied at §3.4.
