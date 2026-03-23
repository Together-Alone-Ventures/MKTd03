DATE: 2026-03-23

CURRENT GOAL: Separate TinyPress into its own repo before Stage 3 begins.
Next: create TinyPress repo, migrate src/tinypress/, then proceed with Stage 3
(comments map) in the new TinyPress repo.

GIT STATE
  MKTd03:                    main @ 21d214a0ddace317951bdaef049e0a7966b28075
  TAV-Engineering-Standards: main @ d6c7d17 (full SHA not yet refreshed this session)

FILES OPEN (edited, not yet committed)
  None

DECISIONS MADE THIS SESSION
  - All domains enabled in Settings > Capabilities (code execution network egress)
  - Stage 2 implemented by Codex: posts map + PostAuthorKey composite key
  - LazyEntry API confirmed for StableBTreeMap::range() in ic-stable-structures 0.7.2
    (yields LazyEntry, not tuple; use entry.key().clone() to extract key)
  - Big-endian encoding confirmed correct for composite key prefix iteration
  - delete_post error semantics confirmed: ProfileNotFound (no profile) vs Forbidden (wrong profile)
  - Invariant panic (not filter_map) required in get_posts_by_author for missing POSTS entry
  - .gitignore created (.dfx/ and target/)
  - tinypress.did updated with Post type and Stage 2 service entries
  - lib.rs mode changed 100755 -> 100644 in Stage 2 commit — not accidental; correct posture
  - Post ID counter is at 1 after acceptance testing (post 1 created then deleted);
    next create_post will return 2
  - All Stage 2 acceptance gates passed
  - TinyPress repo separation decided: TinyPress must be a separate repo before
    Stage 3 begins; G and Claude agreed separation enforces the integration boundary correctly

OPEN QUESTIONS (not yet resolved)
  - claude.md should be renamed to CLAUDE.md for consistency (minor)
  - cargo-audit not installed (WARN on deploy) — install when convenient
  - dfx.json candid metadata warning still present — dfx.json needs metadata block
  - Review session for Playbook updates: .did-before-testing sequencing (§8.1 extension);
    LazyEntry API lesson (verify iterator API against installed crate version)

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
  - StableBTreeMap::range() yields LazyEntry in 0.7.2 — use entry.key().clone(), not tuple destructure
  - Interface file must be updated before acceptance testing — dfx produces misleading type errors
    (not logic errors) when .did is stale; Stage 2 logic was correct before .did update but results
    looked like failures
  - delete_post Forbidden test requires a different identity that has a profile; same identity = Ok;
    no-profile caller = ProfileNotFound — use a second dfx identity for Forbidden testing in Stage 3
  - Commit message exclamation marks trigger bash history expansion in WSL
    Use single quotes around commit message or avoid ! in messages
  - raw.githubusercontent.com reachable via bash_tool curl (All domains setting); web_fetch still blocked

ACCEPTANCE GATES (Stage 3 — before Codex begins)
  [ ] Codex prompt drafted by Claude + G
  [ ] Codex prompt reviewed and approved
  [ ] Codex produces Stage 3 diff (comments map)
  [ ] Stage 3 diff reviewed by Claude (primary) + G (secondary)
  [ ] cargo build passes clean
  [ ] dfx deploy succeeds
  [ ] tinypress_status() returns comment_count correctly after create_comment calls
  [ ] All Stage 3 failure paths tested (NotFound, Forbidden, ProfileNotFound, PostNotFound, InvalidInput)

SAFE RESTART PROMPT
  Fetch https://raw.githubusercontent.com/Together-Alone-Ventures/MKTd03/main/RESTART_PACK.md
  and https://raw.githubusercontent.com/Together-Alone-Ventures/MKTd03/main/MILESTONE_LOG.md
  and confirm your understanding of the current state before we proceed.
  Context if needed: We are building MKTd03 — a zombie-delete / GDPR tombstoning protocol
  on ICP. The toy dApp is TinyPress, a Nuance-inspired single-canister publishing app.
  TinyPress must be zombie-delete naive (no tombstone awareness in v1). The repo is
  Together-Alone-Ventures/MKTd03 on GitHub, main branch. TAV Engineering Standards
  (Playbook + Design Principles) live at Together-Alone-Ventures/TAV-Engineering-Standards.
