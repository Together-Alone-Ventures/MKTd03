## Assumptions

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
