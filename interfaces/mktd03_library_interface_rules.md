<a id="v2"></a>
# MKTd03 Library Interface Rules v2

## Version note
v2 supersedes v1 for the public library surface by adding required
`transition_derivation_version : SemanticVersion` to
`Receipt.core_transition_evidence`. This version bump does not, by
itself, authorise new verifier semantics beyond the explicitly
approved formal-interface change set.

## Status
Frozen-draft companion rules

## Purpose
This file defines only the normative companion rules required because `.did` notation alone is insufficient to safely express MKTd03 public library semantics.

This file is subordinate to:
- `docs/planning/ADR-00-evidentiary-scope.md`
- `docs/planning/ADR-01-library-vs-adapter-boundary.md`
- `docs/planning/ADR-02-tree-structure-choice.md`
- `docs/planning/ADR-03-tree-mode-cvdr-structure.md`
- `docs/planning/tree-mode-invariants-note.md`
- `RESTART_PACK.md` settled closes for interface-prep work
- `interfaces/mktd03_library.did`

If this file conflicts with an approved ADR or the settled closes, the ADR / settled close wins.

## 1. Public artifact interpretation

### 1.1 Receipt vs tree proof
`Receipt` is the full public artifact.
`tree_proof` is a component inside `Receipt.core_transition_evidence`.
No consumer may interpret `tree_proof` alone as the whole public artifact.

### 1.2 Subject vs scope
`subject_reference` identifies the subject of the protocol-relevant transition in the public library surface.
`scope_reference` is distinct from `subject_reference` and must not be treated as an alias for the same concept.
If `scope_reference` is absent, that means the receipt carries no separate scope payload in this surface; it does not mean the subject implicitly equals some hidden scope.

### 1.3 Pre-state commitment vs post-state commitment
`pre_state_commitment` and `post_state_commitment` are distinct commitments bracketing the transition.
No consumer may infer one from the other or treat equality as a default success condition.

### 1.4 Transition material, transition-derivation version, and deletion-state material

`transition_material` carries the transition-specific evidence material.
`transition_derivation_version` declares the version identity of the
derivation scheme under which `transition_material` is to be interpreted.
`deletion_state_material` carries the deletion-state classification
needed to preserve the ADR-02 empty-vs-tombstoned distinction.

No consumer may treat any one of these three fields as a semantic
substitute for the others, and no consumer may infer
`transition_derivation_version` from `protocol_version`,
`receipt_version`, or `interface_version`.

## 2. Compatibility interpretation

### 2.1 Authoritative compatibility classes
The only authoritative compatibility classes in the frozen-draft public library surface are:
- `compatible`
- `conditionally_compatible`
- `unsupported`

No additional compatibility class may be inferred from older draft wording.

### 2.2 Class meaning
`compatible` means the surface and artifact version are processable under the current approved interpretation rules without additional compatibility conditions beyond the ordinary published rules.
`conditionally_compatible` means processing is possible only under explicitly documented compatibility conditions; consumers must not silently treat it as unconditional compatibility.
`unsupported` means the current surface must fail loud and must not attempt fallback interpretation.

### 2.3 Fail-loud rule
`unsupported` must not be collapsed into null, empty, best-effort, or nearest-known interpretation behaviour.

## 3. Blocked-state interpretation

### 3.1 Blocked is first-class
Blocked is a first-class condition distinct from `failed`, `completed`, `in-progress`, or generic error ideas that may exist in downstream implementations.
The library public surface therefore carries both:
- `is_blocked`
- `blocked_reason`

### 3.2 Blocked and lifecycle are not synonyms
`lifecycle_state` and `is_blocked` are distinct.
A consumer must not treat `failed` as equivalent to `blocked`, and must not treat `blocked` as merely another lifecycle label.

### 3.3 Blocked reason rules
If `is_blocked = true`, `blocked_reason` must be present.
If `is_blocked = false`, `blocked_reason` must be absent.
Consumers must not invent blocked reasons when the field is absent.

### 3.4 Human-readable description
`BlockedReason.description` is explanatory only.
`BlockedReason.code` is the programmatic identity.

## 4. Certification / provenance posture interpretation

### 4.1 Mandatory block rule
Every `Receipt` must contain a `certification_provenance` block.
The block is mandatory even where route-dependent payload material is absent.

### 4.2 Posture is authoritative
`CertificationProvenanceBlock.posture` is authoritative for how the block is to be interpreted.
Consumers must not infer posture from payload presence alone.

### 4.3 Route is explicit
`CertificationProvenanceBlock.route` is authoritative for the route shape used by this receipt.
Consumers must not infer route semantics from the presence or absence of only one payload field.

### 4.4 Payload absence is not silent
If `certification_material`, `provenance_material`, or `route_context_material` is absent, that absence must be interpreted through `posture` and `route`, not as empty bytes, implicit success, or “not needed unless fetched later” by default.

### 4.5 No live-fetch fallback by implication
Nothing in `route_dependent_payload` or `route_context_material` may be interpreted as permission to weaken the archival-first baseline or to make later live fetch mandatory by default.

## 5. Status-surface interpretation rules

### 5.1 Minimal authoritative surface
`StatusSurface` is the minimal authoritative status surface for this public library interface.
It must be interpreted as operational and compatibility/reporting state, not as proof of deletion.

### 5.2 Status vs compatibility
`lifecycle_state` and `compatibility` are distinct.
A consumer must not infer one from the other.

### 5.3 Status vs evidence readiness
`get_status` and `get_evidence_readiness` are distinct surfaces.
`StatusSurface` is not evidentiary proof.
`EvidenceReadiness` expresses only whether baseline evidence can currently be produced under protocol predicates.
Structured blocked diagnosis belongs to `get_status`, not to `EvidenceReadiness`.

### 5.4 Version reporting
`protocol_version`, `status_schema_version`, and `interface_version` are independently meaningful.
Consumers must not assume they are always identical.

### 5.5 Operation context
If `operation_context` is absent, that means no additional operation context is being asserted in this surface.
It must not be interpreted as a hidden default operation.

### 5.6 Build identity and module hash
Absence of `BuildIdentity.module_hash` does not mean build provenance is unavailable or unverified.
It means only that this status surface is not itself the provenance attestation path for that deployment or configuration.

## 6. Default / absence interpretation rules

### 6.1 No empty-bytes absence rule
Where a field is optional in the `.did`, absence means absence.
Consumers must not reinterpret empty byte vectors as equivalent to omitted optional fields.

### 6.2 No hidden defaulting
No omitted optional field in the public interface may be given unstated semantic defaults by consumers.
Any default interpretation must be explicitly documented in approved companion rules or ADR-governed artifacts.

### 6.3 Optional scope rule
Absence of `scope_reference` means only that no separate scope payload is carried in that field.
It does not collapse subject and scope into one noun.

## 7. Identifier and readiness disclosure constraints

### 7.1 Identifier disclosure belongs here, not in `.did`
The `.did` uses opaque byte fields for public identifier-bearing references.
Companion rules, not the `.did` itself, govern what identifier detail may be exposed.
Public library artifacts must not expose plaintext application identifiers or app-shaped payload data unless explicitly required by approved protocol meaning.

### 7.2 Readiness disclosure constraint
Status and readiness surfaces must expose enough information to satisfy the minimal authoritative status surface and blocked-state discipline, but must not expose unnecessary host internals.
In particular, host-specific storage layout, scheduler internals, and app-local orchestration details must not be inferred into the public status surface.

### 7.3 TinyPress containment
No rule in this file authorises TinyPress-specific naming, routes, payloads, schema names, fixtures, or examples in the public library interface.

## 8. Issuance atomicity and retrieval semantics

### 8.1 Atomic fixation rule
The evidence-bearing substance of the receipt must be fixed atomically with the deletion transition. A later retrieval, rendering, or export step is acceptable only as a pure projection over already-fixed issuance artefacts, not as a fresh semantic reconstruction from live state.

This rule applies to `subject_reference`, `scope_reference`,
`pre_state_commitment`, `post_state_commitment`, `transition_material`,
`transition_derivation_version`, `tree_proof`, and
`deletion_state_material` collectively; it is not satisfied by fixing
any subset.

### 8.2 Retrieval-operation rule
`get_receipt` is a retrieval operation. The receipt it returns must have been fixed at the deletion transition that produced it. The query may select, project, or re-encode the stored artefact for transport, but must not construct its evidence-bearing substance from live library state at query time.

## 9. Non-goals

This file does not define:
- byte-level encodings
- hash preimage layouts
- machine-readable fixtures
- adapter contract methods
- downstream application routes
- deployment workflow
