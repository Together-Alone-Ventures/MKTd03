DATE: 2026-03-23

CURRENT GOAL: Stage 1 checkpoint complete. Next: hand off to Codex for Stage 2
(posts map: create_post, get_post, get_posts_by_author, delete_post).

GIT STATE
  MKTd03:                    main @ 3be43fa (use git rev-parse HEAD for full SHA)
  TAV-Engineering-Standards: main @ d6c7d17 (use git rev-parse HEAD for full SHA)

NOTE: Always update to full SHAs using git rev-parse HEAD before committing this file.

FILES OPEN (edited, not yet committed)
  None

DECISIONS MADE THIS SESSION
  - GitHub direct fetch confirmed working (public repo + raw URL); no paste needed
  - CLAUDE.md drafted and committed (landed as claude.md — rename pending)
  - Toolchain pins settled: DFX 0.30.2 (conservative hold; 0.31.0 skipped due to
    @icp-sdk/core frontend churn), ic-cdk =0.19.0, ic-stable-structures =0.7.2,
    candid =0.10.24, serde =1.0.228 (exact pins per Principle 9)
  - DFX 0.30.2 is a deliberate conservative hold, NOT latest (0.31.0 is current)
  - Stage 1 scaffold produced and deployed: profiles map + tinypress_status()
  - principal->owner field rename: 'principal' is a reserved Candid keyword
  - handle_index (MemoryId 3) added for O(1) handle uniqueness check
  - #![allow(deprecated)] added; ic_cdk::api::caller() retained over msg_caller
  - StableCell::init() returns directly (not Result) in ic-stable-structures 0.7.2
  - StableCell::set() returns old value (not Result) in ic-stable-structures 0.7.2
  - Stage 1 compatibility checkpoint PASSED: tinypress_status() returns ok
  - Workflow decision: design gates stay (ADR + G review); implementation handed
    to Codex going forward; Claude + G do prompts and review

OPEN QUESTIONS (not yet resolved)
  - claude.md should be renamed to CLAUDE.md for consistency (minor)
  - cargo-audit not installed (WARN on deploy) — install when convenient
  - dfx.json candid metadata warning still present — dfx.json needs metadata block

KNOWN GOTCHAS FOR NEXT SESSION
  - Git repo (MKTd03) is in WSL at /home/stef_savanah/projects/MKTd03
  - Git repo (standards) is in WSL at /home/stef_savanah/projects/TAV-Engineering-Standards
  - Windows working folder: C:\Users\Stef\Dropbox\Van Haas\Bonded\Patents\Zombie Delete\MKTd03
  - WSL Dropbox path: "/mnt/c/Users/Stef/Dropbox/Van Haas/Bonded/Patents/Zombie Delete/MKTd03"
  - Always delete old file from Dropbox MKTd03 folder BEFORE downloading new version
  - Always verify with sed -n 'Np' to confirm new file landed before running cp
  - Always verify filename has no (1) or (2) suffix before running cp
  - Always use full SHA (git rev-parse HEAD) in this file — not short hash
  - Always verify branch after push: git log --oneline -3 origin/main
  - Kill daffydefs replica before starting MKTd03 sessions:
      pkill -9 -f '/home/stef_savanah/projects/daffydefs/.dfx/network/local'
      pkill -9 -f '/home/stef_savanah/.cache/dfinity/versions/0.24.3/pocket-ic'
      pkill -9 -f '/home/stef_savanah/.cache/dfinity/versions/0.24.3/replica'
      pkill -9 -f '/home/stef_savanah/.cache/dfinity/versions/0.24.3/ic-https-outcalls-adapter'
  - raw.githubusercontent.com has CDN cache lag (10+ min after commit)
    Fetch by commit hash if stale:
    https://raw.githubusercontent.com/Together-Alone-Ventures/MKTd03/<sha>/RESTART_PACK.md
  - Commit message exclamation marks trigger bash history expansion in WSL
    Use single quotes around commit message or avoid ! in messages

ACCEPTANCE GATES (Stage 2 — before Codex begins)
  [ ] Codex prompt drafted by Claude + G
  [ ] Codex prompt reviewed and approved
  [ ] Codex produces Stage 2 diff (posts map)
  [ ] Stage 2 diff reviewed by Claude (primary) + G (secondary)
  [ ] cargo build passes clean
  [ ] dfx deploy succeeds
  [ ] tinypress_status() returns post_count correctly after create_post calls
  [ ] All Stage 2 failure paths tested (NotFound, Forbidden, ProfileNotFound, InvalidInput)

SAFE RESTART PROMPT
  Fetch https://raw.githubusercontent.com/Together-Alone-Ventures/MKTd03/main/RESTART_PACK.md
  and https://raw.githubusercontent.com/Together-Alone-Ventures/MKTd03/main/MILESTONE_LOG.md
  and confirm your understanding of the current state before we proceed.
  Context if needed: We are building MKTd03 — a zombie-delete / GDPR tombstoning protocol
  on ICP. The toy dApp is TinyPress, a Nuance-inspired single-canister publishing app.
  TinyPress must be zombie-delete naive (no tombstone awareness in v1). The repo is
  Together-Alone-Ventures/MKTd03 on GitHub, main branch. TAV Engineering Standards
  (Playbook + Design Principles) live at Together-Alone-Ventures/TAV-Engineering-Standards.
