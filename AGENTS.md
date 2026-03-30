This repository is the dApp-agnostic MKTd03 protocol project.

Current boundary
- MKTd03 is protocol-side only.
- TinyPress is a reference target, not the definition of protocol truth.
- Do not reopen completed repo-boundary cleanup during design or implementation unless a specific regression is identified.

Current phase
- Prep is closed.
- Repo-boundary cleanup is closed and must not be reopened unless a specific regression is identified.
- The Phase 1-5 protocol baseline exists in repo artifacts and current work is the bounded formal-interface pre-freeze phase.
- This phase is for interface hygiene, authority cleanup, conceptual-interface refinement, companion-rule tightening, and vector/fixture preparation.
- Do not start protocol-library implementation until the formal-interface gate is explicitly satisfied.

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
