DATE: 2026-03-24

CURRENT GOAL: TinyPress Stage 4 — expose get_comments_by_author as public query.
Storage and index already exist (COMMENTS_BY_AUTHOR, MemoryId 10). This is a
bounded single-method addition following the Stage 3 pattern.

GIT STATE
    MKTd03:                    main @ f55d63615ed6966882484dfaa1c012083bbddc2a
    TinyPress:                 main @ e4d8d67aa5b784d4dc3fbb68455b533867afa996
    TAV-Engineering-Standards: main @ d6c7d17 (full SHA not yet refreshed)

FILES OPEN (edited, not yet committed)
  None

DECISIONS MADE THIS SESSION
  - Stage 3 complete: comments map, COMMENTS_BY_POST, COMMENTS_BY_AUTHOR indexes
  - StoredComment/Comment split: reply_to_comment_id stored internally, absent
    from public API and DID
  - StoredCommentCodec used as candid serialisation helper (CandidType derive on
    codec only, not on StoredComment)
  - get_comments_by_author deferred to Stage 4 — storage ready, public query not yet exposed
  - dfx.json candid path fixed: src/tinypress/tinypress.did -> tinypress.did
  - Stage 3 secondary review (G) passed clean at e4d8d67

OPEN QUESTIONS (not yet resolved)
  - StoredCommentCodec derives CandidType unnecessarily — harmless, park for cleanup pass
  - cargo-audit not installed (WARN on deploy) — install when convenient
  - dfx.json candid metadata warning still present — dfx.json needs metadata block
  - claude.md should be renamed to CLAUDE.md (minor)
  - Review session for Playbook updates: .did-before-testing sequencing (§8.1
    extension); LazyEntry API lesson (verify iterator API against installed crate version)

KNOWN GOTCHAS FOR NEXT SESSION
  - Git repo (MKTd03) is in WSL at /home/stef_savanah/projects/MKTd03
  - Git repo (TinyPress) is in WSL at /home/stef_savanah/projects/TinyPress
  - Git repo (standards) is in WSL at /home/stef_savanah/projects/TAV-Engineering-Standards
  - Windows working folder: C:\Users\Stef\Dropbox\Van Haas\Bonded\Patents\Zombie Delete\MKTd03
  - WSL Dropbox path: "/mnt/c/Users/Stef/Dropbox/Van Haas/Bonded/Patents/Zombie Delete/MKTd03"
  - Always delete old file from Dropbox MKTd03 folder BEFORE downloading new version
  - Always verify with sed -n 'Np' to confirm new file landed before running cp
  - Always verify filename has no (1) or (2) suffix before running cp
  - Always use full SHA (git rev-parse HEAD) in this file — not short hash
  - Always verify branc
