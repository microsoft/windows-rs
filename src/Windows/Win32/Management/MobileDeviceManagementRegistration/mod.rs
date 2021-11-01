#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn ApplyLocalManagementSyncML<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(syncmlrequest: Param0) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ApplyLocalManagementSyncML(syncmlrequest: super::super::Foundation::PWSTR, syncmlresult: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        ApplyLocalManagementSyncML(syncmlrequest.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const DEVICEREGISTRATIONTYPE_MAM: u32 = 5u32;
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const DEVICEREGISTRATIONTYPE_MDM_DEVICEWIDE_WITH_AAD: u32 = 6u32;
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const DEVICEREGISTRATIONTYPE_MDM_ONLY: u32 = 0u32;
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const DEVICEREGISTRATIONTYPE_MDM_USERSPECIFIC_WITH_AAD: u32 = 13u32;
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const DEVICE_ENROLLER_FACILITY_CODE: u32 = 24u32;
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn DiscoverManagementService<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pszupn: Param0) -> ::windows::runtime::Result<*mut MANAGEMENT_SERVICE_INFO> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DiscoverManagementService(pszupn: super::super::Foundation::PWSTR, ppmgmtinfo: *mut *mut MANAGEMENT_SERVICE_INFO) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <*mut MANAGEMENT_SERVICE_INFO as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DiscoverManagementService(pszupn.into_param().abi(), &mut result__).from_abi::<*mut MANAGEMENT_SERVICE_INFO>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn DiscoverManagementServiceEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pszupn: Param0, pszdiscoveryservicecandidate: Param1) -> ::windows::runtime::Result<*mut MANAGEMENT_SERVICE_INFO> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DiscoverManagementServiceEx(pszupn: super::super::Foundation::PWSTR, pszdiscoveryservicecandidate: super::super::Foundation::PWSTR, ppmgmtinfo: *mut *mut MANAGEMENT_SERVICE_INFO) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <*mut MANAGEMENT_SERVICE_INFO as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DiscoverManagementServiceEx(pszupn.into_param().abi(), pszdiscoveryservicecandidate.into_param().abi(), &mut result__).from_abi::<*mut MANAGEMENT_SERVICE_INFO>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn GetDeviceManagementConfigInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(providerid: Param0, configstringbufferlength: *mut u32, configstring: super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDeviceManagementConfigInfo(providerid: super::super::Foundation::PWSTR, configstringbufferlength: *mut u32, configstring: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        GetDeviceManagementConfigInfo(providerid.into_param().abi(), ::std::mem::transmute(configstringbufferlength), ::std::mem::transmute(configstring)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
#[inline]
pub unsafe fn GetDeviceRegistrationInfo(deviceinformationclass: REGISTRATION_INFORMATION_CLASS, ppdeviceregistrationinfo: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDeviceRegistrationInfo(deviceinformationclass: REGISTRATION_INFORMATION_CLASS, ppdeviceregistrationinfo: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        GetDeviceRegistrationInfo(::std::mem::transmute(deviceinformationclass), ::std::mem::transmute(ppdeviceregistrationinfo)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn GetManagementAppHyperlink(cchhyperlink: u32, pszhyperlink: super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetManagementAppHyperlink(cchhyperlink: u32, pszhyperlink: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        GetManagementAppHyperlink(::std::mem::transmute(cchhyperlink), ::std::mem::transmute(pszhyperlink)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn IsDeviceRegisteredWithManagement(pfisdeviceregisteredwithmanagement: *mut super::super::Foundation::BOOL, cchupn: u32, pszupn: super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsDeviceRegisteredWithManagement(pfisdeviceregisteredwithmanagement: *mut super::super::Foundation::BOOL, cchupn: u32, pszupn: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        IsDeviceRegisteredWithManagement(::std::mem::transmute(pfisdeviceregisteredwithmanagement), ::std::mem::transmute(cchupn), ::std::mem::transmute(pszupn)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn IsManagementRegistrationAllowed() -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsManagementRegistrationAllowed(pfismanagementregistrationallowed: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        IsManagementRegistrationAllowed(&mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn IsMdmUxWithoutAadAllowed() -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsMdmUxWithoutAadAllowed(isenrollmentallowed: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        IsMdmUxWithoutAadAllowed(&mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
pub struct MANAGEMENT_REGISTRATION_INFO {
    pub fDeviceRegisteredWithManagement: super::super::Foundation::BOOL,
    pub dwDeviceRegistionKind: u32,
    pub pszUPN: super::super::Foundation::PWSTR,
    pub pszMDMServiceUri: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl MANAGEMENT_REGISTRATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MANAGEMENT_REGISTRATION_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MANAGEMENT_REGISTRATION_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MANAGEMENT_REGISTRATION_INFO").field("fDeviceRegisteredWithManagement", &self.fDeviceRegisteredWithManagement).field("dwDeviceRegistionKind", &self.dwDeviceRegistionKind).field("pszUPN", &self.pszUPN).field("pszMDMServiceUri", &self.pszMDMServiceUri).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MANAGEMENT_REGISTRATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.fDeviceRegisteredWithManagement == other.fDeviceRegisteredWithManagement && self.dwDeviceRegistionKind == other.dwDeviceRegistionKind && self.pszUPN == other.pszUPN && self.pszMDMServiceUri == other.pszMDMServiceUri
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MANAGEMENT_REGISTRATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MANAGEMENT_REGISTRATION_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
pub struct MANAGEMENT_SERVICE_INFO {
    pub pszMDMServiceUri: super::super::Foundation::PWSTR,
    pub pszAuthenticationUri: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl MANAGEMENT_SERVICE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MANAGEMENT_SERVICE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MANAGEMENT_SERVICE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MANAGEMENT_SERVICE_INFO").field("pszMDMServiceUri", &self.pszMDMServiceUri).field("pszAuthenticationUri", &self.pszAuthenticationUri).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MANAGEMENT_SERVICE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszMDMServiceUri == other.pszMDMServiceUri && self.pszAuthenticationUri == other.pszAuthenticationUri
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MANAGEMENT_SERVICE_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MANAGEMENT_SERVICE_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MDM_REGISTRATION_FACILITY_CODE: u32 = 25u32;
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_CERTAUTH_FAILED_TO_FIND_CERT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910744i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_CERTPOLICY_PRIVATEKEYCREATION_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910745i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_CONNECTIVITY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910768i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_DEVICEAPREACHED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910765i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_DEVICECAPREACHED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910765i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_DEVICENOTSUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910764i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_DEVICE_ALREADY_ENROLLED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910774i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_DEVICE_AUTHENTICATION_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910782i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_DEVICE_AUTHORIZATION_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910781i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_DEVICE_CERTIFCATEREQUEST_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910780i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_DEVICE_CERTIFICATEREQUEST_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910780i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_DEVICE_CONFIGMGRSERVER_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910779i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_DEVICE_INTERNALSERVICE_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910778i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_DEVICE_INVALIDSECURITY_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910777i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_DEVICE_MANAGEMENT_BLOCKED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910746i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_DEVICE_MESSAGE_FORMAT_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910783i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_DEVICE_NOT_ENROLLED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910773i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_DEVICE_UNKNOWN_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910776i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_DISCOVERY_SEC_CERT_DATE_INVALID: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910771i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_EMPTY_MESSAGE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910743i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_ENROLLMENTDATAINVALID: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910759i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_ENROLLMENT_IN_PROGRESS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910775i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_INMAINTENANCE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910761i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_INSECUREREDIRECT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910758i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_INVALIDSSLCERT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910766i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_MDM_NOT_CONFIGURED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910735i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_NOTELIGIBLETORENEW: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910762i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_NOTSUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910763i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_NOT_SUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910763i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_PASSWORD_NEEDED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910770i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_PLATFORM_LICENSE_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910756i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_PLATFORM_UNKNOWN_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910755i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_PLATFORM_WRONG_STATE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910757i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_PROV_CSP_APPMGMT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910747i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_PROV_CSP_CERTSTORE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910754i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_PROV_CSP_DMCLIENT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910752i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_PROV_CSP_MISC: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910750i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_PROV_CSP_PFW: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910751i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_PROV_CSP_W7: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910753i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_PROV_SSLCERTNOTFOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910748i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_PROV_UNKNOWN: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910749i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_USERLICENSE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910760i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_USER_CANCELED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910742i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_USER_CANCELLED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910736i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_USER_LICENSE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910760i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MENROLL_E_WAB_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145910769i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MREGISTER_E_DEVICE_ALREADY_REGISTERED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145845238i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MREGISTER_E_DEVICE_AUTHENTICATION_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145845246i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MREGISTER_E_DEVICE_AUTHORIZATION_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145845245i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MREGISTER_E_DEVICE_CERTIFCATEREQUEST_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145845244i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MREGISTER_E_DEVICE_CONFIGMGRSERVER_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145845243i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MREGISTER_E_DEVICE_INTERNALSERVICE_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145845242i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MREGISTER_E_DEVICE_INVALIDSECURITY_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145845241i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MREGISTER_E_DEVICE_MESSAGE_FORMAT_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145845247i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MREGISTER_E_DEVICE_NOT_AD_REGISTERED_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145845235i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MREGISTER_E_DEVICE_NOT_REGISTERED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145845237i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MREGISTER_E_DEVICE_UNKNOWN_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145845240i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MREGISTER_E_DISCOVERY_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145845234i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MREGISTER_E_DISCOVERY_REDIRECTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145845236i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
pub const MREGISTER_E_REGISTRATION_IN_PROGRESS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2145845239i32 as _);
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct REGISTRATION_INFORMATION_CLASS(pub i32);
pub const DeviceRegistrationBasicInfo: REGISTRATION_INFORMATION_CLASS = REGISTRATION_INFORMATION_CLASS(1i32);
pub const MaxDeviceInfoClass: REGISTRATION_INFORMATION_CLASS = REGISTRATION_INFORMATION_CLASS(2i32);
impl ::std::convert::From<i32> for REGISTRATION_INFORMATION_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for REGISTRATION_INFORMATION_CLASS {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn RegisterDeviceWithLocalManagement() -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterDeviceWithLocalManagement(alreadyregistered: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        RegisterDeviceWithLocalManagement(&mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn RegisterDeviceWithManagement<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pszupn: Param0, ppszmdmserviceuri: Param1, ppzsaccesstoken: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterDeviceWithManagement(pszupn: super::super::Foundation::PWSTR, ppszmdmserviceuri: super::super::Foundation::PWSTR, ppzsaccesstoken: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        RegisterDeviceWithManagement(pszupn.into_param().abi(), ppszmdmserviceuri.into_param().abi(), ppzsaccesstoken.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn RegisterDeviceWithManagementUsingAADCredentials<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(usertoken: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterDeviceWithManagementUsingAADCredentials(usertoken: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
        }
        RegisterDeviceWithManagementUsingAADCredentials(usertoken.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
#[inline]
pub unsafe fn RegisterDeviceWithManagementUsingAADDeviceCredentials() -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterDeviceWithManagementUsingAADDeviceCredentials() -> ::windows::runtime::HRESULT;
        }
        RegisterDeviceWithManagementUsingAADDeviceCredentials().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn RegisterDeviceWithManagementUsingAADDeviceCredentials2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(mdmapplicationid: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterDeviceWithManagementUsingAADDeviceCredentials2(mdmapplicationid: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        RegisterDeviceWithManagementUsingAADDeviceCredentials2(mdmapplicationid.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn SetDeviceManagementConfigInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(providerid: Param0, configstring: Param1) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetDeviceManagementConfigInfo(providerid: super::super::Foundation::PWSTR, configstring: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        SetDeviceManagementConfigInfo(providerid.into_param().abi(), configstring.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn SetManagedExternally<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(ismanagedexternally: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetManagedExternally(ismanagedexternally: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        SetManagedExternally(ismanagedexternally.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
#[inline]
pub unsafe fn UnregisterDeviceWithLocalManagement() -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterDeviceWithLocalManagement() -> ::windows::runtime::HRESULT;
        }
        UnregisterDeviceWithLocalManagement().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn UnregisterDeviceWithManagement<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(enrollmentid: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterDeviceWithManagement(enrollmentid: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        UnregisterDeviceWithManagement(enrollmentid.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
