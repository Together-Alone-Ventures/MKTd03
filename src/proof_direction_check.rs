use crate::proof_envelope::ProofEnvelope;
use crate::proof_frame::ProofDirection;

// staged for the future proof-verification slice; no production caller in S7-13.
#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ProofDirectionError {
    DirectionMismatch {
        frame_index: u16,
        expected: ProofDirection,
        actual: ProofDirection,
        key_bit: bool,
    },
}

fn expected_direction_for_key_bit(key_bit: bool) -> ProofDirection {
    if key_bit {
        ProofDirection::Right
    } else {
        ProofDirection::Left
    }
}

fn key_bit_msb_first(key: &[u8; 32], key_bit_index: usize) -> bool {
    let byte_index = key_bit_index / 8;
    let bit_index_in_byte = key_bit_index % 8;
    let mask = 1u8 << (7 - bit_index_in_byte);
    (key[byte_index] & mask) != 0
}

// staged for the future proof-verification slice; no production caller in S7-13.
#[allow(dead_code)]
pub(crate) fn validate_proof_directions(
    envelope: &ProofEnvelope,
    key: &[u8; 32],
) -> Result<(), ProofDirectionError> {
    debug_assert_eq!(envelope.frames.len(), 256);

    for (frame_index, frame) in envelope.frames.iter().enumerate() {
        // Joint convention:
        // - §9.1: parsed proof frames are ordered leaf-to-root.
        // - §9.4: record_position_key bits are consumed MSB-first root-to-leaf.
        // The index spaces therefore invert: key_bit_index = 255 - frame_index.
        let key_bit_index = 255 - frame_index;
        let key_bit = key_bit_msb_first(key, key_bit_index);
        let expected = expected_direction_for_key_bit(key_bit);
        let actual = frame.direction.clone();

        if actual != expected {
            return Err(ProofDirectionError::DirectionMismatch {
                frame_index: frame_index as u16,
                expected,
                actual,
                key_bit,
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proof_envelope::ProofEnvelope;
    use crate::proof_frame::ProofDirection::{Left, Right};
    use crate::proof_frame::{ProofFrame, ProofFrameSibling};

    fn frame(direction: ProofDirection) -> ProofFrame {
        ProofFrame {
            direction,
            sibling: ProofFrameSibling::CanonicalEmpty,
        }
    }

    fn envelope_from_directions(directions: [ProofDirection; 256]) -> ProofEnvelope {
        ProofEnvelope {
            frames: std::array::from_fn(|index| frame(directions[index].clone())),
        }
    }

    fn all_left_envelope() -> ProofEnvelope {
        envelope_from_directions(std::array::from_fn(|_| Left))
    }

    fn all_right_envelope() -> ProofEnvelope {
        envelope_from_directions(std::array::from_fn(|_| Right))
    }

    fn key_for_frame_right(frame_index: usize) -> [u8; 32] {
        let mut key = [0u8; 32];
        let key_bit_index = 255 - frame_index;
        let byte_index = key_bit_index / 8;
        let bit_index_in_byte = key_bit_index % 8;
        key[byte_index] |= 1u8 << (7 - bit_index_in_byte);
        key
    }

    fn alternating_key_and_directions() -> ([u8; 32], [ProofDirection; 256]) {
        let directions = std::array::from_fn(
            |frame_index| {
                if frame_index % 2 == 0 {
                    Left
                } else {
                    Right
                }
            },
        );
        let mut key = [0u8; 32];

        for frame_index in 0..256 {
            if directions[frame_index] == Right {
                let key_bit_index = 255 - frame_index;
                let byte_index = key_bit_index / 8;
                let bit_index_in_byte = key_bit_index % 8;
                key[byte_index] |= 1u8 << (7 - bit_index_in_byte);
            }
        }

        (key, directions)
    }

    #[test]
    fn all_zero_key_and_all_left_frames_pass() {
        let key = [0u8; 32];
        assert_eq!(
            validate_proof_directions(&all_left_envelope(), &key),
            Ok(())
        );
    }

    #[test]
    fn all_one_key_and_all_right_frames_pass() {
        let key = [0xffu8; 32];
        assert_eq!(
            validate_proof_directions(&all_right_envelope(), &key),
            Ok(())
        );
    }

    #[test]
    fn alternating_bit_pattern_passes() {
        let (key, directions) = alternating_key_and_directions();
        let envelope = envelope_from_directions(directions);
        assert_eq!(validate_proof_directions(&envelope, &key), Ok(()));
    }

    #[test]
    fn mismatch_at_frame_index_zero_fails() {
        let key = key_for_frame_right(0);
        let envelope = all_left_envelope();
        assert_eq!(
            validate_proof_directions(&envelope, &key),
            Err(ProofDirectionError::DirectionMismatch {
                frame_index: 0,
                expected: Right,
                actual: Left,
                key_bit: true,
            })
        );
    }

    #[test]
    fn mismatch_at_frame_index_255_fails() {
        let key = key_for_frame_right(255);
        let envelope = all_left_envelope();
        assert_eq!(
            validate_proof_directions(&envelope, &key),
            Err(ProofDirectionError::DirectionMismatch {
                frame_index: 255,
                expected: Right,
                actual: Left,
                key_bit: true,
            })
        );
    }

    #[test]
    fn mismatch_in_middle_fails() {
        let key = key_for_frame_right(128);
        let envelope = all_left_envelope();
        assert_eq!(
            validate_proof_directions(&envelope, &key),
            Err(ProofDirectionError::DirectionMismatch {
                frame_index: 128,
                expected: Right,
                actual: Left,
                key_bit: true,
            })
        );
    }

    #[test]
    fn error_variant_carries_exact_mismatch_details() {
        let key = key_for_frame_right(128);
        let envelope = all_left_envelope();
        assert_eq!(
            validate_proof_directions(&envelope, &key),
            Err(ProofDirectionError::DirectionMismatch {
                frame_index: 128,
                expected: Right,
                actual: Left,
                key_bit: true,
            })
        );
    }

    #[test]
    fn endpoint_inversion_frame_zero_rightmost_key_bit_set_passes() {
        let mut directions = std::array::from_fn(|_| Left);
        directions[0] = Right;
        let envelope = envelope_from_directions(directions);
        let mut key = [0u8; 32];
        key[31] = 0x01;
        assert_eq!(validate_proof_directions(&envelope, &key), Ok(()));
    }

    #[test]
    fn endpoint_inversion_frame_zero_wrong_direction_fails() {
        let envelope = all_left_envelope();
        let mut key = [0u8; 32];
        key[31] = 0x01;
        assert_eq!(
            validate_proof_directions(&envelope, &key),
            Err(ProofDirectionError::DirectionMismatch {
                frame_index: 0,
                expected: Right,
                actual: Left,
                key_bit: true,
            })
        );
    }

    #[test]
    fn endpoint_inversion_frame_255_leftmost_key_bit_set_passes() {
        let mut directions = std::array::from_fn(|_| Left);
        directions[255] = Right;
        let envelope = envelope_from_directions(directions);
        let mut key = [0u8; 32];
        key[0] = 0x80;
        assert_eq!(validate_proof_directions(&envelope, &key), Ok(()));
    }

    #[test]
    fn endpoint_inversion_frame_255_wrong_direction_fails() {
        let envelope = all_left_envelope();
        let mut key = [0u8; 32];
        key[0] = 0x80;
        assert_eq!(
            validate_proof_directions(&envelope, &key),
            Err(ProofDirectionError::DirectionMismatch {
                frame_index: 255,
                expected: Right,
                actual: Left,
                key_bit: true,
            })
        );
    }

    #[test]
    fn first_mismatch_posture_reports_earliest_frame() {
        let mut directions = std::array::from_fn(|_| Left);
        directions[5] = Left;
        directions[200] = Left;
        let envelope = envelope_from_directions(directions);

        let mut key = [0u8; 32];
        for frame_index in [5usize, 200usize] {
            let key_bit_index = 255 - frame_index;
            let byte_index = key_bit_index / 8;
            let bit_index_in_byte = key_bit_index % 8;
            key[byte_index] |= 1u8 << (7 - bit_index_in_byte);
        }

        assert_eq!(
            validate_proof_directions(&envelope, &key),
            Err(ProofDirectionError::DirectionMismatch {
                frame_index: 5,
                expected: Right,
                actual: Left,
                key_bit: true,
            })
        );
    }
}
