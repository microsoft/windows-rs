use proc_macro2::TokenStream;
use quote::quote;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Type {
    Guid,
    HResult,
    EventRegistrationToken,
}

pub(super) fn type_from_name(namespace: &str, name: &str) -> Option<Type> {
    let win32 = namespace == "Windows.Win32" || namespace.starts_with("Windows.Win32.");
    if win32 {
        return match name {
            "GUID" => Some(Type::Guid),
            "HRESULT" => Some(Type::HResult),
            "EventRegistrationToken" => Some(Type::EventRegistrationToken),
            _ => None,
        };
    }
    match (namespace, name) {
        ("System" | "Windows.Foundation" | "", "Guid") => Some(Type::Guid),
        ("Windows.Foundation", "HResult") => Some(Type::HResult),
        ("Windows.Foundation", "EventRegistrationToken") => Some(Type::EventRegistrationToken),
        _ => None,
    }
}

pub(super) fn winrt_type_from_name(namespace: &str, name: &str) -> Option<Type> {
    match (namespace, name) {
        ("System", "Guid") => Some(Type::Guid),
        ("Windows.Foundation", "HResult") => Some(Type::HResult),
        ("Windows.Foundation", "EventRegistrationToken") => Some(Type::EventRegistrationToken),
        _ => None,
    }
}

impl Type {
    pub(super) fn write(self) -> TokenStream {
        match self {
            Self::Guid => quote! { windows_core::GUID },
            Self::HResult => quote! { windows_core::HRESULT },
            Self::EventRegistrationToken => quote! { i64 },
        }
    }

    pub(super) fn write_sys(self) -> TokenStream {
        match self {
            Self::Guid => quote! { windows_sys::core::GUID },
            Self::HResult => quote! { windows_sys::core::HRESULT },
            Self::EventRegistrationToken => quote! { i64 },
        }
    }

    pub(super) const fn is_guid(self) -> bool {
        matches!(self, Self::Guid)
    }

    pub(super) const fn is_hresult(self) -> bool {
        matches!(self, Self::HResult)
    }

    pub(super) const fn is_event_token(self) -> bool {
        matches!(self, Self::EventRegistrationToken)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_canonical_types_cover_win32_and_abi_aliases() {
        assert_eq!(
            type_from_name("Windows.Win32.Foundation", "GUID"),
            Some(Type::Guid)
        );
        assert_eq!(
            type_from_name("Windows.Win32.Foundation", "HRESULT"),
            Some(Type::HResult)
        );
        assert_eq!(
            type_from_name("Windows.Win32.System.WinRT", "EventRegistrationToken"),
            Some(Type::EventRegistrationToken)
        );
        assert_eq!(type_from_name("", "Guid"), Some(Type::Guid));
        assert_eq!(
            type_from_name("Windows.Foundation", "Guid"),
            Some(Type::Guid)
        );
    }

    #[test]
    fn winrt_canonical_types_are_narrowly_scoped() {
        assert_eq!(winrt_type_from_name("System", "Guid"), Some(Type::Guid));
        assert_eq!(
            winrt_type_from_name("Windows.Foundation", "HResult"),
            Some(Type::HResult)
        );
        assert_eq!(
            winrt_type_from_name("Windows.Foundation", "EventRegistrationToken"),
            Some(Type::EventRegistrationToken)
        );
        assert_eq!(winrt_type_from_name("Windows.Foundation", "Guid"), None);
        assert_eq!(
            winrt_type_from_name("Windows.Win32.Foundation", "GUID"),
            None
        );
    }
}
