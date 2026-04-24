DATE: 2026-04-24

CURRENT GOAL:
S7-1 implementation is open. The first implementation slice is the status-surface skeleton with lifecycle scaffold defined by `docs/planning/MKTd03_first_slice_scope_v1.md`.

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
MKTd03 main is at 8204b2c.
MKTd03 remains dApp-agnostic; TinyPress remains a reference target only.

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

NEXT BOUNDED TASK:

Draft and review the Codex bounded implementation prompt for S7-1.

The prompt must:
- implement only the status-surface skeleton with lifecycle scaffold;
- honour the five S7-1 binding constraints above;
- define exact scope and acceptance checks;
- prohibit Codex from making architecture, sequencing, future-slice, receipt, proof, certification, or provenance decisions.

No README, marketing, TinyPress, or housekeeping work is to be batched into S7-1 implementation.

SAFE RESTART PROMPT:

MKTd03 main is at 8204b2c. `docs/planning/MKTd03_first_slice_scope_v1.md` is authoritative and S7-1 is open for implementation. Standard role map is restored: G drafts and judges, C reviews adversarially, Codex executes bounded implementation tasks. S7-1 is status-surface only and is governed by five binding constraints: strict slice containment, `module_hash` status-only discipline, `.did` zero-divergence gate, observable fail-loud behavior, and no forward-semantics leakage. The next bounded task is to draft and review the Codex implementation prompt for S7-1. No housekeeping or README work is to be batched.
