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
        "Windows.Foundation" if matches!(name, "DateTime" | "TimeSpan") => Some("windows_time"),
        "Windows.Foundation.Collections" => Some("windows_collections"),
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

pub(super) fn package_crate(namespace: &str, name: &str) -> bool {
    package_crate_name(namespace, name).is_some()
        || (namespace == "Windows.Foundation"
            && matches!(name, "EventRegistrationToken" | "HResult"))
}

pub(super) fn package_crate_name(namespace: &str, name: &str) -> Option<&'static str> {
    match namespace {
        "Windows.Foundation"
            if matches!(
                name,
                "AsyncActionCompletedHandler"
                    | "AsyncActionProgressHandler"
                    | "AsyncActionWithProgressCompletedHandler"
                    | "AsyncOperationCompletedHandler"
                    | "AsyncOperationProgressHandler"
                    | "AsyncOperationWithProgressCompletedHandler"
                    | "AsyncStatus"
                    | "IAsyncAction"
                    | "IAsyncActionWithProgress"
                    | "IAsyncInfo"
                    | "IAsyncOperation"
                    | "IAsyncOperationWithProgress"
                    | "IReference"
                    | "DateTime"
                    | "EventRegistrationToken"
                    | "HResult"
                    | "TimeSpan"
            ) =>
        {
            Some("windows_future")
        }
        "Windows.Foundation.Collections"
            if matches!(
                name,
                "CollectionChange"
                    | "IIterable"
                    | "IIterator"
                    | "IKeyValuePair"
                    | "IMap"
                    | "IMapChangedEventArgs"
                    | "IMapView"
                    | "IObservableMap"
                    | "IObservableVector"
                    | "IVector"
                    | "IVectorChangedEventArgs"
                    | "IVectorView"
                    | "MapChangedEventHandler"
                    | "VectorChangedEventHandler"
            ) =>
        {
            Some("windows_collections")
        }
        "Windows.Foundation.Numerics"
            if matches!(
                name,
                "Matrix3x2" | "Matrix4x4" | "Vector2" | "Vector3" | "Vector4"
            ) =>
        {
            Some("windows_numerics")
        }
        "Windows.Foundation" if name == "IReference" => Some("windows_reference"),
        "Windows.Foundation" if matches!(name, "DateTime" | "TimeSpan") => Some("windows_time"),
        _ => None,
    }
}
