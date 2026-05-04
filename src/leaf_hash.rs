use crate::hashing::hash_with_tag;
use crate::scope_encoding::{encode_scope_reference, ScopeEncodingError};
use crate::tags::TAG_LEAF;

const EMPTY_LEAF_DISCRIMINANT: [u8; 1] = [0x00];
const OCCUPIED_LEAF_DISCRIMINANT: [u8; 1] = [0x01];
const TOMBSTONED_LEAF_DISCRIMINANT: [u8; 1] = [0x02];
const TOMBSTONED_DELETION_STATE_MATERIAL_V1: [u8; 1] = [0x01];

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum LeafHashError {
    EmptySubjectReference,
    EmptyScopeReference,
    InvalidDeletionStateMaterial(Vec<u8>),
}

impl LeafHashError {
    fn message(&self) -> String {
        match self {
            Self::EmptySubjectReference => {
                "S7-5 leaf hash requires non-empty subject_reference.".to_string()
            }
            Self::EmptyScopeReference => {
                "S7-5 leaf hash scope_reference must not be Some(empty bytes).".to_string()
            }
            Self::InvalidDeletionStateMaterial(bytes) => format!(
                "S7-5 tombstoned leaf requires deletion_state_material == [0x01]; found {:02x?}.",
                bytes
            ),
        }
    }
}

fn trap_on_error<T>(result: Result<T, LeafHashError>) -> T {
    match result {
        Ok(value) => value,
        Err(error) => ic_cdk::trap(&error.message()),
    }
}

fn encode_leaf_scope_reference(scope_reference: Option<&[u8]>) -> Result<Vec<u8>, LeafHashError> {
    encode_scope_reference(scope_reference).map_err(|error| match error {
        ScopeEncodingError::EmptyScopeReference => LeafHashError::EmptyScopeReference,
    })
}

fn validate_subject_reference(subject_reference: &[u8]) -> Result<(), LeafHashError> {
    if subject_reference.is_empty() {
        return Err(LeafHashError::EmptySubjectReference);
    }
    Ok(())
}

fn validate_deletion_state_material(deletion_state_material: &[u8]) -> Result<(), LeafHashError> {
    if deletion_state_material != TOMBSTONED_DELETION_STATE_MATERIAL_V1 {
        return Err(LeafHashError::InvalidDeletionStateMaterial(
            deletion_state_material.to_vec(),
        ));
    }
    Ok(())
}

fn compute_occupied_leaf(
    subject_reference: &[u8],
    scope_reference: Option<&[u8]>,
    transition_material: &[u8; 32],
) -> Result<[u8; 32], LeafHashError> {
    validate_subject_reference(subject_reference)?;
    let encoded_scope = encode_leaf_scope_reference(scope_reference)?;
    Ok(hash_with_tag(
        TAG_LEAF,
        &[
            &OCCUPIED_LEAF_DISCRIMINANT,
            subject_reference,
            &encoded_scope,
            transition_material,
        ],
    ))
}

pub(crate) fn compute_tombstoned_leaf(
    subject_reference: &[u8],
    scope_reference: Option<&[u8]>,
    deletion_state_material: &[u8],
) -> Result<[u8; 32], LeafHashError> {
    validate_subject_reference(subject_reference)?;
    let encoded_scope = encode_leaf_scope_reference(scope_reference)?;
    validate_deletion_state_material(deletion_state_material)?;
    Ok(hash_with_tag(
        TAG_LEAF,
        &[
            &TOMBSTONED_LEAF_DISCRIMINANT,
            subject_reference,
            &encoded_scope,
            deletion_state_material,
        ],
    ))
}

/// Computes only the leaf hash. The caller is responsible for placing the
/// resulting hash at the canonical terminal position derived from
/// record_position_key per spec §4.7. Position derivation is deferred to a
/// future slice.
pub fn hash_empty_leaf() -> [u8; 32] {
    hash_with_tag(TAG_LEAF, &[&EMPTY_LEAF_DISCRIMINANT])
}

/// Computes only the leaf hash. The caller is responsible for placing the
/// resulting hash at the canonical terminal position derived from
/// record_position_key per spec §4.7. Position derivation is deferred to a
/// future slice.
pub fn hash_occupied_leaf(
    subject_reference: &[u8],
    scope_reference: Option<&[u8]>,
    transition_material: &[u8; 32],
) -> [u8; 32] {
    trap_on_error(compute_occupied_leaf(
        subject_reference,
        scope_reference,
        transition_material,
    ))
}

/// Computes only the leaf hash. The caller is responsible for placing the
/// resulting hash at the canonical terminal position derived from
/// record_position_key per spec §4.7. Position derivation is deferred to a
/// future slice.
pub fn hash_tombstoned_leaf(
    subject_reference: &[u8],
    scope_reference: Option<&[u8]>,
    deletion_state_material: &[u8],
) -> [u8; 32] {
    trap_on_error(compute_tombstoned_leaf(
        subject_reference,
        scope_reference,
        deletion_state_material,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED_EMPTY_LEAF_HASH_HEX: &str =
        "c1e1013414a35a60a28d73920ee54dd84a1ae14356498411fb3bfa38d341a747";
    const EXPECTED_OCCUPIED_LEAF_HASH_HEX: &str =
        "8b53027585ef4b88bd4ebc918778a9fb7c019fffe9b18b09b0ed64c5b3f8c27b";
    const EXPECTED_TOMBSTONED_LEAF_HASH_HEX: &str =
        "da7ec83d89d1c8648244362ea4c07a8153348acad4cb8561727ff3836218fff8";

    fn subject_reference() -> [u8; 32] {
        [0x42; 32]
    }

    fn transition_material() -> [u8; 32] {
        [0x00; 32]
    }

    fn to_hex(bytes: [u8; 32]) -> String {
        bytes.iter().map(|byte| format!("{byte:02x}")).collect()
    }

    #[test]
    fn hash_empty_leaf_matches_pinned_golden_vector() {
        assert_eq!(to_hex(hash_empty_leaf()), EXPECTED_EMPTY_LEAF_HASH_HEX);
    }

    #[test]
    fn hash_occupied_leaf_matches_pinned_golden_vector() {
        assert_eq!(
            to_hex(hash_occupied_leaf(
                &subject_reference(),
                None,
                &transition_material()
            )),
            EXPECTED_OCCUPIED_LEAF_HASH_HEX
        );
    }

    #[test]
    fn hash_tombstoned_leaf_matches_pinned_golden_vector() {
        assert_eq!(
            to_hex(hash_tombstoned_leaf(&subject_reference(), None, b"\x01")),
            EXPECTED_TOMBSTONED_LEAF_HASH_HEX
        );
    }

    #[test]
    fn occupied_leaf_differs_from_empty_leaf() {
        assert_ne!(
            hash_occupied_leaf(&subject_reference(), None, &transition_material()),
            hash_empty_leaf()
        );
    }

    #[test]
    fn tombstoned_leaf_differs_from_empty_leaf() {
        assert_ne!(
            hash_tombstoned_leaf(&subject_reference(), None, b"\x01"),
            hash_empty_leaf()
        );
    }

    #[test]
    fn tombstoned_leaf_differs_from_occupied_leaf() {
        assert_ne!(
            hash_tombstoned_leaf(&subject_reference(), None, b"\x01"),
            hash_occupied_leaf(&subject_reference(), None, &transition_material())
        );
    }

    #[test]
    fn none_scope_and_some_scope_produce_different_occupied_hashes() {
        assert_ne!(
            hash_occupied_leaf(&subject_reference(), None, &transition_material()),
            hash_occupied_leaf(&subject_reference(), Some(b"x"), &transition_material())
        );
    }

    #[test]
    fn empty_some_scope_fails_loud() {
        let error = compute_occupied_leaf(&subject_reference(), Some(b""), &transition_material())
            .expect_err("empty scope should fail loud");
        assert_eq!(
            error.message(),
            "S7-5 leaf hash scope_reference must not be Some(empty bytes)."
        );
    }

    #[test]
    fn empty_subject_reference_fails_loud() {
        let error = compute_occupied_leaf(b"", None, &transition_material())
            .expect_err("empty subject should fail loud");
        assert_eq!(
            error.message(),
            "S7-5 leaf hash requires non-empty subject_reference."
        );
    }

    #[test]
    fn wrong_length_deletion_state_material_fails_loud() {
        let wrong_value = compute_tombstoned_leaf(&subject_reference(), None, b"\x02")
            .expect_err("wrong single-byte deletion state should fail loud");
        assert_eq!(
            wrong_value.message(),
            "S7-5 tombstoned leaf requires deletion_state_material == [0x01]; found [02]."
        );

        let wrong_length = compute_tombstoned_leaf(&subject_reference(), None, b"\x01\x02")
            .expect_err("wrong-length deletion state should fail loud");
        assert_eq!(
            wrong_length.message(),
            "S7-5 tombstoned leaf requires deletion_state_material == [0x01]; found [01, 02]."
        );
    }
}
