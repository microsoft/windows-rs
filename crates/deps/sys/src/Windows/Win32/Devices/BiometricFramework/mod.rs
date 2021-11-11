#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioAcquireFocus() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioAsyncEnumBiometricUnits(frameworkhandle: u32, factor: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioAsyncEnumDatabases(frameworkhandle: u32, factor: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioAsyncEnumServiceProviders(frameworkhandle: u32, factor: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioAsyncMonitorFrameworkChanges(frameworkhandle: u32, changetypes: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinBioAsyncOpenFramework(notificationmethod: WINBIO_ASYNC_NOTIFICATION_METHOD, targetwindow: super::super::Foundation::HWND, messagecode: u32, callbackroutine: ::windows::runtime::RawPtr, userdata: *const ::core::ffi::c_void, asynchronousopen: super::super::Foundation::BOOL, frameworkhandle: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinBioAsyncOpenSession(factor: u32, pooltype: WINBIO_POOL, flags: u32, unitarray: *const u32, unitcount: usize, databaseid: *const ::windows::runtime::GUID, notificationmethod: WINBIO_ASYNC_NOTIFICATION_METHOD, targetwindow: super::super::Foundation::HWND, messagecode: u32, callbackroutine: ::windows::runtime::RawPtr, userdata: *const ::core::ffi::c_void, asynchronousopen: super::super::Foundation::BOOL, sessionhandle: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioCancel(sessionhandle: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioCaptureSample(sessionhandle: u32, purpose: u8, flags: u8, unitid: *mut u32, sample: *mut *mut WINBIO_BIR, samplesize: *mut usize, rejectdetail: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioCaptureSampleWithCallback(sessionhandle: u32, purpose: u8, flags: u8, capturecallback: ::windows::runtime::RawPtr, capturecallbackcontext: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioCloseFramework(frameworkhandle: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioCloseSession(sessionhandle: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioControlUnit(sessionhandle: u32, unitid: u32, component: WINBIO_COMPONENT, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioControlUnitPrivileged(sessionhandle: u32, unitid: u32, component: WINBIO_COMPONENT, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioDeleteTemplate(sessionhandle: u32, unitid: u32, identity: *const WINBIO_IDENTITY, subfactor: u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioEnrollBegin(sessionhandle: u32, subfactor: u8, unitid: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioEnrollCapture(sessionhandle: u32, rejectdetail: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioEnrollCaptureWithCallback(sessionhandle: u32, enrollcallback: ::windows::runtime::RawPtr, enrollcallbackcontext: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioEnrollCommit(sessionhandle: u32, identity: *mut WINBIO_IDENTITY, isnewtemplate: *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioEnrollDiscard(sessionhandle: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioEnrollSelect(sessionhandle: u32, selectorvalue: u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioEnumBiometricUnits(factor: u32, unitschemaarray: *mut *mut WINBIO_UNIT_SCHEMA, unitcount: *mut usize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioEnumDatabases(factor: u32, storageschemaarray: *mut *mut WINBIO_STORAGE_SCHEMA, storagecount: *mut usize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioEnumEnrollments(sessionhandle: u32, unitid: u32, identity: *const WINBIO_IDENTITY, subfactorarray: *mut *mut u8, subfactorcount: *mut usize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioEnumServiceProviders(factor: u32, bspschemaarray: *mut *mut WINBIO_BSP_SCHEMA, bspcount: *mut usize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioFree(address: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioGetCredentialState(identity: WINBIO_IDENTITY, r#type: WINBIO_CREDENTIAL_TYPE, credentialstate: *mut WINBIO_CREDENTIAL_STATE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioGetDomainLogonSetting(value: *mut u8, source: *mut WINBIO_SETTING_SOURCE);
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioGetEnabledSetting(value: *mut u8, source: *mut WINBIO_SETTING_SOURCE);
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioGetEnrolledFactors(accountowner: *const WINBIO_IDENTITY, enrolledfactors: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioGetLogonSetting(value: *mut u8, source: *mut WINBIO_SETTING_SOURCE);
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioGetProperty(sessionhandle: u32, propertytype: u32, propertyid: u32, unitid: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, propertybuffer: *mut *mut ::core::ffi::c_void, propertybuffersize: *mut usize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioIdentify(sessionhandle: u32, unitid: *mut u32, identity: *mut WINBIO_IDENTITY, subfactor: *mut u8, rejectdetail: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioIdentifyWithCallback(sessionhandle: u32, identifycallback: ::windows::runtime::RawPtr, identifycallbackcontext: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioImproveBegin(sessionhandle: u32, unitid: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioImproveEnd(sessionhandle: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioLocateSensor(sessionhandle: u32, unitid: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioLocateSensorWithCallback(sessionhandle: u32, locatecallback: ::windows::runtime::RawPtr, locatecallbackcontext: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioLockUnit(sessionhandle: u32, unitid: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioLogonIdentifiedUser(sessionhandle: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioMonitorPresence(sessionhandle: u32, unitid: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioOpenSession(factor: u32, pooltype: WINBIO_POOL, flags: u32, unitarray: *const u32, unitcount: usize, databaseid: *const ::windows::runtime::GUID, sessionhandle: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioRegisterEventMonitor(sessionhandle: u32, eventmask: u32, eventcallback: ::windows::runtime::RawPtr, eventcallbackcontext: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioReleaseFocus() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioRemoveAllCredentials() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioRemoveAllDomainCredentials() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioRemoveCredential(identity: WINBIO_IDENTITY, r#type: WINBIO_CREDENTIAL_TYPE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioSetCredential(r#type: WINBIO_CREDENTIAL_TYPE, credential: *const u8, credentialsize: usize, format: WINBIO_CREDENTIAL_FORMAT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioSetProperty(sessionhandle: u32, propertytype: u32, propertyid: u32, unitid: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, propertybuffer: *const ::core::ffi::c_void, propertybuffersize: usize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioUnlockUnit(sessionhandle: u32, unitid: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioUnregisterEventMonitor(sessionhandle: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioVerify(sessionhandle: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, unitid: *mut u32, r#match: *mut u8, rejectdetail: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinBioVerifyWithCallback(sessionhandle: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, verifycallback: ::windows::runtime::RawPtr, verifycallbackcontext: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioWait(sessionhandle: u32) -> ::windows::runtime::HRESULT;
}
