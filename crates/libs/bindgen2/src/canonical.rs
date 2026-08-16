use proc_macro2::TokenStream;
use quote::quote;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) enum Type {
    Guid,
    HResult,
    EventRegistrationToken,
    Bool,
    BStr,
    HString,
    PStr,
    PWStr,
    PcStr,
    PcWStr,
    NtStatus,
    RpcStatus,
    IUnknown,
    IInspectable,
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

pub(super) fn native_alias_from_name(namespace: &str, name: &str) -> Option<Type> {
    let win32 = namespace == "Windows.Win32" || namespace.starts_with("Windows.Win32.");
    if !win32 {
        return None;
    }
    match name {
        "BOOL" => Some(Type::Bool),
        "BSTR" => Some(Type::BStr),
        "HSTRING" => Some(Type::HString),
        "PSTR" => Some(Type::PStr),
        "PWSTR" => Some(Type::PWStr),
        "PCSTR" => Some(Type::PcStr),
        "PCWSTR" => Some(Type::PcWStr),
        "NTSTATUS" => Some(Type::NtStatus),
        "RPC_STATUS" => Some(Type::RpcStatus),
        _ => None,
    }
}

pub(super) fn native_core_from_name(namespace: &str, name: &str) -> Option<Type> {
    native_alias_from_name(namespace, name).or_else(|| {
        let win32 = namespace == "Windows.Win32" || namespace.starts_with("Windows.Win32.");
        if !win32 {
            return None;
        }
        match name {
            "IUnknown" => Some(Type::IUnknown),
            "IInspectable" => Some(Type::IInspectable),
            _ => None,
        }
    })
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
            Self::Bool => quote! { windows_core::BOOL },
            Self::BStr => quote! { windows_core::BSTR },
            Self::HString => quote! { windows_core::HSTRING },
            Self::PStr => quote! { windows_core::PSTR },
            Self::PWStr => quote! { windows_core::PWSTR },
            Self::PcStr => quote! { windows_core::PCSTR },
            Self::PcWStr => quote! { windows_core::PCWSTR },
            Self::NtStatus => quote! { windows_core::NTSTATUS },
            Self::RpcStatus => quote! { windows_core::RPC_STATUS },
            Self::IUnknown => quote! { windows_core::IUnknown },
            Self::IInspectable => quote! { windows_core::IInspectable },
        }
    }

    pub(super) fn write_sys(self) -> TokenStream {
        match self {
            Self::Guid => quote! { windows_sys::core::GUID },
            Self::HResult => quote! { windows_sys::core::HRESULT },
            Self::EventRegistrationToken => quote! { i64 },
            Self::Bool => quote! { windows_sys::core::BOOL },
            Self::BStr => quote! { windows_sys::core::BSTR },
            Self::HString => quote! { windows_sys::core::HSTRING },
            Self::PStr => quote! { windows_sys::core::PSTR },
            Self::PWStr => quote! { windows_sys::core::PWSTR },
            Self::PcStr => quote! { windows_sys::core::PCSTR },
            Self::PcWStr => quote! { windows_sys::core::PCWSTR },
            Self::NtStatus => quote! { windows_sys::core::NTSTATUS },
            Self::RpcStatus => quote! { windows_sys::core::RPC_STATUS },
            Self::IUnknown => quote! { windows_sys::core::IUnknown },
            Self::IInspectable => quote! { windows_sys::core::IInspectable },
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

    pub(super) const fn is_bool(self) -> bool {
        matches!(self, Self::Bool)
    }

    pub(super) const fn is_bstr(self) -> bool {
        matches!(self, Self::BStr)
    }

    pub(super) const fn is_hstring(self) -> bool {
        matches!(self, Self::HString)
    }

    pub(super) const fn is_pcstr(self) -> bool {
        matches!(self, Self::PcStr)
    }

    pub(super) const fn is_pstr(self) -> bool {
        matches!(self, Self::PStr)
    }

    pub(super) const fn is_pcwstr(self) -> bool {
        matches!(self, Self::PcWStr)
    }

    pub(super) const fn is_const_string(self) -> bool {
        matches!(self, Self::PcStr | Self::PcWStr)
    }

    pub(super) const fn is_mutable_string(self) -> bool {
        matches!(self, Self::PStr | Self::PWStr)
    }

    pub(super) const fn is_ntstatus(self) -> bool {
        matches!(self, Self::NtStatus)
    }

    pub(super) const fn is_native_primitive(self) -> bool {
        matches!(
            self,
            Self::HResult
                | Self::EventRegistrationToken
                | Self::Bool
                | Self::NtStatus
                | Self::RpcStatus
        )
    }

    pub(super) const fn is_com_root(self) -> bool {
        matches!(self, Self::IUnknown | Self::IInspectable)
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
