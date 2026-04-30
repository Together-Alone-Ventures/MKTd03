use crate::library::DeletionStateMaterial;

// staged for future receipt/verifier structural validation; no production caller in S7-14.
#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum DeletionStateMaterialError {
    EmptyPositionVariantNotPermittedInV1,
    TombstonedPositionUnexpectedBytes { found: Vec<u8> },
}

// staged for future receipt/verifier structural validation; no production caller in S7-14.
#[allow(dead_code)]
pub(crate) fn validate_deletion_state_material(
    material: &DeletionStateMaterial,
) -> Result<(), DeletionStateMaterialError> {
    // SPEC §10.4: any future expansion beyond the single-byte b"\x01" tombstone marker
    // requires explicit versioned change control. Do not relax this validator's
    // accepted-byte set without a corresponding versioned spec amendment.
    match material {
        DeletionStateMaterial::TombstonedPosition(bytes) if bytes == &[0x01] => Ok(()),
        DeletionStateMaterial::TombstonedPosition(bytes) => Err(
            DeletionStateMaterialError::TombstonedPositionUnexpectedBytes {
                found: bytes.clone(),
            },
        ),
        DeletionStateMaterial::EmptyPosition(_) => {
            Err(DeletionStateMaterialError::EmptyPositionVariantNotPermittedInV1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tombstoned_position_single_byte_marker_passes() {
        let material = DeletionStateMaterial::TombstonedPosition(vec![0x01]);
        assert_eq!(validate_deletion_state_material(&material), Ok(()));
    }

    #[test]
    fn tombstoned_position_empty_payload_fails() {
        let material = DeletionStateMaterial::TombstonedPosition(vec![]);
        assert_eq!(
            validate_deletion_state_material(&material),
            Err(DeletionStateMaterialError::TombstonedPositionUnexpectedBytes { found: vec![] })
        );
    }

    #[test]
    fn tombstoned_position_zero_byte_fails() {
        let material = DeletionStateMaterial::TombstonedPosition(vec![0x00]);
        assert_eq!(
            validate_deletion_state_material(&material),
            Err(
                DeletionStateMaterialError::TombstonedPositionUnexpectedBytes { found: vec![0x00] }
            )
        );
    }

    #[test]
    fn tombstoned_position_non_marker_byte_fails() {
        let material = DeletionStateMaterial::TombstonedPosition(vec![0x02]);
        assert_eq!(
            validate_deletion_state_material(&material),
            Err(
                DeletionStateMaterialError::TombstonedPositionUnexpectedBytes { found: vec![0x02] }
            )
        );
    }

    #[test]
    fn tombstoned_position_repeated_marker_bytes_fail() {
        let material = DeletionStateMaterial::TombstonedPosition(vec![0x01, 0x01]);
        assert_eq!(
            validate_deletion_state_material(&material),
            Err(
                DeletionStateMaterialError::TombstonedPositionUnexpectedBytes {
                    found: vec![0x01, 0x01],
                }
            )
        );
    }

    #[test]
    fn tombstoned_position_marker_plus_zero_byte_fails() {
        let material = DeletionStateMaterial::TombstonedPosition(vec![0x01, 0x00]);
        assert_eq!(
            validate_deletion_state_material(&material),
            Err(
                DeletionStateMaterialError::TombstonedPositionUnexpectedBytes {
                    found: vec![0x01, 0x00],
                }
            )
        );
    }

    #[test]
    fn empty_position_empty_payload_fails() {
        let material = DeletionStateMaterial::EmptyPosition(vec![]);
        assert_eq!(
            validate_deletion_state_material(&material),
            Err(DeletionStateMaterialError::EmptyPositionVariantNotPermittedInV1)
        );
    }

    #[test]
    fn empty_position_marker_payload_fails() {
        let material = DeletionStateMaterial::EmptyPosition(vec![0x01]);
        assert_eq!(
            validate_deletion_state_material(&material),
            Err(DeletionStateMaterialError::EmptyPositionVariantNotPermittedInV1)
        );
    }

    #[test]
    fn empty_position_thirty_two_byte_payload_fails() {
        let material = DeletionStateMaterial::EmptyPosition(vec![0x42; 32]);
        assert_eq!(
            validate_deletion_state_material(&material),
            Err(DeletionStateMaterialError::EmptyPositionVariantNotPermittedInV1)
        );
    }
}
