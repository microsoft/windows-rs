#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_System_WinRT_Shell'*"]
pub type CreateProcessMethod = i32;
#[doc = "*Required features: 'Win32_System_WinRT_Shell'*"]
pub const CpCreateProcess: CreateProcessMethod = 0i32;
#[doc = "*Required features: 'Win32_System_WinRT_Shell'*"]
pub const CpCreateProcessAsUser: CreateProcessMethod = 1i32;
#[doc = "*Required features: 'Win32_System_WinRT_Shell'*"]
pub const CpAicLaunchAdminProcess: CreateProcessMethod = 2i32;
#[doc = "*Required features: 'Win32_System_WinRT_Shell'*"]
#[repr(transparent)]
pub struct IDDEInitializer(::windows::core::IUnknown);
impl IDDEInitializer {
    #[doc = "*Required features: 'Win32_System_WinRT_Shell', 'Win32_Foundation', 'Win32_UI_Shell'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::UI::Shell::IShellItem>, Param4: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param5: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param6: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param7: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param8: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(
        &self,
        fileextensionorprotocol: Param0,
        method: CreateProcessMethod,
        currentdirectory: Param2,
        exectarget: Param3,
        site: Param4,
        application: Param5,
        targetfile: Param6,
        arguments: Param7,
        verb: Param8,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), fileextensionorprotocol.into_param().abi(), ::core::mem::transmute(method), currentdirectory.into_param().abi(), exectarget.into_param().abi(), site.into_param().abi(), application.into_param().abi(), targetfile.into_param().abi(), arguments.into_param().abi(), verb.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IDDEInitializer> for ::windows::core::IUnknown {
    fn from(value: IDDEInitializer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDDEInitializer> for ::windows::core::IUnknown {
    fn from(value: &IDDEInitializer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDDEInitializer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDDEInitializer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDDEInitializer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Interface for IDDEInitializer {
    type Vtable = IDDEInitializerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30dc931f_33fc_4ffd_a168_942258cf3ca4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDDEInitializerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fileextensionorprotocol: super::super::super::Foundation::PWSTR, method: CreateProcessMethod, currentdirectory: super::super::super::Foundation::PWSTR, exectarget: ::windows::core::RawPtr, site: *mut ::core::ffi::c_void, application: super::super::super::Foundation::PWSTR, targetfile: super::super::super::Foundation::PWSTR, arguments: super::super::super::Foundation::PWSTR, verb: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell")))] usize,
);
