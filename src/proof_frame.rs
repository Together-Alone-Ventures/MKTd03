#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProofDirection {
    /// Direction byte 0x00.
    /// The current node is the left child and the sibling is on the right.
    /// Future root-walking hash order: hash(current, sibling).
    Left,

    /// Direction byte 0x01.
    /// The current node is the right child and the sibling is on the left.
    /// Future root-walking hash order: hash(sibling, current).
    Right,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProofFrameSibling {
    Explicit([u8; 32]),
    CanonicalEmpty,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProofFrame {
    pub direction: ProofDirection,
    pub sibling: ProofFrameSibling,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProofFrameError {
    Empty,
    UnknownDirectionByte(u8),
    UnknownSiblingKindByte(u8),
    TruncatedInput { needed: usize, available: usize },
}

pub fn serialize_proof_frame(frame: &ProofFrame) -> Vec<u8> {
    let direction_byte = match frame.direction {
        ProofDirection::Left => 0x00,
        ProofDirection::Right => 0x01,
    };

    match frame.sibling {
        ProofFrameSibling::Explicit(sibling_hash) => {
            let mut bytes = Vec::with_capacity(34);
            bytes.push(direction_byte);
            bytes.push(0x00);
            bytes.extend_from_slice(&sibling_hash);
            bytes
        }
        ProofFrameSibling::CanonicalEmpty => vec![direction_byte, 0x01],
    }
}

pub fn parse_proof_frame(bytes: &[u8]) -> Result<(ProofFrame, usize), ProofFrameError> {
    if bytes.is_empty() {
        return Err(ProofFrameError::Empty);
    }

    let direction = match bytes[0] {
        0x00 => ProofDirection::Left,
        0x01 => ProofDirection::Right,
        other => return Err(ProofFrameError::UnknownDirectionByte(other)),
    };

    if bytes.len() < 2 {
        return Err(ProofFrameError::TruncatedInput {
            needed: 1,
            available: 0,
        });
    }

    match bytes[1] {
        0x00 => {
            let available = bytes.len().saturating_sub(2);
            if available < 32 {
                return Err(ProofFrameError::TruncatedInput {
                    needed: 32,
                    available,
                });
            }

            let mut sibling_hash = [0u8; 32];
            sibling_hash.copy_from_slice(&bytes[2..34]);
            Ok((
                ProofFrame {
                    direction,
                    sibling: ProofFrameSibling::Explicit(sibling_hash),
                },
                34,
            ))
        }
        0x01 => Ok((
            ProofFrame {
                direction,
                sibling: ProofFrameSibling::CanonicalEmpty,
            },
            2,
        )),
        other => Err(ProofFrameError::UnknownSiblingKindByte(other)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn explicit_42_frame(direction: ProofDirection) -> ProofFrame {
        ProofFrame {
            direction,
            sibling: ProofFrameSibling::Explicit([0x42; 32]),
        }
    }

    fn canonical_empty_frame(direction: ProofDirection) -> ProofFrame {
        ProofFrame {
            direction,
            sibling: ProofFrameSibling::CanonicalEmpty,
        }
    }

    #[test]
    fn serialize_left_explicit_frame_matches_pinned_vector() {
        let bytes = serialize_proof_frame(&explicit_42_frame(ProofDirection::Left));
        assert_eq!(bytes, [vec![0x00, 0x00], vec![0x42; 32]].concat());
        assert_eq!(bytes.len(), 34);
    }

    #[test]
    fn serialize_right_explicit_frame_matches_pinned_vector() {
        let bytes = serialize_proof_frame(&explicit_42_frame(ProofDirection::Right));
        assert_eq!(bytes, [vec![0x01, 0x00], vec![0x42; 32]].concat());
        assert_eq!(bytes.len(), 34);
    }

    #[test]
    fn serialize_left_canonical_empty_matches_pinned_vector() {
        let bytes = serialize_proof_frame(&canonical_empty_frame(ProofDirection::Left));
        assert_eq!(bytes, vec![0x00, 0x01]);
        assert_eq!(bytes.len(), 2);
    }

    #[test]
    fn serialize_right_canonical_empty_matches_pinned_vector() {
        let bytes = serialize_proof_frame(&canonical_empty_frame(ProofDirection::Right));
        assert_eq!(bytes, vec![0x01, 0x01]);
        assert_eq!(bytes.len(), 2);
    }

    #[test]
    fn round_trip_left_explicit() {
        let frame = explicit_42_frame(ProofDirection::Left);
        assert_eq!(
            parse_proof_frame(&serialize_proof_frame(&frame)),
            Ok((frame, 34))
        );
    }

    #[test]
    fn round_trip_right_explicit() {
        let frame = explicit_42_frame(ProofDirection::Right);
        assert_eq!(
            parse_proof_frame(&serialize_proof_frame(&frame)),
            Ok((frame, 34))
        );
    }

    #[test]
    fn round_trip_left_canonical_empty() {
        let frame = canonical_empty_frame(ProofDirection::Left);
        assert_eq!(
            parse_proof_frame(&serialize_proof_frame(&frame)),
            Ok((frame, 2))
        );
    }

    #[test]
    fn round_trip_right_canonical_empty() {
        let frame = canonical_empty_frame(ProofDirection::Right);
        assert_eq!(
            parse_proof_frame(&serialize_proof_frame(&frame)),
            Ok((frame, 2))
        );
    }

    #[test]
    fn canonical_empty_frame_allows_trailing_bytes() {
        let mut bytes = serialize_proof_frame(&canonical_empty_frame(ProofDirection::Left));
        bytes.extend_from_slice(&[9, 8, 7, 6, 5]);
        assert_eq!(
            parse_proof_frame(&bytes),
            Ok((canonical_empty_frame(ProofDirection::Left), 2))
        );
    }

    #[test]
    fn explicit_frame_allows_trailing_bytes() {
        let mut bytes = serialize_proof_frame(&explicit_42_frame(ProofDirection::Right));
        bytes.extend_from_slice(&[9, 8, 7, 6, 5]);
        assert_eq!(
            parse_proof_frame(&bytes),
            Ok((explicit_42_frame(ProofDirection::Right), 34))
        );
    }

    #[test]
    fn empty_input_fails_loud() {
        assert_eq!(parse_proof_frame(&[]), Err(ProofFrameError::Empty));
    }

    #[test]
    fn missing_sibling_kind_byte_is_truncated() {
        assert_eq!(
            parse_proof_frame(&[0x00]),
            Err(ProofFrameError::TruncatedInput {
                needed: 1,
                available: 0,
            })
        );
    }

    #[test]
    fn unknown_direction_byte_fails_loud() {
        assert_eq!(
            parse_proof_frame(&[0x02, 0x01]),
            Err(ProofFrameError::UnknownDirectionByte(0x02))
        );
        assert_eq!(
            parse_proof_frame(&[0xff, 0x01]),
            Err(ProofFrameError::UnknownDirectionByte(0xff))
        );
    }

    #[test]
    fn unknown_sibling_kind_byte_fails_loud() {
        assert_eq!(
            parse_proof_frame(&[0x00, 0x02]),
            Err(ProofFrameError::UnknownSiblingKindByte(0x02))
        );
        assert_eq!(
            parse_proof_frame(&[0x00, 0xff]),
            Err(ProofFrameError::UnknownSiblingKindByte(0xff))
        );
    }

    #[test]
    fn explicit_sibling_with_too_few_bytes_is_truncated() {
        assert_eq!(
            parse_proof_frame(&[0x00, 0x00, 0x42, 0x42, 0x42]),
            Err(ProofFrameError::TruncatedInput {
                needed: 32,
                available: 3,
            })
        );
    }
}
