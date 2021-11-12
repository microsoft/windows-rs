#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyLocalManagementSyncML<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(syncmlrequest: Param0) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ApplyLocalManagementSyncML(syncmlrequest: super::super::Foundation::PWSTR, syncmlresult: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        ApplyLocalManagementSyncML(syncmlrequest.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const DEVICEREGISTRATIONTYPE_MAM: u32 = 5u32;
pub const DEVICEREGISTRATIONTYPE_MDM_DEVICEWIDE_WITH_AAD: u32 = 6u32;
pub const DEVICEREGISTRATIONTYPE_MDM_ONLY: u32 = 0u32;
pub const DEVICEREGISTRATIONTYPE_MDM_USERSPECIFIC_WITH_AAD: u32 = 13u32;
pub const DEVICE_ENROLLER_FACILITY_CODE: u32 = 24u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DiscoverManagementService<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pszupn: Param0) -> ::windows::core::Result<*mut MANAGEMENT_SERVICE_INFO> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DiscoverManagementService(pszupn: super::super::Foundation::PWSTR, ppmgmtinfo: *mut *mut MANAGEMENT_SERVICE_INFO) -> ::windows::core::HRESULT;
        }
        let mut result__: <*mut MANAGEMENT_SERVICE_INFO as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        DiscoverManagementService(pszupn.into_param().abi(), &mut result__).from_abi::<*mut MANAGEMENT_SERVICE_INFO>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DiscoverManagementServiceEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pszupn: Param0, pszdiscoveryservicecandidate: Param1) -> ::windows::core::Result<*mut MANAGEMENT_SERVICE_INFO> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DiscoverManagementServiceEx(pszupn: super::super::Foundation::PWSTR, pszdiscoveryservicecandidate: super::super::Foundation::PWSTR, ppmgmtinfo: *mut *mut MANAGEMENT_SERVICE_INFO) -> ::windows::core::HRESULT;
        }
        let mut result__: <*mut MANAGEMENT_SERVICE_INFO as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        DiscoverManagementServiceEx(pszupn.into_param().abi(), pszdiscoveryservicecandidate.into_param().abi(), &mut result__).from_abi::<*mut MANAGEMENT_SERVICE_INFO>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDeviceManagementConfigInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(providerid: Param0, configstringbufferlength: *mut u32, configstring: super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDeviceManagementConfigInfo(providerid: super::super::Foundation::PWSTR, configstringbufferlength: *mut u32, configstring: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        GetDeviceManagementConfigInfo(providerid.into_param().abi(), ::core::mem::transmute(configstringbufferlength), ::core::mem::transmute(configstring)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetDeviceRegistrationInfo(deviceinformationclass: REGISTRATION_INFORMATION_CLASS, ppdeviceregistrationinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDeviceRegistrationInfo(deviceinformationclass: REGISTRATION_INFORMATION_CLASS, ppdeviceregistrationinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        GetDeviceRegistrationInfo(::core::mem::transmute(deviceinformationclass), ::core::mem::transmute(ppdeviceregistrationinfo)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetManagementAppHyperlink(cchhyperlink: u32, pszhyperlink: super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetManagementAppHyperlink(cchhyperlink: u32, pszhyperlink: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        GetManagementAppHyperlink(::core::mem::transmute(cchhyperlink), ::core::mem::transmute(pszhyperlink)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsDeviceRegisteredWithManagement(pfisdeviceregisteredwithmanagement: *mut super::super::Foundation::BOOL, cchupn: u32, pszupn: super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsDeviceRegisteredWithManagement(pfisdeviceregisteredwithmanagement: *mut super::super::Foundation::BOOL, cchupn: u32, pszupn: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        IsDeviceRegisteredWithManagement(::core::mem::transmute(pfisdeviceregisteredwithmanagement), ::core::mem::transmute(cchupn), ::core::mem::transmute(pszupn)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsManagementRegistrationAllowed() -> ::windows::core::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsManagementRegistrationAllowed(pfismanagementregistrationallowed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        IsManagementRegistrationAllowed(&mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsMdmUxWithoutAadAllowed() -> ::windows::core::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsMdmUxWithoutAadAllowed(isenrollmentallowed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        IsMdmUxWithoutAadAllowed(&mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MANAGEMENT_REGISTRATION_INFO {
    pub fDeviceRegisteredWithManagement: super::super::Foundation::BOOL,
    pub dwDeviceRegistionKind: u32,
    pub pszUPN: super::super::Foundation::PWSTR,
    pub pszMDMServiceUri: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl MANAGEMENT_REGISTRATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MANAGEMENT_REGISTRATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MANAGEMENT_REGISTRATION_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MANAGEMENT_REGISTRATION_INFO").field("fDeviceRegisteredWithManagement", &self.fDeviceRegisteredWithManagement).field("dwDeviceRegistionKind", &self.dwDeviceRegistionKind).field("pszUPN", &self.pszUPN).field("pszMDMServiceUri", &self.pszMDMServiceUri).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MANAGEMENT_REGISTRATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.fDeviceRegisteredWithManagement == other.fDeviceRegisteredWithManagement && self.dwDeviceRegistionKind == other.dwDeviceRegistionKind && self.pszUPN == other.pszUPN && self.pszMDMServiceUri == other.pszMDMServiceUri
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MANAGEMENT_REGISTRATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MANAGEMENT_REGISTRATION_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MANAGEMENT_SERVICE_INFO {
    pub pszMDMServiceUri: super::super::Foundation::PWSTR,
    pub pszAuthenticationUri: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl MANAGEMENT_SERVICE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MANAGEMENT_SERVICE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MANAGEMENT_SERVICE_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MANAGEMENT_SERVICE_INFO").field("pszMDMServiceUri", &self.pszMDMServiceUri).field("pszAuthenticationUri", &self.pszAuthenticationUri).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MANAGEMENT_SERVICE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszMDMServiceUri == other.pszMDMServiceUri && self.pszAuthenticationUri == other.pszAuthenticationUri
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MANAGEMENT_SERVICE_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MANAGEMENT_SERVICE_INFO {
    type Abi = Self;
}
pub const MDM_REGISTRATION_FACILITY_CODE: u32 = 25u32;
pub const MENROLL_E_CERTAUTH_FAILED_TO_FIND_CERT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910744i32 as _);
pub const MENROLL_E_CERTPOLICY_PRIVATEKEYCREATION_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910745i32 as _);
pub const MENROLL_E_CONNECTIVITY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910768i32 as _);
pub const MENROLL_E_DEVICEAPREACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910765i32 as _);
pub const MENROLL_E_DEVICECAPREACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910765i32 as _);
pub const MENROLL_E_DEVICENOTSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910764i32 as _);
pub const MENROLL_E_DEVICE_ALREADY_ENROLLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910774i32 as _);
pub const MENROLL_E_DEVICE_AUTHENTICATION_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910782i32 as _);
pub const MENROLL_E_DEVICE_AUTHORIZATION_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910781i32 as _);
pub const MENROLL_E_DEVICE_CERTIFCATEREQUEST_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910780i32 as _);
pub const MENROLL_E_DEVICE_CERTIFICATEREQUEST_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910780i32 as _);
pub const MENROLL_E_DEVICE_CONFIGMGRSERVER_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910779i32 as _);
pub const MENROLL_E_DEVICE_INTERNALSERVICE_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910778i32 as _);
pub const MENROLL_E_DEVICE_INVALIDSECURITY_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910777i32 as _);
pub const MENROLL_E_DEVICE_MANAGEMENT_BLOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910746i32 as _);
pub const MENROLL_E_DEVICE_MESSAGE_FORMAT_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910783i32 as _);
pub const MENROLL_E_DEVICE_NOT_ENROLLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910773i32 as _);
pub const MENROLL_E_DEVICE_UNKNOWN_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910776i32 as _);
pub const MENROLL_E_DISCOVERY_SEC_CERT_DATE_INVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910771i32 as _);
pub const MENROLL_E_EMPTY_MESSAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910743i32 as _);
pub const MENROLL_E_ENROLLMENTDATAINVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910759i32 as _);
pub const MENROLL_E_ENROLLMENT_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910775i32 as _);
pub const MENROLL_E_INMAINTENANCE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910761i32 as _);
pub const MENROLL_E_INSECUREREDIRECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910758i32 as _);
pub const MENROLL_E_INVALIDSSLCERT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910766i32 as _);
pub const MENROLL_E_MDM_NOT_CONFIGURED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910735i32 as _);
pub const MENROLL_E_NOTELIGIBLETORENEW: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910762i32 as _);
pub const MENROLL_E_NOTSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910763i32 as _);
pub const MENROLL_E_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910763i32 as _);
pub const MENROLL_E_PASSWORD_NEEDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910770i32 as _);
pub const MENROLL_E_PLATFORM_LICENSE_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910756i32 as _);
pub const MENROLL_E_PLATFORM_UNKNOWN_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910755i32 as _);
pub const MENROLL_E_PLATFORM_WRONG_STATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910757i32 as _);
pub const MENROLL_E_PROV_CSP_APPMGMT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910747i32 as _);
pub const MENROLL_E_PROV_CSP_CERTSTORE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910754i32 as _);
pub const MENROLL_E_PROV_CSP_DMCLIENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910752i32 as _);
pub const MENROLL_E_PROV_CSP_MISC: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910750i32 as _);
pub const MENROLL_E_PROV_CSP_PFW: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910751i32 as _);
pub const MENROLL_E_PROV_CSP_W7: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910753i32 as _);
pub const MENROLL_E_PROV_SSLCERTNOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910748i32 as _);
pub const MENROLL_E_PROV_UNKNOWN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910749i32 as _);
pub const MENROLL_E_USERLICENSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910760i32 as _);
pub const MENROLL_E_USER_CANCELED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910742i32 as _);
pub const MENROLL_E_USER_CANCELLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910736i32 as _);
pub const MENROLL_E_USER_LICENSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910760i32 as _);
pub const MENROLL_E_WAB_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910769i32 as _);
pub const MREGISTER_E_DEVICE_ALREADY_REGISTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145845238i32 as _);
pub const MREGISTER_E_DEVICE_AUTHENTICATION_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145845246i32 as _);
pub const MREGISTER_E_DEVICE_AUTHORIZATION_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145845245i32 as _);
pub const MREGISTER_E_DEVICE_CERTIFCATEREQUEST_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145845244i32 as _);
pub const MREGISTER_E_DEVICE_CONFIGMGRSERVER_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145845243i32 as _);
pub const MREGISTER_E_DEVICE_INTERNALSERVICE_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145845242i32 as _);
pub const MREGISTER_E_DEVICE_INVALIDSECURITY_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145845241i32 as _);
pub const MREGISTER_E_DEVICE_MESSAGE_FORMAT_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145845247i32 as _);
pub const MREGISTER_E_DEVICE_NOT_AD_REGISTERED_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145845235i32 as _);
pub const MREGISTER_E_DEVICE_NOT_REGISTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145845237i32 as _);
pub const MREGISTER_E_DEVICE_UNKNOWN_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145845240i32 as _);
pub const MREGISTER_E_DISCOVERY_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145845234i32 as _);
pub const MREGISTER_E_DISCOVERY_REDIRECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145845236i32 as _);
pub const MREGISTER_E_REGISTRATION_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145845239i32 as _);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct REGISTRATION_INFORMATION_CLASS(pub i32);
pub const DeviceRegistrationBasicInfo: REGISTRATION_INFORMATION_CLASS = REGISTRATION_INFORMATION_CLASS(1i32);
pub const MaxDeviceInfoClass: REGISTRATION_INFORMATION_CLASS = REGISTRATION_INFORMATION_CLASS(2i32);
impl ::core::convert::From<i32> for REGISTRATION_INFORMATION_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for REGISTRATION_INFORMATION_CLASS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterDeviceWithLocalManagement() -> ::windows::core::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterDeviceWithLocalManagement(alreadyregistered: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        RegisterDeviceWithLocalManagement(&mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterDeviceWithManagement<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pszupn: Param0, ppszmdmserviceuri: Param1, ppzsaccesstoken: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterDeviceWithManagement(pszupn: super::super::Foundation::PWSTR, ppszmdmserviceuri: super::super::Foundation::PWSTR, ppzsaccesstoken: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        RegisterDeviceWithManagement(pszupn.into_param().abi(), ppszmdmserviceuri.into_param().abi(), ppzsaccesstoken.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterDeviceWithManagementUsingAADCredentials<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(usertoken: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterDeviceWithManagementUsingAADCredentials(usertoken: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
        }
        RegisterDeviceWithManagementUsingAADCredentials(usertoken.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RegisterDeviceWithManagementUsingAADDeviceCredentials() -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterDeviceWithManagementUsingAADDeviceCredentials() -> ::windows::core::HRESULT;
        }
        RegisterDeviceWithManagementUsingAADDeviceCredentials().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterDeviceWithManagementUsingAADDeviceCredentials2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(mdmapplicationid: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterDeviceWithManagementUsingAADDeviceCredentials2(mdmapplicationid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        RegisterDeviceWithManagementUsingAADDeviceCredentials2(mdmapplicationid.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDeviceManagementConfigInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(providerid: Param0, configstring: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetDeviceManagementConfigInfo(providerid: super::super::Foundation::PWSTR, configstring: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        SetDeviceManagementConfigInfo(providerid.into_param().abi(), configstring.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetManagedExternally<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(ismanagedexternally: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetManagedExternally(ismanagedexternally: super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        SetManagedExternally(ismanagedexternally.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn UnregisterDeviceWithLocalManagement() -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterDeviceWithLocalManagement() -> ::windows::core::HRESULT;
        }
        UnregisterDeviceWithLocalManagement().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnregisterDeviceWithManagement<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(enrollmentid: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterDeviceWithManagement(enrollmentid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        UnregisterDeviceWithManagement(enrollmentid.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
