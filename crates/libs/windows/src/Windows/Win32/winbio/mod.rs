#[inline]
pub unsafe fn WinBioAcquireFocus() -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioAcquireFocus() -> windows_core::HRESULT);
    unsafe { WinBioAcquireFocus() }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioAsyncEnumBiometricUnits(frameworkhandle: super::winbio_types::WINBIO_FRAMEWORK_HANDLE, factor: super::winbio_types::WINBIO_BIOMETRIC_TYPE) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioAsyncEnumBiometricUnits(frameworkhandle : super::winbio_types::WINBIO_FRAMEWORK_HANDLE, factor : super::winbio_types::WINBIO_BIOMETRIC_TYPE) -> windows_core::HRESULT);
    unsafe { WinBioAsyncEnumBiometricUnits(frameworkhandle, factor) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioAsyncEnumDatabases(frameworkhandle: super::winbio_types::WINBIO_FRAMEWORK_HANDLE, factor: super::winbio_types::WINBIO_BIOMETRIC_TYPE) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioAsyncEnumDatabases(frameworkhandle : super::winbio_types::WINBIO_FRAMEWORK_HANDLE, factor : super::winbio_types::WINBIO_BIOMETRIC_TYPE) -> windows_core::HRESULT);
    unsafe { WinBioAsyncEnumDatabases(frameworkhandle, factor) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioAsyncEnumServiceProviders(frameworkhandle: super::winbio_types::WINBIO_FRAMEWORK_HANDLE, factor: super::winbio_types::WINBIO_BIOMETRIC_TYPE) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioAsyncEnumServiceProviders(frameworkhandle : super::winbio_types::WINBIO_FRAMEWORK_HANDLE, factor : super::winbio_types::WINBIO_BIOMETRIC_TYPE) -> windows_core::HRESULT);
    unsafe { WinBioAsyncEnumServiceProviders(frameworkhandle, factor) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioAsyncMonitorFrameworkChanges(frameworkhandle: super::winbio_types::WINBIO_FRAMEWORK_HANDLE, changetypes: super::winbio_types::WINBIO_FRAMEWORK_CHANGE_TYPE) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioAsyncMonitorFrameworkChanges(frameworkhandle : super::winbio_types::WINBIO_FRAMEWORK_HANDLE, changetypes : super::winbio_types::WINBIO_FRAMEWORK_CHANGE_TYPE) -> windows_core::HRESULT);
    unsafe { WinBioAsyncMonitorFrameworkChanges(frameworkhandle, changetypes) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
#[inline]
pub unsafe fn WinBioAsyncOpenFramework(notificationmethod: WINBIO_ASYNC_NOTIFICATION_METHOD, targetwindow: Option<super::windef::HWND>, messagecode: Option<u32>, callbackroutine: PWINBIO_ASYNC_COMPLETION_CALLBACK, userdata: Option<*const core::ffi::c_void>, asynchronousopen: bool, frameworkhandle: Option<*mut super::winbio_types::WINBIO_FRAMEWORK_HANDLE>) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioAsyncOpenFramework(notificationmethod : WINBIO_ASYNC_NOTIFICATION_METHOD, targetwindow : super::windef::HWND, messagecode : u32, callbackroutine : PWINBIO_ASYNC_COMPLETION_CALLBACK, userdata : *const core::ffi::c_void, asynchronousopen : windows_core::BOOL, frameworkhandle : *mut super::winbio_types::WINBIO_FRAMEWORK_HANDLE) -> windows_core::HRESULT);
    unsafe { WinBioAsyncOpenFramework(notificationmethod, targetwindow.unwrap_or(core::mem::zeroed()) as _, messagecode.unwrap_or(core::mem::zeroed()) as _, callbackroutine, userdata.unwrap_or(core::mem::zeroed()) as _, asynchronousopen.into(), frameworkhandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
#[inline]
pub unsafe fn WinBioAsyncOpenSession(factor: super::winbio_types::WINBIO_BIOMETRIC_TYPE, pooltype: super::winbio_types::WINBIO_POOL_TYPE, flags: super::winbio_types::WINBIO_SESSION_FLAGS, unitarray: Option<&[super::winbio_types::WINBIO_UNIT_ID]>, databaseid: Option<*const windows_core::GUID>, notificationmethod: WINBIO_ASYNC_NOTIFICATION_METHOD, targetwindow: Option<super::windef::HWND>, messagecode: Option<u32>, callbackroutine: PWINBIO_ASYNC_COMPLETION_CALLBACK, userdata: Option<*const core::ffi::c_void>, asynchronousopen: bool, sessionhandle: Option<*mut super::winbio_types::WINBIO_SESSION_HANDLE>) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioAsyncOpenSession(factor : super::winbio_types::WINBIO_BIOMETRIC_TYPE, pooltype : super::winbio_types::WINBIO_POOL_TYPE, flags : super::winbio_types::WINBIO_SESSION_FLAGS, unitarray : *const super::winbio_types::WINBIO_UNIT_ID, unitcount : usize, databaseid : *const windows_core::GUID, notificationmethod : WINBIO_ASYNC_NOTIFICATION_METHOD, targetwindow : super::windef::HWND, messagecode : u32, callbackroutine : PWINBIO_ASYNC_COMPLETION_CALLBACK, userdata : *const core::ffi::c_void, asynchronousopen : windows_core::BOOL, sessionhandle : *mut super::winbio_types::WINBIO_SESSION_HANDLE) -> windows_core::HRESULT);
    unsafe { WinBioAsyncOpenSession(factor, pooltype, flags, core::mem::transmute(unitarray.map_or(core::ptr::null(), |slice| slice.as_ptr())), unitarray.map_or(0, |slice| slice.len().try_into().unwrap()), databaseid.unwrap_or(core::mem::zeroed()) as _, notificationmethod, targetwindow.unwrap_or(core::mem::zeroed()) as _, messagecode.unwrap_or(core::mem::zeroed()) as _, callbackroutine, userdata.unwrap_or(core::mem::zeroed()) as _, asynchronousopen.into(), sessionhandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioCancel(sessionhandle: super::winbio_types::WINBIO_SESSION_HANDLE) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioCancel(sessionhandle : super::winbio_types::WINBIO_SESSION_HANDLE) -> windows_core::HRESULT);
    unsafe { WinBioCancel(sessionhandle) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioCaptureSample(sessionhandle: super::winbio_types::WINBIO_SESSION_HANDLE, purpose: super::winbio_types::WINBIO_BIR_PURPOSE, flags: super::winbio_types::WINBIO_BIR_DATA_FLAGS, unitid: Option<*mut super::winbio_types::WINBIO_UNIT_ID>, sample: *mut super::winbio_types::PWINBIO_BIR, samplesize: Option<*mut usize>, rejectdetail: Option<*mut super::winbio_types::WINBIO_REJECT_DETAIL>) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioCaptureSample(sessionhandle : super::winbio_types::WINBIO_SESSION_HANDLE, purpose : super::winbio_types::WINBIO_BIR_PURPOSE, flags : super::winbio_types::WINBIO_BIR_DATA_FLAGS, unitid : *mut super::winbio_types::WINBIO_UNIT_ID, sample : *mut super::winbio_types::PWINBIO_BIR, samplesize : *mut usize, rejectdetail : *mut super::winbio_types::WINBIO_REJECT_DETAIL) -> windows_core::HRESULT);
    unsafe { WinBioCaptureSample(sessionhandle, purpose, flags, unitid.unwrap_or(core::mem::zeroed()) as _, sample as _, samplesize.unwrap_or(core::mem::zeroed()) as _, rejectdetail.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioCaptureSampleWithCallback(sessionhandle: super::winbio_types::WINBIO_SESSION_HANDLE, purpose: super::winbio_types::WINBIO_BIR_PURPOSE, flags: super::winbio_types::WINBIO_BIR_DATA_FLAGS, capturecallback: PWINBIO_CAPTURE_CALLBACK, capturecallbackcontext: Option<*const core::ffi::c_void>) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioCaptureSampleWithCallback(sessionhandle : super::winbio_types::WINBIO_SESSION_HANDLE, purpose : super::winbio_types::WINBIO_BIR_PURPOSE, flags : super::winbio_types::WINBIO_BIR_DATA_FLAGS, capturecallback : PWINBIO_CAPTURE_CALLBACK, capturecallbackcontext : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WinBioCaptureSampleWithCallback(sessionhandle, purpose, flags, capturecallback, capturecallbackcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioCloseFramework(frameworkhandle: super::winbio_types::WINBIO_FRAMEWORK_HANDLE) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioCloseFramework(frameworkhandle : super::winbio_types::WINBIO_FRAMEWORK_HANDLE) -> windows_core::HRESULT);
    unsafe { WinBioCloseFramework(frameworkhandle) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioCloseSession(sessionhandle: super::winbio_types::WINBIO_SESSION_HANDLE) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioCloseSession(sessionhandle : super::winbio_types::WINBIO_SESSION_HANDLE) -> windows_core::HRESULT);
    unsafe { WinBioCloseSession(sessionhandle) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioControlUnit(sessionhandle: super::winbio_types::WINBIO_SESSION_HANDLE, unitid: super::winbio_types::WINBIO_UNIT_ID, component: super::winbio_types::WINBIO_COMPONENT, controlcode: u32, sendbuffer: &[u8], receivebuffer: &mut [u8], receivedatasize: *mut usize, operationstatus: Option<*mut u32>) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioControlUnit(sessionhandle : super::winbio_types::WINBIO_SESSION_HANDLE, unitid : super::winbio_types::WINBIO_UNIT_ID, component : super::winbio_types::WINBIO_COMPONENT, controlcode : u32, sendbuffer : *const u8, sendbuffersize : usize, receivebuffer : *mut u8, receivebuffersize : usize, receivedatasize : *mut usize, operationstatus : *mut u32) -> windows_core::HRESULT);
    unsafe { WinBioControlUnit(sessionhandle, unitid, component, controlcode, core::mem::transmute(sendbuffer.as_ptr()), sendbuffer.len().try_into().unwrap(), core::mem::transmute(receivebuffer.as_ptr()), receivebuffer.len().try_into().unwrap(), receivedatasize as _, operationstatus.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioControlUnitPrivileged(sessionhandle: super::winbio_types::WINBIO_SESSION_HANDLE, unitid: super::winbio_types::WINBIO_UNIT_ID, component: super::winbio_types::WINBIO_COMPONENT, controlcode: u32, sendbuffer: &[u8], receivebuffer: &mut [u8], receivedatasize: *mut usize, operationstatus: Option<*mut u32>) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioControlUnitPrivileged(sessionhandle : super::winbio_types::WINBIO_SESSION_HANDLE, unitid : super::winbio_types::WINBIO_UNIT_ID, component : super::winbio_types::WINBIO_COMPONENT, controlcode : u32, sendbuffer : *const u8, sendbuffersize : usize, receivebuffer : *mut u8, receivebuffersize : usize, receivedatasize : *mut usize, operationstatus : *mut u32) -> windows_core::HRESULT);
    unsafe { WinBioControlUnitPrivileged(sessionhandle, unitid, component, controlcode, core::mem::transmute(sendbuffer.as_ptr()), sendbuffer.len().try_into().unwrap(), core::mem::transmute(receivebuffer.as_ptr()), receivebuffer.len().try_into().unwrap(), receivedatasize as _, operationstatus.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioDeleteTemplate(sessionhandle: super::winbio_types::WINBIO_SESSION_HANDLE, unitid: super::winbio_types::WINBIO_UNIT_ID, identity: *const super::winbio_types::WINBIO_IDENTITY, subfactor: super::winbio_types::WINBIO_BIOMETRIC_SUBTYPE) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioDeleteTemplate(sessionhandle : super::winbio_types::WINBIO_SESSION_HANDLE, unitid : super::winbio_types::WINBIO_UNIT_ID, identity : *const super::winbio_types::WINBIO_IDENTITY, subfactor : super::winbio_types::WINBIO_BIOMETRIC_SUBTYPE) -> windows_core::HRESULT);
    unsafe { WinBioDeleteTemplate(sessionhandle, unitid, identity, subfactor) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioEnrollBegin(sessionhandle: super::winbio_types::WINBIO_SESSION_HANDLE, subfactor: super::winbio_types::WINBIO_BIOMETRIC_SUBTYPE, unitid: super::winbio_types::WINBIO_UNIT_ID) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioEnrollBegin(sessionhandle : super::winbio_types::WINBIO_SESSION_HANDLE, subfactor : super::winbio_types::WINBIO_BIOMETRIC_SUBTYPE, unitid : super::winbio_types::WINBIO_UNIT_ID) -> windows_core::HRESULT);
    unsafe { WinBioEnrollBegin(sessionhandle, subfactor, unitid) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioEnrollCapture(sessionhandle: super::winbio_types::WINBIO_SESSION_HANDLE, rejectdetail: Option<*mut super::winbio_types::WINBIO_REJECT_DETAIL>) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioEnrollCapture(sessionhandle : super::winbio_types::WINBIO_SESSION_HANDLE, rejectdetail : *mut super::winbio_types::WINBIO_REJECT_DETAIL) -> windows_core::HRESULT);
    unsafe { WinBioEnrollCapture(sessionhandle, rejectdetail.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioEnrollCaptureWithCallback(sessionhandle: super::winbio_types::WINBIO_SESSION_HANDLE, enrollcallback: PWINBIO_ENROLL_CAPTURE_CALLBACK, enrollcallbackcontext: Option<*const core::ffi::c_void>) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioEnrollCaptureWithCallback(sessionhandle : super::winbio_types::WINBIO_SESSION_HANDLE, enrollcallback : PWINBIO_ENROLL_CAPTURE_CALLBACK, enrollcallbackcontext : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WinBioEnrollCaptureWithCallback(sessionhandle, enrollcallback, enrollcallbackcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioEnrollCommit(sessionhandle: super::winbio_types::WINBIO_SESSION_HANDLE, identity: Option<*mut super::winbio_types::WINBIO_IDENTITY>, isnewtemplate: Option<*mut bool>) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioEnrollCommit(sessionhandle : super::winbio_types::WINBIO_SESSION_HANDLE, identity : *mut super::winbio_types::WINBIO_IDENTITY, isnewtemplate : *mut bool) -> windows_core::HRESULT);
    unsafe { WinBioEnrollCommit(sessionhandle, identity.unwrap_or(core::mem::zeroed()) as _, isnewtemplate.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioEnrollDiscard(sessionhandle: super::winbio_types::WINBIO_SESSION_HANDLE) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioEnrollDiscard(sessionhandle : super::winbio_types::WINBIO_SESSION_HANDLE) -> windows_core::HRESULT);
    unsafe { WinBioEnrollDiscard(sessionhandle) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioEnrollSelect(sessionhandle: super::winbio_types::WINBIO_SESSION_HANDLE, selectorvalue: u64) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioEnrollSelect(sessionhandle : super::winbio_types::WINBIO_SESSION_HANDLE, selectorvalue : u64) -> windows_core::HRESULT);
    unsafe { WinBioEnrollSelect(sessionhandle, selectorvalue) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioEnumBiometricUnits(factor: super::winbio_types::WINBIO_BIOMETRIC_TYPE, unitschemaarray: *mut *mut super::winbio_types::WINBIO_UNIT_SCHEMA, unitcount: *mut usize) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioEnumBiometricUnits(factor : super::winbio_types::WINBIO_BIOMETRIC_TYPE, unitschemaarray : *mut *mut super::winbio_types::WINBIO_UNIT_SCHEMA, unitcount : *mut usize) -> windows_core::HRESULT);
    unsafe { WinBioEnumBiometricUnits(factor, unitschemaarray as _, unitcount as _) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioEnumDatabases(factor: super::winbio_types::WINBIO_BIOMETRIC_TYPE, storageschemaarray: *mut *mut super::winbio_types::WINBIO_STORAGE_SCHEMA, storagecount: *mut usize) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioEnumDatabases(factor : super::winbio_types::WINBIO_BIOMETRIC_TYPE, storageschemaarray : *mut *mut super::winbio_types::WINBIO_STORAGE_SCHEMA, storagecount : *mut usize) -> windows_core::HRESULT);
    unsafe { WinBioEnumDatabases(factor, storageschemaarray as _, storagecount as _) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioEnumEnrollments(sessionhandle: super::winbio_types::WINBIO_SESSION_HANDLE, unitid: super::winbio_types::WINBIO_UNIT_ID, identity: *const super::winbio_types::WINBIO_IDENTITY, subfactorarray: *mut *mut super::winbio_types::WINBIO_BIOMETRIC_SUBTYPE, subfactorcount: Option<*mut usize>) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioEnumEnrollments(sessionhandle : super::winbio_types::WINBIO_SESSION_HANDLE, unitid : super::winbio_types::WINBIO_UNIT_ID, identity : *const super::winbio_types::WINBIO_IDENTITY, subfactorarray : *mut *mut super::winbio_types::WINBIO_BIOMETRIC_SUBTYPE, subfactorcount : *mut usize) -> windows_core::HRESULT);
    unsafe { WinBioEnumEnrollments(sessionhandle, unitid, identity, subfactorarray as _, subfactorcount.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioEnumServiceProviders(factor: super::winbio_types::WINBIO_BIOMETRIC_TYPE, bspschemaarray: *mut *mut super::winbio_types::WINBIO_BSP_SCHEMA, bspcount: *mut usize) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioEnumServiceProviders(factor : super::winbio_types::WINBIO_BIOMETRIC_TYPE, bspschemaarray : *mut *mut super::winbio_types::WINBIO_BSP_SCHEMA, bspcount : *mut usize) -> windows_core::HRESULT);
    unsafe { WinBioEnumServiceProviders(factor, bspschemaarray as _, bspcount as _) }
}
#[inline]
pub unsafe fn WinBioFree(address: *const core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioFree(address : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WinBioFree(address) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioGetCredentialState(identity: super::winbio_types::WINBIO_IDENTITY, r#type: super::winbio_types::WINBIO_CREDENTIAL_TYPE) -> windows_core::Result<super::winbio_types::WINBIO_CREDENTIAL_STATE> {
    windows_core::link!("winbio.dll" "system" fn WinBioGetCredentialState(identity : super::winbio_types::WINBIO_IDENTITY, r#type : super::winbio_types::WINBIO_CREDENTIAL_TYPE, credentialstate : *mut super::winbio_types::WINBIO_CREDENTIAL_STATE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WinBioGetCredentialState(core::mem::transmute(identity), r#type, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WinBioGetDomainLogonSetting(value: *mut bool, source: *mut u32) {
    windows_core::link!("winbio.dll" "system" fn WinBioGetDomainLogonSetting(value : *mut bool, source : *mut u32));
    unsafe { WinBioGetDomainLogonSetting(value as _, source as _) }
}
#[inline]
pub unsafe fn WinBioGetEnabledSetting(value: *mut bool, source: *mut u32) {
    windows_core::link!("winbio.dll" "system" fn WinBioGetEnabledSetting(value : *mut bool, source : *mut u32));
    unsafe { WinBioGetEnabledSetting(value as _, source as _) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioGetEnrolledFactors(accountowner: *const super::winbio_types::WINBIO_IDENTITY) -> windows_core::Result<super::winbio_types::WINBIO_BIOMETRIC_TYPE> {
    windows_core::link!("winbio.dll" "system" fn WinBioGetEnrolledFactors(accountowner : *const super::winbio_types::WINBIO_IDENTITY, enrolledfactors : *mut super::winbio_types::WINBIO_BIOMETRIC_TYPE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WinBioGetEnrolledFactors(accountowner, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WinBioGetLogonSetting(value: *mut bool, source: *mut u32) {
    windows_core::link!("winbio.dll" "system" fn WinBioGetLogonSetting(value : *mut bool, source : *mut u32));
    unsafe { WinBioGetLogonSetting(value as _, source as _) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioGetProperty(sessionhandle: super::winbio_types::WINBIO_SESSION_HANDLE, propertytype: super::winbio_types::WINBIO_PROPERTY_TYPE, propertyid: super::winbio_types::WINBIO_PROPERTY_ID, unitid: Option<super::winbio_types::WINBIO_UNIT_ID>, identity: Option<*const super::winbio_types::WINBIO_IDENTITY>, subfactor: Option<super::winbio_types::WINBIO_BIOMETRIC_SUBTYPE>, propertybuffer: *mut *mut core::ffi::c_void, propertybuffersize: Option<*mut usize>) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioGetProperty(sessionhandle : super::winbio_types::WINBIO_SESSION_HANDLE, propertytype : super::winbio_types::WINBIO_PROPERTY_TYPE, propertyid : super::winbio_types::WINBIO_PROPERTY_ID, unitid : super::winbio_types::WINBIO_UNIT_ID, identity : *const super::winbio_types::WINBIO_IDENTITY, subfactor : super::winbio_types::WINBIO_BIOMETRIC_SUBTYPE, propertybuffer : *mut *mut core::ffi::c_void, propertybuffersize : *mut usize) -> windows_core::HRESULT);
    unsafe { WinBioGetProperty(sessionhandle, propertytype, propertyid, unitid.unwrap_or(core::mem::zeroed()) as _, identity.unwrap_or(core::mem::zeroed()) as _, subfactor.unwrap_or(core::mem::zeroed()) as _, propertybuffer as _, propertybuffersize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioIdentify(sessionhandle: super::winbio_types::WINBIO_SESSION_HANDLE, unitid: Option<*mut super::winbio_types::WINBIO_UNIT_ID>, identity: Option<*mut super::winbio_types::WINBIO_IDENTITY>, subfactor: Option<*mut super::winbio_types::WINBIO_BIOMETRIC_SUBTYPE>, rejectdetail: Option<*mut super::winbio_types::WINBIO_REJECT_DETAIL>) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioIdentify(sessionhandle : super::winbio_types::WINBIO_SESSION_HANDLE, unitid : *mut super::winbio_types::WINBIO_UNIT_ID, identity : *mut super::winbio_types::WINBIO_IDENTITY, subfactor : *mut super::winbio_types::WINBIO_BIOMETRIC_SUBTYPE, rejectdetail : *mut super::winbio_types::WINBIO_REJECT_DETAIL) -> windows_core::HRESULT);
    unsafe { WinBioIdentify(sessionhandle, unitid.unwrap_or(core::mem::zeroed()) as _, identity.unwrap_or(core::mem::zeroed()) as _, subfactor.unwrap_or(core::mem::zeroed()) as _, rejectdetail.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioIdentifyWithCallback(sessionhandle: super::winbio_types::WINBIO_SESSION_HANDLE, identifycallback: PWINBIO_IDENTIFY_CALLBACK, identifycallbackcontext: Option<*const core::ffi::c_void>) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioIdentifyWithCallback(sessionhandle : super::winbio_types::WINBIO_SESSION_HANDLE, identifycallback : PWINBIO_IDENTIFY_CALLBACK, identifycallbackcontext : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WinBioIdentifyWithCallback(sessionhandle, identifycallback, identifycallbackcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioImproveBegin(sessionhandle: super::winbio_types::WINBIO_SESSION_HANDLE, unitid: super::winbio_types::WINBIO_UNIT_ID) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioImproveBegin(sessionhandle : super::winbio_types::WINBIO_SESSION_HANDLE, unitid : super::winbio_types::WINBIO_UNIT_ID) -> windows_core::HRESULT);
    unsafe { WinBioImproveBegin(sessionhandle, unitid) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioImproveEnd(sessionhandle: super::winbio_types::WINBIO_SESSION_HANDLE) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioImproveEnd(sessionhandle : super::winbio_types::WINBIO_SESSION_HANDLE) -> windows_core::HRESULT);
    unsafe { WinBioImproveEnd(sessionhandle) }
}
#[inline]
pub unsafe fn WinBioIsESSCapable() -> windows_core::Result<bool> {
    windows_core::link!("winbio.dll" "system" fn WinBioIsESSCapable(value : *mut bool) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WinBioIsESSCapable(&mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioLocateSensor(sessionhandle: super::winbio_types::WINBIO_SESSION_HANDLE, unitid: Option<*mut super::winbio_types::WINBIO_UNIT_ID>) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioLocateSensor(sessionhandle : super::winbio_types::WINBIO_SESSION_HANDLE, unitid : *mut super::winbio_types::WINBIO_UNIT_ID) -> windows_core::HRESULT);
    unsafe { WinBioLocateSensor(sessionhandle, unitid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioLocateSensorWithCallback(sessionhandle: super::winbio_types::WINBIO_SESSION_HANDLE, locatecallback: PWINBIO_LOCATE_SENSOR_CALLBACK, locatecallbackcontext: Option<*const core::ffi::c_void>) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioLocateSensorWithCallback(sessionhandle : super::winbio_types::WINBIO_SESSION_HANDLE, locatecallback : PWINBIO_LOCATE_SENSOR_CALLBACK, locatecallbackcontext : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WinBioLocateSensorWithCallback(sessionhandle, locatecallback, locatecallbackcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioLockUnit(sessionhandle: super::winbio_types::WINBIO_SESSION_HANDLE, unitid: super::winbio_types::WINBIO_UNIT_ID) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioLockUnit(sessionhandle : super::winbio_types::WINBIO_SESSION_HANDLE, unitid : super::winbio_types::WINBIO_UNIT_ID) -> windows_core::HRESULT);
    unsafe { WinBioLockUnit(sessionhandle, unitid) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioLogonIdentifiedUser(sessionhandle: super::winbio_types::WINBIO_SESSION_HANDLE) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioLogonIdentifiedUser(sessionhandle : super::winbio_types::WINBIO_SESSION_HANDLE) -> windows_core::HRESULT);
    unsafe { WinBioLogonIdentifiedUser(sessionhandle) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioMonitorPresence(sessionhandle: super::winbio_types::WINBIO_SESSION_HANDLE, unitid: super::winbio_types::WINBIO_UNIT_ID) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioMonitorPresence(sessionhandle : super::winbio_types::WINBIO_SESSION_HANDLE, unitid : super::winbio_types::WINBIO_UNIT_ID) -> windows_core::HRESULT);
    unsafe { WinBioMonitorPresence(sessionhandle, unitid) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioOpenSession(factor: super::winbio_types::WINBIO_BIOMETRIC_TYPE, pooltype: super::winbio_types::WINBIO_POOL_TYPE, flags: super::winbio_types::WINBIO_SESSION_FLAGS, unitarray: Option<&[super::winbio_types::WINBIO_UNIT_ID]>, databaseid: Option<*const windows_core::GUID>) -> windows_core::Result<super::winbio_types::WINBIO_SESSION_HANDLE> {
    windows_core::link!("winbio.dll" "system" fn WinBioOpenSession(factor : super::winbio_types::WINBIO_BIOMETRIC_TYPE, pooltype : super::winbio_types::WINBIO_POOL_TYPE, flags : super::winbio_types::WINBIO_SESSION_FLAGS, unitarray : *const super::winbio_types::WINBIO_UNIT_ID, unitcount : usize, databaseid : *const windows_core::GUID, sessionhandle : *mut super::winbio_types::WINBIO_SESSION_HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WinBioOpenSession(factor, pooltype, flags, core::mem::transmute(unitarray.map_or(core::ptr::null(), |slice| slice.as_ptr())), unitarray.map_or(0, |slice| slice.len().try_into().unwrap()), databaseid.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioRegisterEventMonitor(sessionhandle: super::winbio_types::WINBIO_SESSION_HANDLE, eventmask: super::winbio_types::WINBIO_EVENT_TYPE, eventcallback: PWINBIO_EVENT_CALLBACK, eventcallbackcontext: Option<*const core::ffi::c_void>) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioRegisterEventMonitor(sessionhandle : super::winbio_types::WINBIO_SESSION_HANDLE, eventmask : super::winbio_types::WINBIO_EVENT_TYPE, eventcallback : PWINBIO_EVENT_CALLBACK, eventcallbackcontext : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WinBioRegisterEventMonitor(sessionhandle, eventmask, eventcallback, eventcallbackcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn WinBioReleaseFocus() -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioReleaseFocus() -> windows_core::HRESULT);
    unsafe { WinBioReleaseFocus() }
}
#[inline]
pub unsafe fn WinBioRemoveAllCredentials() -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioRemoveAllCredentials() -> windows_core::HRESULT);
    unsafe { WinBioRemoveAllCredentials() }
}
#[inline]
pub unsafe fn WinBioRemoveAllDomainCredentials() -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioRemoveAllDomainCredentials() -> windows_core::HRESULT);
    unsafe { WinBioRemoveAllDomainCredentials() }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioRemoveCredential(identity: super::winbio_types::WINBIO_IDENTITY, r#type: super::winbio_types::WINBIO_CREDENTIAL_TYPE) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioRemoveCredential(identity : super::winbio_types::WINBIO_IDENTITY, r#type : super::winbio_types::WINBIO_CREDENTIAL_TYPE) -> windows_core::HRESULT);
    unsafe { WinBioRemoveCredential(core::mem::transmute(identity), r#type) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioSetCredential(r#type: super::winbio_types::WINBIO_CREDENTIAL_TYPE, credential: &[u8], format: super::winbio_types::WINBIO_CREDENTIAL_FORMAT) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioSetCredential(r#type : super::winbio_types::WINBIO_CREDENTIAL_TYPE, credential : *const u8, credentialsize : usize, format : super::winbio_types::WINBIO_CREDENTIAL_FORMAT) -> windows_core::HRESULT);
    unsafe { WinBioSetCredential(r#type, core::mem::transmute(credential.as_ptr()), credential.len().try_into().unwrap(), format) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioSetProperty(sessionhandle: super::winbio_types::WINBIO_SESSION_HANDLE, propertytype: super::winbio_types::WINBIO_PROPERTY_TYPE, propertyid: super::winbio_types::WINBIO_PROPERTY_ID, unitid: Option<super::winbio_types::WINBIO_UNIT_ID>, identity: Option<*const super::winbio_types::WINBIO_IDENTITY>, subfactor: Option<super::winbio_types::WINBIO_BIOMETRIC_SUBTYPE>, propertybuffer: *const core::ffi::c_void, propertybuffersize: usize) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioSetProperty(sessionhandle : super::winbio_types::WINBIO_SESSION_HANDLE, propertytype : super::winbio_types::WINBIO_PROPERTY_TYPE, propertyid : super::winbio_types::WINBIO_PROPERTY_ID, unitid : super::winbio_types::WINBIO_UNIT_ID, identity : *const super::winbio_types::WINBIO_IDENTITY, subfactor : super::winbio_types::WINBIO_BIOMETRIC_SUBTYPE, propertybuffer : *const core::ffi::c_void, propertybuffersize : usize) -> windows_core::HRESULT);
    unsafe { WinBioSetProperty(sessionhandle, propertytype, propertyid, unitid.unwrap_or(core::mem::zeroed()) as _, identity.unwrap_or(core::mem::zeroed()) as _, subfactor.unwrap_or(core::mem::zeroed()) as _, propertybuffer, propertybuffersize) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioUnlockUnit(sessionhandle: super::winbio_types::WINBIO_SESSION_HANDLE, unitid: super::winbio_types::WINBIO_UNIT_ID) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioUnlockUnit(sessionhandle : super::winbio_types::WINBIO_SESSION_HANDLE, unitid : super::winbio_types::WINBIO_UNIT_ID) -> windows_core::HRESULT);
    unsafe { WinBioUnlockUnit(sessionhandle, unitid) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioUnregisterEventMonitor(sessionhandle: super::winbio_types::WINBIO_SESSION_HANDLE) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioUnregisterEventMonitor(sessionhandle : super::winbio_types::WINBIO_SESSION_HANDLE) -> windows_core::HRESULT);
    unsafe { WinBioUnregisterEventMonitor(sessionhandle) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioVerify(sessionhandle: super::winbio_types::WINBIO_SESSION_HANDLE, identity: *const super::winbio_types::WINBIO_IDENTITY, subfactor: super::winbio_types::WINBIO_BIOMETRIC_SUBTYPE, unitid: Option<*mut super::winbio_types::WINBIO_UNIT_ID>, r#match: Option<*mut bool>, rejectdetail: Option<*mut super::winbio_types::WINBIO_REJECT_DETAIL>) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioVerify(sessionhandle : super::winbio_types::WINBIO_SESSION_HANDLE, identity : *const super::winbio_types::WINBIO_IDENTITY, subfactor : super::winbio_types::WINBIO_BIOMETRIC_SUBTYPE, unitid : *mut super::winbio_types::WINBIO_UNIT_ID, r#match : *mut bool, rejectdetail : *mut super::winbio_types::WINBIO_REJECT_DETAIL) -> windows_core::HRESULT);
    unsafe { WinBioVerify(sessionhandle, identity, subfactor, unitid.unwrap_or(core::mem::zeroed()) as _, r#match.unwrap_or(core::mem::zeroed()) as _, rejectdetail.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioVerifyWithCallback(sessionhandle: super::winbio_types::WINBIO_SESSION_HANDLE, identity: *const super::winbio_types::WINBIO_IDENTITY, subfactor: super::winbio_types::WINBIO_BIOMETRIC_SUBTYPE, verifycallback: PWINBIO_VERIFY_CALLBACK, verifycallbackcontext: Option<*const core::ffi::c_void>) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioVerifyWithCallback(sessionhandle : super::winbio_types::WINBIO_SESSION_HANDLE, identity : *const super::winbio_types::WINBIO_IDENTITY, subfactor : super::winbio_types::WINBIO_BIOMETRIC_SUBTYPE, verifycallback : PWINBIO_VERIFY_CALLBACK, verifycallbackcontext : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WinBioVerifyWithCallback(sessionhandle, identity, subfactor, verifycallback, verifycallbackcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winbio_types")]
#[inline]
pub unsafe fn WinBioWait(sessionhandle: super::winbio_types::WINBIO_SESSION_HANDLE) -> windows_core::HRESULT {
    windows_core::link!("winbio.dll" "system" fn WinBioWait(sessionhandle : super::winbio_types::WINBIO_SESSION_HANDLE) -> windows_core::HRESULT);
    unsafe { WinBioWait(sessionhandle) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
pub type PWINBIO_ASYNC_COMPLETION_CALLBACK = Option<unsafe extern "system" fn(asyncresult: *const WINBIO_ASYNC_RESULT)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWINBIO_ASYNC_NOTIFICATION_METHOD(pub *mut WINBIO_ASYNC_NOTIFICATION_METHOD);
impl PWINBIO_ASYNC_NOTIFICATION_METHOD {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWINBIO_ASYNC_NOTIFICATION_METHOD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWINBIO_ASYNC_RESULT(pub *mut WINBIO_ASYNC_RESULT);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
impl PWINBIO_ASYNC_RESULT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
impl Default for PWINBIO_ASYNC_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winbio_types")]
pub type PWINBIO_CAPTURE_CALLBACK = Option<unsafe extern "system" fn(capturecallbackcontext: *const core::ffi::c_void, operationstatus: windows_core::HRESULT, unitid: super::winbio_types::WINBIO_UNIT_ID, sample: *const super::winbio_types::WINBIO_BIR, samplesize: usize, rejectdetail: super::winbio_types::WINBIO_REJECT_DETAIL)>;
#[cfg(feature = "Win32_winbio_types")]
pub type PWINBIO_ENROLL_CAPTURE_CALLBACK = Option<unsafe extern "system" fn(enrollcallbackcontext: *const core::ffi::c_void, operationstatus: windows_core::HRESULT, rejectdetail: super::winbio_types::WINBIO_REJECT_DETAIL)>;
#[cfg(feature = "Win32_winbio_types")]
pub type PWINBIO_EVENT_CALLBACK = Option<unsafe extern "system" fn(eventcallbackcontext: *const core::ffi::c_void, operationstatus: windows_core::HRESULT, event: *const super::winbio_types::WINBIO_EVENT)>;
#[cfg(feature = "Win32_winbio_types")]
pub type PWINBIO_IDENTIFY_CALLBACK = Option<unsafe extern "system" fn(identifycallbackcontext: *const core::ffi::c_void, operationstatus: windows_core::HRESULT, unitid: super::winbio_types::WINBIO_UNIT_ID, identity: *const super::winbio_types::WINBIO_IDENTITY, subfactor: super::winbio_types::WINBIO_BIOMETRIC_SUBTYPE, rejectdetail: super::winbio_types::WINBIO_REJECT_DETAIL)>;
#[cfg(feature = "Win32_winbio_types")]
pub type PWINBIO_LOCATE_SENSOR_CALLBACK = Option<unsafe extern "system" fn(locatecallbackcontext: *const core::ffi::c_void, operationstatus: windows_core::HRESULT, unitid: super::winbio_types::WINBIO_UNIT_ID)>;
#[cfg(feature = "Win32_winbio_types")]
pub type PWINBIO_VERIFY_CALLBACK = Option<unsafe extern "system" fn(verifycallbackcontext: *const core::ffi::c_void, operationstatus: windows_core::HRESULT, unitid: super::winbio_types::WINBIO_UNIT_ID, r#match: bool, rejectdetail: super::winbio_types::WINBIO_REJECT_DETAIL)>;
pub type WINBIO_ASYNC_NOTIFICATION_METHOD = i32;
pub const WINBIO_ASYNC_NOTIFY_CALLBACK: WINBIO_ASYNC_NOTIFICATION_METHOD = 1;
pub const WINBIO_ASYNC_NOTIFY_MAXIMUM_VALUE: WINBIO_ASYNC_NOTIFICATION_METHOD = 3;
pub const WINBIO_ASYNC_NOTIFY_MESSAGE: WINBIO_ASYNC_NOTIFICATION_METHOD = 2;
pub const WINBIO_ASYNC_NOTIFY_NONE: WINBIO_ASYNC_NOTIFICATION_METHOD = 0;
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct WINBIO_ASYNC_RESULT {
    pub SessionHandle: super::winbio_types::WINBIO_SESSION_HANDLE,
    pub Operation: super::winbio_types::WINBIO_OPERATION_TYPE,
    pub SequenceNumber: u64,
    pub TimeStamp: i64,
    pub ApiStatus: windows_core::HRESULT,
    pub UnitId: super::winbio_types::WINBIO_UNIT_ID,
    pub UserData: *mut core::ffi::c_void,
    pub Parameters: WINBIO_ASYNC_RESULT_0,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
impl Default for WINBIO_ASYNC_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub union WINBIO_ASYNC_RESULT_0 {
    pub Verify: WINBIO_ASYNC_RESULT_0_0,
    pub Identify: WINBIO_ASYNC_RESULT_0_1,
    pub EnrollBegin: WINBIO_ASYNC_RESULT_0_2,
    pub EnrollCapture: WINBIO_ASYNC_RESULT_0_3,
    pub EnrollCommit: WINBIO_ASYNC_RESULT_0_4,
    pub EnumEnrollments: WINBIO_ASYNC_RESULT_0_5,
    pub CaptureSample: WINBIO_ASYNC_RESULT_0_6,
    pub DeleteTemplate: WINBIO_ASYNC_RESULT_0_7,
    pub GetProperty: WINBIO_ASYNC_RESULT_0_8,
    pub SetProperty: WINBIO_ASYNC_RESULT_0_9,
    pub GetEvent: WINBIO_ASYNC_RESULT_0_10,
    pub ControlUnit: WINBIO_ASYNC_RESULT_0_11,
    pub EnumServiceProviders: WINBIO_ASYNC_RESULT_0_12,
    pub EnumBiometricUnits: WINBIO_ASYNC_RESULT_0_13,
    pub EnumDatabases: WINBIO_ASYNC_RESULT_0_14,
    pub VerifyAndReleaseTicket: WINBIO_ASYNC_RESULT_0_15,
    pub IdentifyAndReleaseTicket: WINBIO_ASYNC_RESULT_0_16,
    pub EnrollSelect: WINBIO_ASYNC_RESULT_0_17,
    pub MonitorPresence: WINBIO_ASYNC_RESULT_0_18,
    pub GetProtectionPolicy: WINBIO_ASYNC_RESULT_0_19,
    pub NotifyUnitStatusChange: WINBIO_ASYNC_RESULT_0_20,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
impl Default for WINBIO_ASYNC_RESULT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_ASYNC_RESULT_0_0 {
    pub Match: bool,
    pub RejectDetail: super::winbio_types::WINBIO_REJECT_DETAIL,
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct WINBIO_ASYNC_RESULT_0_1 {
    pub Identity: super::winbio_types::WINBIO_IDENTITY,
    pub SubFactor: super::winbio_types::WINBIO_BIOMETRIC_SUBTYPE,
    pub RejectDetail: super::winbio_types::WINBIO_REJECT_DETAIL,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
impl Default for WINBIO_ASYNC_RESULT_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct WINBIO_ASYNC_RESULT_0_10 {
    pub Event: super::winbio_types::WINBIO_EVENT,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
impl Default for WINBIO_ASYNC_RESULT_0_10 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_ASYNC_RESULT_0_11 {
    pub Component: super::winbio_types::WINBIO_COMPONENT,
    pub ControlCode: u32,
    pub OperationStatus: u32,
    pub SendBuffer: super::minwindef::PUCHAR,
    pub SendBufferSize: usize,
    pub ReceiveBuffer: super::minwindef::PUCHAR,
    pub ReceiveBufferSize: usize,
    pub ReceiveDataSize: usize,
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINBIO_ASYNC_RESULT_0_12 {
    pub BspCount: usize,
    pub BspSchemaArray: *mut super::winbio_types::WINBIO_BSP_SCHEMA,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
impl Default for WINBIO_ASYNC_RESULT_0_12 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINBIO_ASYNC_RESULT_0_13 {
    pub UnitCount: usize,
    pub UnitSchemaArray: *mut super::winbio_types::WINBIO_UNIT_SCHEMA,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
impl Default for WINBIO_ASYNC_RESULT_0_13 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINBIO_ASYNC_RESULT_0_14 {
    pub StorageCount: usize,
    pub StorageSchemaArray: *mut super::winbio_types::WINBIO_STORAGE_SCHEMA,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
impl Default for WINBIO_ASYNC_RESULT_0_14 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_ASYNC_RESULT_0_15 {
    pub Match: bool,
    pub RejectDetail: super::winbio_types::WINBIO_REJECT_DETAIL,
    pub Ticket: super::winbio_types::WINBIO_PROTECTION_TICKET,
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct WINBIO_ASYNC_RESULT_0_16 {
    pub Identity: super::winbio_types::WINBIO_IDENTITY,
    pub SubFactor: super::winbio_types::WINBIO_BIOMETRIC_SUBTYPE,
    pub RejectDetail: super::winbio_types::WINBIO_REJECT_DETAIL,
    pub Ticket: super::winbio_types::WINBIO_PROTECTION_TICKET,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
impl Default for WINBIO_ASYNC_RESULT_0_16 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_ASYNC_RESULT_0_17 {
    pub SelectorValue: u64,
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINBIO_ASYNC_RESULT_0_18 {
    pub ChangeType: super::winbio_types::WINBIO_PRESENCE_CHANGE,
    pub PresenceCount: usize,
    pub PresenceArray: *mut super::winbio_types::WINBIO_PRESENCE,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
impl Default for WINBIO_ASYNC_RESULT_0_18 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct WINBIO_ASYNC_RESULT_0_19 {
    pub Identity: super::winbio_types::WINBIO_IDENTITY,
    pub Policy: super::winbio_types::WINBIO_PROTECTION_POLICY,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
impl Default for WINBIO_ASYNC_RESULT_0_19 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_ASYNC_RESULT_0_2 {
    pub SubFactor: super::winbio_types::WINBIO_BIOMETRIC_SUBTYPE,
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_ASYNC_RESULT_0_20 {
    pub ExtendedStatus: super::winbio_types::WINBIO_EXTENDED_UNIT_STATUS,
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_ASYNC_RESULT_0_3 {
    pub RejectDetail: super::winbio_types::WINBIO_REJECT_DETAIL,
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct WINBIO_ASYNC_RESULT_0_4 {
    pub Identity: super::winbio_types::WINBIO_IDENTITY,
    pub IsNewTemplate: bool,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
impl Default for WINBIO_ASYNC_RESULT_0_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct WINBIO_ASYNC_RESULT_0_5 {
    pub Identity: super::winbio_types::WINBIO_IDENTITY,
    pub SubFactorCount: usize,
    pub SubFactorArray: *mut super::winbio_types::WINBIO_BIOMETRIC_SUBTYPE,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
impl Default for WINBIO_ASYNC_RESULT_0_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINBIO_ASYNC_RESULT_0_6 {
    pub Sample: super::winbio_types::PWINBIO_BIR,
    pub SampleSize: usize,
    pub RejectDetail: super::winbio_types::WINBIO_REJECT_DETAIL,
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct WINBIO_ASYNC_RESULT_0_7 {
    pub Identity: super::winbio_types::WINBIO_IDENTITY,
    pub SubFactor: super::winbio_types::WINBIO_BIOMETRIC_SUBTYPE,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
impl Default for WINBIO_ASYNC_RESULT_0_7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct WINBIO_ASYNC_RESULT_0_8 {
    pub PropertyType: super::winbio_types::WINBIO_PROPERTY_TYPE,
    pub PropertyId: super::winbio_types::WINBIO_PROPERTY_ID,
    pub Identity: super::winbio_types::WINBIO_IDENTITY,
    pub SubFactor: super::winbio_types::WINBIO_BIOMETRIC_SUBTYPE,
    pub PropertyBufferSize: usize,
    pub PropertyBuffer: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
impl Default for WINBIO_ASYNC_RESULT_0_8 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct WINBIO_ASYNC_RESULT_0_9 {
    pub PropertyType: super::winbio_types::WINBIO_PROPERTY_TYPE,
    pub PropertyId: super::winbio_types::WINBIO_PROPERTY_ID,
    pub Identity: super::winbio_types::WINBIO_IDENTITY,
    pub SubFactor: super::winbio_types::WINBIO_BIOMETRIC_SUBTYPE,
    pub PropertyBufferSize: usize,
    pub PropertyBuffer: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winbio_types", feature = "Win32_windef"))]
impl Default for WINBIO_ASYNC_RESULT_0_9 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
