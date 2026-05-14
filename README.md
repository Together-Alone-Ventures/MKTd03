# MKTd03

MKTd03 is a dApp-agnostic GDPR deletion-proof protocol project for ICP Tree-mode applications.

It is not a TinyPress-specific engine.

## Scope

This repo is for:
- protocol-engine design
- adapter and integration design
- deletion-boundary doctrine
- enterprise and product-targeting material
- repo-level continuity for the MKTd03 project itself

This repo is not the live home of TinyPress application code or TinyPress app-governance documents.

## Relationship to TinyPress

TinyPress is the first reference target for MKTd03 integration work.

The live TinyPress app now lives in the separate TinyPress repo:
- Together-Alone-Ventures/TinyPress

Early TinyPress scaffolding and related artifacts began life under MKTd03. Any retained copies in this repo are historical only and must not be treated as the live app surface.

## Status

Repo-boundary cleanup is complete.

MKTd03 now has a host-embeddable issuance API after S7-38.
The standalone canister remains a reference host/wrapper over the same protocol
state and host API.

## Documentation Entry Points

- `MKTd03_Integration_Guide.md` — generated package-readiness and integration guide
- `RESIDUAL_TRUST_STATEMENT.md` — auditor/DPO-facing trust-boundary statement
- `docs/sections/` — source modules for the generated guide and synced trust material
