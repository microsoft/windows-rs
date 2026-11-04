use super::*;

fn underlying(snapshot: &Snapshot, mut ty: TypeId) -> TypeId {
    while let TypeKind::Alias(id) = snapshot.types[ty.0].kind {
        ty = snapshot.aliases[id.0].observations[0].canonical;
    }
    ty
}

pub(super) fn integer(snapshot: &Snapshot, ty: TypeId) -> bool {
    matches!(snapshot.types[underlying(snapshot, ty).0].kind,
        TypeKind::Scalar(ref value) if value.signed_integer().is_some())
}

/// Integer value ranges constrain argument values, not the projected signature, and metadata does
/// not publish them.
pub(super) fn range(value: &sal::Annotation<'_>) -> bool {
    value.unscoped().is_some_and(|value| {
        matches!(
            (value.version.as_str(), value.name.as_str()),
            ("2", "_In_range_" | "_Deref_in_range_" | "_Deref_out_range_")
                | ("1.2", "__RPC__in_range")
        )
    })
}
