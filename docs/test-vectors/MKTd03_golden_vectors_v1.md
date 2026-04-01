# MKTd03 Golden Vectors v1

**Date:** 2026-03-26  
**Status:** Approved semantic baseline  
**Scope:** Human-readable semantic golden vectors for the first MKTd03 formal-interface phase  
**Authority note:** This note is subordinate to the reviewed ADR set, formal interface files, and companion interface rules. It is intentionally **semantic/conceptual first**, not byte-final.

---

## 0. Purpose

This note defines the first human-readable golden vectors for MKTd03. Its job is to lock down the **meaning** of the protocol-facing outcomes before implementation begins, so later code and machine-readable fixtures have a stable target.

At this phase, the vectors are **not** final byte fixtures. They are normative at the level of:

- object role,
- state classification,
- named outcome,
- failure-vs-success distinction,
- version-handling behaviour,
- tombstoned-vs-empty semantics,
- inline certification / provenance baseline expectations.

This document exists because TAV design doctrine requires golden vectors to be written before implementation for protocol or cryptographic work, and because MKTd03 Phase 6 explicitly calls for a human-readable golden-vectors artifact before code begins.

---

## 1. Non-goals for v1

This note does **not** yet freeze:

- final byte encodings,
- final hash preimages,
- final domain tags,
- final witness serialization,
- final receipt-ID derivation bytes,
- final machine-readable fixture filenames.

Those belong in the later fixture set and companion interface rules once the public types, encoding rules, and proof-object fields are fully locked.

---

## 2. Vector-writing rules for this phase

### 2.1 Generic only

Vectors in this note must remain dApp-agnostic. They must not use TinyPress routes, app-local schema names, canister names, or payload shapes.

### 2.2 Fail-loud posture

Every vector must resolve to either:

- a named success class, or
- a named failure class.

No vector may rely on silent defaulting, silent downgrade, or ambiguous “best effort” interpretation.

### 2.3 Semantic-first notation

Until byte-final rules are approved, vectors in this document use the following notation:

- `VERSION_X` = protocol version token
- `SUBJECT_SCOPE_Y` = declared deletion/tombstoning scope
- `ROOT_PRE_A` / `ROOT_POST_B` = conceptual pre- and post-state roots
- `PATH_TOMBSTONED` = witness showing the target position contains a tombstone marker
- `PATH_EMPTY` = witness showing the target position is empty / absent without tombstone occupancy
- `CERT_INLINE_OK` = inline certification material present and internally well-formed for the current ruleset
- `PROVENANCE_INLINE_OK` = inline provenance material present and internally well-formed for the current ruleset

These tokens are placeholders for later concrete fixtures. They are not implementation identifiers.

---

## 3. Status model used by these vectors

For the purposes of this note, status outcomes are grouped into two top-level classes.

### 3.1 Ready

The library has enough approved and self-consistent input to construct, return, or verify the relevant protocol artifact for the requested operation.

### 3.2 Blocked

The library must refuse to proceed because some required precondition is not satisfied, some required artifact is missing or inconsistent, or the input version / retrieval / proof state is not processable under the approved rules.

Important: “blocked” is an operational class, not a single wire error. It must later refine into explicit named errors in the formal interface and fixtures.

---

## 4. Named outcome families covered in v1

This first golden-vectors note covers the minimum families required for the current formal-interface phase:

1. ready vs blocked status,
2. supported vs unsupported version handling,
3. CVDR retrieval success vs named error outcomes,
4. tombstoned-position vs empty-position distinction,
5. inline certification / provenance baseline examples.

These families align with the currently approved direction that MKTd03 must expose explicit failure semantics, explicit version behaviour, explicit diagnostics/status behaviour, and pre-code vectors.

---

## 5. Golden vectors

## Vector GV-01 — Ready status, minimal positive baseline

**Purpose:** Prove the semantic minimum for a processable, internally coherent request.

**Inputs (conceptual):**
- protocol object carries `VERSION_X`, where `VERSION_X` is supported,
- required proof-bearing fields are present,
- required status prerequisites are satisfied,
- retrieval of the target artifact has succeeded,
- inline certification/provenance policy requirements for this operation are satisfied.

**Expected outcome:**
- top-level status class = `Ready`,
- no fallback or downgrade path is taken,
- downstream steps may proceed using the returned artifact.

**What this locks down:**
- readiness is a positive, affirmative state,
- readiness is not inferred from absence of error alone,
- readiness requires both structural sufficiency and version/processability.

---

## Vector GV-02 — Blocked status due to missing required material

**Purpose:** Separate “cannot proceed” from version failure and from retrieval failure.

**Inputs (conceptual):**
- protocol object carries supported `VERSION_X`,
- at least one required input artifact is missing,
- no alternate approved route exists for this operation.

**Expected outcome:**
- top-level status class = `Blocked`,
- named reason family = `MissingRequiredMaterial` (placeholder semantic label),
- operation must not silently continue with empty defaults or inferred substitutes.

**What this locks down:**
- blocked is not only a version problem,
- missing required inputs must fail explicitly,
- silent inference is disallowed.

**Phase-note:** Later interface work must distinguish missing caller-supplied material from missing library-internal state. Those are different failure origins with different recovery implications and must not collapse into one final reason family.

---

## Vector GV-03 — Supported version accepted

**Purpose:** Establish the positive version-dispatch baseline.

**Inputs (conceptual):**
- incoming object declares `VERSION_X`,
- `VERSION_X` is within the explicitly supported set for this implementation slice.

**Expected outcome:**
- version dispatch accepts the object,
- object continues into ordinary processing,
- no warning-level downgrade is used as a substitute for explicit support.

**What this locks down:**
- support is explicit,
- version handling is determined from the declared version field,
- support is not guessed from shape similarity.

---

## Vector GV-04 — Unsupported version rejected

**Purpose:** Lock the fail-loud rule for version mismatch.

**Inputs (conceptual):**
- incoming object declares `VERSION_Z`,
- `VERSION_Z` is not in the explicitly supported set.

**Expected outcome:**
- operation returns `Blocked`,
- named reason family = `UnsupportedVersion`,
- object is not partially processed,
- object is not silently coerced into another version.

**What this locks down:**
- unsupported-version behaviour must be explicit,
- “close enough” parsing is forbidden,
- version mismatch is a first-class protocol failure.

---

## Vector GV-05 — CVDR retrieval success

**Purpose:** Define the success baseline for fetching a Tree-mode CVDR artifact.

**Inputs (conceptual):**
- target identifier is well-formed,
- requested version is supported,
- referenced artifact exists,
- underlying retrieval route completes normally.

**Expected outcome:**
- named result family = `CvdrRetrieved`,
- returned object is the requested artifact, not a summary placeholder,
- result remains distinct from later proof-validation success.

**What this locks down:**
- retrieval success is not the same thing as proof validity,
- the library must distinguish fetch/lookup success from verification success,
- retrieval is a named positive outcome.

---

## Vector GV-05A — Pre-state capture success for a resolved boundary subject/scope

**Purpose:** Define the minimum positive semantic baseline for a successful `capture_pre_state` result at the adapter boundary.

**Inputs (conceptual):**
- a canonical `SubjectScope` has already been resolved at the adapter boundary,
- the adapter boundary is not blocked for this operation,
- the adapter supports pre-state capture for the supplied `SubjectScope`,
- the adapter can return one pre-state boundary object for that same subject/scope pair.

**Expected outcome:**
- the adapter returns a successful `capture_pre_state` result,
- the returned object is one `StateCapture` boundary object, not a receipt, commitment, proof, or mutation result,
- `StateCapture.subject_scope` matches the supplied `SubjectScope`,
- `StateCapture.state_material` is the adapter-asserted pre-state material for that same subject/scope pair,
- `StateCapture.capture_context_material`, if present, is auxiliary boundary context only.

**Minimum later-fixture interpretation rule:**
- a later machine-readable fixture for this vector must preserve the frozen `StateCapture` field structure and optionality exactly,
- `subject_scope`, `state_material`, and optional `capture_context_material` must remain distinct payload domains and must not be silently concatenated, reordered into a different semantic object, or treated as implicit proof/certification material,
- this vector does not freeze final byte encoding, hash-domain tags, or cryptographic preimage layout beyond requiring that later fixtures keep the boundary fields distinct and ordered according to the frozen adapter-contract types.

**What this does not claim:**
- post-state availability,
- mutation success,
- receipt derivation,
- verifier validity,
- commitment correctness,
- proof correctness,
- cryptographic sufficiency beyond the returned boundary object shape.

---

## Vector GV-06 — CVDR retrieval error: not found

**Purpose:** Separate absence of artifact from malformed request and from unsupported version.

**Inputs (conceptual):**
- target identifier is well-formed,
- requested version is supported,
- no matching artifact exists in the queried scope.

**Expected outcome:**
- named result family = `CvdrNotFound`,
- no substitute object is returned,
- result is distinct from internal failure and distinct from invalid input.

**What this locks down:**
- missing artifact is a named retrieval outcome,
- not-found is not rewritten as internal error,
- the caller can distinguish “absent” from “broken.”

---

## Vector GV-06A — CVDR retrieval error: not yet issued

**Purpose:** Separate clean current absence from the narrower case where issuance is expected by lifecycle state but has not yet occurred.

**Inputs (conceptual):**
- target identifier is well-formed,
- requested version is supported,
- referenced artifact is not presently retrievable,
- surrounding lifecycle/readiness state indicates issuance is pending or not yet complete rather than definitively absent.

**Expected outcome:**
- named result family = `CvdrNotYetIssued`,
- operation does not collapse this into `CvdrNotFound`,
- operation does not pretend retrieval succeeded with a placeholder artifact.

**What this locks down:**
- not-yet-issued is a distinct named retrieval outcome,
- lifecycle-pending state is not the same as permanent or clean absence,
- later interface work must preserve this distinction explicitly.

---

## Vector GV-07 — CVDR retrieval error: invalid request

**Purpose:** Separate caller error from artifact absence.

**Inputs (conceptual):**
- request identifier or request shape is malformed,
- retrieval cannot even be meaningfully attempted.

**Expected outcome:**
- named result family = `CvdrInvalidRequest`,
- operation does not perform ambiguous partial lookup,
- operation does not collapse into `CvdrNotFound`.

**What this locks down:**
- malformed input is not the same as “not found,”
- invalid request must fail early and explicitly.

---

## Vector GV-08 — CVDR retrieval error: internal failure

**Purpose:** Preserve the distinction between caller-visible absence and system inability.

**Inputs (conceptual):**
- request is well-formed,
- requested version is supported,
- retrieval route encounters an internal failure that prevents authoritative answer.

**Expected outcome:**
- named result family = `CvdrInternalError`,
- operation does not pretend the artifact is absent,
- operation does not return partial artifact state as success.

**What this locks down:**
- internal failure remains explicit,
- system inability must not masquerade as clean absence.

---

## Vector GV-09 — Tombstoned position detected

**Purpose:** Freeze the semantic distinction between a tombstoned slot and a simply empty slot.

**Inputs (conceptual):**
- witness path identifies the target logical position,
- target position is occupied by a tombstone marker under approved Tree-mode semantics,
- pre/post roots and witness are otherwise coherent.

**Expected outcome:**
- named witness classification = `TombstonedPosition`,
- result must not be normalised to `EmptyPosition`,
- downstream proof semantics may treat this as a deletion/tombstoning-bearing state.

**What this locks down:**
- tombstoned and empty are not synonyms,
- the protocol must preserve the semantic difference,
- later fixtures must encode this distinction unambiguously.

---

## Vector GV-10 — Empty position detected

**Purpose:** Lock the complementary case.

**Inputs (conceptual):**
- witness path identifies the target logical position,
- target position is empty / absent under approved Tree-mode semantics,
- no tombstone occupancy is present for that position.

**Expected outcome:**
- named witness classification = `EmptyPosition`,
- result must not be normalised to `TombstonedPosition`,
- downstream proof semantics must not over-claim a deletion/tombstoning event from emptiness alone.

**What this locks down:**
- absence alone does not imply tombstoning,
- the protocol must keep deletion-bearing and non-deletion-bearing states separate.

---

## Vector GV-11 — Inline certification baseline accepted

**Purpose:** Establish the minimum positive case for inline certification material.

**Inputs (conceptual):**
- proof/receipt object contains inline certification material required by the approved current rules,
- certification material is present in the expected field(s),
- certification material is self-consistent with the artifact it accompanies,
- version is supported.

**Expected outcome:**
- certification baseline classification = `InlineCertificationPresentAndProcessable`,
- object remains eligible for ordinary downstream validation flow,
- no live-only fetch is required merely because inline certification exists.

**What this locks down:**
- inline certification is a first-class baseline path,
- presence + processability matters,
- absence of live corroboration does not automatically block the inline baseline.

---

## Vector GV-12 — Inline certification baseline blocked

**Purpose:** Define the negative counterpart.

**Inputs (conceptual):**
- proof/receipt object is expected to carry inline certification material for the requested operation,
- required inline certification material is missing, malformed, or self-inconsistent.

**Expected outcome:**
- top-level status class = `Blocked`,
- named reason family = `InlineCertificationUnavailableOrInvalid`,
- operation must not silently downgrade to “certification not checked.”

**What this locks down:**
- inline certification failure is explicit,
- missing or malformed inline certification is not a warning-only condition when the route requires it.

---

## Vector GV-13 — Inline provenance baseline accepted

**Purpose:** Establish the positive semantic baseline for provenance material that travels with the artifact.

**Inputs (conceptual):**
- object contains inline provenance material required by the approved current rules,
- provenance material is present in the expected field(s),
- provenance material is internally coherent with the object it accompanies,
- version is supported.

**Expected outcome:**
- provenance baseline classification = `InlineProvenancePresentAndProcessable`,
- object remains eligible for later provenance-aware validation.

**What this locks down:**
- provenance is treated as an explicit artifact surface,
- provenance presence is not inferred from unrelated metadata.

---

## Vector GV-14 — Inline provenance baseline blocked

**Purpose:** Define the negative counterpart for provenance.

**Inputs (conceptual):**
- requested operation requires inline provenance baseline material,
- required provenance material is missing, malformed, or self-inconsistent.

**Expected outcome:**
- top-level status class = `Blocked`,
- named reason family = `InlineProvenanceUnavailableOrInvalid`,
- operation must not silently continue as though provenance were satisfied.

**What this locks down:**
- provenance failure remains explicit,
- no silent downgrade from provenance-bearing route to provenance-free success.

---

## 6. Minimal outcome taxonomy implied by v1

This document does not yet define final wire enums, but it does require the later interface/files/fixtures to preserve at least the following semantic distinctions:

### 6.1 Status class
- `Ready`
- `Blocked`

### 6.2 Version class
- `VersionSupported`
- `UnsupportedVersion`

### 6.3 Retrieval class
- `CvdrRetrieved`
- `CvdrNotFound`
- `CvdrNotYetIssued`
- `CvdrInvalidRequest`
- `CvdrInternalError`

### 6.4 Position / witness class
- `TombstonedPosition`
- `EmptyPosition`

### 6.5 Inline baseline class
- `InlineCertificationPresentAndProcessable`
- `InlineCertificationUnavailableOrInvalid`
- `InlineProvenancePresentAndProcessable`
- `InlineProvenanceUnavailableOrInvalid`

These names are semantic placeholders in v1. The later formal interface may rename them, but it must preserve the distinctions.

---

## 7. Open questions intentionally left for later lock-down

The following remain open at the end of this semantic-first pass and must be resolved before machine fixtures are declared final:

1. Exact byte encodings for all proof-bearing objects.
2. Exact domain tags and hash preimage layouts.
3. Exact public type names in the library and adapter `.did` files.
4. Exact receipt/proof identity derivation fields.
5. Whether inline certification and inline provenance appear as separate fields, separate route guarantees, or both.
6. Exact mapping from semantic outcome families here to concrete error/result enums in the formal interface.

Closed in the current conceptual-interface phase: ready/blocked status exposure is no longer an open design question at the semantic level. The approved conceptual interface already treats overall tree-mode status and evidence readiness as separate explicit query surfaces, so later work should refine names and concrete types rather than reopen that structural decision.

Per the build plan, none of these may remain open if they would still change public types, verifier inputs, hashing/encoding rules, adapter obligations, lifecycle semantics, or version behaviour at the Phase 6 exit gate.

---

## 8. Required follow-on artifacts

This note should be followed by:

1. a companion negative-cases note that mirrors these semantic distinctions for failure fixtures,
2. machine-readable positive fixtures,
3. machine-readable negative fixtures,
4. a fixture manifest mapping each concrete fixture back to the vector IDs in this file,
5. final interface-rule text pinning encoding, version, and optionality semantics.

Note for later fixture-manifest drafting: `GV-06A` must be addressed by the manifest naming convention — confirm whether A-suffix vectors are treated as sub-vectors of `GV-06` or as independent entries.

---

## 9. Review checklist for this note

Before v1 is approved, confirm all of the following:

- the vectors stay dApp-agnostic,
- no TinyPress-local nouns or payload shapes appear,
- unsupported version behaviour is explicit,
- retrieval success is distinguished from retrieval failure,
- not-found is distinguished from invalid request,
- tombstoned vs empty is kept explicit,
- inline certification and inline provenance each have both positive and blocked baselines,
- no vector depends on silent defaults,
- no vector accidentally conflates library-internal state failure with caller-supplied input failure,
- no vector accidentally over-claims full proof validity when it only establishes retrieval or baseline processability.

---

## 10. Summary

MKTd03 golden vectors v1 deliberately freezes the first semantic layer only.

It says, in effect:

- readiness must be affirmative,
- blocked must be explicit,
- supported vs unsupported version handling must fail loud,
- CVDR retrieval outcomes must be named and separable,
- tombstoned and empty positions must never be conflated,
- inline certification and inline provenance baselines must each have explicit positive and negative cases.

That is enough to guide the next interface and fixture pass without pretending the byte-level work is already complete.
