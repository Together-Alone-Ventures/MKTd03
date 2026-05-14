**Sync note:** `docs/sections/12-residual-trust.md` is the source of truth for the residual trust table and mitigations. This document is the auditor/DPO-facing version of the same material. If `12-residual-trust.md` is updated, this document must be updated to match.

# MKTd03 Residual Trust Statement

**Protocol:** MKTd03 Tree mode  
**Issued by:** Together Alone Ventures OÜ  
**Audience:** Data Protection Officers, legal counsel, compliance auditors, relying parties  
**Repository:** This document is published in the MKTd03 repository as the trust-boundary statement for the current package-ready protocol surface. For technical integration details, see `MKTd03_Integration_Guide.md` in the repository root and the source documentation under `docs/sections/`.

---

## Purpose

This document defines what a current MKTd03 receipt proves, what it does not
prove, and what residual trust assumptions remain with the host application,
deployment process, and verification environment.

It is written in the same practical style as the MKTd02 residual trust
statement and should be read as a boundary statement, not as a marketing claim.

---

## What a MKTd03 Receipt Proves

A current MKTd03 receipt is a cryptographic proof of a Tree-mode state
transition within a defined system boundary. Subject to the caveats below, a
finalized receipt proves the following:

**1. A deletion-mode transition was recorded at a specific time.**  
The receipt records a state transition tied to a specific `subject_reference`
and optional `scope_reference`.

**2. The transition evidence is cryptographically bound to the receipt.**  
The receipt structure binds protocol-versioned transition material, state
commitments, and proof material into one verifier-facing artifact.

**3. Module-hash and certification material were embedded into the finalized receipt.**  
The current MKTd03 flow binds host-supplied `module_hash` and host-supplied
certificate material into the receipt structure.

**4. Receipt issuance followed the MKTd03 A→B→C protocol shape.**  
Pending issuance, certificate-material retrieval, and finalization are
structurally separated in a way that matches ICP query/update constraints.

---

## What a MKTd03 Receipt Does Not Prove

A current MKTd03 receipt does not prove the following:

- That the host application assigned the correct `subject_reference` or
  `scope_reference`.
- That the host application selected and derived every relevant pre/post-state
  input correctly.
- That all external copies, logs, backups, or downstream processors were also
  erased.
- That local-demo certificate material carries production-grade BLS trust.
- That verifier-side BLS/root-key authentication has already been completed in
  the current package-ready surface.
- That abandoned pending issuances are automatically recoverable or timed out.
- That stable-storage corruption or write failures are impossible.

The appropriate framing is: MKTd03 provides strong structured evidence of a
Tree-mode deletion transition inside the protocol boundary, but host identity
mapping, host state selection, deployment provenance, and verification
environment trust still matter.

---

## Residual Trust Assumptions

| # | Assumption | Mitigation | Severity |
|---|---|---|---|
| **RT1** | The host/application assigns `subject_reference` and optional `scope_reference` correctly. | Product-specific adapter/source review is still required. The protocol verifies structured evidence, not business identity mapping. | High |
| **RT2** | The host/application derives transition input material correctly. | `transition_material`, pre/post-state derivation, and deletion-state selection remain host/application responsibilities and should be documented and reviewable. | High |
| **RT3** | The host/deployer supplies the correct `module_hash`. | Module-hash provenance should be backed by deployment/build records and reproducible build practice. | Medium |
| **RT4** | Certified-data publication and certificate retrieval are performed honestly by the host. | The protocol defines the A→B→C structure, but host-side ICP orchestration remains an integration responsibility. | Medium |
| **RT5** | Local-demo or partial-verifier flows are not misrepresented as production-grade BLS assurance. | Verifier-side BLS/root-key authentication remains post-handoff and must be stated explicitly. | High |
| **RT6** | The single-pending limitation is managed safely by the host/application. | Operational recovery rules are needed because abandoned pending issuance blocks later issuance. | Medium |
| **RT7** | Stable-memory corruption or write failure does not silently distort protocol state. | Current StableCell write-error propagation is not yet production-grade; storage failure should be treated as fail-loud territory. | Medium |

---

## Guidance for Integrators and Relying Parties

- Publish clear product-specific rules for subject/scope assignment.
- Publish or retain deployment/build provenance for `module_hash`.
- Distinguish local-demo verification from production/mainnet trust claims.
- Document the single-pending limitation operationally.
- Treat host adapter/source review as part of the evidentiary package, not as
  an optional extra.

---

## Disclaimer

This document is produced by Together Alone Ventures OÜ as a reference
statement describing the trust properties of the current MKTd03 package-ready
surface. It does not constitute legal advice and should not be relied upon as
such. The suitability of MKTd03 receipts for any particular regulatory or
commercial purpose is a matter for the data controller and their legal counsel
to determine.
