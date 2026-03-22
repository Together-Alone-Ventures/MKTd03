DATE: 2026-03-23
CURRENT GOAL: Resolve open toolchain questions (DFX version, ic-cdk version), then begin
TinyPress Stage 1 implementation (profiles map).

GIT STATE
  MKTd03:                    main @ 0f44505 (use git rev-parse HEAD for full SHA)
  TAV-Engineering-Standards: main @ d6c7d17  (use git rev-parse HEAD for full SHA)

NOTE: Always update to full SHAs using git rev-parse HEAD before committing this file.

FILES OPEN (edited, not yet committed)
  None

DECISIONS MADE THIS SESSION
  - GitHub connector approach replaced by live fetch via raw.githubusercontent.com
  - Repo made public to enable live fetch
  - TinyPress design review complete against all 11 TAV Design Principles
  - TinyPress ADR + interface spec v1.1 produced, reviewed by Claude + G (x2), committed
  - All Phase 1 Must-Pass gates M1-M8 now green
  - get_comments_by_post returns variant { Ok: vec Comment; Err } not bare vec
  - Hydrated query removed from spec (no hydrated queries in v1)
  - Text field validity rule: handle, display_name, title, content must be non-empty
    and non-whitespace-only; body and bio may be empty

OPEN QUESTIONS (not yet resolved)
  - ICP/DFX toolchain version to pin for TinyPress
  - Which ic-cdk version to pin

KNOWN GOTCHAS FOR NEXT SESSION
  - Git repo (MKTd03) is in WSL at /home/stef_savanah/projects/MKTd03
  - Git repo (standards) is in WSL at /home/stef_savanah/projects/TAV-Engineering-Standards
  - Windows working folder: C:\Users\Stef\Dropbox\Van Haas\Bonded\Patents\Zombie Delete\MKTd03
  - WSL Dropbox path: "/mnt/c/Users/Stef/Dropbox/Van Haas/Bonded/Patents/Zombie Delete/MKTd03"
  - Always move file from Downloads into Windows working folder first, then cp from WSL Dropbox path
  - Always verify filename has no (1) or (2) suffix before running cp
  - Always use full SHA (git rev-parse HEAD) in this file -- not short hash
  - Always verify branch after push: git log --oneline -3 origin/main
  - raw.githubusercontent.com /main/ path has CDN cache lag (can be 10+ min after a commit)
    If Claude fetches stale content, fetch by commit hash instead:
    https://raw.githubusercontent.com/Together-Alone-Ventures/MKTd03/<short-sha>/RESTART_PACK.md
    Short SHA is visible on GitHub repo main page next to latest commit

ACCEPTANCE GATES (Phase 1 -- before any code)
  [x] Repo live on GitHub with main branch
  [x] SSH auth working from WSL
  [x] GitHub live fetch verified (public repo + raw URL in restart prompt)
  [x] RESTART_PACK.md and MILESTONE_LOG.md committed with real content
  [x] G secondary review of Day 0 -- passed
  [x] TAV-Engineering-Standards repo created with Playbook and Design Principles in markdown
  [x] TinyPress design review complete against all 11 TAV Design Principles
  [x] ADR committed for each significant design decision
  [x] Interface spec (data structures + failure semantics) written and reviewed
  [x] Must-Pass checklist (M1-M8) from Design Principles all green

SAFE RESTART PROMPT
  Fetch https://raw.githubusercontent.com/Together-Alone-Ventures/MKTd03/main/RESTART_PACK.md
  and https://raw.githubusercontent.com/Together-Alone-Ventures/MKTd03/main/MILESTONE_LOG.md
  and confirm your understanding of the current state before we proceed.

  Context if needed: We are building MKTd03 -- a zombie-delete / GDPR tombstoning protocol
  on ICP. The toy dApp is TinyPress, a Nuance-inspired single-canister publishing app.
  TinyPress must be zombie-delete naive (no tombstone awareness in v1). The repo is
  Together-Alone-Ventures/MKTd03 on GitHub, main branch. TAV Engineering Standards
  (Playbook + Design Principles) live at Together-Alone-Ventures/TAV-Engineering-Standards.
