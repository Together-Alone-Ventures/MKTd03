DATE: 2026-03-20

CURRENT GOAL: Day 0 complete — begin TinyPress toy dApp design and implementation (MKTd03 Phase 1)

GIT STATE
  MKTd03: main @ 7cd8e06

FILES OPEN (edited, not yet committed)
  None

DECISIONS MADE THIS SESSION
  - Repo initialised at Together-Alone-Ventures/MKTd03 — SSH auth via WSL, connector verified in Claude project
  - TinyPress confirmed as MKTd03 toy dApp (per MILESTONE_LOG_TinyPress_March2026_v2)
  - Comments as separate StableBTreeMap records (not embedded Vec) — settled, do not revisit

OPEN QUESTIONS (not yet resolved)
  - ICP/DFX toolchain version to pin for TinyPress
  - Which canister framework: vanilla Rust + ic-cdk, or scaffold tool?

KNOWN GOTCHAS FOR NEXT SESSION
  - Git repo is in WSL at /home/stef_savanah/projects/MKTd03
  - Windows/Dropbox path: C:\Users\Stef\Dropbox\Van Haas\Bonded\Patents\Zombie Delete\MKTd03
  - Use Dropbox transfer path for any file with special characters (Playbook Appendix B)
  - Always verify branch after push: git log --oneline -3 origin/main
  - Connector access verified in Claude project (Together-Alone-Ventures/MKTd03)

ACCEPTANCE GATES (Day 0)
  [x] Repo live on GitHub with main branch
  [x] SSH auth working from WSL
  [x] GitHub connector verified in Claude project
  [ ] RESTART_PACK.md and MILESTONE_LOG.md committed with real content
  [ ] TAV Design Principles review complete before any TinyPress implementation begins

SAFE RESTART PROMPT
  We are building MKTd03 — a zombie-delete / GDPR tombstoning protocol on ICP. The toy dApp is
  TinyPress, a Nuance-inspired single-canister publishing app. TinyPress must be zombie-delete naive
  (no tombstone awareness in v1). The repo is Together-Alone-Ventures/MKTd03 on GitHub, main branch.
  Please confirm your understanding of the current state before we proceed.
