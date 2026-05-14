## Residual Trust Statement

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
