windows_link::link!("wsdapi.dll" "system" fn WSDCreateHttpAddress(ppaddress : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateHttpMessageParameters(pptxparams : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateUdpAddress(ppaddress : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateUdpMessageParameters(pptxparams : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
pub const ONE_WAY: WSDUdpMessageType = 0;
pub type PWSD_CONFIG_ADDRESSES = *mut WSD_CONFIG_ADDRESSES;
pub type PWSD_CONFIG_PARAM = *mut WSD_CONFIG_PARAM;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_wincrypt"))]
pub type PWSD_SECURITY_CERT_VALIDATION = *mut WSD_SECURITY_CERT_VALIDATION;
pub type PWSD_SECURITY_HTTP_AUTH_SCHEMES = *mut u32;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_wincrypt"))]
pub type PWSD_SECURITY_SIGNATURE_VALIDATION = *mut WSD_SECURITY_SIGNATURE_VALIDATION;
pub const TWO_WAY: WSDUdpMessageType = 1;
pub const WSDAPI_ADDRESSFAMILY_IPV4: u32 = 1;
pub const WSDAPI_ADDRESSFAMILY_IPV6: u32 = 2;
pub const WSDAPI_COMPACTSIG_ACCEPT_ALL_MESSAGES: u32 = 1;
pub const WSDAPI_SSL_CERT_APPLY_DEFAULT_CHECKS: u32 = 0;
pub const WSDAPI_SSL_CERT_IGNORE_EXPIRY: u32 = 2;
pub const WSDAPI_SSL_CERT_IGNORE_INVALID_CN: u32 = 16;
pub const WSDAPI_SSL_CERT_IGNORE_REVOCATION: u32 = 1;
pub const WSDAPI_SSL_CERT_IGNORE_UNKNOWN_CA: u32 = 8;
pub const WSDAPI_SSL_CERT_IGNORE_WRONG_USAGE: u32 = 4;
pub type WSDUdpMessageType = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WSDUdpRetransmitParams {
    pub ulSendDelay: u32,
    pub ulRepeat: u32,
    pub ulRepeatMinDelay: u32,
    pub ulRepeatMaxDelay: u32,
    pub ulRepeatUpperDelay: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_CONFIG_ADDRESSES {
    pub addresses: *mut *mut core::ffi::c_void,
    pub dwAddressCount: u32,
}
impl Default for WSD_CONFIG_ADDRESSES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WSD_CONFIG_DEVICE_ADDRESSES: WSD_CONFIG_PARAM_TYPE = 10;
pub const WSD_CONFIG_HOSTING_ADDRESSES: WSD_CONFIG_PARAM_TYPE = 9;
pub const WSD_CONFIG_MAX_INBOUND_MESSAGE_SIZE: WSD_CONFIG_PARAM_TYPE = 1;
pub const WSD_CONFIG_MAX_OUTBOUND_MESSAGE_SIZE: WSD_CONFIG_PARAM_TYPE = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_CONFIG_PARAM {
    pub configParamType: WSD_CONFIG_PARAM_TYPE,
    pub pConfigData: *mut core::ffi::c_void,
    pub dwConfigDataSize: u32,
}
impl Default for WSD_CONFIG_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WSD_CONFIG_PARAM_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_wincrypt"))]
#[derive(Clone, Copy)]
pub struct WSD_SECURITY_CERT_VALIDATION {
    pub certMatchArray: *mut super::wincrypt::PCCERT_CONTEXT,
    pub dwCertMatchArrayCount: u32,
    pub hCertMatchStore: super::wincrypt::HCERTSTORE,
    pub hCertIssuerStore: super::wincrypt::HCERTSTORE,
    pub dwCertCheckOptions: u32,
    pub pszCNGHashAlgId: windows_sys::core::PCWSTR,
    pub pbCertHash: *mut u8,
    pub dwCertHashSize: u32,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_wincrypt"))]
impl Default for WSD_SECURITY_CERT_VALIDATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_wincrypt"))]
#[derive(Clone, Copy)]
pub struct WSD_SECURITY_CERT_VALIDATION_V1 {
    pub certMatchArray: *mut super::wincrypt::PCCERT_CONTEXT,
    pub dwCertMatchArrayCount: u32,
    pub hCertMatchStore: super::wincrypt::HCERTSTORE,
    pub hCertIssuerStore: super::wincrypt::HCERTSTORE,
    pub dwCertCheckOptions: u32,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_wincrypt"))]
impl Default for WSD_SECURITY_CERT_VALIDATION_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WSD_SECURITY_COMPACTSIG_SIGNING_CERT: WSD_CONFIG_PARAM_TYPE = 7;
pub const WSD_SECURITY_COMPACTSIG_VALIDATION: WSD_CONFIG_PARAM_TYPE = 8;
pub type WSD_SECURITY_HTTP_AUTH_SCHEMES = u32;
pub const WSD_SECURITY_HTTP_AUTH_SCHEME_NEGOTIATE: u32 = 1;
pub const WSD_SECURITY_HTTP_AUTH_SCHEME_NTLM: u32 = 2;
pub const WSD_SECURITY_REQUIRE_CLIENT_CERT_OR_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = 12;
pub const WSD_SECURITY_REQUIRE_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = 11;
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_wincrypt"))]
#[derive(Clone, Copy)]
pub struct WSD_SECURITY_SIGNATURE_VALIDATION {
    pub signingCertArray: *mut super::wincrypt::PCCERT_CONTEXT,
    pub dwSigningCertArrayCount: u32,
    pub hSigningCertStore: super::wincrypt::HCERTSTORE,
    pub dwFlags: u32,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_wincrypt"))]
impl Default for WSD_SECURITY_SIGNATURE_VALIDATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WSD_SECURITY_SSL_CERT_FOR_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = 3;
pub const WSD_SECURITY_SSL_CLIENT_CERT_VALIDATION: WSD_CONFIG_PARAM_TYPE = 5;
pub const WSD_SECURITY_SSL_NEGOTIATE_CLIENT_CERT: WSD_CONFIG_PARAM_TYPE = 6;
pub const WSD_SECURITY_SSL_SERVER_CERT_VALIDATION: WSD_CONFIG_PARAM_TYPE = 4;
pub const WSD_SECURITY_USE_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = 13;
