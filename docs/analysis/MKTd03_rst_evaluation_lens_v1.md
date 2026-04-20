# MKTd03 Residual Trust Statement (RST) Evaluation Lens v1

## Status
Draft — analytical, non-normative.

## Date
2026-04-20

## Purpose
This note is an analytical lens for constructing and evaluating **Residual Trust Statements (RSTs)** about MKTd03. An RST is the companion statement to any claim about what MKTd03 proves: it names the trust that remains with parties, processes, and interpretation layers outside MKTd03's certified evidentiary surface, even after baseline verification succeeds.

The lens exists to make residual trust explicit rather than silently absorbed into overstated claim framings. It is an analytical tool, not a protocol requirement.

## Authority and non-authority
This note is **non-normative**. It does not define, constrain, or alter any MKTd03 protocol requirement, evidentiary scope, interface, companion rule, or verifier obligation.

The authoritative MKTd03 artefacts remain:
- ADR-00: Evidentiary Scope
- ADR-01: Library vs Adapter Boundary
- ADR-02: Tree-Structure Choice
- ADR-03: Tree-Mode CVDR Structure and Verifier Requirements
- Tree-Mode Invariants Note
- MKTd03 Protocol Refresh v1
- the formal interface files and their companion rules

Where this note and an approved ADR or interface artefact appear to conflict, the ADR or interface artefact wins. This note must not be cited as authority from any artefact under `docs/spec/`, `docs/planning/`, or `interfaces/`.

## 1. What a Residual Trust Statement is
A Residual Trust Statement names, for a given claim about MKTd03, the trust that:
- lies **outside** MKTd03's baseline evidentiary surface,
- is **not** collapsed or absorbed by MKTd03 verification succeeding,
- remains with the consumer, verifier, adapter/host, platform, build chain, or interpreter of the claim.

An RST does not identify protocol weaknesses. It identifies protocol **boundaries**. Everything outside those boundaries is residual trust.

## 2. Why the lens exists
A claim about what MKTd03 proves can be written in at least three framings:
- **Technical framing** — what the baseline evidence actually supports (e.g. that the declared transition is consistent with the canonical tree model and the published protocol/build anchors).
- **Interpretive framing** — what an audience might reasonably take the claim to mean in plain language.
- **Commercial or compliance framing** — what the claim suggests for a specific downstream purpose.

Each framing silently relies on residual trust that is different in shape. Without an explicit RST, a claim written in the technical framing is easily heard in a commercial framing — the gap between the two collapses into the consumer without warning.

The lens exists to make that gap legible.

## 3. Categories of residual trust relevant to MKTd03
Each category below names a **class** of trust that remains with parties outside MKTd03's certified surface. The list is not exhaustive. Worked examples are deliberately omitted from this version; categories are named generically.

### 3.1 Platform-layer residual trust
Trust in the correctness and integrity of the underlying ICP platform: consensus, subnet behaviour, certified-query mechanics, and the management-canister-reported module hash at the time of observation.

### 3.2 Build-chain residual trust
Trust in the reproducibility, determinism, and source-to-binary correspondence of the published build, independent of the module-hash material the receipt carries.

### 3.3 Adapter / host residual trust
Trust in the adapter's declaration of in-scope records and the host's scope-selection judgement — neither of which is certified by MKTd03 baseline.

### 3.4 Tree-construction residual trust
Trust in the correctness of the host's implementation of the canonical tree model — for example, that records were placed deterministically and that the structural distinction between empty and tombstoned positions was preserved — to the extent those facts are not themselves established by the artifact under review and its governing protocol/build anchors.

### 3.5 Issuance-context residual trust
Trust that issuance-context and export behaviour did not create a second, inconsistent representation of the evidence-bearing substance after the atomic issuance point described by ADR-03.

### 3.6 Interpretation residual trust
Trust in the mapping from "the declared transition is verified" to any higher-order interpretation — regulatory, contractual, workflow, or commercial. MKTd03 does not perform that mapping; the consumer or downstream framework does.

### 3.7 Scope-completeness residual trust
Trust that the set of records declared in-scope for a deletion transition is itself complete for the downstream purpose in question — which MKTd03 baseline explicitly does not certify.

## 4. Using the lens
For any claim about what MKTd03 proves:

1. State the claim in its **technical framing** first.
2. Identify which categories in §3 apply to the claim.
3. Write, for each applicable category, the residual-trust sentence that must accompany the claim to prevent silent collapse.
4. If the claim as written cannot coexist with those residual-trust sentences without appearing inconsistent, the claim is overreaching and must be re-framed, not re-worded.

This procedure is advisory. It produces a statement; it does not produce a certification.

## 5. What the lens is not
- It is **not** a compliance framework. It does not establish whether MKTd03 satisfies any regulatory, contractual, or jurisdictional requirement.
- It is **not** a verifier procedure. It does not alter what an MKTd03 verifier does or must do.
- It is **not** a protocol-level requirement. Nothing under `docs/spec/`, `docs/planning/`, or `interfaces/` depends on this note.
- It is **not** a comparison against other protocols or prior family members. Residual-trust categories are generic analytical frames, not a competitive-positioning device.
- It is **not** authoritative for claim framings outside MKTd03 itself. Statements about how MKTd03 is characterised in TAV standards, partner communications, or other operator-facing material are governed by their own review processes.

## 6. Cross-reference
Residual-trust categories above correspond to authoritative boundaries defined in the following artefacts. These references are for orientation only; this note does not restate or replace the referenced content.

- ADR-00 — baseline evidentiary scope and non-claims posture
- ADR-01 — library/adapter boundary and host responsibilities
- ADR-02 — canonical tree model and structural distinctions
- ADR-03 — "Explicit non-claims" section, including the interpretation-limit extension
- ADR-03 — "Issuance atomicity" section
- MKTd03 Protocol Refresh v1 — "Evidentiary scope" section
- MKTd03 Security / Privacy Note v1 — "Certification and provenance" section

## 7. Open follow-ups
- Worked examples of RSTs applied to specific MKTd03 claim framings may be added in later versions, provided they remain non-normative and do not import into `docs/spec/` or `docs/planning/`.
