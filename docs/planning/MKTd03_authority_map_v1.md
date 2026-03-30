# MKTd03 Authority Map v1

## Purpose

This map states which current repo artifact is authoritative for each baseline subject area during the formal-interface pre-freeze phase.

If two artifacts overlap, the artifact named here governs unless G explicitly re-gates the subject.

| Subject area | Authoritative artifact | Current status |
|---|---|---|
| Repo boundary and TinyPress containment | `AGENTS.md` + `RESTART_PACK.md` | settled |
| Stale spreadsheet treatment | `docs/spec/MKTd03_stale_spec_inventory_v1.md` | settled |
| Evidentiary scope and non-claims | `docs/planning/ADR-00-evidentiary-scope.md` | settled with draft carrier |
| Tree-mode operational invariants | `docs/planning/tree-mode-invariants-note.md` | settled with draft carrier |
| Tree structure and structural terminology baseline | `docs/planning/ADR-02-tree-structure-choice.md` | settled with draft carrier |
| Library vs adapter boundary | `docs/planning/ADR-01-library-vs-adapter-boundary.md` | settled |
| Receipt / verifier semantics | `docs/planning/ADR-03-tree-mode-cvdr-structure.md` + settled two-layer evidence close | settled with pre-freeze cleanup needed |
| Refreshed overall protocol narrative | `docs/spec/MKTd03_protocol_refresh_v1.md` | settled with draft carrier |
| Diagnostics/status policy | `docs/spec/MKTd03_diagnostics_status_note_v1.md` + settled status-surface close | settled with pre-freeze cleanup needed |
| Versioning and compatibility policy | `docs/spec/MKTd03_versioning_compatibility_note_v1.md` + settled compatibility close | settled with pre-freeze cleanup needed |
| Security/privacy policy | `docs/spec/MKTd03_security_privacy_note_v1.md` | settled with companion-rule carry-through needed |
| Published terminology policy | `docs/planning/ADR-02-tree-structure-choice.md` | settled |
| Conceptual adapter contract | `docs/spec/MKTd03_adapter_contract_concept_v1.md` | pre-freeze draft |
| Companion-rule layer | `docs/spec/MKTd03_companion_rules_v1.md` | pre-freeze draft |
| Conceptual interface seed artifact | `interfaces/mktd03_tree_mode_conceptual_interface_v1.did` | pre-freeze draft; not frozen authority |
| Human-readable semantic vectors | `docs/spec/MKTd03_golden_vectors_v1.md` | pre-freeze draft |
| Frozen formal public interface | not yet created | needs cleanup before frozen interface |
| Frozen machine-readable fixtures | not yet created | not started |

## Operating constraints

- TinyPress must not define protocol truth, interface names, payloads, routes, schemas, fixtures, or examples here.
- MKTd02 implementation history remains bounded audit/reuse input only.
- Companion rules may constrain interface interpretation but must not silently revise ADR meaning.
- The conceptual `.did` may seed frozen interface drafting only after conformance cleanup against the settled closes.
- This map may be updated only as part of the current bounded phase work and must not be used to override reviewed ADR content without explicit G approval.
