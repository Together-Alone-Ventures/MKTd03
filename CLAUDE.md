# Repository Guidance — MKTd03

## Scope

MKTd03 is the repository for tree-mode CVDR / Zombie Delete protocol work, including architecture, integration guidance, product documentation, and related technical analysis.

The repository is intended to remain application-agnostic and reusable across multiple downstream integration contexts.

## In scope

This repository is the appropriate home for:

- protocol and architecture documentation
- integration guidance
- generic verifier and evidence-model discussion
- product-family positioning and commercial framing
- application-agnostic technical analysis

## Out of scope

Application-specific implementation materials should remain in their own repositories. Where downstream applications are referenced here, they should be treated as examples or integration contexts rather than as defining assumptions for MKTd03 itself.

## Relationship to downstream applications

TinyPress and other downstream applications may be referenced where they provide useful implementation context. Such references should remain brief and should not make MKTd03 documentation application-specific.

## Standards and continuity

Shared engineering standards and reusable cross-repository workflow guidance should be maintained centrally in TAV-Engineering-Standards.

For repository-local continuity, use:

- `RESTART_PACK.md`
- `MILESTONE_LOG.md`

## Writing guidance

Prefer precise, evidence-backed wording. Distinguish clearly between current implementation, integration-specific examples, and broader architectural capability.
