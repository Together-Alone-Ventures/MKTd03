# ADR-02: Tree-Structure Choice

## Status
Approved

## Date
2026-03-26

## Context
MKTd03 Tree mode requires a canonical published tree model so that:
- update/delete operations produce deterministic committed state,
- proofs and receipts can be interpreted consistently across implementations,
- terminology does not drift from earlier MKTd02 leaf-mode wording into incorrect Tree-mode assumptions,
- rebuild and verification logic operate against one settled structural model.

The stale-spec inventory surfaced unresolved structure assumptions and terminology leakage, including binary-tree padding rules, "leaf hash" wording, path semantics, and proof-shape implications.
The Tree-mode invariants note has already fixed the operational safety constraints that any tree choice must preserve.

ADR-02 therefore needs to settle the canonical structural model and the baseline vocabulary used to describe it, without spilling into receipt schema, verifier procedure, or library/adapter boundary.

## Decision
MKTd03 baseline Tree mode will use one canonical binary tree model with deterministic record placement, explicit domain separation for hashing, and fixed-capacity semantics for each ready tree instance.

The baseline structural decision is:

1. **Canonical tree model**  
   Baseline MKTd03 uses a canonical binary Merkle tree model.

2. **Record placement / ordering**  
   Target records are mapped into terminal positions by one deterministic published placement rule. Baseline MKTd03 defines that rule as: compute the canonical record-position key from the published record identifier input, hash it with the canonical record-position hash function, interpret the result as an unsigned integer, and map it into the ready tree instance’s fixed-capacity terminal-position space by the canonical published index-mapping rule for that capacity. This rule must be implementation-independent and must not rely on canister-local incidental ordering.

3. **Hash domain separation**  
   Tree hashing must use explicit canonical role separation so that structurally different hash inputs cannot be confused across node roles. Baseline MKTd03 requires distinct published hash-role identifiers for at least:
   - internal nodes,
   - empty positions,
   - live record positions,
   - tombstoned positions.
   Structurally different roles must not share the same hash-role identifier or equivalent separation rule.

4. **Empty versus tombstoned positions**  
   Baseline MKTd03 treats empty or unoccupied positions and tombstoned positions as structurally distinct for proof and deletion-evidence semantics. They must not be represented as interchangeable tree states in the canonical model.

5. **Capacity model**  
   Each ready tree instance has an explicit fixed capacity parameter. Capacity remains fixed for the lifetime of that ready tree instance and does not shrink when deletions make the tree sparser. Any change in capacity requires rebuild or replacement into a new ready tree instance rather than implicit in-place resizing.

6. **Baseline terminology**  
   ADR-02 rejects loose carry-forward of MKTd02 leaf-centric wording where it would blur Tree-mode semantics. Baseline terms must distinguish:
   - record identifier,
   - record position or terminal position,
   - sibling path or proof path,
   - empty position,
   - tombstoned position,
   - internal node.

7. **Rebuild compatibility constraint**  
   A tree may be treated as structurally compatible for reuse only if all of the following remain unchanged:
   - canonical tree structure,
   - fixed-capacity parameter definition and value for the ready tree instance,
   - canonical record-placement rule,
   - canonical hash-role separation scheme,
   - structural interpretation of empty versus tombstoned positions.
   If any of those conditions changes, the existing tree must not be treated as proof-compatible carry-forward state and rebuild or replacement treatment is required.

This ADR does not yet finalise the exact placement formula, exact hash preimage layouts, or final proof/receipt schema wording. Those later artifacts must remain consistent with the structural decisions made here and must not alter the structural baseline by implication.

## Published Terminology Addendum
For interface-prep work, the stable published public nouns are:
- subject
- scope
- pre-state commitment
- post-state commitment
- transition
- tree proof
- receipt
- certification/provenance
- status
- compatibility

The key distinctions are:
- receipt is the full public artifact,
- tree proof is a component inside the receipt,
- subject is distinct from scope,
- status is distinct from compatibility.

Later frozen interface work may still normalise casing, style, and exact field notation, but it must not replace these public nouns by drift.

## Decision Shape
ADR-02 must produce explicit answers for the following:
1. the canonical tree model name and structural definition,
2. the canonical record-placement/order rule,
3. the canonical rule for empty or unoccupied positions,
4. the canonical rule for tombstoned positions, if structurally distinct from empty positions,
5. the baseline terminology for position, path, sibling, proof element, and state transition,
6. the rebuild-compatibility conditions under which an existing tree may be reused versus rebuilt.

ADR-02 is not complete until each of those items is answered in normative language rather than left as descriptive discussion.

## Remaining Questions to Resolve Within This ADR
- What exact casing/style and lower-level notation should later frozen artifacts use for terminal positions and proof-path elements while preserving the published public nouns settled above?
- What exact published notation should baseline MKTd03 use to describe the index-mapping rule and hash-role identifiers?

## Constraints from Earlier Artifacts
- Must stay within ADR-00 evidentiary scope.
- Must preserve all invariants from the Tree-mode invariants note.
- Must not decide ADR-01 boundary questions by implication, including by embedding host-behaviour assumptions into structural terminology or by assuming whether tree storage is library-owned or host-owned.
- ADR-02 may define structural compatibility conditions, but must not assign whether the library or the host is responsible for enforcing them.
- Must not let TinyPress, app-shaped examples, or stale MKTd02 leaf-mode terminology influence baseline structure wording.


## Likely Inventory Drivers
- S8, S9, S10, S11, S26, S33, S37, S39, S40, S47, S62

## Rejected Alternatives
- **Dynamic in-place resizing**  
  Rejected because changing capacity within an already-ready tree instance would alter the terminal-position space and therefore risk changing proof meaning or weakening rebuild verifiability. Baseline MKTd03 requires capacity changes to occur only through rebuild or replacement into a new ready tree instance.

- **Tombstoned positions treated as structurally identical to empty positions**  
  Rejected because baseline MKTd03 deletion evidence depends on preserving a structural distinction between never-occupied/empty state and tombstoned deletion state.

- **Retention of loose leaf-centric terminology**  
  Rejected because carry-forward MKTd02 leaf-centric wording would blur Tree-mode structure and invite proof-semantics drift across later ADRs and spec artifacts.

- **Alternative canonical binary models**  
  Rejected because no alternative binary model was identified that demonstrated clearer proof semantics, rebuild compatibility, or terminology discipline than the chosen canonical binary Merkle model.

- **Non-binary or otherwise materially different canonical tree structures**  
  Rejected from the baseline because they add structural variability without a demonstrated need strong enough to outweigh cross-implementation simplicity, canonical proof interpretation, and rebuild discipline.
