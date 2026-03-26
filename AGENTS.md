This repository is the dApp-agnostic MKTd03 protocol project.

Current boundary
- MKTd03 is protocol-side only.
- TinyPress is a reference target, not the definition of protocol truth.
- Do not reopen completed repo-boundary cleanup during design or implementation unless a specific regression is identified.

Current phase
- We are at the first bounded protocol-library design-prep stage from the clean baseline after repo-boundary cleanup.
- Standards/process extraction into TAV-Engineering-Standards is a separate bounded task.
- Do not start protocol-library implementation until the current prep gates are complete.

Working discipline
- Keep protocol work dApp-agnostic.
- Do not infer protocol truth from TinyPress-local behavior unless explicitly treating it as downstream reference input.
- Prefer the smallest bounded next step.
- Keep review, prep, standards uplift, and implementation clearly separated.

Before coding
- Verify the current task is inside MKTd03 scope.
- Respect current restart/continuity documents at repo root.
- If standards or process questions arise, treat TAV-Engineering-Standards as the doctrine layer, not this AGENTS file.

Non-goals
- No repo-cleanup reopening without a specific regression.
- No TinyPress-local app work in this repo.
- No premature library implementation before prep and standards gates are complete.
