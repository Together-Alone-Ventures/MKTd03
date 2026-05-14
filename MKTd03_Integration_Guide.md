<!-- GENERATED FILE — do not edit directly. Source: docs/sections/ + docs/compose.yaml. See README.md for rebuild instructions. -->

# MKTd03 Integration Guide

**Version 1.0** · Updated 14 May 2026

> **Configuration:** MKTd | Library | ICP Tree mode | Embedded host API | Single host canister wrapper | Subject-based receipt retrieval | ICP certified-data orchestration

This guide covers the current package-readiness integration surface for MKTd03.
It is written for host canister developers embedding the `mktd03` crate
directly. The standalone MKTd03 canister is the reference host/wrapper, not the
preferred downstream integration boundary.

---

# 1. Prerequisites

MKTd03 currently assumes:

- Rust canister development with the `wasm32-unknown-unknown` target.
- `ic-stable-structures`-backed host storage.
- A host canister that can allocate three disjoint stable-memory handles.
- A host-controlled `module_hash` pipeline.
- A host-controlled certified-data flow:
  - publish `certified_commitment` after Phase A
  - call `data_certificate()` in query context before Phase B

MKTd03 is local-demo ready after S7-38.
Verifier-side BLS/root-key authentication remains post-handoff work.

---

# 2. Add the Crate

Add `mktd03` as a normal Rust dependency of the host canister:

```toml
[dependencies]
mktd03 = { path = "../MKTd03" }
```

The crate is designed to run in-process inside the host canister.
There is no required inter-canister call path, no separate receipt canister,
and no generated MKTd03 client/declaration requirement for embedded use.

---

# 3. Host Embedding Model

MKTd03 can now be embedded as a Rust crate by a host canister.

In this guide, "host" means the canister embedding the `mktd03` crate, not an
inter-canister caller. The standalone MKTd03 canister remains a reference
host/wrapper over the same protocol state and host API.

The core host-owned state type is:

```rust
MKTd03State::new(tree_storage, pending_issuance_storage, issued_receipts_storage)
```

The host owns:

- storage allocation
- `module_hash`
- certified-data side effects
- `data_certificate()` retrieval in query context

`MKTd03State` itself makes no `ic_cdk` calls.

---

# 4. Host Responsibilities

The embedding host must:

- allocate three disjoint `Memory` handles
- retain `pending_id` across Phase A/B/C
- maintain host-owned `module_hash`
- publish `certified_commitment` with `certified_data_set` after Phase A
- call `data_certificate()` in query context before Phase B
- decide how to surface local-demo caveats to downstream consumers
- handle the current single-pending limitation operationally

Corrupt or overlapping host storage is a host integration failure, not a
protocol feature.

---

# 5. Host API Flow

The host-side issuance flow is:

```rust
let phase_a = state.host_begin_phase_a(
    &module_hash,
    HostPhaseAInputs {
        subject_reference,
        scope_reference,
        transition_material,
        deletion_state_material,
    },
)?;

let phase_b = state.host_get_phase_b(HostPhaseBInputs {
    pending_id: phase_a.pending_id.clone(),
    host_data_certificate,
})?;

let phase_c = state.host_finalize_phase_c(
    &module_hash,
    HostPhaseCInputs {
        pending_id: phase_a.pending_id,
        certificate_material: phase_b.certificate_material,
    },
)?;

let receipt = state.host_get_receipt(HostReceiptLookupInputs {
    subject_reference,
});
```

Semantically:

- Phase A creates pending issuance state and returns `pending_id` plus
  `certified_commitment`.
- Phase B binds host-supplied certificate bytes to the pending issuance.
- Phase C finalizes the receipt and clears pending state on success.
- Receipt lookup remains subject-reference based.

---

# 6. Stable Memory Layout

Embedded MKTd03 requires three disjoint storage handles:

- issuance tree storage
- pending issuance storage
- issued receipts storage

The type system does not enforce disjointness.
If the host passes overlapping handles to `MKTd03State::new(...)`, protocol
state may be corrupted.

`MKTd03State::new(...)` follows `ic-stable-structures` behavior.
Corrupt memory may panic or fail loud during construction, matching the
standalone canister's fatal-init posture.

---

# 7. Receipt Retrieval

MKTd03 receipt lookup is subject-reference based:

```rust
state.host_get_receipt(HostReceiptLookupInputs { subject_reference })
```

The current host lookup result uses `library::ReceiptResult` semantics:

- `Ok { receipt }`
- `Err { error_code: InvalidSubjectReference }`
- `Err { error_code: NotYetIssued }`
- `Err { error_code: NotFound }`

There is no separate receipt-id scheme in the current host API.

---

# 8. Known Limits

- Single-pending issuance only.
- No cancel/timeout for abandoned pending issuance.
- Stable-cell write-error propagation is not yet production-grade.
- `StorageUnavailable` should be treated as a stable-storage failure/corruption
  condition in host-embedding context.
- Verifier-side BLS authentication against the IC root key remains post-handoff.
- The current package-readiness state is local-demo oriented; production/handoff
  hardening still requires additional storage, provenance, and verifier work.

---

# 9. Assumptions

MKTd03 assumes:

- the host assigns `subject_reference` and optional `scope_reference`
  consistently with the product's subject model
- the host computes `transition_material` correctly for its own transition
  semantics
- the host chooses the correct `deletion_state_material`
- the host controls `module_hash` input honestly
- the host preserves the A→B→C sequencing correctly

MKTd03 proves protocol-structured state transition evidence.
It does not by itself prove that a product mapped every relevant PII field or
business state correctly.

---

# 10. Verification

The verifier scope for MKTd03 should be read as four related checks:

| Check | Scope |
|---|---|
| V1 | Receipt hash-consistency and protocol-formula checks |
| V2 | ICP certificate/BLS-path checks and certified-data commitment checks |
| V3 | Module-hash provenance checks |
| V4 | Live state/tombstone consistency checks where the host/application still exists |

Current status after S7-38:

- MKTd03 issues host-embeddable pending and finalized receipts.
- Finalized receipts embed certificate material and module hash provenance data.
- Verifier-side BLS/root-key authentication remains post-handoff work.
- Local-demo flows should not be overstated as mainnet-grade verification.

Verification output should be read as protocol/evidence verification, not as a
complete audit of host application behavior or PII field selection.

---

# 11. Module Hash and Certified Data

The host owns `module_hash` and supplies it to:

- `host_begin_phase_a`
- `host_finalize_phase_c`

The host also owns certified-data side effects:

- after Phase A, publish `certified_commitment`
- before Phase B, call `data_certificate()` in query context and pass the bytes
  as `host_data_certificate`

As with MKTd02, a canister should not try to derive its own deployed module hash
at runtime. The clean model is host/deployer ownership of the hash input.

The trust model is therefore:

- protocol logic proves how module hash and certificate material are embedded
- host/deployer process determines whether the supplied module hash and
  certified-data publication are operationally trustworthy

---

# 12. Residual Trust Statement

A MKTd03 receipt is a cryptographic proof of a Tree-mode state transition within
a defined system boundary. It can prove that a host application executed a
deletion-mode transition, bound that transition to MKTd03 receipt structure, and
persisted the result through the MKTd03 protocol flow. That does not eliminate
all trust outside the protocol boundary.

A finalized MKTd03 receipt can prove:

1. a deletion-mode transition was recorded at a specific time;
2. the transition evidence is bound to a specific protocol-versioned receipt;
3. the receipt includes host-supplied module-hash and certification material;
4. the transition is tied to a specific `subject_reference` and optional
   `scope_reference` chosen by the host/application.

It does not by itself prove:

- that the host application selected the right subject or scope;
- that every relevant PII field was included in the pre/post-state derivation;
- that external copies, backups, logs, or downstream processors were also erased;
- that local-demo certificate material carries production-grade BLS trust;
- that abandoned pending issuances are automatically recoverable.

### Residual Trust Assumptions

| # | Assumption | Mitigation | Severity |
|---|---|---|---|
| **RT1** | Host/application assigns `subject_reference` and `scope_reference` correctly. | Product-specific adapter/source review is still required. The protocol verifies structured evidence, not business identity mapping. | High |
| **RT2** | Host/application derives transition input material correctly. | `transition_material`, pre/post-state derivation, and deletion-state choice remain host/application responsibilities. | High |
| **RT3** | Host/deployer supplies the correct `module_hash`. | Module-hash provenance must be supported by deployment/build records and reproducible build practice. | Medium |
| **RT4** | Certified-data publication and certificate retrieval are performed honestly by the host. | The protocol defines the A→B→C structure, but host-side ICP orchestration remains an integration responsibility. | Medium |
| **RT5** | Local-demo certificate flows are not misrepresented as production-grade BLS assurance. | Mainnet-grade verifier-side BLS/root-key authentication remains post-handoff work and must be stated clearly. | High |
| **RT6** | Single-pending issuance operational limits are managed safely. | An abandoned pending issuance blocks later issuance until host/application recovery work exists. | Medium |
| **RT7** | Stable-memory corruption or write failure does not silently distort protocol state. | Current StableCell write-error propagation is not yet production-grade; storage failure should be treated as fail-loud territory. | Medium |

### Scope of Verification

Verifier outputs can confirm protocol/evidence properties of the receipt and its
related on-chain material. They do not by themselves confirm complete product
PII mapping, external data-destruction controls, or governance/process honesty.

### Guidance for Integrators and Relying Parties

- Publish clear product-specific rules for subject/scope assignment.
- Publish or retain deployment/build provenance for `module_hash`.
- Distinguish local-demo verification from production/mainnet trust claims.
- Document the single-pending limitation operationally.
- Treat host adapter/source review as part of the evidentiary package, not as an optional extra.
