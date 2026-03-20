DATE: 2026-03-20

CURRENT GOAL: Run TinyPress design review against TAV Design Principles (all 11), produce ADR and interface spec, then begin implementation.

GIT STATE
  MKTd03: main @ 9d9a075a05514c8c43d91dd8b7944648d02a8db3

FILES OPEN (edited, not yet committed)
  None

DECISIONS MADE THIS SESSION
  - Repo initialised at Together-Alone-Ventures/MKTd03 — SSH auth via WSL, connector verified in Claude project
  - TinyPress confirmed as MKTd03 toy dApp (per MILESTONE_LOG_TinyPress_March2026_v2)
  - Comments as separate StableBTreeMap records (not embedded Vec) — settled, do not revisit
  - RESTART_PACK.md and MILESTONE_LOG.md committed as live continuity artefacts
  - G secondary review of Day 0 completed — no alarm bells, two procedural corrections applied

OPEN QUESTIONS (not yet resolved)
  - ICP/DFX toolchain version to pin for TinyPress
  - Which canister framework: vanilla Rust + ic-cdk, or scaffold tool?

KNOWN GOTCHAS FOR NEXT SESSION
  - Git repo is in WSL at /home/stef_savanah/projects/MKTd03
  - Windows/Dropbox path: C:\Users\Stef\Dropbox\Van Haas\Bonded\Patents\Zombie Delete\MKTd03
  - Use Dropbox transfer path for any file with special characters (Playbook Appendix B)
  - Always verify branch after push: git log --oneline -3 origin/main
  - Always use full SHA (git rev-parse HEAD) in this file -- not short hash
  - Connector access verified in Claude project (Together-Alone-Ventures/MKTd03)

ACCEPTANCE GATES (Phase 1 -- before any code)
  [x] Repo live on GitHub with main branch
  [x] SSH auth working from WSL
  [x] GitHub connector verified in Claude project
  [x] RESTART_PACK.md and MILESTONE_LOG.md committed with real content
  [x] G secondary review of Day 0 -- passed
  [ ] TinyPress design review complete against all 11 TAV Design Principles
  [ ] ADR committed for each significant design decision
  [ ] Interface spec (data structures + failure semantics) written and reviewed
  [ ] Must-Pass checklist (M1-M8) from Design Principles all green

SAFE RESTART PROMPT
  We are building MKTd03 -- a zombie-delete / GDPR tombstoning protocol on ICP. The toy dApp is
  TinyPress, a Nuance-inspired single-canister publishing app. TinyPress must be zombie-delete naive
  (no tombstone awareness in v1). The repo is Together-Alone-Ventures/MKTd03 on GitHub, main branch
  @ 9d9a075a05514c8c43d91dd8b7944648d02a8db3. Next task: run TinyPress design review against all 11
  TAV Design Principles and produce the ADR and interface spec before any code is written.
  Please confirm your understanding of the current state before we proceed.
