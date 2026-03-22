DATE: 2026-03-20

CURRENT GOAL: Run TinyPress design review against TAV Design Principles (all 11), produce ADR
and interface spec, then begin implementation.

GIT STATE
  MKTd03:                    main @ 7836102a (use git rev-parse HEAD for full SHA)
  TAV-Engineering-Standards: main @ d6c7d17  (use git rev-parse HEAD for full SHA)

NOTE: Always update to full SHAs using git rev-parse HEAD before committing this file.

FILES OPEN (edited, not yet committed)
  None

DECISIONS MADE THIS SESSION
  - Repo initialised at Together-Alone-Ventures/MKTd03, SSH auth via WSL, connector verified
  - TinyPress confirmed as MKTd03 toy dApp (per MILESTONE_LOG_TinyPress_March2026_v2)
  - Comments as separate StableBTreeMap records (not embedded Vec) -- settled, do not revisit
  - TAV-Engineering-Standards repo created; Playbook v4 and Design Principles v3 migrated to
    markdown and committed -- this is now the authoritative version of both documents
  - Playbook update process established: record lessons in MILESTONE_LOG during project;
    promote to Playbook at project close with G secondary review

OPEN QUESTIONS (not yet resolved)
  - ICP/DFX toolchain version to pin for TinyPress
  - Which canister framework: vanilla Rust + ic-cdk, or scaffold tool?

KNOWN GOTCHAS FOR NEXT SESSION
  - Git repo (MKTd03) is in WSL at /home/stef_savanah/projects/MKTd03
  - Git repo (standards) is in WSL at /home/stef_savanah/projects/TAV-Engineering-Standards
  - Windows/Dropbox path: C:\Users\Stef\Dropbox\Van Haas\Bonded\Patents\Zombie Delete\MKTd03
  - Always delete old file from Downloads before downloading new version from Claude
  - Always verify filename has no (1) or (2) suffix before running cp
  - Always use full SHA (git rev-parse HEAD) in this file -- not short hash
  - Always verify branch after push: git log --oneline -3 origin/main
  - Claude reads this file from GitHub at session start -- manual paste is fallback only
  - Connector access verified in Claude project (Together-Alone-Ventures/MKTd03)

ACCEPTANCE GATES (Phase 1 -- before any code)
  [x] Repo live on GitHub with main branch
  [x] SSH auth working from WSL
  [x] GitHub connector verified in Claude project
  [x] RESTART_PACK.md and MILESTONE_LOG.md committed with real content
  [x] G secondary review of Day 0 -- passed
  [x] TAV-Engineering-Standards repo created with Playbook and Design Principles in markdown
  [ ] TinyPress design review complete against all 11 TAV Design Principles
  [ ] ADR committed for each significant design decision
  [ ] Interface spec (data structures + failure semantics) written and reviewed
  [ ] Must-Pass checklist (M1-M8) from Design Principles all green

SAFE RESTART PROMPT We are building MKTd03 -- a zombie-delete / GDPR tombstoning protocol on ICP. The toy dApp is TinyPress, a Nuance-inspired single-canister publishing app. TinyPress must be zombie-delete naive (no tombstone awareness in v1). The repo is Together-Alone-Ventures/MKTd03 on GitHub, main branch. TAV Engineering Standards (Playbook + Design Principles) live at Together-Alone-Ventures/TAV-Engineering-Standards. Next task: run TinyPress design review against all 11 TAV Design Principles and produce the ADR and interface spec before any code is written. Please confirm your understanding of the current state before we proceed.
