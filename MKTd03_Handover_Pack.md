# MKTd03 — AI Restart Pack for Handover

**Date:** 2026-04-01  
**Audience:** Coder’s AI assistant, assuming no prior project knowledge  
**Purpose:** Full technical and operational handover pack for the current MKTd03 state  
**Current checkpoint:** `MKTd03 main @ 9bd9f1e`  
**Status:** Current phase cleanly closed. Next phase not yet opened.

## 0. How to use this document

Read this file first, then use it as the navigation layer into the real authority chain.

This document is intentionally comprehensive, but it is not the only source of truth. It is a synthesis and orientation pack. When exact semantics, exact types, exact wording, or exact boundaries matter, consult the authoritative repo artifacts listed below.

This pack is designed to make a new AI productive quickly without re-litigating settled work and without silently importing stale assumptions from older docs.

## 1. What MKTd03 is

MKTd03 is the Tree-mode member of the MKTd protocol family.

High-level family framing:
- MKTd02 = Leaf mode
- MKTd03 = Tree mode

MKTd03 is intended to become a dApp-agnostic protocol / integration layer for proving configured deletion / tombstoning transitions in shared ICP state, where data is not isolated cleanly into one per-user record surface.

That is the key difference from MKTd02:
- MKTd02 proved deletion / tombstoning in a Leaf-mode setting with a simpler proof boundary.
- MKTd03 is for Tree-mode shared-state topologies, where:
  - subject-linked data may exist across multiple record surfaces,
  - record boundaries and deletion scope are often application-specific,
  - proof structure and invariants change materially,
  - app-specific discovery / mutation work must be separated cleanly from generic protocol logic.

MKTd03 is not currently a finished engine. It is a protocol/library build in a spec-first, ADR-first, interface-gated phase sequence.

## 2. What problem MKTd03 is trying to solve

In real ICP dApps, especially social/publishing/community-style apps, a user’s personal data is rarely confined to one isolated root record. It may appear in:
- a profile record,
- authored posts,
- authored comments,
- copied handles or identifiers,
- secondary indexes,
- linkages / relationships,
- other structured residues.

A platform may claim to have deleted or tombstoned data, but without a verifiable receipt/proof layer, that claim is hard to audit independently.

MKTd03 aims to make it possible to prove, in a protocol-governed way, that a defined set of in-scope records or state transitions were tombstoned / deleted according to the configured deletion model of the integrated application.

Important scope boundary:

MKTd03 is not intended to prove that “all PII everywhere in the application is gone.” It is intended to prove configured deletion / tombstoning transitions for declared in-scope records in shared ICP state.

That scope boundary is central and must not be loosened casually.

## 3. What has already been learned from MKTd02

MKTd02 is the successful earlier build and provides both:
1. a family baseline for MKTd03, and
2. a large set of process and design lessons.

The most important carry-forwards are:

### 3.1 Design lessons
- interface-definition sequencing before coding matters
- explicit failure semantics must exist before implementation
- diagnostics/status surfaces must exist from the outset
- compatibility/versioning posture must be decided early
- protocol truth, integration truth, and release truth must be kept separate
- golden vectors and negative cases must exist before core protocol implementation
- silent failures are especially dangerous in protocol / cryptographic systems

### 3.2 Workflow lessons
- keep one live restart pack and one append-only milestone log
- use Codex as a bounded operator, not a roaming architect
- use targeted adversarial review at phase gates
- trust repo ground truth over memory or older plan docs
- phase-gated work is necessary, but internal churn inside a phase should be reduced

### 3.3 Technical reuse posture
Reuse from MKTd02 is desirable, but must be demonstrated module by module. Tree mode is not a trivial delta from Leaf mode.

Do not assume “same family” means “same architecture with minor edits.”

## 4. Operating doctrine for this project

The project is run under two standing doctrine documents:

### 4.1 TAV Design Principles
Read first before design decisions.

Core implications for MKTd03:
- composability first
- simplicity before cleverness
- specification before implementation
- diagnostics by design
- fail loud, never fail silent
- incremental integration
- observability over convenience
- idempotence / re-runnability
- stable identifiers + versioning
- security by design
- privacy by design

### 4.2 TAV Vibe Coding Playbook
Read before active implementation work.

Core implications for MKTd03:
- one live restart pack, one continuity log
- explicit source-of-truth layering
- bounded Codex workflow
- review ladder discipline
- CLI truth beats connector truth for uncommitted state
- interface changes require synchronized .did / binding / verification work
- release is evidence-gated, not vibes-gated

This handover pack assumes those doctrines remain active unless explicitly superseded.

## 5. Repo landscape and role of each repo

Coder will have access to multiple related repos. Their roles are distinct.

### 5.1 MKTd03 repo
Primary repo for:
- dApp-agnostic protocol work
- ADRs
- interface definitions
- protocol rules / notes
- vectors / fixtures
- continuity and current handover state

This is the main handover repo.

### 5.2 TinyPress repo
TinyPress is the zombie-delete-naive toy dApp for MKTd03.

It exists to provide a realistic Tree-mode reference target analogous to how DaffyDefs served MKTd02.

Critical constraint:
- TinyPress must remain MKTd03-naive until explicit integration phases.
- TinyPress is not protocol authority.
- TinyPress is a reference target only.

### 5.3 MKTd02 / DaffyDefs related repos
These provide:
- proven prior-family behavior,
- reuse candidates,
- operational lessons,
- verifier / receipt / build / release experience.

Use them as:
- comparison baseline,
- reuse analysis input,
- test strategy inspiration.

Do not let MKTd02 implementation details silently define MKTd03 protocol truth.

### 5.4 CVDR-Verify
Verifier-side and schema-validation context from the MKTd02 world.
Useful for:
- backward-compat lessons,
- dispatch/versioning lessons,
- verifier discipline,
- what went wrong when protocol/interface sequencing was weak.

### 5.5 zombie-core
Shared protocol-core learnings and possible reusable machinery.
Again: use as reuse-analysis input, not automatic truth.

### 5.6 zombie-delete-docs
Shared docs source and generation discipline reference.
Useful for understanding evidence/doc coupling and pinned artifact generation discipline.

### 5.7 TAV-Engineering-Standards
Contains the standing methodology:
- playbook,
- design principles,
- reusable project doctrine.

## 6. What TinyPress is, and why it matters

TinyPress is the chosen MKTd03 toy/reference dApp.

It is deliberately:
- single-canister,
- Nuance-inspired,
- shared-publishing shaped,
- structurally realistic enough to exercise Tree-mode issues,
- but still simpler than a real production target.

### 6.1 TinyPress hard constraints
- zero awareness of MKTd03 tombstoning mechanics
- no deletion-aware API semantics
- no orphan-aware API semantics
- no protocol-local vocabulary leakage into the app surface
- ordinary application behavior only

### 6.2 TinyPress schema shape
TinyPress v1 uses one canister and three separate stable maps:
- profiles
- posts
- comments

Comments are separate records, not embedded vectors inside posts.

### 6.3 Why TinyPress is useful
It gives MKTd03 a test target with:
- real subject-linked records,
- copied residue (`creator_handle`),
- authored posts/comments,
- reverse indexes required for structured-residue coverage,
- meaningful deletion-boundary questions.

### 6.4 What TinyPress is not
- not mainnet
- not the product
- not the protocol
- not a replacement for Nuance or another real target
- not authority for protocol semantics

## 7. Product / target intuition behind MKTd03

MKTd03 is aimed at ICP applications with shared mutable record topologies and meaningful deletion/compliance pressure.

Internally, Nuance has been the strongest conceptual target model:
- shared publishing structure,
- profiles + posts + comments + copied attribution,
- structured residue problems,
- realistic Tree-mode topology.

But current buyer/target thinking is still broader than one app.

Commercially plausible targets include on-chain publishing / social / marketplace / community apps where:
- subject-linked records are spread across shared structures,
- deletion pressure is real,
- auditability matters,
- proving defined deletion scope is valuable.

That product framing is useful context, but architecture and protocol design remain dApp-agnostic.

## 8. Current architecture direction

The settled direction is:

### 8.1 MKTd03 is dApp-agnostic
Do not let TinyPress-specific structures become protocol assumptions unless they are explicitly generalized and adopted.

### 8.2 Library / adapter split
This is the key core boundary.

The generic library should own:
- generic Tree-mode state-transition logic
- generic proof / receipt logic
- generic invariants
- generic lifecycle / diagnostics / version surface
- generic error semantics

The adapter should own:
- subject/record discovery
- app-specific traversal
- app-specific mutation execution
- app-specific scope enumeration
- app-specific topology knowledge

### 8.3 Formal adapter contract exists
The adapter surface is no longer spreadsheet-era pseudo-trait language.
It is formalized around staged boundary operations.

### 8.4 Orchestration is intentionally bounded
Current positive orchestration stops at a legitimate resolve-success boundary.
It does not yet proceed into machine-readable positive pre-state success, mutation success, receipt success, or verifier success.

This stop is deliberate and must not be “helpfully” filled in by assumption.

## 9. What has actually been built so far

### 9.1 Repo cleanup and governance baseline
Completed:
- MKTd03 is cleanly dApp-agnostic
- TinyPress has been split into its own repo as the live app surface
- repo-boundary governance is settled
- standards uplift and Codex setup work were completed earlier

### 9.2 Planning / design baseline
The following major planning/design assets exist and should be treated as part of the committed chain:
- ADR-00 through ADR-03
- build plan
- execution mode rule-set
- close-out plan
- authority map
- stale-spec inventory
- protocol refresh note
- Tree-mode invariants note
- diagnostics / compatibility / security / privacy design notes
- MKTd02 reuse-analysis notes
- vectors / fixtures / rules artifacts
- continuity files now aligned with current published state

### 9.3 Formal artifact baseline already present in MKTd03
These exist in repo form:
- `interfaces/mktd03_library.did`
- `interfaces/mktd03_library_interface_rules.md`
- `interfaces/mktd03_adapter_contract.did`
- `interfaces/mktd03_adapter_contract_rules.md`
- `docs/test-vectors/MKTd03_golden_vectors_v1.md`
- `docs/test-vectors/MKTd03_negative_cases_v1.md`
- `docs/test-vectors/fixtures/manifest.md`
- `docs/test-vectors/fixtures/index.json`

### 9.4 Runtime/reference scaffolding already exists
Reference/runtime files already exist:
- `src/library.rs`
- `src/adapter.rs`
- `src/verifier.rs`
- `src/orchestration.rs`

These are scaffolding/reference work, not evidence that the entire Tree-mode success path is live.

### 9.5 Current orchestration boundary
Current established behavior:
- boundary readiness modeled
- adapter status/capability gating modeled
- negative resolve propagation modeled
- one legitimate positive resolve-success path modeled

Current explicit stop:
- continuation past resolve success is deferred

### 9.6 Recent close-out sequence that must be understood
This was the just-completed close-out before Coder handover:

1. .did gate resolved:
   - `PreStateCaptured` was added as the frozen named positive pre-state result family
2. fixture manifest path alignment completed
3. minimal positive pre-state semantics pinned in adapter companion rules
4. golden-vectors authority references relocated / aligned
5. continuity updated to reflect a clean stop point

Committed close-out sequence:
- `0bf90b9` — interfaces: freeze named pre-state capture result
- `cfacc7f` — docs: align fixture manifest golden vector paths
- `4490f2b` — docs: pin minimal pre-state capture semantics
- `cdfc097` — docs: relocate golden vectors authority references
- `9bd9f1e` — docs: close out current phase continuity state

### 9.7 Current truth at the handover checkpoint
At `9bd9f1e`:
- current phase is closed
- next phase has not started
- no machine-readable positive pre-state fixture exists yet
- no continuation beyond resolve-success has begun
- repo is being handed over from a clean checkpoint, not mid-expansion

## 10. What is explicitly not built yet

Not yet built / not yet opened:
- machine-readable positive pre-state fixture for GV-05A
- positive continuation from resolve-success into pre-state success
- positive continuation into mutation success
- positive continuation into post-state success
- positive receipt-success path
- positive verifier-success path for Tree-mode end-to-end flow
- service-canister / composite-receipt architecture as active baseline
- host-specific enterprise integration flows
- full implementation slices under an opened next phase
- release/tagging posture for a production MKTd03 build

Do not infer any of the above from the presence of notes, stubs, or spreadsheet-era ideas.

## 11. What was stale and had to be corrected

One of the largest themes in MKTd03 so far is that the old spreadsheet tab/spec had drifted badly.

Main stale-spec problems that were identified:
- product architecture framed as more complete than reality
- trait-style adapter model instead of formal contract boundary
- stale public API assumptions
- receipt/CVDR structure overcommitted before ADR/interface settlement
- service-canister / orchestration overreach
- over-specific cryptographic formulas in the spreadsheet
- implementation detail treated as if it were settled protocol truth

The spreadsheet tab update has been planned and is pending execution. The guiding principle remains:

Old spreadsheet material is audit input first, not protocol authority.

## 12. The current authority chain

When in doubt, use this order.

1. Live repo ground truth
2. Committed close-out state ending at `9bd9f1e`
3. ADRs
4. Phase 4–5 spec / design / analysis notes
5. Formal interface files
6. Companion rules notes
7. Golden vectors / negative cases / fixture manifest
8. Continuity files
9. Older plans / spreadsheet / historical material

And more specifically by subject:
- library public interface -> `interfaces/mktd03_library.did`
- adapter boundary -> `interfaces/mktd03_adapter_contract.did`
- adapter semantics -> `interfaces/mktd03_adapter_contract_rules.md`
- library semantics -> `interfaces/mktd03_library_interface_rules.md`
- positive semantic baseline -> `docs/test-vectors/MKTd03_golden_vectors_v1.md`
- negative taxonomy -> `docs/test-vectors/MKTd03_negative_cases_v1.md`
- fixture rules -> `docs/test-vectors/fixtures/manifest.md`
- current handover stop point -> `RESTART_PACK.md`, `MILESTONE_LOG.md`

## 13. How the project has been built so far

MKTd03 has been run under a phase-gated execution model:

### 13.1 Major rule
Keep the gates, cut the churn.

Meaning:
- preserve phase gates
- use one strong draft per phase
- use one targeted adversarial review per phase
- do one close-out pass per phase
- avoid endless micro-loop drafting/review churn

### 13.2 Codex usage
Codex has been used successfully when tightly bounded:
- inspect
- review outside Codex
- bounded edit
- return evidence
- test/checkpoint
- move only after review

Codex must not:
- decide protocol semantics
- infer architecture
- decide CVDR meaning
- decide compatibility/versioning posture
- decide privacy/security claims

### 13.3 Review ladder
Typical high-value pattern:
1. primary drafter produces the phase packet
2. adversarial reviewer performs a targeted gate review
3. human diff / CLI verification checks repo truth
4. commit
5. continuity update at real boundaries

### 13.4 What “G” and “C” meant in the original workflow
Historically in this project:
- “G” referred to the ChatGPT instance acting as primary drafter / protocol lead / gatekeeper for close-out wording
- “C” referred to the Claude instance acting as adversarial reviewer in targeted review passes

Coder does not need to preserve those exact tool identities, but he should preserve the role separation:
- one model or agent drafts / proposes
- a different model or agent performs adversarial review
- the human operator verifies repo truth and signs off

If Coder uses only one AI for both drafting and review, he loses the adversarial gate that caught real issues during MKTd03. The exact tools can change; the two-role discipline should not.

### 13.5 Continuity discipline
At each important stop:
- update `RESTART_PACK.md`
- append to `MILESTONE_LOG.md` when appropriate
- commit them
- use them as restart truth for new sessions

## 14. What still needs to be built

At a high level, the next major categories of work are:

### 14.1 Open the next bounded phase
This has not been done yet. It must be a deliberate new phase decision.

### 14.2 Decide and implement the next positive continuation slice
Likely next question:
- whether to create the machine-readable GV-05A pre-state fixture and thereby legitimately extend the positive continuation path

But that remains deferred at current handover.

### 14.3 Continue formal design / implementation sequence without breaking authority discipline
Depending on the exact repo state and prior artifacts, this may include:
- extending fixture corpus
- extending orchestration with legitimate earned success steps
- implementing bounded code slices
- validating against vectors/fixtures
- later integrating against TinyPress
- later testing against more realistic targets

### 14.4 Eventually perform true integration testing
That includes:
- protocol-layer tests
- reference-target integration tests
- verifier checks
- negative/tamper tests
- version/compat tests

### 14.5 Eventually define release path
Only after actual implementation evidence exists.

## 15. How testing is intended to work

Testing strategy is layered.

### 15.1 Unit / local protocol testing
The protocol core must be testable without a running ICP canister where possible. This is part of the operational meaning of dApp-agnosticism.

### 15.2 Formal vectors / negative cases
Before meaningful implementation slices, behavior should be pinned with:
- human-readable golden vectors
- human-readable negative cases
- machine-readable fixtures

### 15.3 Boundary-specific testing
At each new integration boundary:
- test the boundary in isolation
- confirm compatibility before proceeding downstream
- do not introduce multiple unverified boundaries at once

### 15.4 Reference-target testing
TinyPress is intended to be the first integration/test target.
But:
- TinyPress is not mainnet
- TinyPress is intentionally naive
- TinyPress validates integration/topology reasoning, not production rollout claims

### 15.5 Future verifier testing
Verifier semantics matter a lot in this product family. Testing should eventually include:
- positive proof validation
- stale / replay / malformed / wrong-version / tampered cases
- backward-compat / dispatch behavior where relevant

### 15.6 MKTd02 comparison path
Use MKTd02 and DaffyDefs to compare:
- what a successful prior family build looked like
- how diagnostics/versioning/release evidence were handled
- where MKTd03 is fundamentally different because of Tree mode
- what can or cannot be reused

## 16. Specific constraints Coder’s AI must respect

### 16.1 Do not reopen settled cleanup/prep/governance work
Unless concrete contradictory evidence appears.

### 16.2 Do not let TinyPress shape protocol truth
TinyPress is reference target only.

### 16.3 Do not silently convert stale spreadsheet content into protocol truth
Always check committed repo artifacts.

### 16.4 Do not invent success paths
No positive continuation should exist just because it “seems like the next step.” It must be earned through deliberate spec / fixture / interface growth.

### 16.5 Do not mix service-canister future architecture into current baseline
That area is deferred.

### 16.6 Do not overstate legal/compliance claims
The protocol proves configured deletion / tombstoning transitions for declared in-scope records. It does not automatically prove every broader legal claim.

### 16.7 Do not collapse protocol truth, integration truth, and release truth
Keep layers separate.

## 17. Recommended reading order for Coder’s AI

Read in this order:

1. `RESTART_PACK.md`
2. `MILESTONE_LOG.md`
3. `docs/planning/MKTd03_authority_map_v1.md`

Then the core ADR chain:
4. `docs/adr/ADR-00-evidentiary-scope.md`
5. `docs/adr/ADR-01-library-vs-adapter-boundary.md`
6. `docs/adr/ADR-02-tree-structure-choice.md`
7. `docs/adr/ADR-03-tree-mode-cvdr-structure.md`

Then the Phase 4–5 spec / analysis / design notes:
8. `docs/spec/MKTd03_stale_spec_inventory_v1.md`
9. `docs/spec/MKTd03_protocol_refresh_v1.md`
10. `docs/analysis/MKTd02_module_taxonomy_for_MKTd03.md`
11. `docs/analysis/MKTd02_module_reuse_audit_v1.md`
12. `docs/analysis/MKTd02_generalise_now_backport_later_candidates_v1.md`
13. `docs/spec/MKTd03_tree_mode_invariants_v1.md`
14. `docs/spec/MKTd03_diagnostics_status_v1.md`
15. `docs/spec/MKTd03_compatibility_versioning_v1.md`
16. `docs/spec/MKTd03_security_design_v1.md`
17. `docs/spec/MKTd03_privacy_design_v1.md`

Then the formal interfaces and rules:
18. `interfaces/mktd03_library.did`
19. `interfaces/mktd03_library_interface_rules.md`
20. `interfaces/mktd03_adapter_contract.did`
21. `interfaces/mktd03_adapter_contract_rules.md`

Then the vector / fixture layer:
22. `docs/test-vectors/MKTd03_golden_vectors_v1.md`
23. `docs/test-vectors/MKTd03_negative_cases_v1.md`
24. `docs/test-vectors/fixtures/manifest.md`
25. `docs/test-vectors/fixtures/index.json`

Then the current runtime/reference implementation surface:
26. `src/orchestration.rs`
27. `src/library.rs`
28. `src/adapter.rs`
29. `src/verifier.rs`

Then, for comparison/context:
30. TinyPress ADR / interface spec
31. MKTd03 build plan
32. MKTd03 execution mode note
33. MKTd03 close-out plan
34. MKTd02 lessons learned / playbook / design principles

## 18. Recommended startup procedure for Coder’s AI

When starting a new working session on MKTd03:
1. read current `RESTART_PACK.md`
2. confirm repo head and working tree via CLI / repo truth
3. confirm whether the session is:
   - protocol design
   - fixture growth
   - bounded implementation
   - integration
   - docs/handover
4. restate current open phase and stop point before proposing work
5. identify authority files for the exact topic
6. avoid asking the user to re-explain settled context already captured in repo artifacts

## 19. Suggested first-question posture for Coder’s AI

The first technical decision Coder’s AI should probably frame is not:
- “How do we build the whole engine?”

It should be:
- “From the clean close-out checkpoint, what is the next bounded phase to open, and what exact artifact(s) authorize that next step?”

This keeps continuity intact and avoids skipping the same gates that MKTd02 taught us to respect.

## 20. Comparison with MKTd02 / DaffyDefs

### 20.1 Why the comparison matters
MKTd02 is the working earlier sibling. It shows:
- that the family can be built successfully
- what good evidence discipline looks like
- what release / verifier / integration pain points look like in practice

### 20.2 What carries over
- design doctrine
- continuity discipline
- bounded Codex usage
- review ladder
- vectors / negative cases mindset
- interface-first discipline
- diagnostics/versioning discipline
- evidence-gated releases

### 20.3 What does not carry over trivially
- proof shape
- state representation
- receipt/CVDR assumptions
- adapter obligations
- topology assumptions
- verifier assumptions
- deletion-boundary reasoning in shared-state apps

### 20.4 DaffyDefs vs TinyPress
- DaffyDefs was the MKTd02 toy/reference dApp.
- TinyPress is the MKTd03 toy/reference dApp.
- DaffyDefs supported a successful mainnet-adjacent MKTd02 flow.
- TinyPress is not mainnet yet and is still a reference target for upcoming integration phases.

This distinction should be made explicit to avoid overclaiming TinyPress maturity.

## 21. Current handover summary in one paragraph

MKTd03 is a dApp-agnostic Tree-mode deletion-proof/integration project, not yet a finished engine. The repo has been cleaned, formal interface/rules/vector scaffolding exists, runtime/reference scaffolding exists, and the current phase has been cleanly closed at checkpoint `9bd9f1e`. The latest work resolved the `PreStateCaptured` interface gate, pinned minimal positive pre-state semantics, aligned fixture/golden-vector authority references, and updated continuity to a clean stop point. A legitimate resolve-success orchestration path exists, but no machine-readable positive pre-state fixture or further positive continuation has been opened yet. TinyPress remains the zombie-delete-naive reference dApp only; it is not protocol authority and is not on mainnet. The next work should begin by explicitly deciding the next bounded phase rather than assuming a continuation.

## 22. File checklist for handover completeness

This restart pack assumes the following are available to Coder:

### In MKTd03
- `RESTART_PACK.md`
- `MILESTONE_LOG.md`
- `docs/planning/MKTd03_authority_map_v1.md`
- `docs/adr/ADR-00-evidentiary-scope.md`
- `docs/adr/ADR-01-library-vs-adapter-boundary.md`
- `docs/adr/ADR-02-tree-structure-choice.md`
- `docs/adr/ADR-03-tree-mode-cvdr-structure.md`
- `docs/spec/MKTd03_stale_spec_inventory_v1.md`
- `docs/spec/MKTd03_protocol_refresh_v1.md`
- `docs/analysis/MKTd02_module_taxonomy_for_MKTd03.md`
- `docs/analysis/MKTd02_module_reuse_audit_v1.md`
- `docs/analysis/MKTd02_generalise_now_backport_later_candidates_v1.md`
- `docs/spec/MKTd03_tree_mode_invariants_v1.md`
- `docs/spec/MKTd03_diagnostics_status_v1.md`
- `docs/spec/MKTd03_compatibility_versioning_v1.md`
- `docs/spec/MKTd03_security_design_v1.md`
- `docs/spec/MKTd03_privacy_design_v1.md`
- `interfaces/*`
- `docs/test-vectors/*`
- `src/*`

### In TinyPress
- TinyPress ADR + interface spec
- repo-local continuity docs
- live app code / frontend / deployment surface

### In related repos
- MKTd02 family comparison material
- verifier/schema comparison material
- doctrine / standards docs

### Spreadsheet note
- the MKTd03 spreadsheet tab update is planned but not yet executed
- treat the current spreadsheet tab as stale-spec audit input, not current authority

If any of these are missing, restore visibility before expecting a new AI to take over smoothly.

## 23. Final rule

Do not let Coder’s AI start from the words:
“I think the idea is…”

It should start from:
“The current committed baseline says…”

That is the difference between a smooth takeover and another expensive re-anchoring cycle.
