DATE: 2026-04-29

CURRENT GOAL:
S7-11 is closed and pushed after the S7-5 through S7-10 hashing / commitment-wrapper line. The next likely bounded decision is whether to open S7-12 tree-proof envelope serialization/parsing.

IMPORTANT SCOPE RULE:
This file is for MKTd03 protocol work only.
TinyPress implementation sessions must use the TinyPress repo's own RESTART_PACK.md, not this file.

CURRENT STATUS:

Prep closed.
Repo-boundary cleanup closed.
Standards uplift complete.
Formal-interface/conformance phase closed at cdfc097.
Specification-tightening stream (Sessions 1 + 2 + 3) fully landed.
Authority-block housekeeping landed at 4ea134a.
Authority-map / AGENTS refresh packet landed at 274f6015.
§5 reuse-audit close-out landed at 776aff7.
Pre-implementation adversarial review continuity update landed at 1dcdee1.
Pre-implementation milestone update landed at c284c59.
First-slice scope promotion landed at 8204b2c — `planning: add first-slice scope v1 (S7-1) for coding-start`.
S7-5 landed at f0c9493 — `implementation: add S7-5 leaf hash constructors`.
S7-6 landed at 6b36448 — `implementation: add S7-6 record-position-key derivation`.
S7-7 landed at af091e2 — `implementation: add S7-7 internal-node hashing`.
S7-8 landed at 9866217 — `implementation: add S7-8 empty-subtree root ladder`.
S7-9 landed at ce32acd — `implementation: add S7-9 transition_material derivation`.
Parallel type-surface debt was logged at 523fe00 — `milestone_log: track parallel type-surface debt`.
S7-10 landed at 4a056a6 — `implementation: add S7-10 state commitment wrappers`.
S7-11 landed at 0dca301 — `implementation: add S7-11 per-frame proof serialization/parsing`.
S7-11 continuity close landed at b72624c — `continuity: record S7-11 close`.
MKTd03 main is at b72624c.
MKTd03 remains dApp-agnostic; TinyPress remains a reference target only.

HASHING / SMT-FOUNDATION BLOCK SUMMARY:

S7-5 through S7-8 establish the pure hashing / SMT-foundation layer:
- leaf hashes
- record-position keys
- internal-node hashes
- empty-subtree roots

These were source-only implementation slices.

No `.did`, Cargo, docs/spec, fixtures, commitment, proof, receipt, or canister public API changes were made in these slices.

Current library test count after S7-8: 52 tests passing.
wasm build passes.

POST-FOUNDATION HASH / COMMITMENT WRAPPER SUMMARY:

S7-9 added §4.4 `transition_material` derivation, including private §4.1 `SemanticVersion` big-endian encoding.

S7-10 added wrapper-only §7.2 / §8.2 `pre_state_commitment` and `post_state_commitment` functions.

S7-10 deliberately does not compute roots; §6.3 root computation remains deferred.

No `.did`, Cargo, docs/spec, fixtures, proof-frame, certified-commitment, receipt-ID, or public canister API changes were made in S7-9/S7-10.

Current library test count after S7-10: 67 tests passing.
wasm build passes.

PROOF-FRAME SERIALIZATION SUMMARY:

S7-11 added deterministic byte-level serialization and parsing for individual tree-proof frames under §9.3–§9.5.

Worktree was clean at the previous close.

KNOWN TRACKED DEBT:

Parallel Candid-bound and reference-runtime type surfaces are tracked in `MILESTONE_LOG` at 523fe00.

Do not consolidate those type surfaces unless a concrete call site requires it, or after §11/§12 receipt-construction work makes the pressure visible.

ROLE MAP:

Standard role map is restored as of 8204b2c.

- G: primary drafter and architectural judgment.
- C: adversarial reviewer.
- Codex: bounded implementation executor.

The prior role-swap allowance was scoped to the review window and is closed.

FIRST-SLICE SCOPE PROMOTION — 2026-04-24:

`docs/planning/MKTd03_first_slice_scope_v1.md` is now authoritative for S7-1.

It was promoted from the first-slice planning draft at commit 8204b2c, parent c284c59.

Promotion applied four mechanical edits:
1. Title desuffixed from draft-v2 form to authoritative v1 title.
2. Status line changed from draft / non-authoritative / for-review language to authoritative coding-start status.
3. Intended-path line deleted because the file now lives at the promoted path.
4. §1 gating paragraph replaced with a state statement that all coding-start gates are closed as of anchor c284c59 and the document is authorised for S7-1 coding-start.

The in-file context anchor was also updated from 3319c3f to c284c59 during promotion. This is the context anchor inside the promoted document, not the promotion commit SHA.

S7-1 IS OPEN — BINDING CONSTRAINTS:

S7-1 is open under G's coding-start ruling, with five binding constraints:

1. Strict slice containment:
   - S7-1 is status-surface only.
   - No deletion execution.
   - No receipt construction.
   - No tree state or Merkle logic.
   - No proof material.
   - No certification or BLS handling.
   - No provenance binding.

2. `module_hash` discipline:
   - Allowed only for persistence and surfacing in `StatusSurface.build_identity`.
   - Must not be included in any hash preimage.
   - Must not be interpreted as provenance evidence.
   - Must not be bound into any evidence-bearing structure.

3. `.did` zero-divergence gate:
   - Generated `.did` for exposed S7-1 types and methods must have zero divergence from the corresponding subset of `interfaces/mktd03_library.did`.
   - No partial-match or “compatible but different” outcome is acceptable.

4. Observable fail-loud behavior:
   - Invalid lifecycle or unreadable state conditions must trap or fail loudly.
   - No partial/default success-like status response may mask failure.

5. No forward-semantics leakage:
   - Implementation must not hint at receipt lifecycle, issuance, proof, deletion, certification, or later-slice semantics.
   - Storage must not be pre-shaped for future slices beyond what S7-1 strictly requires.

CURRENT AUTHORITIES FOR S7-1:

- `docs/planning/MKTd03_first_slice_scope_v1.md` — authoritative S7-1 scope and readiness surface.
- `docs/planning/MKTd03_authority_map_v2.md` — authority map for the Phase 6 artifact set.
- `interfaces/mktd03_library.did` — frozen public library interface.
- `interfaces/mktd03_library_interface_rules.md` — companion rules v2.
- `docs/test-vectors/fixtures/` — fixture corpus; S7-1 must use only fixtures relevant to the status surface.

OPEN CANDIDATES FOR FUTURE BOUNDED SESSIONS:

The six queued candidates remain unchanged and none blocks S7-1 implementation:

1. `docs/spec/MKTd03_protocol_refresh_v1.md` `## Status` field still reads "Draft"; confirm intended status before editing.
2. AGENTS adjacent drift: old interface-prep / formal-interface wording remains in preserved historical lines; light-touch cleanup candidate.
3. Authority map v2 conceptual-interface row wording should later clarify non-authoritative status and non-divergence obligation on overlapping types.
4. Authority map v1 disposition: preferred relocation to `docs/planning/history/`.
5. Phase-4 reuse-generalisation audit v1 still references the old MKTd02 repo URL; update to renamed `ICP-Delete-Leaf` URL on next touch.
6. Playbook uplift candidates: acronym-gap stop-and-escalate, standing-constraint propagation, repo-rename SHA carry-over, and multi-line web-editor edit lesson.

NEXT BOUNDED DECISION:

S7-12 is a likely candidate for §9.2 tree-proof envelope serialization/parsing.

This is not yet approved for implementation and must not be treated as settled continuity.

The first step in the next chat should be C pre-execution adversarial review of the exact S7-12 scope.

Standing constraints to carry forward:
- S7-12 envelope work must compose `parse_proof_frame` over the §9.2 fixed proof envelope.
- §9.2 envelope is 2-byte big-endian step count followed by exactly 256 serialized frames.
- Future implementation review bundles must include full source file contents and full unified diffs, not placeholders or stats alone.

SAFE RESTART PROMPT:

MKTd03 main is at b72624c. S7-9 added `transition_material` derivation, S7-10 added wrapper-only `pre_state_commitment` / `post_state_commitment`, and S7-11 added per-frame tree-proof serialization/parsing. S7-10 still does not compute roots; §6.3 remains deferred. No `.did`, Cargo, docs/spec, fixtures, proof-frame envelope, certified-commitment, receipt-ID, or public canister API changes were made in S7-9/S7-11 beyond the bounded slice scopes. Current library test count remains 67 passing and wasm build passes at the recorded close. Parallel Candid-bound and reference-runtime type-surface debt is tracked in `MILESTONE_LOG` at 523fe00 and should not be consolidated absent a concrete call site or later §11/§12 pressure. The next likely bounded decision is whether to open S7-12 tree-proof envelope serialization/parsing, with C adversarial review first. Standing constraints: S7-12 must compose `parse_proof_frame` over the fixed §9.2 envelope; the envelope is 2-byte big-endian step count plus exactly 256 serialized frames; and future implementation review bundles must include full source file contents and full unified diffs.
