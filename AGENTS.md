This repository is the dApp-agnostic MKTd03 protocol project.

Current boundary
- MKTd03 is protocol-side only.
- TinyPress is a reference target, not the definition of protocol truth.
- Do not reopen completed repo-boundary cleanup during design or implementation unless a specific regression is identified.

Current phase
- Prep is closed.
- Repo-boundary cleanup is closed and must not be reopened unless a specific regression is identified.
- The Phase 1–5 protocol baseline is landed; the formal-interface pre-freeze phase is closed; the specification-tightening stream (Sessions 1, 2, 3) is complete; the authority-block housekeeping pass is complete. Current work is Phase-7 planning.
- Phase-7 planning activities: first-slice scope planning, §5 reuse-audit close-out, C pre-implementation adversarial review of the Phase 6 artifact set as a post-Session-3 whole, and G's explicit coding-start decision. No library code is written at this stage.
- Do not start protocol-library implementation until the coding-start gate is explicitly satisfied per `docs/planning/MKTd03_first_slice_scope_v1.md` §6 readiness checklist (currently `MKTd03_first_slice_scope_planning_DRAFT_v2.md` §6 until promoted).
- The readiness checklist in `docs/planning/MKTd03_first_slice_scope_v1.md` §6 is the sole authoritative definition of this gate. AGENTS does not restate or reinterpret it.

Working discipline
- Keep protocol work dApp-agnostic.
- Do not infer protocol truth from TinyPress-local behavior unless explicitly treating it as downstream reference input.
- Prefer the smallest bounded next step.
- Keep review, prep, standards uplift, and implementation clearly separated.
- Treat settled protocol closes recorded in current continuity artifacts as binding for interface-prep work unless explicitly re-gated by G.
- Keep conceptual interface drafting separate from frozen interface approval.

Before coding
- Verify the current task is inside MKTd03 scope.
- Respect current restart/continuity documents at repo root.
- If standards or process questions arise, treat TAV-Engineering-Standards as the doctrine layer, not this AGENTS file.

Non-goals
- No repo-cleanup reopening without a specific regression.
- No TinyPress-local app work in this repo.
- No premature library implementation before the formal-interface gate is complete.
- No premature first-slice coding before authority-map refresh is landed, §5 reuse-audit is G-approved, C pre-implementation adversarial review is complete, and G has given explicit coding-start approval.
