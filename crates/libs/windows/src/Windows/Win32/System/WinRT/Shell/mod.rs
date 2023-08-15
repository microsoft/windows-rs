#[doc = "*Required features: `\"Win32_System_WinRT_Shell\"`*"]
#[repr(transparent)]
pub struct IDDEInitializer(::windows_core::IUnknown);
impl IDDEInitializer {
    #[doc = "*Required features: `\"Win32_UI_Shell\"`*"]
    #[cfg(feature = "Win32_UI_Shell")]
    pub unsafe fn Initialize<P0, P1, P2, P3, P4, P5, P6, P7>(&self, fileextensionorprotocol: P0, method: CreateProcessMethod, currentdirectory: P1, exectarget: P2, site: P3, application: P4, targetfile: P5, arguments: P6, verb: P7) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<super::super::super::UI::Shell::IShellItem>,
        P3: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P4: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P5: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P6: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P7: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), fileextensionorprotocol.into_param().abi(), method, currentdirectory.into_param().abi(), exectarget.into_param().abi(), site.into_param().abi(), application.into_param().abi(), targetfile.into_param().abi(), arguments.into_param().abi(), verb.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDDEInitializer, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IDDEInitializer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDDEInitializer {}
impl ::core::fmt::Debug for IDDEInitializer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDDEInitializer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDDEInitializer {
    type Vtable = IDDEInitializer_Vtbl;
}
impl ::core::clone::Clone for IDDEInitializer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDDEInitializer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30dc931f_33fc_4ffd_a168_942258cf3ca4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDDEInitializer_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_Shell")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fileextensionorprotocol: ::windows_core::PCWSTR, method: CreateProcessMethod, currentdirectory: ::windows_core::PCWSTR, exectarget: *mut ::core::ffi::c_void, site: *mut ::core::ffi::c_void, application: ::windows_core::PCWSTR, targetfile: ::windows_core::PCWSTR, arguments: ::windows_core::PCWSTR, verb: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell"))]
    Initialize: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Shell\"`*"]
pub const CpAicLaunchAdminProcess: CreateProcessMethod = CreateProcessMethod(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Shell\"`*"]
pub const CpCreateProcess: CreateProcessMethod = CreateProcessMethod(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Shell\"`*"]
pub const CpCreateProcessAsUser: CreateProcessMethod = CreateProcessMethod(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Shell\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CreateProcessMethod(pub i32);
impl ::core::marker::Copy for CreateProcessMethod {}
impl ::core::clone::Clone for CreateProcessMethod {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CreateProcessMethod {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CreateProcessMethod {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CreateProcessMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CreateProcessMethod").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
