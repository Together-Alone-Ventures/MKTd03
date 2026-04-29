use crate::proof_frame::{parse_proof_frame, serialize_proof_frame, ProofFrame, ProofFrameError};

const BASELINE_STEP_COUNT: u16 = 256;
const HEADER_LEN: usize = 2;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProofEnvelope {
    pub frames: [ProofFrame; 256],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProofEnvelopeError {
    HeaderTruncated { needed: usize, available: usize },
    InvalidStepCount { declared: u16 },
    Frame { index: u16, error: ProofFrameError },
    TrailingBytes { trailing: usize },
}

pub fn serialize_proof_envelope(envelope: &ProofEnvelope) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&BASELINE_STEP_COUNT.to_be_bytes());
    for frame in &envelope.frames {
        bytes.extend_from_slice(&serialize_proof_frame(frame));
    }
    bytes
}

pub fn parse_proof_envelope(bytes: &[u8]) -> Result<ProofEnvelope, ProofEnvelopeError> {
    if bytes.len() < HEADER_LEN {
        return Err(ProofEnvelopeError::HeaderTruncated {
            needed: HEADER_LEN,
            available: bytes.len(),
        });
    }

    let declared = u16::from_be_bytes([bytes[0], bytes[1]]);
    if declared != BASELINE_STEP_COUNT {
        return Err(ProofEnvelopeError::InvalidStepCount { declared });
    }

    let mut offset = HEADER_LEN;
    let mut frames = Vec::with_capacity(BASELINE_STEP_COUNT as usize);

    for index in 0..BASELINE_STEP_COUNT {
        let (frame, consumed) = parse_proof_frame(&bytes[offset..])
            .map_err(|error| ProofEnvelopeError::Frame { index, error })?;
        frames.push(frame);
        offset += consumed;
    }

    if offset != bytes.len() {
        return Err(ProofEnvelopeError::TrailingBytes {
            trailing: bytes.len() - offset,
        });
    }

    let frames: [ProofFrame; 256] = match frames.try_into() {
        Ok(frames) => frames,
        Err(_) => unreachable!("exactly 256 frames must be collected"),
    };

    Ok(ProofEnvelope { frames })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proof_frame::{ProofDirection, ProofFrameSibling};

    fn left_canonical_empty() -> ProofFrame {
        ProofFrame {
            direction: ProofDirection::Left,
            sibling: ProofFrameSibling::CanonicalEmpty,
        }
    }

    fn left_explicit_42() -> ProofFrame {
        ProofFrame {
            direction: ProofDirection::Left,
            sibling: ProofFrameSibling::Explicit([0x42; 32]),
        }
    }

    fn right_explicit_ab() -> ProofFrame {
        ProofFrame {
            direction: ProofDirection::Right,
            sibling: ProofFrameSibling::Explicit([0xab; 32]),
        }
    }

    fn canonical_empty_envelope() -> ProofEnvelope {
        ProofEnvelope {
            frames: std::array::from_fn(|_| left_canonical_empty()),
        }
    }

    fn explicit_left_envelope() -> ProofEnvelope {
        ProofEnvelope {
            frames: std::array::from_fn(|_| left_explicit_42()),
        }
    }

    fn mixed_envelope() -> ProofEnvelope {
        ProofEnvelope {
            frames: std::array::from_fn(|index| {
                if index % 2 == 0 {
                    left_canonical_empty()
                } else {
                    right_explicit_ab()
                }
            }),
        }
    }

    #[test]
    fn round_trip_all_canonical_empty() {
        let envelope = canonical_empty_envelope();
        let bytes = serialize_proof_envelope(&envelope);
        assert_eq!(bytes.len(), 514);
        assert_eq!(&bytes[..2], &[0x01, 0x00]);
        let parsed = parse_proof_envelope(&bytes).expect("canonical-empty envelope should parse");
        assert_eq!(serialize_proof_envelope(&parsed), bytes);
    }

    #[test]
    fn round_trip_all_explicit_left() {
        let envelope = explicit_left_envelope();
        let bytes = serialize_proof_envelope(&envelope);
        assert_eq!(bytes.len(), 8706);
        assert_eq!(&bytes[..2], &[0x01, 0x00]);
        let parsed = parse_proof_envelope(&bytes).expect("explicit-left envelope should parse");
        assert_eq!(serialize_proof_envelope(&parsed), bytes);
    }

    #[test]
    fn round_trip_mixed_pattern() {
        let envelope = mixed_envelope();
        let bytes = serialize_proof_envelope(&envelope);
        assert_eq!(bytes.len(), 2 + 128 * 2 + 128 * 34);
        let parsed = parse_proof_envelope(&bytes).expect("mixed envelope should parse");
        assert_eq!(serialize_proof_envelope(&parsed), bytes);
    }

    #[test]
    fn empty_input_fails_header_truncated() {
        assert_eq!(
            parse_proof_envelope(&[]),
            Err(ProofEnvelopeError::HeaderTruncated {
                needed: 2,
                available: 0,
            })
        );
    }

    #[test]
    fn single_byte_header_fails_truncated() {
        assert_eq!(
            parse_proof_envelope(&[0x01]),
            Err(ProofEnvelopeError::HeaderTruncated {
                needed: 2,
                available: 1,
            })
        );
    }

    #[test]
    fn step_count_zero_rejected() {
        assert_eq!(
            parse_proof_envelope(&[0x00, 0x00]),
            Err(ProofEnvelopeError::InvalidStepCount { declared: 0 })
        );
    }

    #[test]
    fn step_count_255_rejected() {
        assert_eq!(
            parse_proof_envelope(&[0x00, 0xff]),
            Err(ProofEnvelopeError::InvalidStepCount { declared: 255 })
        );
    }

    #[test]
    fn step_count_257_rejected() {
        assert_eq!(
            parse_proof_envelope(&[0x01, 0x01]),
            Err(ProofEnvelopeError::InvalidStepCount { declared: 257 })
        );
    }

    #[test]
    fn step_count_max_u16_rejected() {
        assert_eq!(
            parse_proof_envelope(&[0xff, 0xff]),
            Err(ProofEnvelopeError::InvalidStepCount { declared: 65535 })
        );
    }

    #[test]
    fn truncated_at_first_frame_reports_index_zero() {
        assert_eq!(
            parse_proof_envelope(&[0x01, 0x00]),
            Err(ProofEnvelopeError::Frame {
                index: 0,
                error: ProofFrameError::Empty,
            })
        );
    }

    #[test]
    fn truncated_mid_envelope_reports_correct_index() {
        let bytes = [vec![0x01, 0x00], vec![0x00, 0x01]].concat();
        assert_eq!(
            parse_proof_envelope(&bytes),
            Err(ProofEnvelopeError::Frame {
                index: 1,
                error: ProofFrameError::Empty,
            })
        );
    }

    #[test]
    fn bad_direction_byte_at_known_frame_index() {
        let mut bytes = serialize_proof_envelope(&canonical_empty_envelope());
        let offset = 2 + 5 * 2;
        bytes[offset] = 0x02;
        assert_eq!(
            parse_proof_envelope(&bytes),
            Err(ProofEnvelopeError::Frame {
                index: 5,
                error: ProofFrameError::UnknownDirectionByte(0x02),
            })
        );
    }

    #[test]
    fn bad_sibling_kind_byte_at_known_frame_index() {
        let mut bytes = serialize_proof_envelope(&canonical_empty_envelope());
        let offset = 2 + 7 * 2 + 1;
        bytes[offset] = 0xff;
        assert_eq!(
            parse_proof_envelope(&bytes),
            Err(ProofEnvelopeError::Frame {
                index: 7,
                error: ProofFrameError::UnknownSiblingKindByte(0xff),
            })
        );
    }

    #[test]
    fn trailing_bytes_after_256th_frame_rejected() {
        let mut bytes = serialize_proof_envelope(&canonical_empty_envelope());
        bytes.extend_from_slice(&[9, 8, 7]);
        assert_eq!(
            parse_proof_envelope(&bytes),
            Err(ProofEnvelopeError::TrailingBytes { trailing: 3 })
        );
    }

    #[test]
    fn serialize_then_parse_then_serialize_is_idempotent() {
        let bytes = serialize_proof_envelope(&mixed_envelope());
        let parsed = parse_proof_envelope(&bytes).expect("mixed envelope should parse");
        assert_eq!(serialize_proof_envelope(&parsed), bytes);
    }
}
