## 2026-03-20 -- SESSION LESSON: Windows duplicate file naming trap

What happened:
  - Downloaded updated files from Claude to Windows Downloads folder, but previous versions were
    still there. Windows silently renamed the new files to RESTART_PACK (1).md etc, so the cp
    command copied the old versions into the repo instead of the new ones.

The lesson:
  - Always delete previous versions from C:\Users\Stef\Downloads\ before downloading a new version
    of any file from Claude. Verify the filename has no (1) or (2) suffix before running cp.

Category: process

Apply next time:
  - Before downloading any file from Claude: delete the old version from Downloads first.
  - After cp: run `head -5 <filename>` to verify the content is what you expect before committing.

---

## 2026-03-20 -- SESSION LESSON: GitHub connector replaces restart pack paste

What happened:
  - Playbook §3.3 requires pasting RESTART_PACK.md as the first message in every new chat.
    But Claude now has direct GitHub connector access to the live repo.

The lesson:
  - At the start of each new session, Claude reads RESTART_PACK.md directly from GitHub and
    confirms the state back to the operator. Manual paste is the fallback if the connector is
    down or the file looks stale. Claude must flag explicitly if the file is missing or stale.

Category: process | tooling

Apply next time:
  - Do not paste the restart pack manually unless Claude flags a problem with the connector read.
  - Claude opens every session with: read RESTART_PACK.md from GitHub, confirm contents, proceed.

---

## 2026-03-20 -- MILESTONE: Day 0 -- Repo and tooling initialised

Decisions made:
  - MKTd03 repo created at Together-Alone-Ventures/MKTd03, main branch, SSH auth from WSL
  - GitHub connector verified in Claude project -- all three root files readable
  - TinyPress confirmed as toy dApp per MILESTONE_LOG_TinyPress_March2026_v2 (design doc in project)
  - TAV Vibe Coding Playbook v4 and TAV Design Principles v3 confirmed as governing documents
  - Restart pack must always use full SHA (git rev-parse HEAD), not short hash
  - TAV Playbook and Design Principles to be moved into their own GitHub repo for version control

Irreversible actions taken:
  - None (design phase only, no code committed)

Do not revisit:
  - Comments as embedded Vec inside posts -- rejected. Separate StableBTreeMap only.
  - TinyPress having any knowledge of tombstoning mechanics in v1 -- out of scope by design.

Secondary review (G):
  - Day 0 reviewed -- no alarm bells
  - Corrections applied: full SHA in restart pack; design gate is mandatory before any code

---

## 2026-03-23 -- MILESTONE: Phase 1 complete -- TinyPress ADR and interface spec

Decisions made:
  - TinyPress design review completed against all 11 TAV Design Principles v3
  - ADR-01 through ADR-11 settled (see docs/TinyPress_ADR_v1.1.docx)
  - Interface spec complete: three data structures, all operations, full error type,
    failure semantics table, auth model, privacy boundary, M1-M8 all green
  - Key decisions: vanilla ic-cdk (no scaffold); caller-derived author identity;
    creator_handle copy documented as deliberate tradeoff; no deletion-aware
    terminology anywhere in TinyPress API; get_comments_by_post returns variant not
    bare vec; body and bio may be empty; handle/title/content must be non-whitespace
  - GitHub connector approach replaced by live raw.githubusercontent.com fetch;
    repo made public to enable this

Irreversible actions taken:
  - docs/TinyPress_ADR_v1.1.docx committed at 0f44505

Do not revisit:
  - Comments as embedded Vec inside posts -- rejected, separate StableBTreeMap only
  - Scaffold tools for TinyPress -- rejected, vanilla ic-cdk only
  - author_profile_id accepted as caller input -- rejected, derived from caller always
  - Deletion-aware terminology in TinyPress API -- banned by ADR-06
  - Hydrated queries in v1 -- out of scope

Secondary review (G):
  - Two adversarial passes completed
  - Pass 1: six issues raised, all accepted and incorporated
  - Pass 2: three consistency fixes raised, all accepted and incorporated
  - Verdict after pass 2: gate genuinely passed, coding may begin

---

## 2026-03-23 -- SESSION LESSON: Windows file transfer workflow

What happened:
  - Confusion between Downloads folder and working Dropbox folder caused cp failures.
    Claude initially gave instructions for Downloads path; file was actually in Dropbox.

The lesson:
  - Always move downloaded files from Windows Downloads into the Windows working folder
    (C:\Users\Stef\Dropbox\Van Haas\Bonded\Patents\Zombie Delete\MKTd03) first.
  - Then cp from WSL Dropbox path:
    "/mnt/c/Users/Stef/Dropbox/Van Haas/Bonded/Patents/Zombie Delete/MKTd03"
  - Never cp directly from Downloads.

Category: process

Apply next time:
  - Claude should always give cp commands using the WSL Dropbox path, not Downloads.
  - Verify with ls before cp to confirm file is present at the expected path.

---

## 2026-03-23 -- SESSION LESSON: raw.githubusercontent.com CDN cache lag

What happened:
  - After committing a RESTART_PACK.md update via GitHub web editor, fetching via
    the /main/ path returned stale content for 10+ minutes. Fetching by commit hash
    returned current content immediately.

The lesson:
  - raw.githubusercontent.com /main/ path is CDN-cached and can lag significantly.
  - If Claude fetches stale content at session start, fetch by commit hash instead:
    https://raw.githubusercontent.com/Together-Alone-Ventures/MKTd03/<short-sha>/RESTART_PACK.md
  - Short SHA is visible on the GitHub repo main page next to the latest commit.

Category: tooling

Apply next time:
  - If fetched content looks stale (current goal or decisions don't match expectations),
    paste the commit-hash URL into the chat for Claude to fetch directly.
  - Claude must flag explicitly if fetched content appears stale rather than proceeding silently.
