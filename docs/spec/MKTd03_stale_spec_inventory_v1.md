# MKTd03 Stale Spec Inventory v1

## Purpose
Audit the old MKTd03 spreadsheet spec against current MKTd03 build intent and MKTd02-derived lessons.
This file is an audit artifact, not protocol authority.

## Classification buckets
- Keep
- Revise
- Drop
- Unresolved

## Review criteria
- Still valid for Tree-mode MKTd03
- Still dApp-agnostic
- Not contaminated by TinyPress/test-app assumptions
- Not relying on MKTd02 implementation history as authority
- Compatible with spec-first, ADR-first, interface-gated sequencing

## Inventory table
| Item ID | Old spec statement/assumption | Current assessment | Why | Follow-up artifact |
|---|---|---|---|---|
| S1 | MKTd03 targets shared multi-subject canister state and remains dApp-agnostic | Keep | Still valid | none |
| S2 | Target canister integrates Zombie Delete core plus an MKTd03 pack via a data adapter boundary | Unresolved | Boundary issue pending ADR-01 | ADR-01 |
| S3 | Deletion flow is initiated by an authorised deletion request and executed atomically within one message | Keep | Still valid | none |
| S4 | Current tree state supplies the pre-delete root; continual full snapshots are not required | Keep | Still valid | none |
| S5 | Existing write operations must guard against overwriting tombstoned records | Keep | Still valid | spec refresh note |
| S6 | Receipt contains Merkle roots, Merkle path, and H(record_id), with no plaintext record content or plaintext record ID | Revise | Verifier/evidentiary issue pending ADR-03 | ADR-03 |
| S7 | MKTd03 requires public source, deterministic build instructions, and published cryptographic specification | Keep | Still valid | ADR-00 |
| S8 | Tree/hash section uses leaf-hash terminology as if already settled Tree-mode language | Revise | Tree-mode uncertainty pending ADR-02; terminology leak | ADR-02 |
| S9 | Canonical CBOR key encoding and bytewise lexicographic ordering define cross-implementation tree determinism | Keep | Still valid | ADR-02 |
| S10 | Tree construction is specified as canonical bottom-up binary Merkle tree with empty-subtree padding and constant propagation | Unresolved | Tree-mode uncertainty pending ADR-02 | ADR-02 |
| S11 | Incremental single-record updates recompute only the changed position and O(log N) ancestors | Keep | Still valid | ADR-02 |
| S12 | Tombstone constant is fixed and published; tree-state tombstone proof and receipt-level tombstone hash are separate checks | Revise | Verifier/evidentiary issue pending ADR-03 | ADR-03 |
| S13 | Certified state binds merkle_root together with event_log_hash rather than certifying raw root alone | Revise | MKTd02 lessons learned; verifier/evidentiary issue pending ADR-03 | ADR-03 |
| S14 | Event-log chaining is part of the protocol evidence model | Revise | Verifier/evidentiary issue pending ADR-03 | ADR-03 |
| S15 | Verification procedures must be published for paths, tombstone hash, signatures, module hash, certified-state queries, and composite receipts | Revise | Verifier/evidentiary issue pending ADR-03 | ADR-03 |
| S16 | Module-hash publication and reproducible build chain are part of the trust model | Keep | Still valid | ADR-00 |
| S17 | Target canister adds an in-process MKTd03 library crate that runs inside the host canister execution context | Keep | Still valid | ADR-01 |
| S18 | Canister-specific integration is expressed through an MKTdDataSource trait adapter over host storage | Unresolved | Boundary issue pending ADR-01 | ADR-01 |
| S19 | Standard StableBTreeMap usage may allow a near-reference adapter with minimal canister-specific code | Revise | Boundary issue pending ADR-01; do not let one storage pattern become protocol authority | ADR-01 |
| S20 | Target canister should expose additive MKTd03-specific Candid endpoints such as delete, get_root, verify_inclusion, and get_receipt | Revise | Boundary issue pending ADR-01; interface surface pending formal interface phase | ADR-01 |
| S21 | Deletion endpoint must be restricted to authorised principals while read/query verification endpoints are public | Keep | Still valid | ADR-00 |
| S22 | Initial tree construction on first deployment or manifest-changing upgrade may require a multi-message initialisation window | Keep | Still valid | Tree-mode invariants note |
| S23 | Initialisation progress should be persisted in stable state with resumable progress markers | Keep | Still valid | Tree-mode invariants note |
| S24 | While initialisation is incomplete, PII-modifying operations must be frozen and only read-only queries continue | Keep | Still valid | Tree-mode invariants note |
| S25 | Initialisation/resume should be heartbeat-driven inside the host canister | Revise | Boundary issue pending ADR-01; implementation mechanism should not become protocol truth prematurely | ADR-01 |
| S26 | Tree node storage layout is frozen as a specific stable-memory structure and key schema | Unresolved | Tree-mode uncertainty pending ADR-02 | ADR-02 |
| S27 | Manifest-hash mismatch on upgrade forces full tree rebuild and deletion requests are rejected until rebuild completes | Keep | Still valid | Tree-mode invariants note |
| S28 | No deletion queue is implemented during initialisation because queueing would weaken pre_root invariants | Keep | Still valid | Tree-mode invariants note |
| S29 | Registration with a TAV-operated MKTd03 service canister is part of base integration | Drop | Boundary issue pending ADR-01; external service assumption is not yet protocol authority | ADR-01 |
| S30 | Multi-canister topology registration and service-canister assembly of composite receipts are part of the base design | Revise | Verifier/evidentiary issue pending ADR-03; may be evidentiary architecture, not core baseline | ADR-03 |
| S31 | Tombstone protection must reject any write that would overwrite a tombstoned record | Keep | Still valid | spec refresh note |
| S32 | Guard coverage across all PII-modifying code paths is an integration requirement, not a library guarantee | Keep | Still valid | ADR-01 |
| S33 | Merkle maintenance occurs incrementally on every insert, update, or delete in the same atomic host-canister execution | Keep | Still valid | ADR-02 |
| S34 | After each state change, the current certified value is updated to reflect the current tree-bound deletion state | Revise | MKTd02 lessons learned; verifier/evidentiary issue pending ADR-03 | ADR-03 |
| S35 | Old sheet describes certified publication as raw root publication, but earlier rows define certified binding as root plus event log hash | Revise | Verifier/evidentiary issue pending ADR-03; internal inconsistency in old spec | ADR-03 |
| S36 | Deletion request arrives as an authorised update call processed atomically by the canister | Keep | Still valid | none |
| S37 | Pre-transition capture includes pre_root, pre_leaf_hash, merkle_path, and hashed record key | Revise | Tree-mode uncertainty pending ADR-02; terminology leak and receipt-shape pending ADR-03 | ADR-02 / ADR-03 |
| S38 | Tombstone execution replaces stored value with the published tombstone constant and does not retain the original value in live state | Keep | Still valid | ADR-00 |
| S39 | Post-transition capture includes post_root, post_leaf_hash, and post-transition path data | Revise | Tree-mode uncertainty pending ADR-02; terminology leak and receipt-shape pending ADR-03 | ADR-02 / ADR-03 |
| S40 | Old sheet assumes exactly one leaf changed during deletion and describes the proof in leaf-specific terms | Revise | Tree-mode uncertainty pending ADR-02; terminology leak | ADR-02 |
| S41 | Tombstone-hash computation remains a separate receipt-level computation from tree-state proof | Keep | Still valid | ADR-03 |
| S42 | tombstone_hash is computed as H(record_key_hash || tombstone_constant || timestamp) using a published serialisation specification | Revise | Verifier/evidentiary issue pending ADR-03; exact receipt math must be re-settled, not inherited blindly | ADR-03 |
| S43 | Receipt identifier is derived from canister_id + record_key_hash + timestamp under a fixed domain tag | Revise | MKTd02 lessons learned; receipt ID derivation should not be accepted from old spec without re-checking current family rules | ADR-03 |
| S44 | Per-canister receipt includes subnet_id as a first-class field | Revise | MKTd02 lessons learned; old spec field set may no longer match current evidentiary requirements | ADR-03 |
| S45 | Per-canister receipt includes record_key_hash rather than plaintext key | Keep | Still valid | ADR-03 |
| S46 | Per-canister receipt includes pre_root and post_root as the core state transition evidence | Keep | Still valid | ADR-03 |
| S47 | Per-canister receipt includes pre_leaf_hash and post_leaf_hash as named fields | Revise | Tree-mode uncertainty pending ADR-02; terminology leak and receipt-shape pending ADR-03 | ADR-02 / ADR-03 |
| S48 | Per-canister receipt includes a Merkle path with sibling hash plus left/right position metadata | Keep | Still valid | ADR-03 |
| S49 | Per-canister receipt timestamp is ICP system time in nanoseconds | Keep | Still valid | ADR-03 |
| S50 | Receipt includes canister_module_hash captured at execution time via ic0::canister_module_hash() | Revise | MKTd02 lessons learned; module-hash capture mechanism must reflect actual ICP/platform constraints | ADR-00 / ADR-03 |
| S51 | Old sheet assumes receipt becomes self-contained only when certified query material is later fetched on demand | Revise | MKTd02 lessons learned; evidentiary model shifted toward archival-first embedded certification | ADR-03 |
| S52 | Receipt storage and post-root certification happen immediately after deletion and receipt_id is returned to caller | Keep | Still valid | ADR-03 |
| S53 | Base MKTd03 design depends on a shared TAV-operated service canister interface for registration, topology, listing, retries, and composite receipt assembly | Drop | Boundary issue pending ADR-01; external shared service is not established protocol baseline | ADR-01 |
| S54 | Composite receipt orchestration is part of the old sheet’s default baseline rather than an optional higher-layer evidentiary architecture | Revise | Verifier/evidentiary issue pending ADR-03; must distinguish core baseline from optional orchestration layer | ADR-03 |
| S55 | Registration model includes canister role metadata and pii_bearing flags as part of composite orchestration | Revise | Boundary issue pending ADR-01; service-layer contract should not be treated as core library truth yet | ADR-01 / ADR-03 |
| S56 | Composite finalisation requires all PII-bearing canisters completed unless requester explicitly accepts partial completion | Revise | Verifier/evidentiary issue pending ADR-03; completeness semantics belong in explicit evidentiary-scope decisions | ADR-00 / ADR-03 |
| S57 | Retry/failure state model for composite deletion is specified in the old sheet as if already part of the core build | Revise | Boundary issue pending ADR-01; orchestration failure model is not yet core library authority | ADR-01 / ADR-03 |
| S58 | Service canister internal design is out of scope, but its interface is still treated as a dependency for the library build | Revise | Boundary issue pending ADR-01; external dependency assumptions need explicit sign-off before they shape baseline | ADR-01 |
| S59 | Composite deletion is requested using data-subject identifier plus canister-group ID | Revise | Verifier/evidentiary issue pending ADR-03; orchestration input model should not be baseline protocol truth yet | ADR-03 |
| S60 | Service canister dispatches mktd_delete across canisters and collects certified-root evidence from each | Revise | Verifier/evidentiary issue pending ADR-03; may describe one future orchestration architecture, not core baseline | ADR-03 |
| S61 | Verifier can recompute the record position/path deterministically from the published key-hash and tree rules | Keep | Still valid | ADR-03 |
| S62 | Verification assumes one canonical published tree algorithm and serialisation profile across all implementations | Keep | Still valid | ADR-02 / ADR-03 |
| S63 | Verification flow is specified as if live network access is always available for certified-state corroboration | Revise | MKTd02 lessons learned; archival-first vs live corroboration needs explicit evidentiary scoping | ADR-03 |
| S64 | Old sheet treats inclusion/path verification and tombstone-hash verification as sufficient on their own for full evidentiary confidence | Revise | Verifier/evidentiary issue pending ADR-03; trust model must include certified-state and build provenance scope | ADR-03 |
| S65 | Public documentation set must include deletion algorithm, tombstone constant, tree rules, receipt schema, and verification procedure | Keep | Still valid | ADR-00 |
| S66 | Composite receipts are treated as a standard published verification surface rather than a higher-layer optional mechanism | Revise | Verifier/evidentiary issue pending ADR-03 | ADR-03 |
| S67 | Old sheet assumes specific service-canister retry/list/recovery flows belong in the protocol specification | Drop | Boundary issue pending ADR-01; orchestration operations are not baseline protocol truth | ADR-01 |
| S68 | Published interface surface should be formalised before implementation rather than inferred from example canisters | Keep | Still valid | formal interface phase |
| S69 | App-specific examples or payloads may be used to illustrate the design before the core interface is frozen | Drop | TinyPress contamination | spec refresh note |
| S70 | MKTd03 baseline should stay upstream of any later TinyPress integration or fixture naming | Keep | Still valid | none |
| S71 | Reuse from MKTd02 is permitted only as bounded analysis input, not as automatic inheritance of fields, formulas, or flows | Keep | Still valid | MKTd02 reuse/generalisation audit |
| S72 | Any audit finding that pressures ADR-01 or ADR-02 must trigger explicit sign-off and phase re-gate rather than silent drift | Keep | Still valid | spec refresh note |
