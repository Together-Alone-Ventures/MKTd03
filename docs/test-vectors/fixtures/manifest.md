# MKTd03 Fixture Manifest Rules v1

## Status
Frozen-draft fixture manifest note

## Purpose
This note defines the naming, grouping, metadata, and authority mapping rules for the first machine-readable MKTd03 fixture set.

This note is machine-readable-fixture oriented.
It does not generate the actual fixture files.

## Authority
This note is subordinate to:
- `docs/planning/MKTd03_authority_map_v1.md`
- `interfaces/mktd03_library.did`
- `interfaces/mktd03_library_interface_rules.md`
- `interfaces/mktd03_adapter_contract.did`
- `interfaces/mktd03_adapter_contract_rules.md`
- `docs/test-vectors/MKTd03_negative_cases_v1.md`
- `docs/test-vectors/MKTd03_golden_vectors_v1.md`

If this note conflicts with a frozen interface or interface-rules artifact, the frozen interface artifact wins.

## 1. Fixture-set structure

### 1.1 Positive vs negative split
Machine-readable fixtures must be split into:
- positive fixtures
- negative fixtures

A fixture must belong to exactly one of those sets.

### 1.2 Surface split
Fixtures must also be classified by authoritative surface:
- library
- adapter
- verifier

A fixture must map to exactly one authoritative surface.

### 1.3 Human-readable note linkage
Each machine-readable fixture must link back to at least one human-readable authority note:
- `docs/test-vectors/MKTd03_golden_vectors_v1.md` for positive semantic baselines
- `docs/test-vectors/MKTd03_negative_cases_v1.md` for negative families

## 2. Fixture naming convention

### 2.1 Canonical filename pattern
Fixture filenames must use this pattern:

`mktd03_<surface>_<polarity>_<family>_<case_id>_v<major>.json`

Where:
- `<surface>` is `library`, `adapter`, or `verifier`
- `<polarity>` is `positive` or `negative`
- `<family>` is the canonical family name
- `<case_id>` is a short stable identifier for the concrete case within the family
- `<major>` is the fixture-schema major version for that fixture file

### 2.2 Family naming rule
`<family>` must use the canonical family nouns from the frozen interfaces and negative-case taxonomy where possible.

Examples of valid family names:
- `receipt`
- `status`
- `version_support`
- `evidence_readiness`
- `wrong_tree_proof`
- `wrong_commitment_relationship`
- `malformed_certification_provenance`
- `missing_transition_derivation_version`
- `receipt_subject_scope_mismatch`
- `stale_precondition`
- `transition_mutation_rejected`
- `blocked_rebuild_required`

Examples of invalid family naming:
- TinyPress route names
- app-local business action names
- generic labels such as `error_case_1`
- note-level semantic groupings that are not explicit interface outcomes for that surface

### 2.3 Stable case-id rule
`<case_id>` must be short, stable, and semantically meaningful.
It must not encode environment-local path details, host implementation names, or app-specific entity names.

## 3. Version tagging convention

### 3.1 Fixture file version
The `v<major>` suffix in the filename identifies the machine-readable fixture schema version, not the protocol version under test.

### 3.2 In-fixture version fields
Each fixture must carry explicit metadata fields for:
- protocol version under test
- interface version under test
- fixture schema version
- fixture note / taxonomy version reference

### 3.3 No implicit version inference
Consumers must not infer protocol version from fixture filename alone.
Version identity must appear in fixture metadata.

## 4. Mapping rules

### 4.1 Library interface mapping
A library fixture must identify the target library surface explicitly.
Allowed target values are:
- `get_status`
- `get_evidence_readiness`
- `get_version_info`
- `check_version_support`
- `get_receipt`

### 4.2 Adapter-contract mapping
An adapter fixture must identify the target adapter surface explicitly.
Allowed target values are:
- `resolve_subject_scope`
- `capture_pre_state`
- `execute_transition_mutation`
- `capture_post_state`
- `get_adapter_status_facts`
- `get_adapter_capabilities`

### 4.3 Verifier-input mapping
A verifier fixture must identify verifier-input semantics explicitly.
Allowed target values for the first fixture pass are:
- `receipt_validation`

Verifier fixtures are limited here to receipt-artifact validation semantics already implied by the frozen receipt shape and interface rules.
This does not broaden into a full verifier architecture specification.

### 4.4 Human-readable authority mapping
Each fixture must carry references to:
- the primary human-readable source note
- the primary interface authority
- the primary rules authority when interpretation depends on companion rules

### 4.5 One primary outcome rule
Each fixture must declare one primary expected outcome family.
Supporting context may be recorded separately, but the fixture must not depend on multiple competing primary outcomes.

## 5. Minimal metadata required in every fixture

Every machine-readable fixture must carry at least:

- `fixture_id`
- `fixture_schema_version`
- `surface`
- `polarity`
- `family`
- `case_id`
- `title`
- `authority_refs`
- `target_method`
- `protocol_version`
- `interface_version`
- `rules_version_ref`
- `input_summary`
- `expected_outcome`
- `notes`

### 5.1 `authority_refs`
`authority_refs` must contain enough references to identify:
- the controlling interface file
- the controlling interface-rules file when relevant
- the controlling human-readable vector or negative-case note

### 5.2 `input_summary`
`input_summary` must describe the semantic shape under test without using application-shaped examples.

### 5.3 `expected_outcome`
`expected_outcome` must identify:
- expected result variant or status class
- named error / blocked code / compatibility class where applicable
- any required distinction from nearby failure families

## 6. Positive fixture rules

### 6.1 Positive library fixtures
Positive library fixtures must cover only processable, coherent public-library outcomes.
They must not silently rely on blocked-state, unsupported-version, or malformed receipt semantics.

### 6.2 Positive adapter fixtures
Positive adapter fixtures must cover only explicit, contract-returned adapter outcomes.
For the first adapter-positive pass, a `resolve_subject_scope` success fixture means only that one request/context shape yields one canonical `SubjectScope` object at the adapter boundary.
It must not be interpreted as evidence of real host lookup success, downstream capture success, mutation success, receipt generation, verifier success, or cryptographic proof validity.

If the frozen adapter method result does not carry `contract_version`, a positive adapter success fixture must not invent a contract-version field inside the method result shape.
Version identity remains in the fixture metadata unless the frozen method result itself carries version information.

### 6.3 No mixed-polarity fixtures
A positive fixture must not encode a primary failure expectation.
If a scenario contains both success context and failure context, it must be split into separate fixtures.

## 7. Negative fixture rules

### 7.1 Negative family authority
Every negative fixture family must map to a named family in `docs/test-vectors/MKTd03_negative_cases_v1.md`.

### 7.2 Surface-specific error discipline
Negative fixtures must use the error/status vocabulary of their own interface surface.
Library fixtures must not use adapter-only error codes.
Adapter fixtures must not use receipt-validation outcomes as their primary result family.
Verifier fixtures must not be modeled as `get_receipt` retrieval errors when the negative outcome is post-retrieval receipt invalidity.

### 7.3 Rebuild-required representation rule
Rebuild-required must be represented exactly as defined by the frozen interface surface being tested.
In particular, adapter status fixtures must not invent a separate rebuild-required flag outside blocked-state signaling.

## 8. Recommended directory layout

This note recommends, but does not yet create, the following machine-readable layout:

- `docs/test-vectors/fixtures/library/positive/`
- `docs/test-vectors/fixtures/adapter/positive/`
- `docs/test-vectors/fixtures/library/negative/`
- `docs/test-vectors/fixtures/adapter/negative/`
- `docs/test-vectors/fixtures/verifier/negative/`

Positive adapter fixtures must remain narrowly scoped to explicit contract-returned boundary examples and must not imply broader host-backed execution.

## 9. Non-goals

This note does not define:
- concrete JSON schema files
- concrete fixture payload bytes
- hash preimage encodings
- TinyPress-shaped fixture examples
- fixture runner tooling
