DATE: 2026-04-24

CURRENT GOAL:
C pre-implementation adversarial review of the Phase 6 artifact set complete; ruled Approve-with-flags at HEAD anchor 776aff7. No flag blocks the next step. Per G's four-step sequence, step 4 (G's explicit coding-start decision) is the next bounded session.

IMPORTANT SCOPE RULE:
This file is for MKTd03 protocol work only.
TinyPress implementation sessions must use the TinyPress repo's own RESTART_PACK.md, not this file.

CURRENT STATUS:

Prep closed.
Repo-boundary cleanup closed.
Standards uplift complete.
Formal-interface/conformance phase closed at cdfc097.
Session 1 complete at substantive checkpoint 24db28f.
Session 2 complete and pushed to main at 7b4db16.
Session 3 complete and pushed to main at 0a21274.
TAV-Engineering-Standards Session 3 companion commit landed at cd719a3.
Specification-tightening stream (Sessions 1 + 2 + 3) fully landed.
Authority-block housekeeping commit landed at 4ea134a.
Authority-map / AGENTS refresh packet landed at 274f6015b44977ad7338996d620a75d3074ea517 (see MILESTONE entry for full commit sequence).
§5 reuse-audit close-out landed at 936b873 (analysis promotion), 23b70c6 (RESTART_PACK update), and 776aff7 (MILESTONE_LOG update); see MILESTONE entry dated 2026-04-24 for full commit sequence.
C pre-implementation adversarial review of the Phase 6 artifact set complete; anchored to HEAD 776aff7; ruled Approve-with-flags (see MILESTONE entry dated 2026-04-24 for the full ruling).
RESTART_PACK update for pre-implementation review outcome landed at 68d6cb9af957aaae6a2266880ecfc6b00c1da406.
MKTd03 main is at 68d6cb9af957aaae6a2266880ecfc6b00c1da406.
MKTd03 remains dApp-agnostic; TinyPress remains a reference target only.

AUTHORITY-BLOCK HOUSEKEEPING — 2026-04-23:

`docs/spec/MKTd03_protocol_refresh_v1.md` Authority block had stale "ADR-03 is currently an intermediate draft" language from before ADR-03 reached Approved status. Removed across three commits on 2026-04-23 (16f1124 via web editor removed first sentence; 44f849b via web editor re-added the second sentence by mistake; 4ea134a via CLI finished removing it). Net effect: both stale sentences are gone. Tiebreaker line "Where this document and an ADR differ, the ADR wins." retained. File's `## Status` line still reads "Draft" — not in scope for that housekeeping pass; remains in the open-candidates list below.

AUTHORITY-MAP / AGENTS REFRESH — 2026-04-24:

`docs/planning/MKTd03_authority_map_v2.md` created as the post-Session-3 successor to v1. v1 retained in place as historical. v2 absorbs: frozen formal public interface (both `.did` files reviewed at HEAD, `interface_version = 2.0.0`); frozen fixture corpus at `docs/test-vectors/fixtures/` with `#v2` anchors; post-Session-3 status on receipt/verifier, diagnostics, versioning, and security/privacy rows; authority-block housekeeping closure at `4ea134a`; new rows for `transition_derivation_version`, library interface version, `docs/analysis/` convention, RST evaluation lens, and the TAV-Engineering-Standards residual-trust note (labelled explicitly as non-normative cross-repo reference, not MKTd03 authority). `AGENTS.md` refreshed with post-Phase-6 "Current phase" framing, coding-start gate re-scoped against `docs/planning/MKTd03_first_slice_scope_v1.md` §6, and a new Non-goals bullet covering the full coding-start dependency chain.

§5 REUSE-AUDIT CLOSE-OUT — 2026-04-24:

Three analysis artifacts promoted from v2 draft to approved status at `docs/analysis/`:
- `MKTd02_module_taxonomy_for_MKTd03.md`
- `MKTd02_module_reuse_audit_v1.md`
- `MKTd02_generalise_now_backport_later_candidates_v1.md`

Anchors pinned. MKTd02 primary anchor: repo `Together-Alone-Ventures/ICP-Delete-Leaf` (formerly `MKTd02`) at commit `54f1e2dc24dd0b79705a66894b2f25138e28a9ad` — same SHA as the Phase-4 reuse-generalisation audit v1 at `docs/spec/MKTd03_mktd02_reuse_generalisation_audit_v1.md`; the SHA resolves transparently under the renamed repo. Secondary anchor: `Together-Alone-Ventures/zombie-core` tag `zombie-core-v0.3.1` at commit `508f2f8bb88f4395293168c6ef25c92a67dee894`. G review 2026-04-24 approved all three drafts with one directed edit (taxonomy §3.4 explicit Category E "absent in MKTd02" row added). Eight open questions across the three artifacts resolved; rulings recorded per-artifact in their "Resolved questions" sections and cross-indexed in the MILESTONE entry.

PRE-IMPLEMENTATION ADVERSARIAL REVIEW — 2026-04-24:

C adversarial review of the Phase 6 artifact set as a post-Session-3 whole executed per G's four-step sequence step 3. Anchored to HEAD = 776aff7. Role-swap from §5 close-out carried into this session per Stef's ruling 2026-04-24: C drafted the review brief; G reviewed the brief (one gap identified and resolved: HEAD SHA pinning) and executed the adversarial review. Overall ruling: Approve-with-flags. No flag blocks step 4 from opening.

Stress-test result: the expensive failure pattern — CVDR-as-declaration rather than certified record of execution — is not structurally present in the Phase 6 set. Lazy receipt construction is blocked at the spec layer by the absence of receipt-producing paths in S7-1 and by A→B→C discipline preserved as later-slice input.

Four flags surfaced. None are net-new to the open-candidates queue; all fold into existing items:
- `AGENTS.md` + `RESTART_PACK.md` adjacent-drift flag (pre-Phase-6 "formal-interface gate" / interface-prep wording) — folds into the AGENTS adjacent-drift cleanup queue item; scope expanded to include RESTART_PACK.md.
- `docs/spec/MKTd03_protocol_refresh_v1.md` `## Status` field still reads "Draft" — existing queue item confirmed as active.
- Authority map v2 row 15 ("Conceptual interface seed artifact") retains "pre-freeze draft" descriptor — existing queue item confirmed as active.
- First-slice scope draft v2: update context anchor from old `3319c3f` at promotion time — attached to the promotion queue item as a promotion-time detail.

All other Phase 6 artifacts approved without flag: stale spec inventory; ADR-00 evidentiary scope; Tree-mode invariants note; ADR-02 tree structure; ADR-01 boundary; ADR-03 receipt/verifier semantics; diagnostics/status note; versioning/compatibility note; security/privacy note; published terminology policy; conceptual adapter contract; companion-rule layer; golden vectors / negative cases; frozen `.did` interfaces; machine-readable fixtures; `transition_derivation_version` row; library interface version row; `docs/analysis/` convention / RST lens / Standards note; coding-start readiness gate.

SESSION LESSONS — 2026-04-23:

Method drift during multi-line web-editor edit: the Playbook rule (web editor only for single-line typo fixes; CLI for anything else) was restated by G at the start of the housekeeping pass but not consistently enforced in the instructions that followed. The web editor treated a two-sentence paragraph as two separate lines and only deleted one, requiring a CLI follow-up commit. The lesson is that "multi-line edit" means any edit that deletes more than one line, even if they render as one paragraph. Candidate for the Playbook-uplift queue; not yet promoted.

SESSION LESSONS — 2026-04-24:

Pure repository renames do not invalidate previously-pinned commit SHAs. The Phase-4 audit's MKTd02 SHA (`54f1e2dc…`) resolved transparently under the renamed `ICP-Delete-Leaf` repository and required no re-pinning. General rule: when a repo is renamed without history rewrite, SHAs remain canonical across the rename; subsequent artifacts may pin the same SHA under the new repo name without re-auditing. Candidate for the Playbook-uplift queue on repository-rename handling; not yet promoted.

File-transfer hygiene: Dropbox → WSL → git transfers via `/mnt/c/` produce `mode 100755` files on `git add` (NTFS executable-bit carries through). Required a `chmod 644` + `git commit --amend` cycle on Commit 1 of the §5 close-out session sequence. Candidate for the Playbook-uplift queue: add a standing `chmod 644` step between Dropbox-copy and `git add` for any non-executable asset going through this transfer path; not yet promoted.

HEAD-anchor ambiguity in review briefs: the pre-implementation review brief drafted by C under role-swap anchored the artifact set against "authority map v2 at HEAD" without pinning the HEAD SHA. G's brief-review step flagged this as the only gap in the brief; the SHA (`776aff7`) was resolved via `git log -1` and pinned before the review executed. Lesson: any review brief anchored to a live repo state must pin the commit SHA explicitly at draft time — "HEAD" alone is ambiguous because HEAD is a moving reference. Candidate for the Playbook-uplift queue on review-brief drafting patterns; not yet promoted.

OPEN CANDIDATES FOR A FUTURE BOUNDED SESSION:

Housekeeping: `docs/spec/MKTd03_protocol_refresh_v1.md` `## Status` field still reads "Draft". Recommend confirming intended status (Approved vs. still Draft given the document is a refresh artifact, not an ADR) before editing. Confirmed as an active flag by pre-implementation review 2026-04-24.
Housekeeping: AGENTS Working discipline bullet ("…binding for interface-prep work…") carries pre-Phase-6 language; AGENTS Non-goals bullet 3 ("formal-interface gate") is now a historical marker superseded by bullet 4; authority map v2 row 15 ("Conceptual interface seed artifact") retains "pre-freeze draft" descriptor; RESTART_PACK.md carries pre-Phase-6 descriptor wording in adjacent surfaces. All preserved verbatim per the refresh-packet scope's strict preserve-unchanged instruction; confirmed as active flags by pre-implementation review 2026-04-24. G's preferred remediations captured in chat history: Non-goals bullet 3 to be prefixed "(Historical — pre-Phase-6)"; conceptual-interface row wording to explicitly state "non-authoritative and must not diverge from frozen interface on overlapping types". Candidates for a light-touch adjacent-drift cleanup pass.
Housekeeping: v1 authority map disposition — G's preferred disposition is relocation to `docs/planning/history/`. Deferred from the authority-map refresh packet as a follow-up commit.
Promotion: first-slice scope draft (`MKTd03_first_slice_scope_planning_DRAFT_v2.md`) awaits G review and promotion to `docs/planning/MKTd03_first_slice_scope_v1.md`. AGENTS and authority map v2 point to the promoted path. Promotion-time detail from pre-implementation review 2026-04-24: update the draft's context anchor from old `3319c3f` to current HEAD before or at promotion.
Housekeeping: Phase-4 reuse-generalisation audit v1 `MKTd02 repo URL` field points to the pre-rename URL (now a redirect). SHA remains canonical under the rename; URL update is a hygiene item only.
Playbook uplift: SESSION LESSON entries from Session 3 (acronym-gap stop-and-escalate; standing-constraint propagation), 2026-04-23 (multi-line web-editor edit), and 2026-04-24 (repo-rename SHA carry-over; Dropbox→WSL mode-transfer; HEAD-anchor ambiguity in review briefs) are candidates for promotion into TAV-Engineering-Standards doctrine via a separate uplift session with G secondary review.

NEXT BOUNDED SESSION — G CODING-START DECISION:

Goal: G's explicit coding-start decision per build plan §11 Phase 6 exit-gate bullet eleven, executing step 4 of G's four-step sequence. This is the final bounded session before actual engine build work begins.

Scope for that session:
1. G reviews the Phase 6 artifact set in its approved state (Approve-with-flags ruling from pre-implementation review 2026-04-24, HEAD anchor 776aff7) and decides whether to open coding.
2. If coding is opened: first-slice scope document (`MKTd03_first_slice_scope_planning_DRAFT_v2.md`) is promoted to `docs/planning/MKTd03_first_slice_scope_v1.md` and becomes authoritative at that point. Context anchor in the draft (currently `3319c3f`) updated to current HEAD at promotion.
3. Any open flag from the pre-implementation review that G deems blocking at the decision point must be resolved before coding opens. Current ruling: no flag blocks; G may revisit at decision time.
4. Role map: G decides; C is advisory on the coding-start decision itself; any bounded artifact edit associated with the decision (first-slice promotion, AGENTS/authority-map post-coding-start updates) follows standard role map at that point.

No code is written in the coding-start decision session itself. Actual code writing begins after step 4 closes.

OPERATING CONSTRAINTS:

No TinyPress leakage.
No MKTd02 implementation history treated as authority.
Rhetorical/comparative framing stays out of normative MKTd03 spec material.
Regulatory/legal terminology remains bounded to non-claim contexts.

SAFE RESTART PROMPT:
MKTd03 main is at 68d6cb9af957aaae6a2266880ecfc6b00c1da406. The specification-tightening stream (Sessions 1, 2, 3), the authority-block housekeeping commit, the authority-map / AGENTS refresh packet, the §5 reuse-audit close-out, and the C pre-implementation adversarial review are all fully landed. The pre-implementation review ruled the Phase 6 artifact set Approve-with-flags against HEAD anchor 776aff7; no flag blocks the next step. Four flags surfaced, all folding into existing open-candidates queue items. The three §5 analysis artifacts remain at `docs/analysis/` with anchors pinned (MKTd02 `54f1e2dc24dd0b79705a66894b2f25138e28a9ad`; `zombie-core-v0.3.1` `508f2f8bb88f4395293168c6ef25c92a67dee894`). The next bounded session is G's explicit coding-start decision per build plan §11 Phase 6 exit-gate. No code is written in that session.
