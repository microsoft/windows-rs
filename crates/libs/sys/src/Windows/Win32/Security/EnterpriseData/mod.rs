#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
    pub fn ProtectFileToEnterpriseIdentity(fileorfolderpath: ::windows_sys::core::PCWSTR, identity: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SrpCloseThreadNetworkContext(threadnetworkcontext: *mut HTHREAD_NETWORK_CONTEXT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SrpCreateThreadNetworkContext(enterpriseid: ::windows_sys::core::PCWSTR, threadnetworkcontext: *mut HTHREAD_NETWORK_CONTEXT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
    pub fn SrpDisablePermissiveModeFileEncryption() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Appx\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Appx"))]
    pub fn SrpDoesPolicyAllowAppExecution(packageid: *const super::super::Storage::Packaging::Appx::PACKAGE_ID, isallowed: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
    pub fn SrpEnablePermissiveModeFileEncryption(enterpriseid: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SrpGetEnterpriseIds(tokenhandle: super::super::Foundation::HANDLE, numberofbytes: *mut u32, enterpriseids: *mut ::windows_sys::core::PWSTR, enterpriseidcount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SrpGetEnterprisePolicy(tokenhandle: super::super::Foundation::HANDLE, policyflags: *mut ENTERPRISE_DATA_POLICIES) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
    pub fn SrpHostingInitialize(version: SRPHOSTING_VERSION, r#type: SRPHOSTING_TYPE, pvdata: *const ::core::ffi::c_void, cbdata: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
    pub fn SrpHostingTerminate(r#type: SRPHOSTING_TYPE);
    #[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SrpIsTokenService(tokenhandle: super::super::Foundation::HANDLE, istokenservice: *mut u8) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SrpSetTokenEnterpriseId(tokenhandle: super::super::Foundation::HANDLE, enterpriseid: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
    pub fn UnprotectFile(fileorfolderpath: ::windows_sys::core::PCWSTR, options: *const FILE_UNPROTECT_OPTIONS) -> ::windows_sys::core::HRESULT;
}
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
pub type ENTERPRISE_DATA_POLICIES = u32;
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
pub const ENTERPRISE_POLICY_NONE: ENTERPRISE_DATA_POLICIES = 0u32;
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
pub const ENTERPRISE_POLICY_ALLOWED: ENTERPRISE_DATA_POLICIES = 1u32;
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
pub const ENTERPRISE_POLICY_ENLIGHTENED: ENTERPRISE_DATA_POLICIES = 2u32;
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
pub const ENTERPRISE_POLICY_EXEMPT: ENTERPRISE_DATA_POLICIES = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
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
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`, `\"Win32_Foundation\"`*"]
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
pub type IProtectionPolicyManagerInterop = *mut ::core::ffi::c_void;
pub type IProtectionPolicyManagerInterop2 = *mut ::core::ffi::c_void;
pub type IProtectionPolicyManagerInterop3 = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
pub type SRPHOSTING_TYPE = i32;
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
pub const SRPHOSTING_TYPE_NONE: SRPHOSTING_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
pub const SRPHOSTING_TYPE_WINHTTP: SRPHOSTING_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
pub const SRPHOSTING_TYPE_WININET: SRPHOSTING_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
pub type SRPHOSTING_VERSION = i32;
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
pub const SRPHOSTING_VERSION1: SRPHOSTING_VERSION = 1i32;
