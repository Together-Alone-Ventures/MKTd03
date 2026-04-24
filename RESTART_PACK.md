DATE: 2026-04-24

CURRENT GOAL:
§5 reuse-audit close-out complete. Authority-map / AGENTS refresh packet complete. Specification-and-documentation phase remains closed. Next bounded session is C pre-implementation adversarial review of the Phase 6 artifact set as a post-Session-3 whole, per G's four-step sequence step 3.

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
§5 reuse-audit close-out landed at 936b873 (analysis promotion) and c60e79d (this RESTART_PACK update); see MILESTONE entry dated 2026-04-24 for full commit sequence.
MKTd03 main is at c60e79d.
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

SESSION LESSONS — 2026-04-23:

Method drift during multi-line web-editor edit: the Playbook rule (web editor only for single-line typo fixes; CLI for anything else) was restated by G at the start of the housekeeping pass but not consistently enforced in the instructions that followed. The web editor treated a two-sentence paragraph as two separate lines and only deleted one, requiring a CLI follow-up commit. The lesson is that "multi-line edit" means any edit that deletes more than one line, even if they render as one paragraph. Candidate for the Playbook-uplift queue; not yet promoted.

SESSION LESSONS — 2026-04-24:

Pure repository renames do not invalidate previously-pinned commit SHAs. The Phase-4 audit's MKTd02 SHA (`54f1e2dc…`) resolved transparently under the renamed `ICP-Delete-Leaf` repository and required no re-pinning. General rule: when a repo is renamed without history rewrite, SHAs remain canonical across the rename; subsequent artifacts may pin the same SHA under the new repo name without re-auditing. Candidate for the Playbook-uplift queue on repository-rename handling; not yet promoted.

File-transfer hygiene: Dropbox → WSL → git transfers via `/mnt/c/` produce `mode 100755` files on `git add` (NTFS executable-bit carries through). Required a `chmod 644` + `git commit --amend` cycle on Commit 1 of this session's sequence. Candidate for the Playbook-uplift queue: add a standing `chmod 644` step between Dropbox-copy and `git add` for any non-executable asset going through this transfer path; not yet promoted.

OPEN CANDIDATES FOR A FUTURE BOUNDED SESSION:

Housekeeping: `docs/spec/MKTd03_protocol_refresh_v1.md` `## Status` field still reads "Draft". Recommend confirming intended status (Approved vs. still Draft given the document is a refresh artifact, not an ADR) before editing.
Housekeeping: AGENTS Working discipline bullet ("…binding for interface-prep work…") carries pre-Phase-6 language; AGENTS Non-goals bullet 3 ("formal-interface gate") is now a historical marker superseded by bullet 4; authority map v2 row 15 ("Conceptual interface seed artifact") retains "pre-freeze draft" descriptor. All three were preserved verbatim per the refresh-packet scope's strict preserve-unchanged instruction; candidates for a light-touch adjacent-drift cleanup pass. G's preferred remediations captured in chat history: bullet 3 to be prefixed "(Historical — pre-Phase-6)"; conceptual-interface row wording to explicitly state "non-authoritative and must not diverge from frozen interface on overlapping types".
Housekeeping: v1 authority map disposition — G's preferred disposition is relocation to `docs/planning/history/`. Deferred from this packet as a follow-up commit.
Promotion: first-slice scope draft (`MKTd03_first_slice_scope_planning_DRAFT_v2.md`) awaits G review and promotion to `docs/planning/MKTd03_first_slice_scope_v1.md`. AGENTS and authority map v2 point to the promoted path.
Playbook uplift: SESSION LESSON entries from Session 3 (acronym-gap stop-and-escalate; standing-constraint propagation), 2026-04-23 (multi-line web-editor edit), and 2026-04-24 (repo-rename SHA carry-over; Dropbox→WSL mode-transfer) are candidates for promotion into TAV-Engineering-Standards doctrine via a separate uplift session with G secondary review.

NEXT BOUNDED SESSION — C PRE-IMPLEMENTATION ADVERSARIAL REVIEW:

Goal: C adversarial review of the Phase 6 artifact set as a post-Session-3 whole, per G's four-step sequence step 3 and build plan §12 Session 6 review trigger. Distinct from Session-1/2/3 reviews — this is a post-spec-tightening whole-set review before coding starts.
Scope for that session:
1. Review the Phase 6 artifact set as declared by `docs/planning/MKTd03_authority_map_v2.md` at HEAD.
2. Filter that review through the now-approved §5 audit artifacts at `docs/analysis/` — i.e., read the Phase 6 set through the reuse-audit and candidate-list lens to surface any fault-lines that materialise only when both views are held together.
3. Produce a C review report with explicit findings classified as approve-as-set, flag-for-revision, or flag-for-coding-start-blocker. No artifact edits in that session; findings only.
4. Role map: G drafts the review brief if needed; C executes the adversarial review; G provides secondary review of C's findings.

AFTER STEP 3 — REMAINING SEQUENCE PER G'S FOUR-STEP ORDERING:

4. G's explicit coding-start decision per build plan §11 Phase 6 exit-gate bullet eleven; first-slice scope document (`MKTd03_first_slice_scope_planning_DRAFT_v2.md`) promoted to `docs/planning/MKTd03_first_slice_scope_v1.md` and authoritative at that point.

No code is to be written in step 3. Coding-start is a separate decision gate after step 3.

OPERATING CONSTRAINTS:

No TinyPress leakage.
No MKTd02 implementation history treated as authority.
Rhetorical/comparative framing stays out of normative MKTd03 spec material.
Regulatory/legal terminology remains bounded to non-claim contexts.

SAFE RESTART PROMPT:
MKTd03 main is at c60e79d. The specification-tightening stream (Sessions 1, 2, 3), the authority-block housekeeping commit, the authority-map / AGENTS refresh packet, and the §5 reuse-audit close-out are all fully landed. The three §5 analysis artifacts are at `docs/analysis/` with anchors pinned (MKTd02 `54f1e2dc24dd0b79705a66894b2f25138e28a9ad`; `zombie-core-v0.3.1` `508f2f8bb88f4395293168c6ef25c92a67dee894`). The next bounded session is C pre-implementation adversarial review of the Phase 6 artifact set as a post-Session-3 whole, filtered through the approved §5 audit. Subsequent session: G's coding-start decision. No code is written in the pre-implementation review session.
