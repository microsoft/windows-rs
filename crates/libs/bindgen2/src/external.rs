pub(super) fn minimal_crate(namespace: &str, name: &str) -> Option<&'static str> {
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
        "Windows.Foundation.Numerics"
            if matches!(
                name,
                "Matrix3x2" | "Matrix4x4" | "Vector2" | "Vector3" | "Vector4"
            ) =>
        {
            Some("windows_numerics")
        }
        _ => None,
    }
}
