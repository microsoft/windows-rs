pub(super) fn winrt_crate(namespace: &str, name: &str) -> Option<&'static str> {
    match namespace {
        "Windows.Foundation"
            if matches!(
                name,
                "IAsyncAction"
                    | "IAsyncActionWithProgress"
                    | "IAsyncOperation"
                    | "IAsyncOperationWithProgress"
            ) =>
        {
            Some("windows_future")
        }
        "Windows.Foundation" if matches!(name, "IReference" | "IReferenceArray") => {
            Some("windows_reference")
        }
        "Windows.Foundation.Collections" => Some("windows_collections"),
        _ => None,
    }
}

pub(super) fn minimal_crate(namespace: &str, name: &str) -> Option<&'static str> {
    winrt_crate(namespace, name).or(match namespace {
        "Windows.Foundation.Numerics"
            if matches!(
                name,
                "Matrix3x2" | "Matrix4x4" | "Vector2" | "Vector3" | "Vector4"
            ) =>
        {
            Some("windows_numerics")
        }
        _ => None,
    })
}
