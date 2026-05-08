use crate::library::{CertificationProvenancePosture, CertificationProvenanceRoute};

pub(crate) fn certification_provenance_shape_is_consistent(
    posture: &CertificationProvenancePosture,
    route: &CertificationProvenanceRoute,
    has_certification_material: bool,
    has_provenance_material: bool,
    has_route_context_material: bool,
) -> bool {
    match posture {
        CertificationProvenancePosture::InlinePayload => {
            *route == CertificationProvenanceRoute::DirectInline
                && has_certification_material
                && has_provenance_material
                && !has_route_context_material
        }
        CertificationProvenancePosture::RouteDependentPayload => match route {
            CertificationProvenanceRoute::DirectInline => false,
            CertificationProvenanceRoute::RouteContextRequired => {
                has_route_context_material
                    && (has_certification_material || has_provenance_material)
            }
            CertificationProvenanceRoute::RouteContextOnly => has_route_context_material,
        },
        CertificationProvenancePosture::NoPayloadForRoute => {
            !has_certification_material && !has_provenance_material && !has_route_context_material
        }
    }
}
