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
