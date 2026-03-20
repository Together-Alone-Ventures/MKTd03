## 2026-03-20 -- MILESTONE: Day 0 -- Repo and tooling initialised

Decisions made:
  - MKTd03 repo created at Together-Alone-Ventures/MKTd03, main branch, SSH auth from WSL
  - GitHub connector verified in Claude project -- all three root files readable
  - TinyPress confirmed as toy dApp per MILESTONE_LOG_TinyPress_March2026_v2 (design doc in project)
  - TAV Vibe Coding Playbook v4 and TAV Design Principles v3 confirmed as governing documents
  - Restart pack must always use full SHA (git rev-parse HEAD), not short hash

Irreversible actions taken:
  - None (design phase only, no code committed)

Do not revisit:
  - Comments as embedded Vec inside posts -- rejected. Separate StableBTreeMap only.
  - TinyPress having any knowledge of tombstoning mechanics in v1 -- out of scope by design.

Secondary review (G):
  - Day 0 reviewed -- no alarm bells
  - Corrections applied: full SHA in restart pack; design gate is mandatory before any code
