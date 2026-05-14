## Verification

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
