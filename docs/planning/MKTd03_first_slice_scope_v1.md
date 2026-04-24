# MKTd03 First-Slice Implementation Scope — Planning Artifact (Slice S7-1)

**Status:** AUTHORITATIVE — approved for S7-1 coding-start
**Build plan reference:** §11 Phase 7
**Drafted by:** C (Claude), at G's explicit request per ongoing session role-swap allowance
**Supersedes:** draft v1 (`MKTd03_first_slice_scope_planning_DRAFT.md`)

**v2 changes:**
- §3.2 `module_hash` guardrail tightened per G: surfaced strictly as build-identity/status material implied by the frozen interface, not as provenance binding for this slice. §3.3 exclusions updated accordingly.
- §3.4 `.did` acceptance criterion re-worded per G: "no divergence on the types/methods this slice exposes," replacing the earlier partial-match phrasing.
- Minor cross-reference updates to reflect reuse-audit v2 reclassification of `zombie-core::receipt`.

---

## 1. Purpose

Planning-only artifact. It:

- proposes a specific first implementation slice (S7-1) with explicit scope boundaries,
- states what the slice deliberately does **not** contain,
- identifies the fixtures and interface surfaces the slice exercises,
- lists the hard constraints the slice honours,
- flags the readiness conditions that must be met before coding on S7-1 begins.

All coding-start gates listed below are closed as of anchor c284c59. This document is authorised for S7-1 coding-start.

---

## 2. Context anchors

- **MKTd03 HEAD (user-supplied):** `c284c59`.
- **Frozen-draft library interface:** `interfaces/mktd03_library.did` at `interface_version = 2.0.0`.
- **Companion rules:** `interfaces/mktd03_library_interface_rules.md` at v2 (fixtures reference `rules_version_ref: "...md#v2"`).
- **Adapter-side interface:** `interfaces/mktd03_adapter_contract.did` with `PreStateCaptured` frozen at `0bf90b9`.
- **Fixture corpus:** machine-readable positive and negative fixtures under `docs/test-vectors/fixtures/`; negative families include `missing_transition_derivation_version` added in Session 2.
- **Reuse decisions:** draft reuse-audit v2 classifications are non-authoritative until anchors pinned and G-reviewed. S7-1 does not consume any MKTd02 or `zombie-core` code directly; reuse-audit status therefore does not block S7-1.

---

## 3. Slice S7-1: Status-surface skeleton with lifecycle scaffold

### 3.1 One-sentence scope

Implement a minimal MKTd03 library crate exposing a single query `get_tree_mode_status() -> StatusSurface` and the ICP lifecycle hooks (`init`, `post_upgrade`) needed to bring that query online, with fresh MKTd03-owned storage, fail-loud semantics, and no deletion, receipt, tree-state, proof, provenance-binding, or integration logic.

### 3.2 In scope

- **MKTd03 library crate skeleton.**
  - New Rust crate under the MKTd03 repo (path TBD; candidate: `src/lib.rs` or `crates/mktd03-lib/src/lib.rs` — G to choose).
  - `ic-cdk` dependency matching the MKTd03 toolchain (TAV Principle 9 exact-pinning required; G to set the pinned version explicitly as part of slice approval).
  - `ic-stable-structures` dependency, exact-pinned.
  - `candid` dependency, exact-pinned.
- **Lifecycle hooks.**
  - `#[init]` handler that sets up stable memory and transitions `lifecycle_state` through `uninitialised → initialising → ready` in a single synchronous update.
  - `#[post_upgrade]` handler that reconnects stable memory and re-verifies `lifecycle_state` is `ready` post-upgrade.
  - On any failure during init, trap with an explicit fail-loud message. No degraded-success paths.
- **Storage schema (MKTd03-owned, fresh).**
  - Single stable-memory slot at a named `MemoryId` constant (G to choose; slot map documented as a §Storage header in the crate root).
  - Single cell holding `lifecycle_state: LifecycleState` (per `mktd03_library.did`).
  - Module-hash cell: **see §3.2a below** — scoped strictly to `StatusSurface.build_identity.module_hash` surfacing, not to any provenance-binding role.
  - No receipt map, no tree state, no pending-issuance set, no finalization lock. These belong in later slices.
- **Status-surface query.**
  - `get_tree_mode_status() -> StatusSurface` exposed as `query`.
  - Returns a fully-populated `StatusSurface` per the `.did` type:
    - `protocol_version`, `status_schema_version`, `interface_version = 2.0.0` — all from hard-coded crate constants.
    - `build_identity`: populated with crate build-version constants and the persisted module-hash (wrapped as `opt vec nat8`), strictly as build-identity surfacing per the frozen interface — see §3.2a.
    - `lifecycle_state`: read from the persisted cell.
    - `is_blocked`: `false` during this slice (no blocking conditions are yet possible).
    - `blocked_reason`: `null`.
    - `compatibility`: `compatible` (the slice's own surface is trivially compatible with itself).
    - `operation_context`: `opt some(status_check)`.
- **Companion-rules compliance.**
  - Slice satisfies library interface rules v2 §2 (compatibility-class semantics) and §3 (blocked-state semantics) trivially: never enters a blocked state, always returns `compatibility = compatible`.
  - Library interface rules v2 §4 (certification/provenance posture) does **not** apply to this slice because the slice issues no receipts.

### 3.2a `module_hash` scope guardrail (v2 tightening per G)

The module-hash cell is in S7-1's scope **only** because the frozen `.did` declares `BuildIdentity.module_hash : opt vec nat8` and `StatusSurface.build_identity : BuildIdentity`. S7-1 must surface `module_hash` on every status query because the frozen interface says so; leaving it empty would itself be a partial-match claim against the frozen surface.

**In-scope role for module-hash in S7-1:**
- Received from the deployer via `init(..., module_hash: [u8; 32])` and stored in a stable-memory cell.
- Updated unconditionally on `post_upgrade` (receives the new WASM's module-hash at upgrade time).
- Returned inside `StatusSurface.build_identity.module_hash` on each `get_tree_mode_status()` call.

**Out-of-scope for module-hash in S7-1 (explicit exclusions):**
- Not used as an input to any hash preimage.
- Not bound into any receipt (trivially satisfied — no receipts).
- Not used to compute any certified-data commitment.
- Not treated as provenance evidence in any verifier-facing surface.
- Not subject to any build-gate / trust-root handling beyond persistence and surfacing.

**Rationale:** per G's v2 ruling, `module_hash` in S7-1 exists strictly as build-identity/status material already implied by the frozen interface and companion-rules §3 blocked-state / status surface. Provenance binding belongs in a later receipt/certification slice (S7-4 or later), where it will be scope-planned separately with full treatment of preimage placement, upgrade-continuity of the hash across receipts, and compatibility with Candidate 4 from the candidate-list draft. S7-1 does not smuggle any of that in.

### 3.3 Out of scope (hard exclusions; any of these would expand the slice)

- **No deletion execution of any kind.** No `execute_tree_mode_deletion()` surface.
- **No receipt issuance.** `Receipt` type is not produced by this slice.
- **No tree state.** No Merkle tree construction, rebuild, or traversal.
- **No proof material.** No `tree_proof`, no `proof_material`, no `CoreTransitionEvidence` instantiation.
- **No `transition_derivation_version` handling beyond type-system exposure via the `.did`.** The type is referenced (it appears in the frozen library interface), but no function in this slice's surface accepts or returns a `CoreTransitionEvidence`, so no field validation is needed yet.
- **No certification material.** No `certification_material`, no BLS certificate capture, no Phase B/C.
- **No provenance binding of `module_hash`.** Per §3.2a, `module_hash` is build-identity/status surfacing only; no preimage, receipt, or verifier-facing consumption in S7-1.
- **No version-support probing surface.** No `check_receipt_version_support()` or equivalent. (Reserved for slice S7-2.)
- **No MKTd02 code consumption.** Not a direct dependency, not a vendored copy, not a copied module.
- **No `zombie-core` code consumption** (same reason).
- **No TinyPress integration surface.** No reference-target payloads, route names, schema names, canister names, or fixture examples. ADR-06 contamination guard applies.
- **No orchestration or service-layer surfaces.** Consistent with ADR-01's narrow adapter seam.

### 3.4 Acceptance criteria for S7-1

- Crate compiles to `wasm32-unknown-unknown` under exact-pinned toolchain.
- `dfx deploy` (or equivalent local-replica deploy) succeeds; `init` runs without traps.
- `get_tree_mode_status()` returns `lifecycle_state = ready` after successful init.
- Negative paths: simulated init-time storage failure traps with a specific fail-loud message; no degraded-success fallback.
- One upgrade test: post-upgrade `lifecycle_state = ready`, `module_hash` cell updated to the new WASM's hash and reflected in the next status query.
- At least one existing positive-status-surface machine-readable fixture from `docs/test-vectors/fixtures/` is satisfied by the returned `StatusSurface` shape (fixture selection to be named in the slice-opening prompt; fixture set is already in the repo).
- **`.did` non-divergence gate (v2 wording per G):** the generated `.did` for this slice's exposed crate surface has **zero divergence** — type-for-type and method-for-method — from the subset of `interfaces/mktd03_library.did` that this slice exposes. The gate is a binary non-divergence check on types and methods S7-1 exposes (`SemanticVersion`, `BuildIdentity`, `LifecycleState`, `Compatibility`, `OperationContext`, `BlockedCode`, `BlockedReason`, `StatusSurface`, and `get_tree_mode_status`); any type the slice declares but does not expose is not in scope for the check. "Partial match" is not an acceptable outcome; a divergence on any exposed type or method fails the slice.

### 3.5 Explicit fail-loud invariants this slice establishes

- Any attempt to query `get_tree_mode_status()` when `lifecycle_state` is not persistently readable → trap with specific message. No empty/null/default fallback.
- Any attempt to upgrade with storage schema mismatch → trap. No silent migration.
- Any compatibility-class value other than `compatible`, `conditionally_compatible`, or `unsupported` → unreachable-by-construction (enum closure); slice exercises only `compatible`.
- Any lifecycle state other than `uninitialised`, `initialising`, `ready`, `rebuilding`, or `failed` → unreachable-by-construction (enum closure); slice exercises only the first three.

### 3.6 Slice size estimate

- New Rust code: 200–400 lines in `src/lib.rs` (estimate for G validation).
- `.did` surface exposed by slice: subset of existing frozen interface; no new types introduced.
- Test code: 50–150 lines (init / upgrade / status-surface shape / `.did` non-divergence check).
- Build system / crate metadata: ~30 lines of `Cargo.toml`.

Narrow enough for a single Codex bounded task per build plan §11 Phase 7 ("Each implementation slice should be narrow enough for isolated review and reversal").

---

## 4. Hard constraints S7-1 must honour (restated)

Derived from existing artifacts; enforced by C at review time:

1. No dependence on a fresh MKTd03 reuse decision from MKTd02/`zombie-core` code. **Honoured by construction.**
2. Status-surface contract exercised from day 1. **Honoured by construction.**
3. No post-state / mutation / receipt / verifier / crypto success invented where fixtures don't support it. **Honoured by construction.**
4. No TinyPress-shaped payloads / routes / schemas / canister names / fixtures. **Honoured by construction.**
5. Bounded Codex task, isolated review and reversal possible. **Honoured by construction.**
6. Target `interface_version = 2.0.0`; companion-rule v2. **Honoured explicitly.**
7. No certified record of declaration (without execution). **Honoured by construction** — slice produces no certified record at all.
8. No silent fallback in compatibility / status paths. **Honoured explicitly** — every non-happy-path traps.
9. **`module_hash` scope (new in v2):** surfaced strictly as build-identity/status material per §3.2a; no provenance-binding semantics. **Honoured explicitly** — §3.2a in-scope/out-of-scope list is enforced at review.

---

## 5. What comes after S7-1 (indicative only; not authorising any subsequent slice)

Forward indicator, not a roadmap. Each subsequent slice is separately scope-planned, separately reviewed, separately approved.

- **S7-2 (candidate):** `check_receipt_version_support(SemanticVersion) -> Compatibility`. Exercises the compatibility-class discipline in companion-rules v2 §2. Still produces no receipts. Still consumes no MKTd02 code.
- **S7-3 (candidate):** Receipt storage scaffold — persisted receipt map backed by `ic-stable-structures`, with no receipt issuance yet (insertion path not wired).
- **S7-4 (candidate):** First Phase A→B→C atomic-issuance path on a trivial/seeded-only tree state. **First slice where §5 reuse-audit decisions actually start mattering**, because it touches the A→B→C pattern (Candidate 1 in candidate-list draft). **Also first slice where `module_hash` acquires provenance-binding semantics** (Candidate 4 in candidate-list draft), separately scope-planned at that time. S7-4 cannot be scope-planned until the §5 audit is G-approved.

---

## 6. Readiness conditions before S7-1 coding starts

Restating from §1, now as an explicit checklist (ordering per G's ruling):

1. [ ] Authority-map refresh packet landed. `docs/planning/MKTd03_authority_map_v1.md` updated to reflect formal-interface existence, companion-rules v2, `interface_version = 2.0.0`, `transition_derivation_version` field, `docs/analysis/` directory convention, and post-Session-3 status of all "pre-freeze cleanup needed" rows. Scope-proposed separately.
2. [ ] `AGENTS.md` updated to move from "formal-interface pre-freeze phase" framing to post-spec-tightening / Phase-7-planning framing; implementation-start gate re-stated against the refreshed authority map.
3. [ ] G review close on the §5 reuse-audit draft packet (taxonomy v2 + reuse audit v2 + candidate list v2), with both anchors (MKTd02 commit + `zombie-core` commit-at-tag) pinned. Draft status lifted.
4. [ ] C pre-implementation adversarial review of Phase 6 artifact set as a post-Session-3 whole, per build plan §12 Session 6 review trigger. This is distinct from Session-1/2/3 reviews.
5. [ ] G's explicit coding-start approval per build plan §11 Phase 6 exit-gate bullet eleven.
6. [ ] This slice-scope document (or its successor) approved by G as authoritative, not draft.
7. [ ] Exact toolchain pin (DFX version, Rust toolchain, `ic-cdk` version, `ic-stable-structures` version, `candid` version) recorded in the slice-opening prompt and in the crate `Cargo.toml` per TAV Principle 9.

Items 1 and 2 are grouped as the authority-map / AGENTS refresh packet (scope proposal produced separately in this session).

---

## 7. Explicit non-decisions

This document does not decide:

- Which path in the repo the new crate lives at. G's call.
- The exact `MemoryId` constants (lifecycle cell, module-hash cell). G's call.
- The exact toolchain pin. G's call.
- Whether module-hash cell is in the same stable region as the lifecycle cell or a separate one. G's call (with §3.2a constraint that neither cell is bound into any preimage in S7-1).
- Which positive-status-surface fixture is named in the slice-opening prompt's acceptance criterion. G's call.
- Exact fail-loud trap message strings. Codex's call under bounded prompt.

---

## 8. Open questions for G

1. Is S7-1 the right first slice, or does G prefer a different narrowest slice?
2. Does G want S7-2 and S7-3 scope-planned in parallel with S7-1, or strictly sequentially after S7-1 lands?
3. Is the S7-4 dependency on §5 audit approval (before scope-plan) acceptable?
4. Does G want the readiness checklist in §6 reformatted into a separate tracking document (`MKTd03_Phase7_Readiness_Checklist.md`), or kept inline in this scope proposal?
