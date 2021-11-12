#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn ProtectFileToEnterpriseIdentity(fileorfolderpath: super::super::Foundation::PWSTR, identity: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SrpCloseThreadNetworkContext(threadnetworkcontext: *mut HTHREAD_NETWORK_CONTEXT) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SrpCreateThreadNetworkContext(enterpriseid: super::super::Foundation::PWSTR, threadnetworkcontext: *mut HTHREAD_NETWORK_CONTEXT) -> ::windows_sys::core::HRESULT;
    pub fn SrpDisablePermissiveModeFileEncryption() -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Appx"))]
    pub fn SrpDoesPolicyAllowAppExecution(packageid: *const super::super::Storage::Packaging::Appx::PACKAGE_ID, isallowed: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SrpEnablePermissiveModeFileEncryption(enterpriseid: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SrpGetEnterpriseIds(tokenhandle: super::super::Foundation::HANDLE, numberofbytes: *mut u32, enterpriseids: *mut super::super::Foundation::PWSTR, enterpriseidcount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SrpGetEnterprisePolicy(tokenhandle: super::super::Foundation::HANDLE, policyflags: *mut ENTERPRISE_DATA_POLICIES) -> ::windows_sys::core::HRESULT;
    pub fn SrpHostingInitialize(version: SRPHOSTING_VERSION, r#type: SRPHOSTING_TYPE, pvdata: *const ::core::ffi::c_void, cbdata: u32) -> ::windows_sys::core::HRESULT;
    pub fn SrpHostingTerminate(r#type: SRPHOSTING_TYPE);
    #[cfg(feature = "Win32_Foundation")]
    pub fn SrpIsTokenService(tokenhandle: super::super::Foundation::HANDLE, istokenservice: *mut u8) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SrpSetTokenEnterpriseId(tokenhandle: super::super::Foundation::HANDLE, enterpriseid: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnprotectFile(fileorfolderpath: super::super::Foundation::PWSTR, options: *const FILE_UNPROTECT_OPTIONS) -> ::windows_sys::core::HRESULT;
}
#[repr(transparent)]
pub struct ENTERPRISE_DATA_POLICIES(pub u32);
pub const ENTERPRISE_POLICY_NONE: ENTERPRISE_DATA_POLICIES = ENTERPRISE_DATA_POLICIES(0u32);
pub const ENTERPRISE_POLICY_ALLOWED: ENTERPRISE_DATA_POLICIES = ENTERPRISE_DATA_POLICIES(1u32);
pub const ENTERPRISE_POLICY_ENLIGHTENED: ENTERPRISE_DATA_POLICIES = ENTERPRISE_DATA_POLICIES(2u32);
pub const ENTERPRISE_POLICY_EXEMPT: ENTERPRISE_DATA_POLICIES = ENTERPRISE_DATA_POLICIES(4u32);
impl ::core::marker::Copy for ENTERPRISE_DATA_POLICIES {}
impl ::core::clone::Clone for ENTERPRISE_DATA_POLICIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct FILE_UNPROTECT_OPTIONS {
    pub audit: bool,
}
impl ::core::marker::Copy for FILE_UNPROTECT_OPTIONS {}
impl ::core::clone::Clone for FILE_UNPROTECT_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HTHREAD_NETWORK_CONTEXT {
    pub ThreadId: u32,
    pub ThreadContext: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTHREAD_NETWORK_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTHREAD_NETWORK_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProtectionPolicyManagerInterop(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProtectionPolicyManagerInterop {}
impl ::core::clone::Clone for IProtectionPolicyManagerInterop {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProtectionPolicyManagerInterop2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProtectionPolicyManagerInterop2 {}
impl ::core::clone::Clone for IProtectionPolicyManagerInterop2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProtectionPolicyManagerInterop3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProtectionPolicyManagerInterop3 {}
impl ::core::clone::Clone for IProtectionPolicyManagerInterop3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SRPHOSTING_TYPE(pub i32);
pub const SRPHOSTING_TYPE_NONE: SRPHOSTING_TYPE = SRPHOSTING_TYPE(0i32);
pub const SRPHOSTING_TYPE_WINHTTP: SRPHOSTING_TYPE = SRPHOSTING_TYPE(1i32);
pub const SRPHOSTING_TYPE_WININET: SRPHOSTING_TYPE = SRPHOSTING_TYPE(2i32);
impl ::core::marker::Copy for SRPHOSTING_TYPE {}
impl ::core::clone::Clone for SRPHOSTING_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SRPHOSTING_VERSION(pub i32);
pub const SRPHOSTING_VERSION1: SRPHOSTING_VERSION = SRPHOSTING_VERSION(1i32);
impl ::core::marker::Copy for SRPHOSTING_VERSION {}
impl ::core::clone::Clone for SRPHOSTING_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
