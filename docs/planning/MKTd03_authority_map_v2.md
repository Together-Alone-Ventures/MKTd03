# MKTd03 Authority Map v2

**Status:** Active authority map (post-Session-3).
**Supersedes:** `docs/planning/MKTd03_authority_map_v1.md` (pre-Phase-6 baseline).
**Refreshed:** 2026-04-24.
**Drafted by:** C (Claude), under operator-directed role-swap. G secondary review complete.

## Purpose

This map states which current repo artifact is authoritative for each baseline subject area in the post-Session-3 Phase-7-planning phase.

If two artifacts overlap, the artifact named here governs unless G explicitly re-gates the subject.

## Scope note

v1 described a pre-Phase-6 formal-interface-pre-freeze world. v2 describes the post-specification-tightening world: frozen `.did` interfaces exist, companion-rules are at v2, the fixture corpus is in place, the `transition_derivation_version` field has landed, and Sessions 1–3 of the specification-tightening stream plus authority-block housekeeping are all complete.

## Authority table

| Subject area | Authoritative artifact | Current status |
|---|---|---|
| Repo boundary and TinyPress containment | `AGENTS.md` + `RESTART_PACK.md` | settled |
| Stale spreadsheet treatment | `docs/spec/MKTd03_stale_spec_inventory_v1.md` | settled |
| Evidentiary scope and non-claims | `docs/planning/ADR-00-evidentiary-scope.md` | settled with draft carrier |
| Tree-mode operational invariants | `docs/planning/tree-mode-invariants-note.md` | settled with draft carrier |
| Tree structure and structural terminology baseline | `docs/planning/ADR-02-tree-structure-choice.md` | settled with draft carrier |
| Library vs adapter boundary | `docs/planning/ADR-01-library-vs-adapter-boundary.md` | settled |
| Receipt / verifier semantics | `docs/planning/ADR-03-tree-mode-cvdr-structure.md` + settled two-layer evidence close + Sessions 1–3 spec-tightening | settled (post-Session-3) |
| Refreshed overall protocol narrative | `docs/spec/MKTd03_protocol_refresh_v1.md` | settled (authority-block housekeeping complete at `4ea134a`) |
| Diagnostics/status policy | `docs/spec/MKTd03_diagnostics_status_note_v1.md` + settled status-surface close + companion-rules v2 §3 | settled (post-Session-3) |
| Versioning and compatibility policy | `docs/spec/MKTd03_versioning_compatibility_note_v1.md` + settled compatibility close + companion-rules v2 §2 + Session 2 `interface_version` bump | settled (post-Session-3) |
| Security/privacy policy | `docs/spec/MKTd03_security_privacy_note_v1.md` + companion-rules v2 + Session 3 Change 3.3 strengthening of §6 | settled (post-Session-3) |
| Published terminology policy | `docs/planning/ADR-02-tree-structure-choice.md` | settled |
| Conceptual adapter contract | `docs/spec/MKTd03_adapter_contract_concept_v1.md` | reference-only; superseded by frozen adapter contract at `interfaces/mktd03_adapter_contract.did` |
| Companion-rule layer | `interfaces/mktd03_library_interface_rules.md` (v2) + `interfaces/mktd03_adapter_contract_rules.md` | reviewed; v2 governs |
| Conceptual interface seed artifact | `interfaces/mktd03_tree_mode_conceptual_interface_v1.did` | pre-freeze draft; not frozen authority |
| Human-readable semantic vectors | `docs/test-vectors/MKTd03_golden_vectors_v1.md` + `docs/test-vectors/MKTd03_negative_cases_v1.md` | reviewed |
| Frozen formal public interface | `interfaces/mktd03_library.did` (`interface_version = 2.0.0`) + `interfaces/mktd03_adapter_contract.did` (`PreStateCaptured` frozen at `0bf90b9`) | frozen-draft; reviewed; at HEAD |
| Frozen machine-readable fixtures | `docs/test-vectors/fixtures/` (≥14 positive fixtures; negative families including `missing_transition_derivation_version`; `rules_version_ref` anchors at `#v2`); manifest at `docs/test-vectors/fixtures/manifest.md` | reviewed; rules-version anchor at v2 |
| `transition_derivation_version` field | `docs/planning/ADR-03-tree-mode-cvdr-structure.md` (semantics) + `interfaces/mktd03_library.did` (type declaration on `CoreTransitionEvidence`) | Session 2 change; applies only to `core_transition_evidence` |
| Library interface version | `interfaces/mktd03_library.did` (constant, currently `2.0.0`) + `interfaces/mktd03_library_interface_rules.md` v2 (rule set governing this interface version) | Session 2 bump (1.0.0 → 2.0.0); rules v2 is canonical |
| Non-normative analytical artefacts convention | `docs/analysis/` directory (established Session 3 Change 3.4) | established; non-normative repository of analytical notes |
| RST evaluation lens | `docs/analysis/MKTd03_rst_evaluation_lens_v1.md` | non-normative analytical note; not MKTd03 authority |
| TAV-Engineering-Standards residual-trust note | `Together-Alone-Ventures/TAV-Engineering-Standards` repo, Session 3 companion commit `cd719a3` (external) | non-normative cross-repo reference; not MKTd03 authority |
| Coding-start readiness gate | `docs/planning/MKTd03_first_slice_scope_v1.md` §6 | binding; must not be re-authored outside this artifact |

## Operating constraints

- TinyPress must not define protocol truth, interface names, payloads, routes, schemas, fixtures, or examples here.
- MKTd02 implementation history remains bounded audit/reuse input only.
- Companion rules may constrain interface interpretation but must not silently revise ADR meaning.
- The conceptual `.did` may seed frozen interface drafting only after conformance cleanup against the settled closes; as of v2, the frozen library and adapter `.did` files exist and carry authority.
- This map may be updated only as part of the current bounded phase work and must not be used to override reviewed ADR content without explicit G approval.

## Changes since v1

- Added: `transition_derivation_version` field row (Session 2).
- Added: Library interface version row (Session 2).
- Added: Non-normative analytical artefacts convention row (Session 3).
- Added: RST evaluation lens row (Session 3 Change 3.4).
- Added: TAV-Engineering-Standards residual-trust note row (Session 3 Change 3.5) — explicitly labelled non-normative cross-repo reference.
- Added: Coding-start readiness gate row — points to first-slice scope §6 as sole authority; added per G's secondary review on this refresh packet to prevent future re-authoring of the gate outside that artifact.
- Updated: Frozen formal public interface row — from "not yet created" to the two frozen `.did` files at HEAD.
- Updated: Frozen machine-readable fixtures row — from "not started" to corpus at `docs/test-vectors/fixtures/` with `#v2` anchors.
- Updated: Receipt/verifier semantics, Diagnostics/status policy, Versioning and compatibility policy rows — "pre-freeze cleanup needed" struck; now "settled (post-Session-3)".
- Updated: Security/privacy policy row — companion-rules v2 and Session 3 Change 3.3 strengthening cited; status elevated to "settled (post-Session-3)".
- Updated: Refreshed overall protocol narrative row — "settled with draft carrier" replaced with "settled", citing authority-block housekeeping commit `4ea134a`.
- Updated: Conceptual adapter contract row — status shifted to "reference-only; superseded by frozen adapter contract".
- Updated: Companion-rule layer row — cites library rules v2 and adapter rules; status "reviewed; v2 governs".
- Updated: Human-readable semantic vectors row — cites both golden vectors and negative cases files; status "reviewed".
- Preserved unchanged: rows 1–6, 12, 15 (Conceptual interface seed artifact).
