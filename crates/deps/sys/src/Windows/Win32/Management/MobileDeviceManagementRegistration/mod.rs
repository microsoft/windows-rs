#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyLocalManagementSyncML(syncmlrequest: super::super::Foundation::PWSTR, syncmlresult: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DiscoverManagementService(pszupn: super::super::Foundation::PWSTR, ppmgmtinfo: *mut *mut MANAGEMENT_SERVICE_INFO) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DiscoverManagementServiceEx(pszupn: super::super::Foundation::PWSTR, pszdiscoveryservicecandidate: super::super::Foundation::PWSTR, ppmgmtinfo: *mut *mut MANAGEMENT_SERVICE_INFO) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDeviceManagementConfigInfo(providerid: super::super::Foundation::PWSTR, configstringbufferlength: *mut u32, configstring: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
    pub fn GetDeviceRegistrationInfo(deviceinformationclass: REGISTRATION_INFORMATION_CLASS, ppdeviceregistrationinfo: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetManagementAppHyperlink(cchhyperlink: u32, pszhyperlink: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsDeviceRegisteredWithManagement(pfisdeviceregisteredwithmanagement: *mut super::super::Foundation::BOOL, cchupn: u32, pszupn: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsManagementRegistrationAllowed(pfismanagementregistrationallowed: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsMdmUxWithoutAadAllowed(isenrollmentallowed: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterDeviceWithLocalManagement(alreadyregistered: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterDeviceWithManagement(pszupn: super::super::Foundation::PWSTR, ppszmdmserviceuri: super::super::Foundation::PWSTR, ppzsaccesstoken: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterDeviceWithManagementUsingAADCredentials(usertoken: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
    pub fn RegisterDeviceWithManagementUsingAADDeviceCredentials() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterDeviceWithManagementUsingAADDeviceCredentials2(mdmapplicationid: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDeviceManagementConfigInfo(providerid: super::super::Foundation::PWSTR, configstring: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetManagedExternally(ismanagedexternally: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
    pub fn UnregisterDeviceWithLocalManagement() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterDeviceWithManagement(enrollmentid: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
}
