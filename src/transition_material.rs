use crate::hashing::hash_with_tag;
use crate::library::SemanticVersion;
use crate::tags::TAG_TRANSITION_MATERIAL;

fn encode_semantic_version_be(version: &SemanticVersion) -> [u8; 12] {
    let mut encoded = [0u8; 12];
    encoded[..4].copy_from_slice(&version.major.to_be_bytes());
    encoded[4..8].copy_from_slice(&version.minor.to_be_bytes());
    encoded[8..12].copy_from_slice(&version.patch.to_be_bytes());
    encoded
}

/// Derives the §4.4 transition_material field from a transition-derivation
/// version encoded per §4.1 and adapter-owned canonical transition-source
/// bytes. The source bytes are opaque to the library; root computation,
/// commitments, proof material, and receipt construction are deferred to
/// future slices.
pub fn derive_transition_material(
    transition_derivation_version: &SemanticVersion,
    canonical_transition_source_bytes: &[u8],
) -> [u8; 32] {
    let version_bytes = encode_semantic_version_be(transition_derivation_version);
    hash_with_tag(
        TAG_TRANSITION_MATERIAL,
        &[&version_bytes, canonical_transition_source_bytes],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED_TRANSITION_MATERIAL_HEX: &str =
        "bc739f3e9d65b8d8fa8e67368d798ab2580c6230341dc2cba0bb6211f6820bea";

    fn version_1_0_0() -> SemanticVersion {
        SemanticVersion {
            major: 1,
            minor: 0,
            patch: 0,
        }
    }

    fn to_hex(bytes: [u8; 32]) -> String {
        bytes.iter().map(|byte| format!("{byte:02x}")).collect()
    }

    #[test]
    fn transition_material_matches_pinned_golden_vector() {
        assert_eq!(
            to_hex(derive_transition_material(
                &version_1_0_0(),
                b"canonical-transition-source"
            )),
            EXPECTED_TRANSITION_MATERIAL_HEX
        );
    }

    #[test]
    fn same_version_and_source_are_deterministic() {
        assert_eq!(
            derive_transition_material(&version_1_0_0(), b"canonical-transition-source"),
            derive_transition_material(&version_1_0_0(), b"canonical-transition-source")
        );
    }

    #[test]
    fn different_version_changes_digest() {
        assert_ne!(
            derive_transition_material(&version_1_0_0(), b"canonical-transition-source"),
            derive_transition_material(
                &SemanticVersion {
                    major: 1,
                    minor: 0,
                    patch: 1
                },
                b"canonical-transition-source"
            )
        );
    }

    #[test]
    fn different_source_changes_digest() {
        assert_ne!(
            derive_transition_material(&version_1_0_0(), b"canonical-transition-source"),
            derive_transition_material(&version_1_0_0(), b"canonical-transition-source-alt")
        );
    }

    #[test]
    fn empty_source_bytes_are_accepted() {
        assert_eq!(
            derive_transition_material(&version_1_0_0(), b""),
            derive_transition_material(&version_1_0_0(), b"")
        );
    }

    #[test]
    fn semantic_version_encodes_to_exactly_twelve_bytes() {
        assert_eq!(
            encode_semantic_version_be(&SemanticVersion {
                major: 1,
                minor: 2,
                patch: 3
            }),
            [0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03]
        );
    }

    #[test]
    fn semantic_version_encoding_is_big_endian() {
        assert_eq!(
            encode_semantic_version_be(&SemanticVersion {
                major: 0x0102_0304,
                minor: 0x0506_0708,
                patch: 0x0a0b_0c0d
            }),
            [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x0a, 0x0b, 0x0c, 0x0d]
        );
    }
}
