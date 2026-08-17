#[derive(Clone, Copy, Eq, PartialEq)]
enum Policy {
    Winrt,
    Package,
}

struct Route {
    namespace: &'static str,
    names: &'static [&'static str],
    crate_name: &'static str,
    policies: &'static [Policy],
}

const WINRT: &[Policy] = &[Policy::Winrt];
const PACKAGE: &[Policy] = &[Policy::Package];
const BOTH: &[Policy] = &[Policy::Winrt, Policy::Package];

const ROUTES: &[Route] = &[
    Route {
        namespace: "Windows.Foundation",
        names: &[
            "IAsyncAction",
            "IAsyncActionWithProgress",
            "IAsyncOperation",
            "IAsyncOperationWithProgress",
        ],
        crate_name: "windows_future",
        policies: BOTH,
    },
    Route {
        namespace: "Windows.Foundation",
        names: &[
            "AsyncActionCompletedHandler",
            "AsyncActionProgressHandler",
            "AsyncActionWithProgressCompletedHandler",
            "AsyncOperationCompletedHandler",
            "AsyncOperationProgressHandler",
            "AsyncOperationWithProgressCompletedHandler",
            "AsyncStatus",
            "IAsyncInfo",
        ],
        crate_name: "windows_future",
        policies: PACKAGE,
    },
    Route {
        namespace: "Windows.Foundation",
        names: &["IReference"],
        crate_name: "windows_reference",
        policies: BOTH,
    },
    Route {
        namespace: "Windows.Foundation",
        names: &["IReferenceArray"],
        crate_name: "windows_reference",
        policies: WINRT,
    },
    Route {
        namespace: "Windows.Foundation",
        names: &["DateTime", "TimeSpan"],
        crate_name: "windows_time",
        policies: BOTH,
    },
    Route {
        namespace: "Windows.Foundation.Collections",
        names: &[],
        crate_name: "windows_collections",
        policies: WINRT,
    },
    Route {
        namespace: "Windows.Foundation.Collections",
        names: &[
            "CollectionChange",
            "IIterable",
            "IIterator",
            "IKeyValuePair",
            "IMap",
            "IMapChangedEventArgs",
            "IMapView",
            "IObservableMap",
            "IObservableVector",
            "IVector",
            "IVectorChangedEventArgs",
            "IVectorView",
            "MapChangedEventHandler",
            "VectorChangedEventHandler",
        ],
        crate_name: "windows_collections",
        policies: PACKAGE,
    },
    Route {
        namespace: "Windows.Foundation.Numerics",
        names: &["Matrix3x2", "Matrix4x4", "Vector2", "Vector3", "Vector4"],
        crate_name: "windows_numerics",
        policies: BOTH,
    },
];

fn crate_name(namespace: &str, name: &str, policy: Policy) -> Option<&'static str> {
    ROUTES
        .iter()
        .find(|route| {
            route.namespace == namespace
                && route.policies.contains(&policy)
                && (route.names.is_empty() || route.names.contains(&name))
        })
        .map(|route| route.crate_name)
}

pub(super) fn winrt_crate(namespace: &str, name: &str) -> Option<&'static str> {
    crate_name(namespace, name, Policy::Winrt)
}

pub(super) fn minimal_crate(namespace: &str, name: &str) -> Option<&'static str> {
    crate_name(namespace, name, Policy::Winrt)
}

pub(super) fn package_crate_name(namespace: &str, name: &str) -> Option<&'static str> {
    if matches!(
        crate::canonical::winrt_type_from_name(namespace, name),
        Some(crate::canonical::Type::HResult | crate::canonical::Type::EventRegistrationToken)
    ) {
        return Some("windows_future");
    }
    crate_name(namespace, name, Policy::Package)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routes_are_unambiguous() {
        for (position, left) in ROUTES.iter().enumerate() {
            for right in &ROUTES[position + 1..] {
                if left.namespace != right.namespace
                    || !left
                        .policies
                        .iter()
                        .any(|policy| right.policies.contains(policy))
                {
                    continue;
                }
                let names_overlap = left.names.is_empty()
                    || right.names.is_empty()
                    || left.names.iter().any(|name| right.names.contains(name));
                assert!(
                    !names_overlap || left.crate_name == right.crate_name,
                    "conflicting routes for {}",
                    left.namespace
                );
            }
        }
    }
}
