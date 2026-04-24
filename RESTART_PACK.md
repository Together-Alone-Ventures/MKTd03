DATE: 2026-04-24

CURRENT GOAL:
Authority-map / AGENTS refresh packet complete. Specification-and-documentation phase remains closed. Next bounded session closes out the §5 reuse-audit draft packet.

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
MKTd03 main is at 274f6015b44977ad7338996d620a75d3074ea517.
MKTd03 remains dApp-agnostic; TinyPress remains a reference target only.

AUTHORITY-BLOCK HOUSEKEEPING — 2026-04-23:

`docs/spec/MKTd03_protocol_refresh_v1.md` Authority block had stale "ADR-03 is currently an intermediate draft" language from before ADR-03 reached Approved status. Removed across three commits on 2026-04-23 (16f1124 via web editor removed first sentence; 44f849b via web editor re-added the second sentence by mistake; 4ea134a via CLI finished removing it). Net effect: both stale sentences are gone. Tiebreaker line "Where this document and an ADR differ, the ADR wins." retained. File's `## Status` line still reads "Draft" — not in scope for that housekeeping pass; remains in the open-candidates list below.

AUTHORITY-MAP / AGENTS REFRESH — 2026-04-24:

`docs/planning/MKTd03_authority_map_v2.md` created as the post-Session-3 successor to v1. v1 retained in place as historical. v2 absorbs: frozen formal public interface (both `.did` files reviewed at HEAD, `interface_version = 2.0.0`); frozen fixture corpus at `docs/test-vectors/fixtures/` with `#v2` anchors; post-Session-3 status on receipt/verifier, diagnostics, versioning, and security/privacy rows; authority-block housekeeping closure at `4ea134a`; new rows for `transition_derivation_version`, library interface version, `docs/analysis/` convention, RST evaluation lens, and the TAV-Engineering-Standards residual-trust note (labelled explicitly as non-normative cross-repo reference, not MKTd03 authority). `AGENTS.md` refreshed with post-Phase-6 "Current phase" framing, coding-start gate re-scoped against `docs/planning/MKTd03_first_slice_scope_v1.md` §6, and a new Non-goals bullet covering the full coding-start dependency chain.

SESSION LESSONS — 2026-04-23:

Method drift during multi-line web-editor edit: the Playbook rule (web editor only for single-line typo fixes; CLI for anything else) was restated by G at the start of the housekeeping pass but not consistently enforced in the instructions that followed. The web editor treated a two-sentence paragraph as two separate lines and only deleted one, requiring a CLI follow-up commit. The lesson is that "multi-line edit" means any edit that deletes more than one line, even if they render as one paragraph. Candidate for the Playbook-uplift queue; not yet promoted.

OPEN CANDIDATES FOR A FUTURE BOUNDED SESSION:

Housekeeping: `docs/spec/MKTd03_protocol_refresh_v1.md` `## Status` field still reads "Draft". Recommend confirming intended status (Approved vs. still Draft given the document is a refresh artifact, not an ADR) before editing.
Housekeeping: AGENTS Working discipline bullet ("…binding for interface-prep work…") carries pre-Phase-6 language; AGENTS Non-goals bullet 3 ("formal-interface gate") is now a historical marker superseded by bullet 4; authority map v2 row 15 ("Conceptual interface seed artifact") retains "pre-freeze draft" descriptor. All three were preserved verbatim per the refresh-packet scope's strict preserve-unchanged instruction; candidates for a light-touch adjacent-drift cleanup pass. G's preferred remediations captured in chat history: bullet 3 to be prefixed "(Historical — pre-Phase-6)"; conceptual-interface row wording to explicitly state "non-authoritative and must not diverge from frozen interface on overlapping types".
Housekeeping: v1 authority map disposition — G's preferred disposition is relocation to `docs/planning/history/`. Deferred from this packet as a follow-up commit.
Promotion: first-slice scope draft (`MKTd03_first_slice_scope_planning_DRAFT_v2.md`) awaits G review and promotion to `docs/planning/MKTd03_first_slice_scope_v1.md`. AGENTS and authority map v2 point to the promoted path.
Playbook uplift: SESSION LESSON entries from Session 3 (acronym-gap stop-and-escalate; standing-constraint propagation) plus 2026-04-23 (multi-line web-editor edit) are candidates for promotion into TAV-Engineering-Standards doctrine via a separate uplift session with G secondary review.

NEXT BOUNDED SESSION — §5 REUSE-AUDIT CLOSE-OUT:

Goal: G review close on the §5 reuse-audit draft packet, per G's four-step sequence step 2.
Scope for that session:
1. Review and approve (or revise) `MKTd02_module_taxonomy_for_MKTd03_DRAFT_v2.md`.
2. Review and approve (or revise) `MKTd02_module_reuse_audit_v1_DRAFT_v2.md`.
3. Review and approve (or revise) `MKTd02_generalise_now_backport_later_candidates_v1_DRAFT_v2.md`.
4. Pin both anchors: the MKTd02 audited-commit SHA and the `zombie-core` commit-at-tag.
5. Lift draft status on the three artifacts and land committed versions at their promoted paths.
Prerequisites before opening that session:
- MKTd02 repo URL and raw GitHub access to its key files (module structure, BLS-slip three-step flow, module-hash mechanism) confirmed — this was noted as a prerequisite in the previous RESTART_PACK and should be re-confirmed before session open.
- `zombie-core` commit-at-tag identified.

AFTER §5 CLOSE-OUT — REMAINING SEQUENCE PER G'S FOUR-STEP ORDERING:

3. C pre-implementation adversarial review of the Phase 6 artifact set as a post-Session-3 whole (per build plan §12 Session 6 review trigger; distinct from Session-1/2/3 reviews).
4. G's explicit coding-start decision per build plan §11 Phase 6 exit-gate bullet eleven; first-slice scope document promoted and authoritative at that point.

No code is to be written in steps 2 or 3. Coding-start is a separate decision gate after step 3.

OPERATING CONSTRAINTS:

No TinyPress leakage.
No MKTd02 implementation history treated as authority.
Rhetorical/comparative framing stays out of normative MKTd03 spec material.
Regulatory/legal terminology remains bounded to non-claim contexts.

SAFE RESTART PROMPT:
MKTd03 main is at 274f6015b44977ad7338996d620a75d3074ea517. The specification-tightening stream (Sessions 1, 2, 3), the authority-block housekeeping commit, and the authority-map / AGENTS refresh packet are all fully landed. The next bounded session closes out the §5 reuse-audit draft packet per G's four-step sequence (step 2): review and approve the taxonomy, reuse audit, and generalise-now-backport-later candidate list drafts; pin the MKTd02 and `zombie-core` anchors; promote the three drafts to committed paths. Subsequent sessions: C pre-implementation adversarial review, then G's coding-start decision. No code is written in the §5 close-out or pre-implementation review sessions.
