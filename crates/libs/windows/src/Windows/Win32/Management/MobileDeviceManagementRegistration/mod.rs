#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
#[inline]
pub unsafe fn ApplyLocalManagementSyncML<P0>(syncmlrequest: P0, syncmlresult: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "mdmlocalmanagement.dll""system" fn ApplyLocalManagementSyncML ( syncmlrequest : :: windows::core::PCWSTR , syncmlresult : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    ApplyLocalManagementSyncML(syncmlrequest.into_param().abi(), ::core::mem::transmute(syncmlresult.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
#[inline]
pub unsafe fn DiscoverManagementService<P0>(pszupn: P0) -> ::windows::core::Result<*mut MANAGEMENT_SERVICE_INFO>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "mdmregistration.dll""system" fn DiscoverManagementService ( pszupn : :: windows::core::PCWSTR , ppmgmtinfo : *mut *mut MANAGEMENT_SERVICE_INFO ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<*mut MANAGEMENT_SERVICE_INFO>();
    DiscoverManagementService(pszupn.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
#[inline]
pub unsafe fn DiscoverManagementServiceEx<P0, P1>(pszupn: P0, pszdiscoveryservicecandidate: P1) -> ::windows::core::Result<*mut MANAGEMENT_SERVICE_INFO>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "mdmregistration.dll""system" fn DiscoverManagementServiceEx ( pszupn : :: windows::core::PCWSTR , pszdiscoveryservicecandidate : :: windows::core::PCWSTR , ppmgmtinfo : *mut *mut MANAGEMENT_SERVICE_INFO ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<*mut MANAGEMENT_SERVICE_INFO>();
    DiscoverManagementServiceEx(pszupn.into_param().abi(), pszdiscoveryservicecandidate.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
#[inline]
pub unsafe fn GetDeviceManagementConfigInfo<P0>(providerid: P0, configstringbufferlength: *mut u32, configstring: ::windows::core::PWSTR) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "mdmregistration.dll""system" fn GetDeviceManagementConfigInfo ( providerid : :: windows::core::PCWSTR , configstringbufferlength : *mut u32 , configstring : :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    GetDeviceManagementConfigInfo(providerid.into_param().abi(), configstringbufferlength, ::core::mem::transmute(configstring)).ok()
}
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
#[inline]
pub unsafe fn GetDeviceRegistrationInfo(deviceinformationclass: REGISTRATION_INFORMATION_CLASS, ppdeviceregistrationinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "mdmregistration.dll""system" fn GetDeviceRegistrationInfo ( deviceinformationclass : REGISTRATION_INFORMATION_CLASS , ppdeviceregistrationinfo : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    GetDeviceRegistrationInfo(deviceinformationclass, ppdeviceregistrationinfo).ok()
}
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
#[inline]
pub unsafe fn GetManagementAppHyperlink(pszhyperlink: &mut [u16]) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "mdmregistration.dll""system" fn GetManagementAppHyperlink ( cchhyperlink : u32 , pszhyperlink : :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    GetManagementAppHyperlink(pszhyperlink.len() as _, ::core::mem::transmute(pszhyperlink.as_ptr())).ok()
}
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsDeviceRegisteredWithManagement(pfisdeviceregisteredwithmanagement: *mut super::super::Foundation::BOOL, pszupn: ::core::option::Option<&mut [u16]>) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "mdmregistration.dll""system" fn IsDeviceRegisteredWithManagement ( pfisdeviceregisteredwithmanagement : *mut super::super::Foundation:: BOOL , cchupn : u32 , pszupn : :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    IsDeviceRegisteredWithManagement(pfisdeviceregisteredwithmanagement, pszupn.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pszupn.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr()))).ok()
}
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsManagementRegistrationAllowed() -> ::windows::core::Result<super::super::Foundation::BOOL> {
    ::windows::imp::link ! ( "mdmregistration.dll""system" fn IsManagementRegistrationAllowed ( pfismanagementregistrationallowed : *mut super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
    IsManagementRegistrationAllowed(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsMdmUxWithoutAadAllowed() -> ::windows::core::Result<super::super::Foundation::BOOL> {
    ::windows::imp::link ! ( "mdmregistration.dll""system" fn IsMdmUxWithoutAadAllowed ( isenrollmentallowed : *mut super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
    IsMdmUxWithoutAadAllowed(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterDeviceWithLocalManagement(alreadyregistered: ::core::option::Option<*mut super::super::Foundation::BOOL>) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "mdmlocalmanagement.dll""system" fn RegisterDeviceWithLocalManagement ( alreadyregistered : *mut super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    RegisterDeviceWithLocalManagement(::core::mem::transmute(alreadyregistered.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
#[inline]
pub unsafe fn RegisterDeviceWithManagement<P0, P1, P2>(pszupn: P0, ppszmdmserviceuri: P1, ppzsaccesstoken: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "mdmregistration.dll""system" fn RegisterDeviceWithManagement ( pszupn : :: windows::core::PCWSTR , ppszmdmserviceuri : :: windows::core::PCWSTR , ppzsaccesstoken : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    RegisterDeviceWithManagement(pszupn.into_param().abi(), ppszmdmserviceuri.into_param().abi(), ppzsaccesstoken.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterDeviceWithManagementUsingAADCredentials<P0>(usertoken: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "mdmregistration.dll""system" fn RegisterDeviceWithManagementUsingAADCredentials ( usertoken : super::super::Foundation:: HANDLE ) -> :: windows::core::HRESULT );
    RegisterDeviceWithManagementUsingAADCredentials(usertoken.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
#[inline]
pub unsafe fn RegisterDeviceWithManagementUsingAADDeviceCredentials() -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "mdmregistration.dll""system" fn RegisterDeviceWithManagementUsingAADDeviceCredentials ( ) -> :: windows::core::HRESULT );
    RegisterDeviceWithManagementUsingAADDeviceCredentials().ok()
}
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
#[inline]
pub unsafe fn RegisterDeviceWithManagementUsingAADDeviceCredentials2<P0>(mdmapplicationid: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "mdmregistration.dll""system" fn RegisterDeviceWithManagementUsingAADDeviceCredentials2 ( mdmapplicationid : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    RegisterDeviceWithManagementUsingAADDeviceCredentials2(mdmapplicationid.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
#[inline]
pub unsafe fn SetDeviceManagementConfigInfo<P0, P1>(providerid: P0, configstring: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "mdmregistration.dll""system" fn SetDeviceManagementConfigInfo ( providerid : :: windows::core::PCWSTR , configstring : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    SetDeviceManagementConfigInfo(providerid.into_param().abi(), configstring.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetManagedExternally<P0>(ismanagedexternally: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "mdmregistration.dll""system" fn SetManagedExternally ( ismanagedexternally : super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    SetManagedExternally(ismanagedexternally.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
#[inline]
pub unsafe fn UnregisterDeviceWithLocalManagement() -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "mdmlocalmanagement.dll""system" fn UnregisterDeviceWithLocalManagement ( ) -> :: windows::core::HRESULT );
    UnregisterDeviceWithLocalManagement().ok()
}
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
#[inline]
pub unsafe fn UnregisterDeviceWithManagement<P0>(enrollmentid: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "mdmregistration.dll""system" fn UnregisterDeviceWithManagement ( enrollmentid : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    UnregisterDeviceWithManagement(enrollmentid.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const DEVICEREGISTRATIONTYPE_MAM: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const DEVICEREGISTRATIONTYPE_MDM_DEVICEWIDE_WITH_AAD: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const DEVICEREGISTRATIONTYPE_MDM_ONLY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const DEVICEREGISTRATIONTYPE_MDM_USERSPECIFIC_WITH_AAD: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const DEVICE_ENROLLER_FACILITY_CODE: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MDM_REGISTRATION_FACILITY_CODE: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_CERTAUTH_FAILED_TO_FIND_CERT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910744i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_CERTPOLICY_PRIVATEKEYCREATION_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910745i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_CONNECTIVITY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910768i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_DEVICEAPREACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910765i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_DEVICECAPREACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910765i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_DEVICENOTSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910764i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_DEVICE_ALREADY_ENROLLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910774i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_DEVICE_AUTHENTICATION_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910782i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_DEVICE_AUTHORIZATION_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910781i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_DEVICE_CERTIFCATEREQUEST_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910780i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_DEVICE_CERTIFICATEREQUEST_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910780i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_DEVICE_CONFIGMGRSERVER_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910779i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_DEVICE_INTERNALSERVICE_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910778i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_DEVICE_INVALIDSECURITY_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910777i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_DEVICE_MANAGEMENT_BLOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910746i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_DEVICE_MESSAGE_FORMAT_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910783i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_DEVICE_NOT_ENROLLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910773i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_DEVICE_UNKNOWN_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910776i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_DISCOVERY_SEC_CERT_DATE_INVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910771i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_EMPTY_MESSAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910743i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_ENROLLMENTDATAINVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910759i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_ENROLLMENT_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910775i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_INMAINTENANCE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910761i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_INSECUREREDIRECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910758i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_INVALIDSSLCERT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910766i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_MDM_NOT_CONFIGURED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910735i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_NOTELIGIBLETORENEW: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910762i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_NOTSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910763i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910763i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_PASSWORD_NEEDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910770i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_PLATFORM_LICENSE_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910756i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_PLATFORM_UNKNOWN_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910755i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_PLATFORM_WRONG_STATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910757i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_PROV_CSP_APPMGMT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910747i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_PROV_CSP_CERTSTORE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910754i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_PROV_CSP_DMCLIENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910752i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_PROV_CSP_MISC: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910750i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_PROV_CSP_PFW: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910751i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_PROV_CSP_W7: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910753i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_PROV_SSLCERTNOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910748i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_PROV_UNKNOWN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910749i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_USERLICENSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910760i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_USER_CANCELED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910742i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_USER_CANCELLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910736i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_USER_LICENSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910760i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MENROLL_E_WAB_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145910769i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MREGISTER_E_DEVICE_ALREADY_REGISTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145845238i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MREGISTER_E_DEVICE_AUTHENTICATION_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145845246i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MREGISTER_E_DEVICE_AUTHORIZATION_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145845245i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MREGISTER_E_DEVICE_CERTIFCATEREQUEST_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145845244i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MREGISTER_E_DEVICE_CONFIGMGRSERVER_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145845243i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MREGISTER_E_DEVICE_INTERNALSERVICE_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145845242i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MREGISTER_E_DEVICE_INVALIDSECURITY_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145845241i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MREGISTER_E_DEVICE_MESSAGE_FORMAT_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145845247i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MREGISTER_E_DEVICE_NOT_AD_REGISTERED_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145845235i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MREGISTER_E_DEVICE_NOT_REGISTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145845237i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MREGISTER_E_DEVICE_UNKNOWN_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145845240i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MREGISTER_E_DISCOVERY_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145845234i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MREGISTER_E_DISCOVERY_REDIRECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145845236i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MREGISTER_E_REGISTRATION_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145845239i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct REGISTRATION_INFORMATION_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const DeviceRegistrationBasicInfo: REGISTRATION_INFORMATION_CLASS = REGISTRATION_INFORMATION_CLASS(1i32);
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub const MaxDeviceInfoClass: REGISTRATION_INFORMATION_CLASS = REGISTRATION_INFORMATION_CLASS(2i32);
impl ::core::marker::Copy for REGISTRATION_INFORMATION_CLASS {}
impl ::core::clone::Clone for REGISTRATION_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REGISTRATION_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for REGISTRATION_INFORMATION_CLASS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for REGISTRATION_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REGISTRATION_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MANAGEMENT_REGISTRATION_INFO {
    pub fDeviceRegisteredWithManagement: super::super::Foundation::BOOL,
    pub dwDeviceRegistionKind: u32,
    pub pszUPN: ::windows::core::PWSTR,
    pub pszMDMServiceUri: ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MANAGEMENT_REGISTRATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MANAGEMENT_REGISTRATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MANAGEMENT_REGISTRATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MANAGEMENT_REGISTRATION_INFO").field("fDeviceRegisteredWithManagement", &self.fDeviceRegisteredWithManagement).field("dwDeviceRegistionKind", &self.dwDeviceRegistionKind).field("pszUPN", &self.pszUPN).field("pszMDMServiceUri", &self.pszMDMServiceUri).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for MANAGEMENT_REGISTRATION_INFO {
    type TypeKind = ::windows::core::CopyType;
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
impl ::core::default::Default for MANAGEMENT_REGISTRATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Management_MobileDeviceManagementRegistration\"`*"]
pub struct MANAGEMENT_SERVICE_INFO {
    pub pszMDMServiceUri: ::windows::core::PWSTR,
    pub pszAuthenticationUri: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for MANAGEMENT_SERVICE_INFO {}
impl ::core::clone::Clone for MANAGEMENT_SERVICE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MANAGEMENT_SERVICE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MANAGEMENT_SERVICE_INFO").field("pszMDMServiceUri", &self.pszMDMServiceUri).field("pszAuthenticationUri", &self.pszAuthenticationUri).finish()
    }
}
impl ::windows::core::TypeKind for MANAGEMENT_SERVICE_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MANAGEMENT_SERVICE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszMDMServiceUri == other.pszMDMServiceUri && self.pszAuthenticationUri == other.pszAuthenticationUri
    }
}
impl ::core::cmp::Eq for MANAGEMENT_SERVICE_INFO {}
impl ::core::default::Default for MANAGEMENT_SERVICE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
