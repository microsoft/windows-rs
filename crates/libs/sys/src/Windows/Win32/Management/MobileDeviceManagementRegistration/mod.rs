::windows_targets::link!("mdmlocalmanagement.dll" "system" fn ApplyLocalManagementSyncML(syncmlrequest : ::windows_sys::core::PCWSTR, syncmlresult : *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("mdmregistration.dll" "system" fn DiscoverManagementService(pszupn : ::windows_sys::core::PCWSTR, ppmgmtinfo : *mut *mut MANAGEMENT_SERVICE_INFO) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("mdmregistration.dll" "system" fn DiscoverManagementServiceEx(pszupn : ::windows_sys::core::PCWSTR, pszdiscoveryservicecandidate : ::windows_sys::core::PCWSTR, ppmgmtinfo : *mut *mut MANAGEMENT_SERVICE_INFO) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("mdmregistration.dll" "system" fn GetDeviceManagementConfigInfo(providerid : ::windows_sys::core::PCWSTR, configstringbufferlength : *mut u32, configstring : ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("mdmregistration.dll" "system" fn GetDeviceRegistrationInfo(deviceinformationclass : REGISTRATION_INFORMATION_CLASS, ppdeviceregistrationinfo : *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("mdmregistration.dll" "system" fn GetManagementAppHyperlink(cchhyperlink : u32, pszhyperlink : ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("mdmregistration.dll" "system" fn IsDeviceRegisteredWithManagement(pfisdeviceregisteredwithmanagement : *mut super::super::Foundation:: BOOL, cchupn : u32, pszupn : ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("mdmregistration.dll" "system" fn IsManagementRegistrationAllowed(pfismanagementregistrationallowed : *mut super::super::Foundation:: BOOL) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("mdmregistration.dll" "system" fn IsMdmUxWithoutAadAllowed(isenrollmentallowed : *mut super::super::Foundation:: BOOL) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("mdmlocalmanagement.dll" "system" fn RegisterDeviceWithLocalManagement(alreadyregistered : *mut super::super::Foundation:: BOOL) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("mdmregistration.dll" "system" fn RegisterDeviceWithManagement(pszupn : ::windows_sys::core::PCWSTR, ppszmdmserviceuri : ::windows_sys::core::PCWSTR, ppzsaccesstoken : ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("mdmregistration.dll" "system" fn RegisterDeviceWithManagementUsingAADCredentials(usertoken : super::super::Foundation:: HANDLE) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("mdmregistration.dll" "system" fn RegisterDeviceWithManagementUsingAADDeviceCredentials() -> ::windows_sys::core::HRESULT);
::windows_targets::link!("mdmregistration.dll" "system" fn RegisterDeviceWithManagementUsingAADDeviceCredentials2(mdmapplicationid : ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("mdmregistration.dll" "system" fn SetDeviceManagementConfigInfo(providerid : ::windows_sys::core::PCWSTR, configstring : ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("mdmregistration.dll" "system" fn SetManagedExternally(ismanagedexternally : super::super::Foundation:: BOOL) -> ::windows_sys::core::HRESULT);
::windows_targets::link!("mdmlocalmanagement.dll" "system" fn UnregisterDeviceWithLocalManagement() -> ::windows_sys::core::HRESULT);
::windows_targets::link!("mdmregistration.dll" "system" fn UnregisterDeviceWithManagement(enrollmentid : ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT);
pub const DEVICEREGISTRATIONTYPE_MAM: u32 = 5u32;
pub const DEVICEREGISTRATIONTYPE_MDM_DEVICEWIDE_WITH_AAD: u32 = 6u32;
pub const DEVICEREGISTRATIONTYPE_MDM_ONLY: u32 = 0u32;
pub const DEVICEREGISTRATIONTYPE_MDM_USERSPECIFIC_WITH_AAD: u32 = 13u32;
pub const DEVICE_ENROLLER_FACILITY_CODE: u32 = 24u32;
pub const DeviceRegistrationBasicInfo: REGISTRATION_INFORMATION_CLASS = 1i32;
pub const MDM_REGISTRATION_FACILITY_CODE: u32 = 25u32;
pub const MENROLL_E_CERTAUTH_FAILED_TO_FIND_CERT: ::windows_sys::core::HRESULT = 0x80180028u32 as _;
pub const MENROLL_E_CERTPOLICY_PRIVATEKEYCREATION_FAILED: ::windows_sys::core::HRESULT = 0x80180027u32 as _;
pub const MENROLL_E_CONNECTIVITY: ::windows_sys::core::HRESULT = 0x80180010u32 as _;
pub const MENROLL_E_CUSTOMSERVERERROR: ::windows_sys::core::HRESULT = 0x80180032u32 as _;
pub const MENROLL_E_DEVICECAPREACHED: ::windows_sys::core::HRESULT = 0x80180013u32 as _;
pub const MENROLL_E_DEVICENOTSUPPORTED: ::windows_sys::core::HRESULT = 0x80180014u32 as _;
pub const MENROLL_E_DEVICE_ALREADY_ENROLLED: ::windows_sys::core::HRESULT = 0x8018000Au32 as _;
pub const MENROLL_E_DEVICE_AUTHENTICATION_ERROR: ::windows_sys::core::HRESULT = 0x80180002u32 as _;
pub const MENROLL_E_DEVICE_AUTHORIZATION_ERROR: ::windows_sys::core::HRESULT = 0x80180003u32 as _;
pub const MENROLL_E_DEVICE_CERTIFCATEREQUEST_ERROR: ::windows_sys::core::HRESULT = 0x80180004u32 as _;
pub const MENROLL_E_DEVICE_CERTIFICATEREQUEST_ERROR: ::windows_sys::core::HRESULT = 0x80180004u32 as _;
pub const MENROLL_E_DEVICE_CONFIGMGRSERVER_ERROR: ::windows_sys::core::HRESULT = 0x80180005u32 as _;
pub const MENROLL_E_DEVICE_INTERNALSERVICE_ERROR: ::windows_sys::core::HRESULT = 0x80180006u32 as _;
pub const MENROLL_E_DEVICE_INVALIDSECURITY_ERROR: ::windows_sys::core::HRESULT = 0x80180007u32 as _;
pub const MENROLL_E_DEVICE_MANAGEMENT_BLOCKED: ::windows_sys::core::HRESULT = 0x80180026u32 as _;
pub const MENROLL_E_DEVICE_MESSAGE_FORMAT_ERROR: ::windows_sys::core::HRESULT = 0x80180001u32 as _;
pub const MENROLL_E_DEVICE_NOT_ENROLLED: ::windows_sys::core::HRESULT = 0x8018000Bu32 as _;
pub const MENROLL_E_DEVICE_UNKNOWN_ERROR: ::windows_sys::core::HRESULT = 0x80180008u32 as _;
pub const MENROLL_E_DISCOVERY_SEC_CERT_DATE_INVALID: ::windows_sys::core::HRESULT = 0x8018000Du32 as _;
pub const MENROLL_E_EMPTY_MESSAGE: ::windows_sys::core::HRESULT = 0x80180029u32 as _;
pub const MENROLL_E_ENROLLMENTDATAINVALID: ::windows_sys::core::HRESULT = 0x80180019u32 as _;
pub const MENROLL_E_ENROLLMENT_IN_PROGRESS: ::windows_sys::core::HRESULT = 0x80180009u32 as _;
pub const MENROLL_E_INMAINTENANCE: ::windows_sys::core::HRESULT = 0x80180017u32 as _;
pub const MENROLL_E_INSECUREREDIRECT: ::windows_sys::core::HRESULT = 0x8018001Au32 as _;
pub const MENROLL_E_INVALIDSSLCERT: ::windows_sys::core::HRESULT = 0x80180012u32 as _;
pub const MENROLL_E_MDM_NOT_CONFIGURED: ::windows_sys::core::HRESULT = 0x80180031u32 as _;
pub const MENROLL_E_NOTELIGIBLETORENEW: ::windows_sys::core::HRESULT = 0x80180016u32 as _;
pub const MENROLL_E_NOTSUPPORTED: ::windows_sys::core::HRESULT = 0x80180015u32 as _;
pub const MENROLL_E_NOT_SUPPORTED: ::windows_sys::core::HRESULT = 0x80180015u32 as _;
pub const MENROLL_E_PASSWORD_NEEDED: ::windows_sys::core::HRESULT = 0x8018000Eu32 as _;
pub const MENROLL_E_PLATFORM_LICENSE_ERROR: ::windows_sys::core::HRESULT = 0x8018001Cu32 as _;
pub const MENROLL_E_PLATFORM_UNKNOWN_ERROR: ::windows_sys::core::HRESULT = 0x8018001Du32 as _;
pub const MENROLL_E_PLATFORM_WRONG_STATE: ::windows_sys::core::HRESULT = 0x8018001Bu32 as _;
pub const MENROLL_E_PROV_CSP_APPMGMT: ::windows_sys::core::HRESULT = 0x80180025u32 as _;
pub const MENROLL_E_PROV_CSP_CERTSTORE: ::windows_sys::core::HRESULT = 0x8018001Eu32 as _;
pub const MENROLL_E_PROV_CSP_DMCLIENT: ::windows_sys::core::HRESULT = 0x80180020u32 as _;
pub const MENROLL_E_PROV_CSP_MISC: ::windows_sys::core::HRESULT = 0x80180022u32 as _;
pub const MENROLL_E_PROV_CSP_PFW: ::windows_sys::core::HRESULT = 0x80180021u32 as _;
pub const MENROLL_E_PROV_CSP_W7: ::windows_sys::core::HRESULT = 0x8018001Fu32 as _;
pub const MENROLL_E_PROV_SSLCERTNOTFOUND: ::windows_sys::core::HRESULT = 0x80180024u32 as _;
pub const MENROLL_E_PROV_UNKNOWN: ::windows_sys::core::HRESULT = 0x80180023u32 as _;
pub const MENROLL_E_USERLICENSE: ::windows_sys::core::HRESULT = 0x80180018u32 as _;
pub const MENROLL_E_USER_CANCELED: ::windows_sys::core::HRESULT = 0x8018002Au32 as _;
pub const MENROLL_E_USER_CANCELLED: ::windows_sys::core::HRESULT = 0x80180030u32 as _;
pub const MENROLL_E_USER_LICENSE: ::windows_sys::core::HRESULT = 0x80180018u32 as _;
pub const MENROLL_E_WAB_ERROR: ::windows_sys::core::HRESULT = 0x8018000Fu32 as _;
pub const MREGISTER_E_DEVICE_ALREADY_REGISTERED: ::windows_sys::core::HRESULT = 0x8019000Au32 as _;
pub const MREGISTER_E_DEVICE_AUTHENTICATION_ERROR: ::windows_sys::core::HRESULT = 0x80190002u32 as _;
pub const MREGISTER_E_DEVICE_AUTHORIZATION_ERROR: ::windows_sys::core::HRESULT = 0x80190003u32 as _;
pub const MREGISTER_E_DEVICE_CERTIFCATEREQUEST_ERROR: ::windows_sys::core::HRESULT = 0x80190004u32 as _;
pub const MREGISTER_E_DEVICE_CONFIGMGRSERVER_ERROR: ::windows_sys::core::HRESULT = 0x80190005u32 as _;
pub const MREGISTER_E_DEVICE_INTERNALSERVICE_ERROR: ::windows_sys::core::HRESULT = 0x80190006u32 as _;
pub const MREGISTER_E_DEVICE_INVALIDSECURITY_ERROR: ::windows_sys::core::HRESULT = 0x80190007u32 as _;
pub const MREGISTER_E_DEVICE_MESSAGE_FORMAT_ERROR: ::windows_sys::core::HRESULT = 0x80190001u32 as _;
pub const MREGISTER_E_DEVICE_NOT_AD_REGISTERED_ERROR: ::windows_sys::core::HRESULT = 0x8019000Du32 as _;
pub const MREGISTER_E_DEVICE_NOT_REGISTERED: ::windows_sys::core::HRESULT = 0x8019000Bu32 as _;
pub const MREGISTER_E_DEVICE_UNKNOWN_ERROR: ::windows_sys::core::HRESULT = 0x80190008u32 as _;
pub const MREGISTER_E_DISCOVERY_FAILED: ::windows_sys::core::HRESULT = 0x8019000Eu32 as _;
pub const MREGISTER_E_DISCOVERY_REDIRECTED: ::windows_sys::core::HRESULT = 0x8019000Cu32 as _;
pub const MREGISTER_E_REGISTRATION_IN_PROGRESS: ::windows_sys::core::HRESULT = 0x80190009u32 as _;
pub const MaxDeviceInfoClass: REGISTRATION_INFORMATION_CLASS = 2i32;
pub type REGISTRATION_INFORMATION_CLASS = i32;
#[repr(C)]
pub struct MANAGEMENT_REGISTRATION_INFO {
    pub fDeviceRegisteredWithManagement: super::super::Foundation::BOOL,
    pub dwDeviceRegistionKind: u32,
    pub pszUPN: ::windows_sys::core::PWSTR,
    pub pszMDMServiceUri: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for MANAGEMENT_REGISTRATION_INFO {}
impl ::core::clone::Clone for MANAGEMENT_REGISTRATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MANAGEMENT_SERVICE_INFO {
    pub pszMDMServiceUri: ::windows_sys::core::PWSTR,
    pub pszAuthenticationUri: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for MANAGEMENT_SERVICE_INFO {}
impl ::core::clone::Clone for MANAGEMENT_SERVICE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
