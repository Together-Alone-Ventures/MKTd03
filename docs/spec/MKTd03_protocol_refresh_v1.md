# MKTd03 Protocol Refresh v1

## Status
Draft

## Date
2026-03-26

## Purpose
Restate the current baseline MKTd03 protocol position after Phase 1–3 ADR closure, using the approved ADRs as authority.

This document is a consolidated protocol/spec refresh artifact.
It does not replace ADR authority.
Its purpose is to give one readable current-state protocol view before later interface files, companion rules, and vectors are drafted.

## Authority

This document is subordinate to:
- ADR-00: Evidentiary Scope
- ADR-01: Library vs Adapter Boundary
- ADR-02: Tree-Structure Choice
- ADR-03: Tree-Mode CVDR Structure and Verifier Requirements
- Tree-Mode Invariants Note

ADR-03 is currently an intermediate draft. Where this refresh relies on ADR-03 content beyond its already-reviewed baseline decisions, that content remains provisional pending ADR-03 completion.
Where this document and an ADR differ, the ADR wins.

## Current Baseline Position

### 1. Protocol scope
MKTd03 is the dApp-agnostic Tree-mode protocol/library line.
It is not TinyPress-specific and must not inherit TinyPress routes, payloads, fixtures, or naming into protocol truth.

### 2. Evidentiary scope
Baseline MKTd03 evidentiary scope is conservative and archival-first.
Baseline Tree-mode CVDR verification must not depend on live fetch as a mandatory first step.
Baseline scope does not, by default, claim whole-application deletion completeness, cross-canister completion beyond the single baseline receipt scope, or service-canister orchestration truth, and does not treat baseline verification of a declared transition as equivalent to broader completeness properties of the host deployment.
The full baseline non-claims interpretation is governed by ADR-03's explicit non-claims section. Where this section and ADR-03 differ, ADR-03 wins.

### 3. Tree structure
Baseline Tree mode uses one canonical binary Merkle tree model with deterministic record placement, explicit hash-role/domain separation, and fixed-capacity semantics for each ready tree instance.
- structural distinction between empty and tombstoned positions,
- rebuild/replacement rather than implicit in-place resize.

### 4. Operational invariants
Tree mode must not produce deletion evidence from incomplete or structurally stale tree state.
Initialisation/rebuild freeze, resumable progress discipline, explicit rebuild triggers, no silent queueing, rebuild completion that must verify its output against current live host state before transitioning to ready, and externally legible readiness state are baseline invariants.

### 5. Boundary model
Baseline MKTd03 uses a library-plus-host model with a narrow adapter seam.
The library owns protocol-critical Tree-mode logic and protocol predicates.
The host owns application storage, authority policy, scheduling, external interface exposure, and the operational readiness state machine.
The adapter is narrow and protocol-boundary only; it is not an orchestration or service-layer surface.

### 6. CVDR baseline
Baseline Tree-mode CVDR is self-contained at issuance for baseline verification purposes.
It binds together:
- target record identifier or canonical record-position reference,
- pre-state and post-state tree commitments,
- tree-position proof material,
- deletion-state semantics, preserving the structural distinction from empty-position state per ADR-02,
- ICP certification material,
- build/module provenance material.

Tree proof alone is insufficient without deletion-state semantics.
Deletion-state semantics alone are insufficient without the corresponding tree-bound transition evidence.

### 7. Explicit baseline exclusions
Baseline MKTd03 does not include by default:
- service-canister registration/topology architecture,
- composite deletion orchestration,
- retry/list/recovery workflow semantics,
- app-shaped endpoint suites,
- live-network availability as a prerequisite for baseline verification.

## Current Open Follow-On Work
- verify this refresh wording against final approved ADR text at session close
- MKTd02 reuse/generalisation audit
- diagnostics/versioning/security/privacy notes
- later formal interface artifacts
- companion rules and golden vectors

## Source ADRs
- ADR-00
- ADR-01
- ADR-02
- ADR-03
- Tree-Mode Invariants Note
