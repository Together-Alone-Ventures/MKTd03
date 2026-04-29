# MKTd03 Commitment and Preimage Spec v1

**Status:** DRAFT — FOR C REVIEW  
**Authority:** Subordinate implementation specification with ADR-grade closure effect for selected ADR-03 and ADR-02 open questions.  
**Purpose:** Make ADR-03 executable by fixing the byte-level commitment, preimage, serialization, and encoding rules required for baseline Tree-mode implementation.

---

## 0. Authority statement

This document does **not** reopen the semantic decisions already settled by ADR-02 or ADR-03. It exists to make those decisions implementable by fixing the byte-level rules that ADR-03 explicitly leaves unresolved and the remaining baseline publication rules ADR-02 leaves open.

This document adopts the frozen public receipt field names and field ordering already reflected in `interfaces/mktd03_library.did`, including the `CoreTransitionEvidence` ordering:

1. `subject_reference`
2. `scope_reference`
3. `pre_state_commitment`
4. `post_state_commitment`
5. `transition_material`
6. `transition_derivation_version`
7. `tree_proof`
8. `deletion_state_material`

G approval of this document constitutes ADR-grade closure for the following ADR-03 open questions in the baseline MKTd03 Tree-mode line:

- certification/provenance placement
- exact byte-level composition of pre-state and post-state commitments
- exact byte-level serialization of `tree_proof`
- exact byte-level structure of `DeletionStateMaterial`
- exact certified-commitment composition rule
- exact receipt-ID derivation rule
- adoption of the frozen `.did` field names and ordering as the baseline publication structure

This document also closes the remaining baseline publication questions left open by ADR-02 for:

- canonical record-position mapping formula
- baseline SMT depth
- baseline proof-frame count and no-compression rule

G approval of this document therefore constitutes ADR-grade closure for those baseline Tree-mode publication rules as well.

This document is authoritative for byte-level implementation rules. If a later implementation artifact conflicts with this document on byte composition, encoding, serialization, domain-tag selection, canonical record-position mapping, SMT depth, or proof-frame count, this document wins unless explicitly reopened by G.

---

## 1. Certification and provenance placement ruling

### 1.1 Baseline ruling

Baseline MKTd03 uses **inline certification material** and **inline build/module provenance** in the issued receipt.

The baseline does **not** use a stable published pointer for either certification material or build/module provenance.

### 1.2 Reason for the ruling

This baseline preserves ADR-03’s archival-first and self-contained-at-issuance posture:

- verification must not depend on a later live fetch to complete the baseline evidence set
- the receipt must be complete enough at issuance that an archival verifier can operate on the issued artifact plus the already-captured ICP certificate material
- a stable published pointer would introduce a hidden “fetch later to complete the receipt” dependency unless the pointed-to content were also fully embedded or independently fixed at issuance

### 1.3 Relationship to ADR-03 rejected alternatives

This ruling is explicitly chosen to remain compatible with ADR-03’s rejected alternatives:

- **Rejected:** live-fetch-first baseline verification
- **Rejected:** receipt incomplete at issuance

Accordingly:

- baseline verification must not require a later fetch of certification or provenance material
- a pointer-only baseline is not permitted in v1
- any future pointer-based variant would require explicit reopening and must demonstrate that it does **not** recreate those rejected alternatives in disguised form

### 1.4 Baseline provenance payload

For v1 baseline receipts, inline build/module provenance bytes are exactly the 32-byte deployed WASM module hash used by the library’s existing provenance discipline.

No richer provenance chain, transparency-log proof, or reproducible-build attestation bundle is part of the v1 baseline.

---

## 2. Domain tag table

All protocol domain tags are ASCII byte strings, version-suffixed, and consumed exactly as written.

The following tags are fixed for v1:

| Symbol | Byte value |
|---|---|
| `TAG_RECORD_POSITION_KEY` | `b"MKTD03_RECORD_POSITION_KEY_V1"` |
| `TAG_LEAF` | `b"MKTD03_LEAF_V1"` |
| `TAG_INTERNAL_NODE` | `b"MKTD03_INTERNAL_NODE_V1"` |
| `TAG_PRE_STATE_COMMITMENT` | `b"MKTD03_PRE_STATE_COMMITMENT_V1"` |
| `TAG_POST_STATE_COMMITMENT` | `b"MKTD03_POST_STATE_COMMITMENT_V1"` |
| `TAG_TRANSITION_MATERIAL` | `b"MKTD03_TRANSITION_MATERIAL_V1"` |
| `TAG_CERTIFIED_COMMITMENT` | `b"MKTD03_CERTIFIED_COMMITMENT_V1"` |
| `TAG_RECEIPT_ID` | `b"MKTD03_RECEIPT_ID_V1"` |

### 2.1 Domain-separation invariant

`TAG_RECEIPT_ID` is a distinct derivation domain and is **not** interchangeable with any protocol commitment tag.

Receipt-ID derivation is therefore cryptographically separated from:

- record-position-key derivation
- leaf hashing
- internal-node hashing
- pre-state commitment
- post-state commitment
- transition-material derivation
- certified-commitment publication

No tag aliasing is permitted between receipt identity and protocol commitments.

---

## 3. Inheritance from S7-4 hashing discipline

This document inherits the generic hashing behavior already landed in the protocol-owned hashing helper:

- `hash_with_tag(tag, parts)` computes SHA-256 over the exact byte concatenation  
  `tag || parts[0] || parts[1] || ...`
- tag bytes are consumed exactly as supplied
- part bytes are consumed exactly as supplied
- no separator, null terminator, length-prefix, Unicode normalization, or hidden transform is added implicitly

This document therefore does **not** restate generic hashing behavior except where commitment-specific byte ordering must be fixed.

If a composite encoding is required, this document spells it out explicitly before passing bytes to `hash_with_tag`.

---

## 4. Input field encoding rules

### 4.1 `SemanticVersion` byte encoding

When `SemanticVersion` is consumed by a preimage in this document, it is encoded as:

- `major` as 4-byte unsigned big-endian
- `minor` as 4-byte unsigned big-endian
- `patch` as 4-byte unsigned big-endian

Concatenated in that order:

`major || minor || patch`

Total encoded length: 12 bytes.

### 4.2 `subject_reference` byte production

`subject_reference` is the canonical byte representation of the targeted record key or subject key as produced by the adapter-side canonicalization contract.

When this document refers to `subject_reference` bytes in a preimage, it means the exact `subject_reference` bytes published in the receipt.

This document does **not** permit later re-encoding of that field for preimage use.

If the underlying identifier is an ICP principal, the bytes must be the raw principal bytes, not text.

### 4.3 `scope_reference` byte production

`scope_reference` is optional.

For all preimage use in this document, the option encoding is fixed as:

- `0x00` for `None`
- `0x01 || scope_reference_bytes` for `Some(scope_reference)`

Where `scope_reference_bytes` are the exact published receipt bytes.

If the underlying scope identifier is an ICP principal, the bytes must be raw principal bytes, not text.

### 4.4 `transition_material` byte production

`transition_material` is a derived v1 field, not an arbitrary free-form payload.

Let:

- `transition_derivation_version_bytes` be the 12-byte encoding from §4.1
- `canonical_transition_source_bytes` be the adapter-produced canonical byte string for the targeted record’s pre-delete application-state material under the declared `transition_derivation_version`

Then:

`transition_material = hash_with_tag(TAG_TRANSITION_MATERIAL, &[transition_derivation_version_bytes, canonical_transition_source_bytes])`

The receipt field stores the resulting 32-byte digest.

This document does **not** define the internal structure of `canonical_transition_source_bytes`; that remains adapter-owned. It does require that, for a fixed `transition_derivation_version`, the adapter’s canonicalization be deterministic.

### 4.5 Inline provenance byte production

For v1 baseline receipts, inline build/module provenance bytes are exactly the 32-byte deployed module hash.

No text encoding, wrapper object, or auxiliary metadata is added to this field in v1.

### 4.6 Optional-field rule

Unless this document explicitly says otherwise, optional-field handling in preimages must use the exact option encoding from §4.3 and must not rely on omission-by-concatenation.

### 4.7 Canonical record-position mapping

Baseline v1 binds a target record to one canonical terminal position by deriving a record-position key from the published record identifier fields already present in the receipt.

The exact byte inputs are:

- `subject_reference`
- `encoded_scope_reference` from §4.3

The canonical record-position key is:

`record_position_key = hash_with_tag(TAG_RECORD_POSITION_KEY, &[subject_reference, encoded_scope_reference])`

Baseline v1 SMT depth is fixed at **256 levels**, yielding a fixed-capacity terminal-position space of **2^256** positions.

The terminal-position path bits are the 256 bits of `record_position_key`, consumed **most-significant-bit first** from root to leaf.

No alternative index-mapping rule is permitted in baseline v1.

---

## 5. Leaf-hash preimage rule

### 5.1 Leaf state discriminants

The first payload byte after `TAG_LEAF` is the leaf-state discriminant:

- `0x00` = empty leaf
- `0x01` = occupied leaf
- `0x02` = tombstoned leaf

### 5.2 Empty leaf

The canonical empty leaf hash is:

`hash_with_tag(TAG_LEAF, &[b"\x00"])`

No `subject_reference`, `scope_reference`, `transition_material`, or `deletion_state_material` appears in the empty-leaf preimage.

### 5.3 Occupied leaf

For a targeted record that exists before deletion, the occupied leaf hash is:

`hash_with_tag(TAG_LEAF, &[b"\x01", subject_reference, encoded_scope_reference, transition_material])`

Where:

- `subject_reference` is the exact field bytes from §4.2
- `encoded_scope_reference` is the option encoding from §4.3
- `transition_material` is the exact 32-byte field bytes from §4.4

The occupied leaf is placed at the canonical terminal position derived from `record_position_key` per §4.7. No other position is permitted for this subject.

### 5.4 Tombstoned leaf

For the corresponding record after deletion, the tombstoned leaf hash is:

`hash_with_tag(TAG_LEAF, &[b"\x02", subject_reference, encoded_scope_reference, deletion_state_material])`

Where:

- `subject_reference` is the exact field bytes from §4.2
- `encoded_scope_reference` is the option encoding from §4.3
- `deletion_state_material` is the exact field bytes from §10

`transition_material` is deliberately excluded from the tombstoned-leaf preimage because the transition is bound at receipt level, while the post-state leaf represents only the tombstoned record position.

The tombstoned leaf is placed at the canonical terminal position derived from `record_position_key` per §4.7. No other position is permitted for this subject.

### 5.5 Non-collapse rule

A tombstoned leaf must **not** hash to the same value as an empty leaf.

This is achieved by:

- distinct discriminant bytes (`0x02` vs `0x00`)
- distinct preimage payloads
- explicit presence of `subject_reference`, `scope_reference`, and `deletion_state_material` in the tombstoned case

This rule is load-bearing and is the byte-level mechanism by which ADR-03’s “deletion-state semantics do not collapse into empty-position representation” requirement is realized.

---

## 6. Internal-node hash preimage rule

### 6.1 Node composition

An internal node hash is always:

`hash_with_tag(TAG_INTERNAL_NODE, &[left_child_hash, right_child_hash])`

Where:

- `left_child_hash` is the 32-byte hash of the left child
- `right_child_hash` is the 32-byte hash of the right child

No sorting, normalization, or reordering is permitted.

### 6.2 Empty-subtree convention

The canonical empty-subtree roots are defined recursively.

Let:

- `EMPTY_ROOT[0] = empty_leaf_hash` from §5.2

Then:

- `EMPTY_ROOT[h + 1] = hash_with_tag(TAG_INTERNAL_NODE, &[EMPTY_ROOT[h], EMPTY_ROOT[h]])`

This recursion is the sole v1 empty-subtree convention.

### 6.3 Root computation

The tree root is the repeated application of the internal-node rule up the SMT path, using:

- concrete leaf hashes for occupied/tombstoned leaves
- canonical empty-subtree roots for all absent branches not explicitly populated by a sibling hash

---

## 7. Pre-state commitment composition

### 7.1 What `pre_state_commitment` commits to

`pre_state_commitment` commits to the canonical pre-delete SMT root only.

It does **not** directly concatenate `subject_reference`, `scope_reference`, `transition_material`, or `transition_derivation_version` into the commitment preimage.

### 7.2 Exact composition rule

Let `pre_state_root` be the SMT root of the tree immediately before the deletion transition, computed using:

- the occupied leaf rule from §5.3 at the targeted position
- the current tree contents at all other populated positions
- the empty-subtree convention from §6.2 where appropriate

Then:

`pre_state_commitment = hash_with_tag(TAG_PRE_STATE_COMMITMENT, &[pre_state_root])`

### 7.3 Justification

This is an explicit protocol decision.

The cryptographic binding of the targeted subject and its pre-delete material is carried by the combination of:

- `subject_reference`
- `scope_reference`
- `transition_material`
- the occupied leaf rule
- the `tree_proof` path
- the pre-state SMT root

Repeating those fields inside `pre_state_commitment` would be redundant and would make one root-specific proof less reusable across multiple independently provable subjects against the same root.

In v1, the commitment is therefore a root commitment, not a root-plus-subject commitment.

---

## 8. Post-state commitment composition

### 8.1 What `post_state_commitment` commits to

`post_state_commitment` commits to the canonical post-delete SMT root only.

It does **not** directly concatenate `subject_reference`, `scope_reference`, `deletion_state_material`, or `transition_derivation_version` into the commitment preimage.

### 8.2 Exact composition rule

Let `post_state_root` be the SMT root of the tree immediately after the deletion transition, computed using:

- the tombstoned leaf rule from §5.4 at the targeted position
- the current tree contents at all other populated positions
- the empty-subtree convention from §6.2 where appropriate

Then:

`post_state_commitment = hash_with_tag(TAG_POST_STATE_COMMITMENT, &[post_state_root])`

### 8.3 Justification

This mirrors the reasoning in §7.

The cryptographic binding of the targeted subject to the post-delete state is carried by:

- `subject_reference`
- `scope_reference`
- `deletion_state_material`
- the tombstoned leaf rule
- the `tree_proof` path
- the post-state SMT root

The post-state commitment therefore remains a root commitment, not a root-plus-subject commitment.

---

## 9. `tree_proof` serialization format

### 9.1 Ordering convention

`tree_proof` serializes the Merkle path **from leaf to root**.

The first serialized step is the sibling relation immediately above the leaf.  
The final serialized step is the sibling relation immediately below the root.

### 9.2 Envelope

The byte sequence begins with a 2-byte unsigned big-endian step count:

`step_count_u16_be`

For a valid baseline v1 proof, `step_count` must equal **256** exactly.

Path compression is **not permitted** in baseline v1. Every level is represented by exactly one frame.

This is followed by exactly `step_count` serialized path frames.

### 9.3 Per-level frame format

Each frame is encoded as:

1. `direction_byte` — 1 byte
2. `sibling_kind_byte` — 1 byte
3. `sibling_hash_bytes` — present only when `sibling_kind_byte` requires it

### 9.4 Direction encoding

`direction_byte` is defined as:

- `0x00` = current node is the **left** child; sibling is on the **right**
- `0x01` = current node is the **right** child; sibling is on the **left**

This direction byte is mandatory at every level.

For a valid baseline v1 proof, the encoded direction at level `h` must equal the corresponding bit of `record_position_key` from §4.7, consumed most-significant-bit first from root to leaf.

The verifier reads the direction from the proof bytes for deterministic replay, but a divergence between the encoded direction and the key-derived direction is a malformed proof.

Direction must not be inferred from external tree state.

### 9.5 Sibling-kind encoding

`sibling_kind_byte` is defined as:

- `0x00` = explicit sibling hash follows as 32 raw bytes
- `0x01` = sibling is the canonical empty-subtree root for this level; no explicit hash bytes follow

No other sibling-kind values are valid in v1.

### 9.6 Empty-sibling handling

When `sibling_kind_byte = 0x01`, the verifier reconstructs the sibling hash using the canonical empty-subtree root for that level from §6.2.

Empty siblings are therefore represented explicitly in the proof by kind code, but their bytes are not redundantly embedded.

### 9.7 Verifier determinism requirement

A verifier given:

- `subject_reference`
- `scope_reference`
- `transition_material`
- `deletion_state_material`
- the relevant pre/post commitment
- the serialized `tree_proof`

must be able to reconstruct the path deterministically using only the rules in this document.

---

## 10. `DeletionStateMaterial` byte structure

### 10.1 Baseline v1 structure

For v1 baseline Tree-mode deletion receipts, `DeletionStateMaterial` is exactly one byte:

- `0x01` = tombstoned

The receipt field stores exactly:

`b"\x01"`

### 10.2 What this field does and does not mean

This field is a baseline tombstone-state marker only.

It does **not** embed:

- timestamp
- actor identity
- reason code
- adapter-local metadata
- application-local deletion policy details

### 10.3 Empty-state distinction

Empty state is **not** encoded through `DeletionStateMaterial`.

Empty state is represented exclusively by the empty-leaf rule in §5.2.

This keeps the tombstoned-vs-empty distinction explicit and prevents “no bytes present” from being overloaded as a tombstone encoding.

### 10.4 Future evolution rule

Any future expansion of `DeletionStateMaterial` beyond the single-byte tombstone marker requires explicit versioned change control and must not silently reinterpret `b"\x01"`.

---

## 11. Certified-commitment composition rule

### 11.1 Baseline rule

Baseline v1 uses a **single-receipt certified-data window**.

The exact 32-byte value published through ICP certified data for the deletion transition is:

`certified_commitment = hash_with_tag(TAG_CERTIFIED_COMMITMENT, &[receipt_id])`

### 11.2 Rationale

This keeps the ICP-certified surface minimal while still cryptographically anchoring the issued receipt.

The certified commitment does **not** republish every evidence component individually.  
Instead, it anchors the receipt identity, and the receipt identity in turn binds the core transition evidence and inline provenance per §12.

### 11.3 Relationship to certification material in the receipt

The receipt’s inline certification material proves that ICP certified data contained the `certified_commitment` value from §11.1.

Because `certified_commitment` commits to `receipt_id`, and `receipt_id` commits to the v1 evidence set from §12, archival verification can connect:

- ICP-certified publication
- receipt identity
- core transition evidence
- inline provenance

without requiring a later fetch of external baseline data.

### 11.4 Inline certification material byte structure

The receipt’s inline certification material carries the exact byte string returned by `ic0.data_certificate()` at Phase B, with no transformation, wrapper framing, or re-encoding by MKTd03.

Baseline v1 does not redefine ICP certificate format.

An empty or absent certificate is not a valid baseline receipt artifact and must fail loud at issuance/finalization.

### 11.5 Certified-data window rule

Baseline v1 anchors exactly one `receipt_id` per certified-data publication window.

Phase B certificate capture for that published value must complete before any subsequent Phase A publication overwrites certified data.

Baseline v1 does **not** support multi-receipt batching through a Merkle root of multiple `receipt_id` values. That batching pattern is explicitly deferred to a future version.

---

## 12. Receipt-ID derivation rule

### 12.1 Domain-separation rule

Receipt-ID derivation must use `TAG_RECEIPT_ID` and no other protocol tag.

It is not permitted to reuse:

- `TAG_RECORD_POSITION_KEY`
- `TAG_PRE_STATE_COMMITMENT`
- `TAG_POST_STATE_COMMITMENT`
- `TAG_CERTIFIED_COMMITMENT`
- `TAG_LEAF`
- `TAG_INTERNAL_NODE`
- `TAG_TRANSITION_MATERIAL`

### 12.2 Exact ordered field set

The v1 receipt ID is derived over the following fields, in this exact order:

1. `subject_reference`
2. `encoded_scope_reference`
3. `pre_state_commitment`
4. `post_state_commitment`
5. `transition_material`
6. `transition_derivation_version_bytes`
7. `tree_proof`
8. `deletion_state_material`
9. `inline_module_hash_provenance_bytes`

Where:

- `encoded_scope_reference` uses the option encoding from §4.3
- `transition_derivation_version_bytes` uses the version encoding from §4.1
- `inline_module_hash_provenance_bytes` is the exact 32-byte module hash from §4.5

### 12.3 Exact derivation rule

`receipt_id = hash_with_tag(TAG_RECEIPT_ID, &[subject_reference, encoded_scope_reference, pre_state_commitment, post_state_commitment, transition_material, transition_derivation_version_bytes, tree_proof, deletion_state_material, inline_module_hash_provenance_bytes])`

### 12.4 Certification-material exclusion

Inline certification material is deliberately **not** included in the receipt-ID preimage.

Reason:  
the receipt ID must exist before the Phase B/Phase C certification capture/embedding steps complete, so that:

- `certified_commitment` can be published in Phase A
- the certificate can later be captured against that published commitment
- the final receipt can embed the certification material without circularity

### 12.5 Consequence

In v1 baseline:

- core transition evidence and inline provenance are bound by `receipt_id`
- certification material binds to `receipt_id` indirectly through `certified_commitment`

That asymmetry is intentional and is the baseline A→B→C-compatible design.

Receipt identity is therefore intentionally path-dependent and tree-state-specific, not an abstract deletion-event identifier divorced from a particular committed tree state.

---

## 13. Explicit deferrals

The following remain explicitly deferred after this document:

- verifier implementation mechanics beyond consuming the byte rules fixed here
- receipt issuance choreography (Phase A → Phase B → Phase C)
- storage layout and stable-memory schema
- adapter-specific canonicalization internals for `canonical_transition_source_bytes`
- adapter integration details
- retry/recovery semantics
- pointer-based certification/provenance variants
- any richer provenance chain beyond inline module hash
- any richer `DeletionStateMaterial` payload beyond the tombstone marker
- any future relaxation or introduction of `conditionally_compatible` policy for version support

## Immediate next-slice consequence

With this document approved, the next smallest safe implementation slice is leaf-hash construction against these fixed rules.?
