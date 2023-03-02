#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioAcquireFocus() -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioAcquireFocus ( ) -> :: windows::core::HRESULT );
    WinBioAcquireFocus().ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioAsyncEnumBiometricUnits(frameworkhandle: u32, factor: u32) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioAsyncEnumBiometricUnits ( frameworkhandle : u32 , factor : u32 ) -> :: windows::core::HRESULT );
    WinBioAsyncEnumBiometricUnits(frameworkhandle, factor).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioAsyncEnumDatabases(frameworkhandle: u32, factor: u32) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioAsyncEnumDatabases ( frameworkhandle : u32 , factor : u32 ) -> :: windows::core::HRESULT );
    WinBioAsyncEnumDatabases(frameworkhandle, factor).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioAsyncEnumServiceProviders(frameworkhandle: u32, factor: u32) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioAsyncEnumServiceProviders ( frameworkhandle : u32 , factor : u32 ) -> :: windows::core::HRESULT );
    WinBioAsyncEnumServiceProviders(frameworkhandle, factor).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioAsyncMonitorFrameworkChanges(frameworkhandle: u32, changetypes: u32) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioAsyncMonitorFrameworkChanges ( frameworkhandle : u32 , changetypes : u32 ) -> :: windows::core::HRESULT );
    WinBioAsyncMonitorFrameworkChanges(frameworkhandle, changetypes).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinBioAsyncOpenFramework<P0, P1>(notificationmethod: WINBIO_ASYNC_NOTIFICATION_METHOD, targetwindow: P0, messagecode: u32, callbackroutine: PWINBIO_ASYNC_COMPLETION_CALLBACK, userdata: ::core::option::Option<*const ::core::ffi::c_void>, asynchronousopen: P1, frameworkhandle: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioAsyncOpenFramework ( notificationmethod : WINBIO_ASYNC_NOTIFICATION_METHOD , targetwindow : super::super::Foundation:: HWND , messagecode : u32 , callbackroutine : PWINBIO_ASYNC_COMPLETION_CALLBACK , userdata : *const ::core::ffi::c_void , asynchronousopen : super::super::Foundation:: BOOL , frameworkhandle : *mut u32 ) -> :: windows::core::HRESULT );
    WinBioAsyncOpenFramework(notificationmethod, targetwindow.into_param().abi(), messagecode, callbackroutine, ::core::mem::transmute(userdata.unwrap_or(::std::ptr::null())), asynchronousopen.into_param().abi(), ::core::mem::transmute(frameworkhandle.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinBioAsyncOpenSession<P0, P1>(factor: u32, pooltype: WINBIO_POOL, flags: u32, unitarray: ::core::option::Option<&[u32]>, databaseid: ::core::option::Option<*const ::windows::core::GUID>, notificationmethod: WINBIO_ASYNC_NOTIFICATION_METHOD, targetwindow: P0, messagecode: u32, callbackroutine: PWINBIO_ASYNC_COMPLETION_CALLBACK, userdata: ::core::option::Option<*const ::core::ffi::c_void>, asynchronousopen: P1, sessionhandle: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioAsyncOpenSession ( factor : u32 , pooltype : WINBIO_POOL , flags : u32 , unitarray : *const u32 , unitcount : usize , databaseid : *const :: windows::core::GUID , notificationmethod : WINBIO_ASYNC_NOTIFICATION_METHOD , targetwindow : super::super::Foundation:: HWND , messagecode : u32 , callbackroutine : PWINBIO_ASYNC_COMPLETION_CALLBACK , userdata : *const ::core::ffi::c_void , asynchronousopen : super::super::Foundation:: BOOL , sessionhandle : *mut u32 ) -> :: windows::core::HRESULT );
    WinBioAsyncOpenSession(
        factor,
        pooltype,
        flags,
        ::core::mem::transmute(unitarray.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        unitarray.as_deref().map_or(0, |slice| slice.len() as _),
        ::core::mem::transmute(databaseid.unwrap_or(::std::ptr::null())),
        notificationmethod,
        targetwindow.into_param().abi(),
        messagecode,
        callbackroutine,
        ::core::mem::transmute(userdata.unwrap_or(::std::ptr::null())),
        asynchronousopen.into_param().abi(),
        ::core::mem::transmute(sessionhandle.unwrap_or(::std::ptr::null_mut())),
    )
    .ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioCancel(sessionhandle: u32) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioCancel ( sessionhandle : u32 ) -> :: windows::core::HRESULT );
    WinBioCancel(sessionhandle).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioCaptureSample(sessionhandle: u32, purpose: u8, flags: u8, unitid: ::core::option::Option<*mut u32>, sample: *mut *mut WINBIO_BIR, samplesize: ::core::option::Option<*mut usize>, rejectdetail: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioCaptureSample ( sessionhandle : u32 , purpose : u8 , flags : u8 , unitid : *mut u32 , sample : *mut *mut WINBIO_BIR , samplesize : *mut usize , rejectdetail : *mut u32 ) -> :: windows::core::HRESULT );
    WinBioCaptureSample(sessionhandle, purpose, flags, ::core::mem::transmute(unitid.unwrap_or(::std::ptr::null_mut())), sample, ::core::mem::transmute(samplesize.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(rejectdetail.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioCaptureSampleWithCallback(sessionhandle: u32, purpose: u8, flags: u8, capturecallback: PWINBIO_CAPTURE_CALLBACK, capturecallbackcontext: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioCaptureSampleWithCallback ( sessionhandle : u32 , purpose : u8 , flags : u8 , capturecallback : PWINBIO_CAPTURE_CALLBACK , capturecallbackcontext : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    WinBioCaptureSampleWithCallback(sessionhandle, purpose, flags, capturecallback, ::core::mem::transmute(capturecallbackcontext.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioCloseFramework(frameworkhandle: u32) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioCloseFramework ( frameworkhandle : u32 ) -> :: windows::core::HRESULT );
    WinBioCloseFramework(frameworkhandle).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioCloseSession(sessionhandle: u32) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioCloseSession ( sessionhandle : u32 ) -> :: windows::core::HRESULT );
    WinBioCloseSession(sessionhandle).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioControlUnit(sessionhandle: u32, unitid: u32, component: WINBIO_COMPONENT, controlcode: u32, sendbuffer: &[u8], receivebuffer: &mut [u8], receivedatasize: *mut usize, operationstatus: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioControlUnit ( sessionhandle : u32 , unitid : u32 , component : WINBIO_COMPONENT , controlcode : u32 , sendbuffer : *const u8 , sendbuffersize : usize , receivebuffer : *mut u8 , receivebuffersize : usize , receivedatasize : *mut usize , operationstatus : *mut u32 ) -> :: windows::core::HRESULT );
    WinBioControlUnit(sessionhandle, unitid, component, controlcode, ::core::mem::transmute(sendbuffer.as_ptr()), sendbuffer.len() as _, ::core::mem::transmute(receivebuffer.as_ptr()), receivebuffer.len() as _, receivedatasize, ::core::mem::transmute(operationstatus.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioControlUnitPrivileged(sessionhandle: u32, unitid: u32, component: WINBIO_COMPONENT, controlcode: u32, sendbuffer: &[u8], receivebuffer: &mut [u8], receivedatasize: *mut usize, operationstatus: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioControlUnitPrivileged ( sessionhandle : u32 , unitid : u32 , component : WINBIO_COMPONENT , controlcode : u32 , sendbuffer : *const u8 , sendbuffersize : usize , receivebuffer : *mut u8 , receivebuffersize : usize , receivedatasize : *mut usize , operationstatus : *mut u32 ) -> :: windows::core::HRESULT );
    WinBioControlUnitPrivileged(sessionhandle, unitid, component, controlcode, ::core::mem::transmute(sendbuffer.as_ptr()), sendbuffer.len() as _, ::core::mem::transmute(receivebuffer.as_ptr()), receivebuffer.len() as _, receivedatasize, ::core::mem::transmute(operationstatus.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioDeleteTemplate(sessionhandle: u32, unitid: u32, identity: *const WINBIO_IDENTITY, subfactor: u8) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioDeleteTemplate ( sessionhandle : u32 , unitid : u32 , identity : *const WINBIO_IDENTITY , subfactor : u8 ) -> :: windows::core::HRESULT );
    WinBioDeleteTemplate(sessionhandle, unitid, identity, subfactor).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioEnrollBegin(sessionhandle: u32, subfactor: u8, unitid: u32) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioEnrollBegin ( sessionhandle : u32 , subfactor : u8 , unitid : u32 ) -> :: windows::core::HRESULT );
    WinBioEnrollBegin(sessionhandle, subfactor, unitid).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioEnrollCapture(sessionhandle: u32, rejectdetail: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioEnrollCapture ( sessionhandle : u32 , rejectdetail : *mut u32 ) -> :: windows::core::HRESULT );
    WinBioEnrollCapture(sessionhandle, ::core::mem::transmute(rejectdetail.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioEnrollCaptureWithCallback(sessionhandle: u32, enrollcallback: PWINBIO_ENROLL_CAPTURE_CALLBACK, enrollcallbackcontext: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioEnrollCaptureWithCallback ( sessionhandle : u32 , enrollcallback : PWINBIO_ENROLL_CAPTURE_CALLBACK , enrollcallbackcontext : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    WinBioEnrollCaptureWithCallback(sessionhandle, enrollcallback, ::core::mem::transmute(enrollcallbackcontext.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioEnrollCommit(sessionhandle: u32, identity: ::core::option::Option<*mut WINBIO_IDENTITY>, isnewtemplate: ::core::option::Option<*mut u8>) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioEnrollCommit ( sessionhandle : u32 , identity : *mut WINBIO_IDENTITY , isnewtemplate : *mut u8 ) -> :: windows::core::HRESULT );
    WinBioEnrollCommit(sessionhandle, ::core::mem::transmute(identity.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(isnewtemplate.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioEnrollDiscard(sessionhandle: u32) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioEnrollDiscard ( sessionhandle : u32 ) -> :: windows::core::HRESULT );
    WinBioEnrollDiscard(sessionhandle).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioEnrollSelect(sessionhandle: u32, selectorvalue: u64) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioEnrollSelect ( sessionhandle : u32 , selectorvalue : u64 ) -> :: windows::core::HRESULT );
    WinBioEnrollSelect(sessionhandle, selectorvalue).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioEnumBiometricUnits(factor: u32, unitschemaarray: *mut *mut WINBIO_UNIT_SCHEMA, unitcount: *mut usize) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioEnumBiometricUnits ( factor : u32 , unitschemaarray : *mut *mut WINBIO_UNIT_SCHEMA , unitcount : *mut usize ) -> :: windows::core::HRESULT );
    WinBioEnumBiometricUnits(factor, unitschemaarray, unitcount).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioEnumDatabases(factor: u32, storageschemaarray: *mut *mut WINBIO_STORAGE_SCHEMA, storagecount: *mut usize) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioEnumDatabases ( factor : u32 , storageschemaarray : *mut *mut WINBIO_STORAGE_SCHEMA , storagecount : *mut usize ) -> :: windows::core::HRESULT );
    WinBioEnumDatabases(factor, storageschemaarray, storagecount).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioEnumEnrollments(sessionhandle: u32, unitid: u32, identity: *const WINBIO_IDENTITY, subfactorarray: *mut *mut u8, subfactorcount: ::core::option::Option<*mut usize>) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioEnumEnrollments ( sessionhandle : u32 , unitid : u32 , identity : *const WINBIO_IDENTITY , subfactorarray : *mut *mut u8 , subfactorcount : *mut usize ) -> :: windows::core::HRESULT );
    WinBioEnumEnrollments(sessionhandle, unitid, identity, subfactorarray, ::core::mem::transmute(subfactorcount.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioEnumServiceProviders(factor: u32, bspschemaarray: *mut *mut WINBIO_BSP_SCHEMA, bspcount: *mut usize) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioEnumServiceProviders ( factor : u32 , bspschemaarray : *mut *mut WINBIO_BSP_SCHEMA , bspcount : *mut usize ) -> :: windows::core::HRESULT );
    WinBioEnumServiceProviders(factor, bspschemaarray, bspcount).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioFree(address: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioFree ( address : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    WinBioFree(address).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioGetCredentialState(identity: WINBIO_IDENTITY, r#type: WINBIO_CREDENTIAL_TYPE) -> ::windows::core::Result<WINBIO_CREDENTIAL_STATE> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioGetCredentialState ( identity : WINBIO_IDENTITY , r#type : WINBIO_CREDENTIAL_TYPE , credentialstate : *mut WINBIO_CREDENTIAL_STATE ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<WINBIO_CREDENTIAL_STATE>();
    WinBioGetCredentialState(::core::mem::transmute(identity), r#type, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioGetDomainLogonSetting(value: *mut u8, source: *mut WINBIO_SETTING_SOURCE) {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioGetDomainLogonSetting ( value : *mut u8 , source : *mut WINBIO_SETTING_SOURCE ) -> ( ) );
    WinBioGetDomainLogonSetting(value, source)
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioGetEnabledSetting(value: *mut u8, source: *mut WINBIO_SETTING_SOURCE) {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioGetEnabledSetting ( value : *mut u8 , source : *mut WINBIO_SETTING_SOURCE ) -> ( ) );
    WinBioGetEnabledSetting(value, source)
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioGetEnrolledFactors(accountowner: *const WINBIO_IDENTITY) -> ::windows::core::Result<u32> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioGetEnrolledFactors ( accountowner : *const WINBIO_IDENTITY , enrolledfactors : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u32>();
    WinBioGetEnrolledFactors(accountowner, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioGetLogonSetting(value: *mut u8, source: *mut WINBIO_SETTING_SOURCE) {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioGetLogonSetting ( value : *mut u8 , source : *mut WINBIO_SETTING_SOURCE ) -> ( ) );
    WinBioGetLogonSetting(value, source)
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioGetProperty(sessionhandle: u32, propertytype: u32, propertyid: u32, unitid: u32, identity: ::core::option::Option<*const WINBIO_IDENTITY>, subfactor: u8, propertybuffer: *mut *mut ::core::ffi::c_void, propertybuffersize: ::core::option::Option<*mut usize>) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioGetProperty ( sessionhandle : u32 , propertytype : u32 , propertyid : u32 , unitid : u32 , identity : *const WINBIO_IDENTITY , subfactor : u8 , propertybuffer : *mut *mut ::core::ffi::c_void , propertybuffersize : *mut usize ) -> :: windows::core::HRESULT );
    WinBioGetProperty(sessionhandle, propertytype, propertyid, unitid, ::core::mem::transmute(identity.unwrap_or(::std::ptr::null())), subfactor, propertybuffer, ::core::mem::transmute(propertybuffersize.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioIdentify(sessionhandle: u32, unitid: ::core::option::Option<*mut u32>, identity: ::core::option::Option<*mut WINBIO_IDENTITY>, subfactor: ::core::option::Option<*mut u8>, rejectdetail: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioIdentify ( sessionhandle : u32 , unitid : *mut u32 , identity : *mut WINBIO_IDENTITY , subfactor : *mut u8 , rejectdetail : *mut u32 ) -> :: windows::core::HRESULT );
    WinBioIdentify(sessionhandle, ::core::mem::transmute(unitid.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(identity.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(subfactor.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(rejectdetail.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioIdentifyWithCallback(sessionhandle: u32, identifycallback: PWINBIO_IDENTIFY_CALLBACK, identifycallbackcontext: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioIdentifyWithCallback ( sessionhandle : u32 , identifycallback : PWINBIO_IDENTIFY_CALLBACK , identifycallbackcontext : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    WinBioIdentifyWithCallback(sessionhandle, identifycallback, ::core::mem::transmute(identifycallbackcontext.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioImproveBegin(sessionhandle: u32, unitid: u32) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioImproveBegin ( sessionhandle : u32 , unitid : u32 ) -> :: windows::core::HRESULT );
    WinBioImproveBegin(sessionhandle, unitid).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioImproveEnd(sessionhandle: u32) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioImproveEnd ( sessionhandle : u32 ) -> :: windows::core::HRESULT );
    WinBioImproveEnd(sessionhandle).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioLocateSensor(sessionhandle: u32, unitid: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioLocateSensor ( sessionhandle : u32 , unitid : *mut u32 ) -> :: windows::core::HRESULT );
    WinBioLocateSensor(sessionhandle, ::core::mem::transmute(unitid.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioLocateSensorWithCallback(sessionhandle: u32, locatecallback: PWINBIO_LOCATE_SENSOR_CALLBACK, locatecallbackcontext: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioLocateSensorWithCallback ( sessionhandle : u32 , locatecallback : PWINBIO_LOCATE_SENSOR_CALLBACK , locatecallbackcontext : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    WinBioLocateSensorWithCallback(sessionhandle, locatecallback, ::core::mem::transmute(locatecallbackcontext.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioLockUnit(sessionhandle: u32, unitid: u32) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioLockUnit ( sessionhandle : u32 , unitid : u32 ) -> :: windows::core::HRESULT );
    WinBioLockUnit(sessionhandle, unitid).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioLogonIdentifiedUser(sessionhandle: u32) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioLogonIdentifiedUser ( sessionhandle : u32 ) -> :: windows::core::HRESULT );
    WinBioLogonIdentifiedUser(sessionhandle).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioMonitorPresence(sessionhandle: u32, unitid: u32) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioMonitorPresence ( sessionhandle : u32 , unitid : u32 ) -> :: windows::core::HRESULT );
    WinBioMonitorPresence(sessionhandle, unitid).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioOpenSession(factor: u32, pooltype: WINBIO_POOL, flags: u32, unitarray: ::core::option::Option<&[u32]>, databaseid: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<u32> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioOpenSession ( factor : u32 , pooltype : WINBIO_POOL , flags : u32 , unitarray : *const u32 , unitcount : usize , databaseid : *const :: windows::core::GUID , sessionhandle : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u32>();
    WinBioOpenSession(factor, pooltype, flags, ::core::mem::transmute(unitarray.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), unitarray.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(databaseid.unwrap_or(::std::ptr::null())), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioRegisterEventMonitor(sessionhandle: u32, eventmask: u32, eventcallback: PWINBIO_EVENT_CALLBACK, eventcallbackcontext: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioRegisterEventMonitor ( sessionhandle : u32 , eventmask : u32 , eventcallback : PWINBIO_EVENT_CALLBACK , eventcallbackcontext : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    WinBioRegisterEventMonitor(sessionhandle, eventmask, eventcallback, ::core::mem::transmute(eventcallbackcontext.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioReleaseFocus() -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioReleaseFocus ( ) -> :: windows::core::HRESULT );
    WinBioReleaseFocus().ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioRemoveAllCredentials() -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioRemoveAllCredentials ( ) -> :: windows::core::HRESULT );
    WinBioRemoveAllCredentials().ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioRemoveAllDomainCredentials() -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioRemoveAllDomainCredentials ( ) -> :: windows::core::HRESULT );
    WinBioRemoveAllDomainCredentials().ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioRemoveCredential(identity: WINBIO_IDENTITY, r#type: WINBIO_CREDENTIAL_TYPE) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioRemoveCredential ( identity : WINBIO_IDENTITY , r#type : WINBIO_CREDENTIAL_TYPE ) -> :: windows::core::HRESULT );
    WinBioRemoveCredential(::core::mem::transmute(identity), r#type).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioSetCredential(r#type: WINBIO_CREDENTIAL_TYPE, credential: &[u8], format: WINBIO_CREDENTIAL_FORMAT) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioSetCredential ( r#type : WINBIO_CREDENTIAL_TYPE , credential : *const u8 , credentialsize : usize , format : WINBIO_CREDENTIAL_FORMAT ) -> :: windows::core::HRESULT );
    WinBioSetCredential(r#type, ::core::mem::transmute(credential.as_ptr()), credential.len() as _, format).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioSetProperty(sessionhandle: u32, propertytype: u32, propertyid: u32, unitid: u32, identity: ::core::option::Option<*const WINBIO_IDENTITY>, subfactor: u8, propertybuffer: *const ::core::ffi::c_void, propertybuffersize: usize) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioSetProperty ( sessionhandle : u32 , propertytype : u32 , propertyid : u32 , unitid : u32 , identity : *const WINBIO_IDENTITY , subfactor : u8 , propertybuffer : *const ::core::ffi::c_void , propertybuffersize : usize ) -> :: windows::core::HRESULT );
    WinBioSetProperty(sessionhandle, propertytype, propertyid, unitid, ::core::mem::transmute(identity.unwrap_or(::std::ptr::null())), subfactor, propertybuffer, propertybuffersize).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioUnlockUnit(sessionhandle: u32, unitid: u32) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioUnlockUnit ( sessionhandle : u32 , unitid : u32 ) -> :: windows::core::HRESULT );
    WinBioUnlockUnit(sessionhandle, unitid).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioUnregisterEventMonitor(sessionhandle: u32) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioUnregisterEventMonitor ( sessionhandle : u32 ) -> :: windows::core::HRESULT );
    WinBioUnregisterEventMonitor(sessionhandle).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioVerify(sessionhandle: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, unitid: ::core::option::Option<*mut u32>, r#match: ::core::option::Option<*mut u8>, rejectdetail: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioVerify ( sessionhandle : u32 , identity : *const WINBIO_IDENTITY , subfactor : u8 , unitid : *mut u32 , r#match : *mut u8 , rejectdetail : *mut u32 ) -> :: windows::core::HRESULT );
    WinBioVerify(sessionhandle, identity, subfactor, ::core::mem::transmute(unitid.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(r#match.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(rejectdetail.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinBioVerifyWithCallback(sessionhandle: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, verifycallback: PWINBIO_VERIFY_CALLBACK, verifycallbackcontext: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioVerifyWithCallback ( sessionhandle : u32 , identity : *const WINBIO_IDENTITY , subfactor : u8 , verifycallback : PWINBIO_VERIFY_CALLBACK , verifycallbackcontext : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    WinBioVerifyWithCallback(sessionhandle, identity, subfactor, verifycallback, ::core::mem::transmute(verifycallbackcontext.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[inline]
pub unsafe fn WinBioWait(sessionhandle: u32) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "winbio.dll""system" fn WinBioWait ( sessionhandle : u32 ) -> :: windows::core::HRESULT );
    WinBioWait(sessionhandle).ok()
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const FACILITY_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const FACILITY_WINBIO: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const GUID_DEVINTERFACE_BIOMETRIC_READER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2b5183a_99ea_4cc3_ad6b_80ca8d715b80);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const IOCTL_BIOMETRIC_VENDOR: u32 = 4464640u32;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_BIR_ALGIN_SIZE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_BIR_ALIGN_SIZE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_ADAPTER_INTEGRITY_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860995i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_AUTO_LOGON_DISABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860989i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_BAD_CAPTURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861048i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_CALIBRATION_BUFFER_INVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860975i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_CALIBRATION_BUFFER_TOO_LARGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860976i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_CALIBRATION_BUFFER_TOO_SMALL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860977i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_CANCELED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861052i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_CAPTURE_ABORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861050i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_CONFIGURATION_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861005i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_CRED_PROV_DISABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861008i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_CRED_PROV_NO_CREDENTIAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861007i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_CRED_PROV_SECURITY_LOCKOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860985i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_DATABASE_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861034i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_DATABASE_BAD_INDEX_VECTOR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861022i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_DATABASE_CANT_CLOSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861037i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_DATABASE_CANT_CREATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861039i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_DATABASE_CANT_ERASE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861036i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_DATABASE_CANT_FIND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861035i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_DATABASE_CANT_OPEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861038i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_DATABASE_CORRUPTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861030i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_DATABASE_EOF: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861023i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_DATABASE_FULL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861032i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_DATABASE_LOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861031i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_DATABASE_NO_MORE_RECORDS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861024i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_DATABASE_NO_RESULTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861025i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_DATABASE_NO_SUCH_RECORD: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861029i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_DATABASE_READ_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861027i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_DATABASE_WRITE_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861026i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_DATA_COLLECTION_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861045i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_DATA_PROTECTION_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860986i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_DEADLOCK_DETECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860992i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_DEVICE_BUSY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861040i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_DEVICE_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861002i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_DISABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861006i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_DUPLICATE_ENROLLMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861028i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_DUPLICATE_TEMPLATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861013i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_ENROLLMENT_CANCELED_BY_SUSPEND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860965i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_ENROLLMENT_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861049i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_EVENT_MONITOR_ACTIVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860999i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_FAST_USER_SWITCH_DISABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861001i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_INCORRECT_BSP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861020i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_INCORRECT_SENSOR_POOL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861019i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_INCORRECT_SESSION_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860994i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_INSECURE_SENSOR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860969i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_INVALID_BUFFER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860967i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_INVALID_BUFFER_ID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860968i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_INVALID_CALIBRATION_FORMAT_ARRAY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860980i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_INVALID_CONTROL_CODE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861047i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_INVALID_DEVICE_STATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861041i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_INVALID_KEY_IDENTIFIER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860974i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_INVALID_OPERATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861012i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_INVALID_PROPERTY_ID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860997i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_INVALID_PROPERTY_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860998i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_INVALID_SENSOR_MODE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861017i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_INVALID_SUBFACTOR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860981i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_INVALID_TICKET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860988i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_INVALID_UNIT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861054i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_KEY_CREATION_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860973i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_KEY_IDENTIFIER_BUFFER_TOO_SMALL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860972i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_LOCK_VIOLATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861014i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_MAX_ERROR_COUNT_EXCEEDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860990i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_NOT_ACTIVE_CONSOLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861000i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_NO_CAPTURE_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861018i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_NO_MATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861051i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_NO_PREBOOT_IDENTITY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860991i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_NO_SUPPORTED_CALIBRATION_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860979i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_POLICY_PROTECTION_UNAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860970i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_PRESENCE_MONITOR_ACTIVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860982i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_PROPERTY_UNAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860971i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_SAS_ENABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861003i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_SELECTION_REQUIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860983i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_SENSOR_UNAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861004i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_SESSION_BUSY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861011i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_SESSION_HANDLE_CLOSED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860993i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_TICKET_QUOTA_EXCEEDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860987i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_TRUSTLET_INTEGRITY_FAIL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860966i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_UNKNOWN_ID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861053i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_UNSUPPORTED_DATA_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861044i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_UNSUPPORTED_DATA_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861043i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_UNSUPPORTED_FACTOR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861055i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_UNSUPPORTED_POOL_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860984i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_UNSUPPORTED_PROPERTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860996i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_UNSUPPORTED_PURPOSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861042i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_E_UNSUPPORTED_SENSOR_CALIBRATION_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860978i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_I_EXTENDED_STATUS_INFORMATION: ::windows::core::HRESULT = ::windows::core::HRESULT(589826i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_I_MORE_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(589825i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_MAX_STRING_LEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_SCP_CURVE_FIELD_SIZE_V1: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_SCP_DIGEST_SIZE_V1: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_SCP_ENCRYPTION_BLOCK_SIZE_V1: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_SCP_ENCRYPTION_KEY_SIZE_V1: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_SCP_PRIVATE_KEY_SIZE_V1: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_SCP_PUBLIC_KEY_SIZE_V1: u32 = 65u32;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_SCP_RANDOM_SIZE_V1: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_SCP_SIGNATURE_SIZE_V1: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_SCP_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_WBDI_MAJOR_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_WBDI_MINOR_VERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINBIO_ANTI_SPOOF_POLICY_ACTION(pub i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_ANTI_SPOOF_DISABLE: WINBIO_ANTI_SPOOF_POLICY_ACTION = WINBIO_ANTI_SPOOF_POLICY_ACTION(0i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_ANTI_SPOOF_ENABLE: WINBIO_ANTI_SPOOF_POLICY_ACTION = WINBIO_ANTI_SPOOF_POLICY_ACTION(1i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_ANTI_SPOOF_REMOVE: WINBIO_ANTI_SPOOF_POLICY_ACTION = WINBIO_ANTI_SPOOF_POLICY_ACTION(2i32);
impl ::core::marker::Copy for WINBIO_ANTI_SPOOF_POLICY_ACTION {}
impl ::core::clone::Clone for WINBIO_ANTI_SPOOF_POLICY_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINBIO_ANTI_SPOOF_POLICY_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WINBIO_ANTI_SPOOF_POLICY_ACTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WINBIO_ANTI_SPOOF_POLICY_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINBIO_ANTI_SPOOF_POLICY_ACTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINBIO_ASYNC_NOTIFICATION_METHOD(pub i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_ASYNC_NOTIFY_NONE: WINBIO_ASYNC_NOTIFICATION_METHOD = WINBIO_ASYNC_NOTIFICATION_METHOD(0i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_ASYNC_NOTIFY_CALLBACK: WINBIO_ASYNC_NOTIFICATION_METHOD = WINBIO_ASYNC_NOTIFICATION_METHOD(1i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_ASYNC_NOTIFY_MESSAGE: WINBIO_ASYNC_NOTIFICATION_METHOD = WINBIO_ASYNC_NOTIFICATION_METHOD(2i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_ASYNC_NOTIFY_MAXIMUM_VALUE: WINBIO_ASYNC_NOTIFICATION_METHOD = WINBIO_ASYNC_NOTIFICATION_METHOD(3i32);
impl ::core::marker::Copy for WINBIO_ASYNC_NOTIFICATION_METHOD {}
impl ::core::clone::Clone for WINBIO_ASYNC_NOTIFICATION_METHOD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINBIO_ASYNC_NOTIFICATION_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WINBIO_ASYNC_NOTIFICATION_METHOD {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WINBIO_ASYNC_NOTIFICATION_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINBIO_ASYNC_NOTIFICATION_METHOD").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINBIO_COMPONENT(pub u32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_COMPONENT_SENSOR: WINBIO_COMPONENT = WINBIO_COMPONENT(1u32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_COMPONENT_ENGINE: WINBIO_COMPONENT = WINBIO_COMPONENT(2u32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_COMPONENT_STORAGE: WINBIO_COMPONENT = WINBIO_COMPONENT(3u32);
impl ::core::marker::Copy for WINBIO_COMPONENT {}
impl ::core::clone::Clone for WINBIO_COMPONENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINBIO_COMPONENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WINBIO_COMPONENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WINBIO_COMPONENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINBIO_COMPONENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINBIO_CREDENTIAL_FORMAT(pub i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_PASSWORD_GENERIC: WINBIO_CREDENTIAL_FORMAT = WINBIO_CREDENTIAL_FORMAT(1i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_PASSWORD_PACKED: WINBIO_CREDENTIAL_FORMAT = WINBIO_CREDENTIAL_FORMAT(2i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_PASSWORD_PROTECTED: WINBIO_CREDENTIAL_FORMAT = WINBIO_CREDENTIAL_FORMAT(3i32);
impl ::core::marker::Copy for WINBIO_CREDENTIAL_FORMAT {}
impl ::core::clone::Clone for WINBIO_CREDENTIAL_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINBIO_CREDENTIAL_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WINBIO_CREDENTIAL_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WINBIO_CREDENTIAL_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINBIO_CREDENTIAL_FORMAT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINBIO_CREDENTIAL_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_CREDENTIAL_NOT_SET: WINBIO_CREDENTIAL_STATE = WINBIO_CREDENTIAL_STATE(1i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_CREDENTIAL_SET: WINBIO_CREDENTIAL_STATE = WINBIO_CREDENTIAL_STATE(2i32);
impl ::core::marker::Copy for WINBIO_CREDENTIAL_STATE {}
impl ::core::clone::Clone for WINBIO_CREDENTIAL_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINBIO_CREDENTIAL_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WINBIO_CREDENTIAL_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WINBIO_CREDENTIAL_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINBIO_CREDENTIAL_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINBIO_CREDENTIAL_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_CREDENTIAL_PASSWORD: WINBIO_CREDENTIAL_TYPE = WINBIO_CREDENTIAL_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_CREDENTIAL_ALL: WINBIO_CREDENTIAL_TYPE = WINBIO_CREDENTIAL_TYPE(-1i32);
impl ::core::marker::Copy for WINBIO_CREDENTIAL_TYPE {}
impl ::core::clone::Clone for WINBIO_CREDENTIAL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINBIO_CREDENTIAL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WINBIO_CREDENTIAL_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WINBIO_CREDENTIAL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINBIO_CREDENTIAL_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINBIO_POLICY_SOURCE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_POLICY_UNKNOWN: WINBIO_POLICY_SOURCE = WINBIO_POLICY_SOURCE(0i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_POLICY_DEFAULT: WINBIO_POLICY_SOURCE = WINBIO_POLICY_SOURCE(1i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_POLICY_LOCAL: WINBIO_POLICY_SOURCE = WINBIO_POLICY_SOURCE(2i32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_POLICY_ADMIN: WINBIO_POLICY_SOURCE = WINBIO_POLICY_SOURCE(3i32);
impl ::core::marker::Copy for WINBIO_POLICY_SOURCE {}
impl ::core::clone::Clone for WINBIO_POLICY_SOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINBIO_POLICY_SOURCE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WINBIO_POLICY_SOURCE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WINBIO_POLICY_SOURCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINBIO_POLICY_SOURCE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINBIO_POOL(pub u32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_POOL_SYSTEM: WINBIO_POOL = WINBIO_POOL(1u32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_POOL_PRIVATE: WINBIO_POOL = WINBIO_POOL(2u32);
impl ::core::marker::Copy for WINBIO_POOL {}
impl ::core::clone::Clone for WINBIO_POOL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINBIO_POOL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WINBIO_POOL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WINBIO_POOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINBIO_POOL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINBIO_SETTING_SOURCE(pub u32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_SETTING_SOURCE_INVALID: WINBIO_SETTING_SOURCE = WINBIO_SETTING_SOURCE(0u32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_SETTING_SOURCE_DEFAULT: WINBIO_SETTING_SOURCE = WINBIO_SETTING_SOURCE(1u32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_SETTING_SOURCE_LOCAL: WINBIO_SETTING_SOURCE = WINBIO_SETTING_SOURCE(3u32);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub const WINBIO_SETTING_SOURCE_POLICY: WINBIO_SETTING_SOURCE = WINBIO_SETTING_SOURCE(2u32);
impl ::core::marker::Copy for WINBIO_SETTING_SOURCE {}
impl ::core::clone::Clone for WINBIO_SETTING_SOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINBIO_SETTING_SOURCE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WINBIO_SETTING_SOURCE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WINBIO_SETTING_SOURCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINBIO_SETTING_SOURCE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_ACCOUNT_POLICY {
    pub Identity: WINBIO_IDENTITY,
    pub AntiSpoofBehavior: WINBIO_ANTI_SPOOF_POLICY_ACTION,
}
impl ::core::marker::Copy for WINBIO_ACCOUNT_POLICY {}
impl ::core::clone::Clone for WINBIO_ACCOUNT_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WINBIO_ACCOUNT_POLICY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WINBIO_ACCOUNT_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_ADAPTER_INTERFACE_VERSION {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
}
impl ::core::marker::Copy for WINBIO_ADAPTER_INTERFACE_VERSION {}
impl ::core::clone::Clone for WINBIO_ADAPTER_INTERFACE_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_ADAPTER_INTERFACE_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_ADAPTER_INTERFACE_VERSION").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_ADAPTER_INTERFACE_VERSION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_ADAPTER_INTERFACE_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion
    }
}
impl ::core::cmp::Eq for WINBIO_ADAPTER_INTERFACE_VERSION {}
impl ::core::default::Default for WINBIO_ADAPTER_INTERFACE_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_ANTI_SPOOF_POLICY {
    pub Action: WINBIO_ANTI_SPOOF_POLICY_ACTION,
    pub Source: WINBIO_POLICY_SOURCE,
}
impl ::core::marker::Copy for WINBIO_ANTI_SPOOF_POLICY {}
impl ::core::clone::Clone for WINBIO_ANTI_SPOOF_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_ANTI_SPOOF_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_ANTI_SPOOF_POLICY").field("Action", &self.Action).field("Source", &self.Source).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_ANTI_SPOOF_POLICY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_ANTI_SPOOF_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Action == other.Action && self.Source == other.Source
    }
}
impl ::core::cmp::Eq for WINBIO_ANTI_SPOOF_POLICY {}
impl ::core::default::Default for WINBIO_ANTI_SPOOF_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT {
    pub SessionHandle: u32,
    pub Operation: u32,
    pub SequenceNumber: u64,
    pub TimeStamp: i64,
    pub ApiStatus: ::windows::core::HRESULT,
    pub UnitId: u32,
    pub UserData: *mut ::core::ffi::c_void,
    pub Parameters: WINBIO_ASYNC_RESULT_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_ASYNC_RESULT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union WINBIO_ASYNC_RESULT_0 {
    pub Verify: WINBIO_ASYNC_RESULT_0_20,
    pub Identify: WINBIO_ASYNC_RESULT_0_15,
    pub EnrollBegin: WINBIO_ASYNC_RESULT_0_3,
    pub EnrollCapture: WINBIO_ASYNC_RESULT_0_4,
    pub EnrollCommit: WINBIO_ASYNC_RESULT_0_5,
    pub EnumEnrollments: WINBIO_ASYNC_RESULT_0_9,
    pub CaptureSample: WINBIO_ASYNC_RESULT_0_0,
    pub DeleteTemplate: WINBIO_ASYNC_RESULT_0_2,
    pub GetProperty: WINBIO_ASYNC_RESULT_0_12,
    pub SetProperty: WINBIO_ASYNC_RESULT_0_18,
    pub GetEvent: WINBIO_ASYNC_RESULT_0_11,
    pub ControlUnit: WINBIO_ASYNC_RESULT_0_1,
    pub EnumServiceProviders: WINBIO_ASYNC_RESULT_0_10,
    pub EnumBiometricUnits: WINBIO_ASYNC_RESULT_0_7,
    pub EnumDatabases: WINBIO_ASYNC_RESULT_0_8,
    pub VerifyAndReleaseTicket: WINBIO_ASYNC_RESULT_0_19,
    pub IdentifyAndReleaseTicket: WINBIO_ASYNC_RESULT_0_14,
    pub EnrollSelect: WINBIO_ASYNC_RESULT_0_6,
    pub MonitorPresence: WINBIO_ASYNC_RESULT_0_16,
    pub GetProtectionPolicy: WINBIO_ASYNC_RESULT_0_13,
    pub NotifyUnitStatusChange: WINBIO_ASYNC_RESULT_0_17,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_ASYNC_RESULT_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_0 {
    pub Sample: *mut WINBIO_BIR,
    pub SampleSize: usize,
    pub RejectDetail: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINBIO_ASYNC_RESULT_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_ASYNC_RESULT_0_0").field("Sample", &self.Sample).field("SampleSize", &self.SampleSize).field("RejectDetail", &self.RejectDetail).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_ASYNC_RESULT_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Sample == other.Sample && self.SampleSize == other.SampleSize && self.RejectDetail == other.RejectDetail
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_1 {
    pub Component: WINBIO_COMPONENT,
    pub ControlCode: u32,
    pub OperationStatus: u32,
    pub SendBuffer: *mut u8,
    pub SendBufferSize: usize,
    pub ReceiveBuffer: *mut u8,
    pub ReceiveBufferSize: usize,
    pub ReceiveDataSize: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINBIO_ASYNC_RESULT_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_ASYNC_RESULT_0_1").field("Component", &self.Component).field("ControlCode", &self.ControlCode).field("OperationStatus", &self.OperationStatus).field("SendBuffer", &self.SendBuffer).field("SendBufferSize", &self.SendBufferSize).field("ReceiveBuffer", &self.ReceiveBuffer).field("ReceiveBufferSize", &self.ReceiveBufferSize).field("ReceiveDataSize", &self.ReceiveDataSize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_ASYNC_RESULT_0_1 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Component == other.Component && self.ControlCode == other.ControlCode && self.OperationStatus == other.OperationStatus && self.SendBuffer == other.SendBuffer && self.SendBufferSize == other.SendBufferSize && self.ReceiveBuffer == other.ReceiveBuffer && self.ReceiveBufferSize == other.ReceiveBufferSize && self.ReceiveDataSize == other.ReceiveDataSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_2 {
    pub Identity: WINBIO_IDENTITY,
    pub SubFactor: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_ASYNC_RESULT_0_2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_3 {
    pub SubFactor: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINBIO_ASYNC_RESULT_0_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_ASYNC_RESULT_0_3").field("SubFactor", &self.SubFactor).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_ASYNC_RESULT_0_3 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.SubFactor == other.SubFactor
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT_0_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_4 {
    pub RejectDetail: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINBIO_ASYNC_RESULT_0_4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_ASYNC_RESULT_0_4").field("RejectDetail", &self.RejectDetail).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_ASYNC_RESULT_0_4 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_4 {
    fn eq(&self, other: &Self) -> bool {
        self.RejectDetail == other.RejectDetail
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT_0_4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_5 {
    pub Identity: WINBIO_IDENTITY,
    pub IsNewTemplate: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_ASYNC_RESULT_0_5 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_6 {
    pub SelectorValue: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINBIO_ASYNC_RESULT_0_6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_ASYNC_RESULT_0_6").field("SelectorValue", &self.SelectorValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_ASYNC_RESULT_0_6 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_6 {
    fn eq(&self, other: &Self) -> bool {
        self.SelectorValue == other.SelectorValue
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT_0_6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_7 {
    pub UnitCount: usize,
    pub UnitSchemaArray: *mut WINBIO_UNIT_SCHEMA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_7 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINBIO_ASYNC_RESULT_0_7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_ASYNC_RESULT_0_7").field("UnitCount", &self.UnitCount).field("UnitSchemaArray", &self.UnitSchemaArray).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_ASYNC_RESULT_0_7 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_7 {
    fn eq(&self, other: &Self) -> bool {
        self.UnitCount == other.UnitCount && self.UnitSchemaArray == other.UnitSchemaArray
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT_0_7 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_7 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_8 {
    pub StorageCount: usize,
    pub StorageSchemaArray: *mut WINBIO_STORAGE_SCHEMA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_8 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_8 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINBIO_ASYNC_RESULT_0_8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_ASYNC_RESULT_0_8").field("StorageCount", &self.StorageCount).field("StorageSchemaArray", &self.StorageSchemaArray).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_ASYNC_RESULT_0_8 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_8 {
    fn eq(&self, other: &Self) -> bool {
        self.StorageCount == other.StorageCount && self.StorageSchemaArray == other.StorageSchemaArray
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT_0_8 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_8 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_9 {
    pub Identity: WINBIO_IDENTITY,
    pub SubFactorCount: usize,
    pub SubFactorArray: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_9 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_ASYNC_RESULT_0_9 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_9 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_10 {
    pub BspCount: usize,
    pub BspSchemaArray: *mut WINBIO_BSP_SCHEMA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_10 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_10 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINBIO_ASYNC_RESULT_0_10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_ASYNC_RESULT_0_10").field("BspCount", &self.BspCount).field("BspSchemaArray", &self.BspSchemaArray).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_ASYNC_RESULT_0_10 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_10 {
    fn eq(&self, other: &Self) -> bool {
        self.BspCount == other.BspCount && self.BspSchemaArray == other.BspSchemaArray
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT_0_10 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_10 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_11 {
    pub Event: WINBIO_EVENT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_11 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_11 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_ASYNC_RESULT_0_11 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_11 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_12 {
    pub PropertyType: u32,
    pub PropertyId: u32,
    pub Identity: WINBIO_IDENTITY,
    pub SubFactor: u8,
    pub PropertyBufferSize: usize,
    pub PropertyBuffer: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_12 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_12 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_ASYNC_RESULT_0_12 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_12 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_13 {
    pub Identity: WINBIO_IDENTITY,
    pub Policy: WINBIO_PROTECTION_POLICY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_13 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_13 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_ASYNC_RESULT_0_13 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_13 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_14 {
    pub Identity: WINBIO_IDENTITY,
    pub SubFactor: u8,
    pub RejectDetail: u32,
    pub Ticket: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_14 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_14 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_ASYNC_RESULT_0_14 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_14 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_15 {
    pub Identity: WINBIO_IDENTITY,
    pub SubFactor: u8,
    pub RejectDetail: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_15 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_15 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_ASYNC_RESULT_0_15 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_15 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_16 {
    pub ChangeType: u32,
    pub PresenceCount: usize,
    pub PresenceArray: *mut WINBIO_PRESENCE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_16 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_16 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINBIO_ASYNC_RESULT_0_16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_ASYNC_RESULT_0_16").field("ChangeType", &self.ChangeType).field("PresenceCount", &self.PresenceCount).field("PresenceArray", &self.PresenceArray).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_ASYNC_RESULT_0_16 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_16 {
    fn eq(&self, other: &Self) -> bool {
        self.ChangeType == other.ChangeType && self.PresenceCount == other.PresenceCount && self.PresenceArray == other.PresenceArray
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT_0_16 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_16 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_17 {
    pub ExtendedStatus: WINBIO_EXTENDED_UNIT_STATUS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_17 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_17 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINBIO_ASYNC_RESULT_0_17 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_ASYNC_RESULT_0_17").field("ExtendedStatus", &self.ExtendedStatus).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_ASYNC_RESULT_0_17 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_17 {
    fn eq(&self, other: &Self) -> bool {
        self.ExtendedStatus == other.ExtendedStatus
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT_0_17 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_17 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_18 {
    pub PropertyType: u32,
    pub PropertyId: u32,
    pub Identity: WINBIO_IDENTITY,
    pub SubFactor: u8,
    pub PropertyBufferSize: usize,
    pub PropertyBuffer: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_18 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_18 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_ASYNC_RESULT_0_18 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_18 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_19 {
    pub Match: super::super::Foundation::BOOLEAN,
    pub RejectDetail: u32,
    pub Ticket: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_19 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_19 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINBIO_ASYNC_RESULT_0_19 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_ASYNC_RESULT_0_19").field("Match", &self.Match).field("RejectDetail", &self.RejectDetail).field("Ticket", &self.Ticket).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_ASYNC_RESULT_0_19 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_19 {
    fn eq(&self, other: &Self) -> bool {
        self.Match == other.Match && self.RejectDetail == other.RejectDetail && self.Ticket == other.Ticket
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT_0_19 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_19 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_20 {
    pub Match: super::super::Foundation::BOOLEAN,
    pub RejectDetail: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_20 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_20 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINBIO_ASYNC_RESULT_0_20 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_ASYNC_RESULT_0_20").field("Match", &self.Match).field("RejectDetail", &self.RejectDetail).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_ASYNC_RESULT_0_20 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_20 {
    fn eq(&self, other: &Self) -> bool {
        self.Match == other.Match && self.RejectDetail == other.RejectDetail
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT_0_20 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_20 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_BDB_ANSI_381_HEADER {
    pub RecordLength: u64,
    pub FormatIdentifier: u32,
    pub VersionNumber: u32,
    pub ProductId: WINBIO_REGISTERED_FORMAT,
    pub CaptureDeviceId: u16,
    pub ImageAcquisitionLevel: u16,
    pub HorizontalScanResolution: u16,
    pub VerticalScanResolution: u16,
    pub HorizontalImageResolution: u16,
    pub VerticalImageResolution: u16,
    pub ElementCount: u8,
    pub ScaleUnits: u8,
    pub PixelDepth: u8,
    pub ImageCompressionAlg: u8,
    pub Reserved: u16,
}
impl ::core::marker::Copy for WINBIO_BDB_ANSI_381_HEADER {}
impl ::core::clone::Clone for WINBIO_BDB_ANSI_381_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_BDB_ANSI_381_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_BDB_ANSI_381_HEADER")
            .field("RecordLength", &self.RecordLength)
            .field("FormatIdentifier", &self.FormatIdentifier)
            .field("VersionNumber", &self.VersionNumber)
            .field("ProductId", &self.ProductId)
            .field("CaptureDeviceId", &self.CaptureDeviceId)
            .field("ImageAcquisitionLevel", &self.ImageAcquisitionLevel)
            .field("HorizontalScanResolution", &self.HorizontalScanResolution)
            .field("VerticalScanResolution", &self.VerticalScanResolution)
            .field("HorizontalImageResolution", &self.HorizontalImageResolution)
            .field("VerticalImageResolution", &self.VerticalImageResolution)
            .field("ElementCount", &self.ElementCount)
            .field("ScaleUnits", &self.ScaleUnits)
            .field("PixelDepth", &self.PixelDepth)
            .field("ImageCompressionAlg", &self.ImageCompressionAlg)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_BDB_ANSI_381_HEADER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_BDB_ANSI_381_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.RecordLength == other.RecordLength && self.FormatIdentifier == other.FormatIdentifier && self.VersionNumber == other.VersionNumber && self.ProductId == other.ProductId && self.CaptureDeviceId == other.CaptureDeviceId && self.ImageAcquisitionLevel == other.ImageAcquisitionLevel && self.HorizontalScanResolution == other.HorizontalScanResolution && self.VerticalScanResolution == other.VerticalScanResolution && self.HorizontalImageResolution == other.HorizontalImageResolution && self.VerticalImageResolution == other.VerticalImageResolution && self.ElementCount == other.ElementCount && self.ScaleUnits == other.ScaleUnits && self.PixelDepth == other.PixelDepth && self.ImageCompressionAlg == other.ImageCompressionAlg && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for WINBIO_BDB_ANSI_381_HEADER {}
impl ::core::default::Default for WINBIO_BDB_ANSI_381_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_BDB_ANSI_381_RECORD {
    pub BlockLength: u32,
    pub HorizontalLineLength: u16,
    pub VerticalLineLength: u16,
    pub Position: u8,
    pub CountOfViews: u8,
    pub ViewNumber: u8,
    pub ImageQuality: u8,
    pub ImpressionType: u8,
    pub Reserved: u8,
}
impl ::core::marker::Copy for WINBIO_BDB_ANSI_381_RECORD {}
impl ::core::clone::Clone for WINBIO_BDB_ANSI_381_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_BDB_ANSI_381_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_BDB_ANSI_381_RECORD").field("BlockLength", &self.BlockLength).field("HorizontalLineLength", &self.HorizontalLineLength).field("VerticalLineLength", &self.VerticalLineLength).field("Position", &self.Position).field("CountOfViews", &self.CountOfViews).field("ViewNumber", &self.ViewNumber).field("ImageQuality", &self.ImageQuality).field("ImpressionType", &self.ImpressionType).field("Reserved", &self.Reserved).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_BDB_ANSI_381_RECORD {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_BDB_ANSI_381_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.BlockLength == other.BlockLength && self.HorizontalLineLength == other.HorizontalLineLength && self.VerticalLineLength == other.VerticalLineLength && self.Position == other.Position && self.CountOfViews == other.CountOfViews && self.ViewNumber == other.ViewNumber && self.ImageQuality == other.ImageQuality && self.ImpressionType == other.ImpressionType && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for WINBIO_BDB_ANSI_381_RECORD {}
impl ::core::default::Default for WINBIO_BDB_ANSI_381_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_BIR {
    pub HeaderBlock: WINBIO_BIR_DATA,
    pub StandardDataBlock: WINBIO_BIR_DATA,
    pub VendorDataBlock: WINBIO_BIR_DATA,
    pub SignatureBlock: WINBIO_BIR_DATA,
}
impl ::core::marker::Copy for WINBIO_BIR {}
impl ::core::clone::Clone for WINBIO_BIR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_BIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_BIR").field("HeaderBlock", &self.HeaderBlock).field("StandardDataBlock", &self.StandardDataBlock).field("VendorDataBlock", &self.VendorDataBlock).field("SignatureBlock", &self.SignatureBlock).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_BIR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_BIR {
    fn eq(&self, other: &Self) -> bool {
        self.HeaderBlock == other.HeaderBlock && self.StandardDataBlock == other.StandardDataBlock && self.VendorDataBlock == other.VendorDataBlock && self.SignatureBlock == other.SignatureBlock
    }
}
impl ::core::cmp::Eq for WINBIO_BIR {}
impl ::core::default::Default for WINBIO_BIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_BIR_DATA {
    pub Size: u32,
    pub Offset: u32,
}
impl ::core::marker::Copy for WINBIO_BIR_DATA {}
impl ::core::clone::Clone for WINBIO_BIR_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_BIR_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_BIR_DATA").field("Size", &self.Size).field("Offset", &self.Offset).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_BIR_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_BIR_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Offset == other.Offset
    }
}
impl ::core::cmp::Eq for WINBIO_BIR_DATA {}
impl ::core::default::Default for WINBIO_BIR_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_BIR_HEADER {
    pub ValidFields: u16,
    pub HeaderVersion: u8,
    pub PatronHeaderVersion: u8,
    pub DataFlags: u8,
    pub Type: u32,
    pub Subtype: u8,
    pub Purpose: u8,
    pub DataQuality: i8,
    pub CreationDate: i64,
    pub ValidityPeriod: WINBIO_BIR_HEADER_0,
    pub BiometricDataFormat: WINBIO_REGISTERED_FORMAT,
    pub ProductId: WINBIO_REGISTERED_FORMAT,
}
impl ::core::marker::Copy for WINBIO_BIR_HEADER {}
impl ::core::clone::Clone for WINBIO_BIR_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_BIR_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_BIR_HEADER")
            .field("ValidFields", &self.ValidFields)
            .field("HeaderVersion", &self.HeaderVersion)
            .field("PatronHeaderVersion", &self.PatronHeaderVersion)
            .field("DataFlags", &self.DataFlags)
            .field("Type", &self.Type)
            .field("Subtype", &self.Subtype)
            .field("Purpose", &self.Purpose)
            .field("DataQuality", &self.DataQuality)
            .field("CreationDate", &self.CreationDate)
            .field("ValidityPeriod", &self.ValidityPeriod)
            .field("BiometricDataFormat", &self.BiometricDataFormat)
            .field("ProductId", &self.ProductId)
            .finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_BIR_HEADER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_BIR_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.ValidFields == other.ValidFields && self.HeaderVersion == other.HeaderVersion && self.PatronHeaderVersion == other.PatronHeaderVersion && self.DataFlags == other.DataFlags && self.Type == other.Type && self.Subtype == other.Subtype && self.Purpose == other.Purpose && self.DataQuality == other.DataQuality && self.CreationDate == other.CreationDate && self.ValidityPeriod == other.ValidityPeriod && self.BiometricDataFormat == other.BiometricDataFormat && self.ProductId == other.ProductId
    }
}
impl ::core::cmp::Eq for WINBIO_BIR_HEADER {}
impl ::core::default::Default for WINBIO_BIR_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_BIR_HEADER_0 {
    pub BeginDate: i64,
    pub EndDate: i64,
}
impl ::core::marker::Copy for WINBIO_BIR_HEADER_0 {}
impl ::core::clone::Clone for WINBIO_BIR_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_BIR_HEADER_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_BIR_HEADER_0").field("BeginDate", &self.BeginDate).field("EndDate", &self.EndDate).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_BIR_HEADER_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_BIR_HEADER_0 {
    fn eq(&self, other: &Self) -> bool {
        self.BeginDate == other.BeginDate && self.EndDate == other.EndDate
    }
}
impl ::core::cmp::Eq for WINBIO_BIR_HEADER_0 {}
impl ::core::default::Default for WINBIO_BIR_HEADER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_BLANK_PAYLOAD {
    pub PayloadSize: u32,
    pub WinBioHresult: ::windows::core::HRESULT,
}
impl ::core::marker::Copy for WINBIO_BLANK_PAYLOAD {}
impl ::core::clone::Clone for WINBIO_BLANK_PAYLOAD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_BLANK_PAYLOAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_BLANK_PAYLOAD").field("PayloadSize", &self.PayloadSize).field("WinBioHresult", &self.WinBioHresult).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_BLANK_PAYLOAD {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_BLANK_PAYLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.WinBioHresult == other.WinBioHresult
    }
}
impl ::core::cmp::Eq for WINBIO_BLANK_PAYLOAD {}
impl ::core::default::Default for WINBIO_BLANK_PAYLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_BSP_SCHEMA {
    pub BiometricFactor: u32,
    pub BspId: ::windows::core::GUID,
    pub Description: [u16; 256],
    pub Vendor: [u16; 256],
    pub Version: WINBIO_VERSION,
}
impl ::core::marker::Copy for WINBIO_BSP_SCHEMA {}
impl ::core::clone::Clone for WINBIO_BSP_SCHEMA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_BSP_SCHEMA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_BSP_SCHEMA").field("BiometricFactor", &self.BiometricFactor).field("BspId", &self.BspId).field("Description", &self.Description).field("Vendor", &self.Vendor).field("Version", &self.Version).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_BSP_SCHEMA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_BSP_SCHEMA {
    fn eq(&self, other: &Self) -> bool {
        self.BiometricFactor == other.BiometricFactor && self.BspId == other.BspId && self.Description == other.Description && self.Vendor == other.Vendor && self.Version == other.Version
    }
}
impl ::core::cmp::Eq for WINBIO_BSP_SCHEMA {}
impl ::core::default::Default for WINBIO_BSP_SCHEMA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_CALIBRATION_INFO {
    pub PayloadSize: u32,
    pub WinBioHresult: ::windows::core::HRESULT,
    pub CalibrationData: WINBIO_DATA,
}
impl ::core::marker::Copy for WINBIO_CALIBRATION_INFO {}
impl ::core::clone::Clone for WINBIO_CALIBRATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_CALIBRATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_CALIBRATION_INFO").field("PayloadSize", &self.PayloadSize).field("WinBioHresult", &self.WinBioHresult).field("CalibrationData", &self.CalibrationData).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_CALIBRATION_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_CALIBRATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.WinBioHresult == other.WinBioHresult && self.CalibrationData == other.CalibrationData
    }
}
impl ::core::cmp::Eq for WINBIO_CALIBRATION_INFO {}
impl ::core::default::Default for WINBIO_CALIBRATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_CAPTURE_DATA {
    pub PayloadSize: u32,
    pub WinBioHresult: ::windows::core::HRESULT,
    pub SensorStatus: u32,
    pub RejectDetail: u32,
    pub CaptureData: WINBIO_DATA,
}
impl ::core::marker::Copy for WINBIO_CAPTURE_DATA {}
impl ::core::clone::Clone for WINBIO_CAPTURE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_CAPTURE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_CAPTURE_DATA").field("PayloadSize", &self.PayloadSize).field("WinBioHresult", &self.WinBioHresult).field("SensorStatus", &self.SensorStatus).field("RejectDetail", &self.RejectDetail).field("CaptureData", &self.CaptureData).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_CAPTURE_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_CAPTURE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.WinBioHresult == other.WinBioHresult && self.SensorStatus == other.SensorStatus && self.RejectDetail == other.RejectDetail && self.CaptureData == other.CaptureData
    }
}
impl ::core::cmp::Eq for WINBIO_CAPTURE_DATA {}
impl ::core::default::Default for WINBIO_CAPTURE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_CAPTURE_PARAMETERS {
    pub PayloadSize: u32,
    pub Purpose: u8,
    pub Format: WINBIO_REGISTERED_FORMAT,
    pub VendorFormat: ::windows::core::GUID,
    pub Flags: u8,
}
impl ::core::marker::Copy for WINBIO_CAPTURE_PARAMETERS {}
impl ::core::clone::Clone for WINBIO_CAPTURE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_CAPTURE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_CAPTURE_PARAMETERS").field("PayloadSize", &self.PayloadSize).field("Purpose", &self.Purpose).field("Format", &self.Format).field("VendorFormat", &self.VendorFormat).field("Flags", &self.Flags).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_CAPTURE_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_CAPTURE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.Purpose == other.Purpose && self.Format == other.Format && self.VendorFormat == other.VendorFormat && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for WINBIO_CAPTURE_PARAMETERS {}
impl ::core::default::Default for WINBIO_CAPTURE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_DATA {
    pub Size: u32,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for WINBIO_DATA {}
impl ::core::clone::Clone for WINBIO_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_DATA").field("Size", &self.Size).field("Data", &self.Data).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for WINBIO_DATA {}
impl ::core::default::Default for WINBIO_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_DIAGNOSTICS {
    pub PayloadSize: u32,
    pub WinBioHresult: ::windows::core::HRESULT,
    pub SensorStatus: u32,
    pub VendorDiagnostics: WINBIO_DATA,
}
impl ::core::marker::Copy for WINBIO_DIAGNOSTICS {}
impl ::core::clone::Clone for WINBIO_DIAGNOSTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_DIAGNOSTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_DIAGNOSTICS").field("PayloadSize", &self.PayloadSize).field("WinBioHresult", &self.WinBioHresult).field("SensorStatus", &self.SensorStatus).field("VendorDiagnostics", &self.VendorDiagnostics).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_DIAGNOSTICS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_DIAGNOSTICS {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.WinBioHresult == other.WinBioHresult && self.SensorStatus == other.SensorStatus && self.VendorDiagnostics == other.VendorDiagnostics
    }
}
impl ::core::cmp::Eq for WINBIO_DIAGNOSTICS {}
impl ::core::default::Default for WINBIO_DIAGNOSTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_ENCRYPTED_CAPTURE_PARAMS {
    pub PayloadSize: u32,
    pub Purpose: u8,
    pub Format: WINBIO_REGISTERED_FORMAT,
    pub VendorFormat: ::windows::core::GUID,
    pub Flags: u8,
    pub NonceSize: u32,
}
impl ::core::marker::Copy for WINBIO_ENCRYPTED_CAPTURE_PARAMS {}
impl ::core::clone::Clone for WINBIO_ENCRYPTED_CAPTURE_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_ENCRYPTED_CAPTURE_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_ENCRYPTED_CAPTURE_PARAMS").field("PayloadSize", &self.PayloadSize).field("Purpose", &self.Purpose).field("Format", &self.Format).field("VendorFormat", &self.VendorFormat).field("Flags", &self.Flags).field("NonceSize", &self.NonceSize).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_ENCRYPTED_CAPTURE_PARAMS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_ENCRYPTED_CAPTURE_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.Purpose == other.Purpose && self.Format == other.Format && self.VendorFormat == other.VendorFormat && self.Flags == other.Flags && self.NonceSize == other.NonceSize
    }
}
impl ::core::cmp::Eq for WINBIO_ENCRYPTED_CAPTURE_PARAMS {}
impl ::core::default::Default for WINBIO_ENCRYPTED_CAPTURE_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub struct WINBIO_ENGINE_INTERFACE {
    pub Version: WINBIO_ADAPTER_INTERFACE_VERSION,
    pub Type: u32,
    pub Size: usize,
    pub AdapterId: ::windows::core::GUID,
    pub Attach: PIBIO_ENGINE_ATTACH_FN,
    pub Detach: PIBIO_ENGINE_DETACH_FN,
    pub ClearContext: PIBIO_ENGINE_CLEAR_CONTEXT_FN,
    pub QueryPreferredFormat: PIBIO_ENGINE_QUERY_PREFERRED_FORMAT_FN,
    pub QueryIndexVectorSize: PIBIO_ENGINE_QUERY_INDEX_VECTOR_SIZE_FN,
    pub QueryHashAlgorithms: PIBIO_ENGINE_QUERY_HASH_ALGORITHMS_FN,
    pub SetHashAlgorithm: PIBIO_ENGINE_SET_HASH_ALGORITHM_FN,
    pub QuerySampleHint: PIBIO_ENGINE_QUERY_SAMPLE_HINT_FN,
    pub AcceptSampleData: PIBIO_ENGINE_ACCEPT_SAMPLE_DATA_FN,
    pub ExportEngineData: PIBIO_ENGINE_EXPORT_ENGINE_DATA_FN,
    pub VerifyFeatureSet: PIBIO_ENGINE_VERIFY_FEATURE_SET_FN,
    pub IdentifyFeatureSet: PIBIO_ENGINE_IDENTIFY_FEATURE_SET_FN,
    pub CreateEnrollment: PIBIO_ENGINE_CREATE_ENROLLMENT_FN,
    pub UpdateEnrollment: PIBIO_ENGINE_UPDATE_ENROLLMENT_FN,
    pub GetEnrollmentStatus: PIBIO_ENGINE_GET_ENROLLMENT_STATUS_FN,
    pub GetEnrollmentHash: PIBIO_ENGINE_GET_ENROLLMENT_HASH_FN,
    pub CheckForDuplicate: PIBIO_ENGINE_CHECK_FOR_DUPLICATE_FN,
    pub CommitEnrollment: PIBIO_ENGINE_COMMIT_ENROLLMENT_FN,
    pub DiscardEnrollment: PIBIO_ENGINE_DISCARD_ENROLLMENT_FN,
    pub ControlUnit: PIBIO_ENGINE_CONTROL_UNIT_FN,
    pub ControlUnitPrivileged: PIBIO_ENGINE_CONTROL_UNIT_PRIVILEGED_FN,
    pub NotifyPowerChange: PIBIO_ENGINE_NOTIFY_POWER_CHANGE_FN,
    pub Reserved_1: PIBIO_ENGINE_RESERVED_1_FN,
    pub PipelineInit: PIBIO_ENGINE_PIPELINE_INIT_FN,
    pub PipelineCleanup: PIBIO_ENGINE_PIPELINE_CLEANUP_FN,
    pub Activate: PIBIO_ENGINE_ACTIVATE_FN,
    pub Deactivate: PIBIO_ENGINE_DEACTIVATE_FN,
    pub QueryExtendedInfo: PIBIO_ENGINE_QUERY_EXTENDED_INFO_FN,
    pub IdentifyAll: PIBIO_ENGINE_IDENTIFY_ALL_FN,
    pub SetEnrollmentSelector: PIBIO_ENGINE_SET_ENROLLMENT_SELECTOR_FN,
    pub SetEnrollmentParameters: PIBIO_ENGINE_SET_ENROLLMENT_PARAMETERS_FN,
    pub QueryExtendedEnrollmentStatus: PIBIO_ENGINE_QUERY_EXTENDED_ENROLLMENT_STATUS_FN,
    pub RefreshCache: PIBIO_ENGINE_REFRESH_CACHE_FN,
    pub SelectCalibrationFormat: PIBIO_ENGINE_SELECT_CALIBRATION_FORMAT_FN,
    pub QueryCalibrationData: PIBIO_ENGINE_QUERY_CALIBRATION_DATA_FN,
    pub SetAccountPolicy: PIBIO_ENGINE_SET_ACCOUNT_POLICY_FN,
    pub CreateKey: PIBIO_ENGINE_CREATE_KEY_FN,
    pub IdentifyFeatureSetSecure: PIBIO_ENGINE_IDENTIFY_FEATURE_SET_SECURE_FN,
    pub AcceptPrivateSensorTypeInfo: PIBIO_ENGINE_ACCEPT_PRIVATE_SENSOR_TYPE_INFO_FN,
    pub CreateEnrollmentAuthenticated: PIBIO_ENGINE_CREATE_ENROLLMENT_AUTHENTICATED_FN,
    pub IdentifyFeatureSetAuthenticated: PIBIO_ENGINE_IDENTIFY_FEATURE_SET_AUTHENTICATED_FN,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::marker::Copy for WINBIO_ENGINE_INTERFACE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::clone::Clone for WINBIO_ENGINE_INTERFACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::fmt::Debug for WINBIO_ENGINE_INTERFACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_ENGINE_INTERFACE").field("Version", &self.Version).field("Type", &self.Type).field("Size", &self.Size).field("AdapterId", &self.AdapterId).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::windows::core::TypeKind for WINBIO_ENGINE_INTERFACE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::default::Default for WINBIO_ENGINE_INTERFACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_EVENT {
    pub Type: u32,
    pub Parameters: WINBIO_EVENT_0,
}
impl ::core::marker::Copy for WINBIO_EVENT {}
impl ::core::clone::Clone for WINBIO_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WINBIO_EVENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WINBIO_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub union WINBIO_EVENT_0 {
    pub Unclaimed: WINBIO_EVENT_0_2,
    pub UnclaimedIdentify: WINBIO_EVENT_0_1,
    pub Error: WINBIO_EVENT_0_0,
}
impl ::core::marker::Copy for WINBIO_EVENT_0 {}
impl ::core::clone::Clone for WINBIO_EVENT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WINBIO_EVENT_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WINBIO_EVENT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_EVENT_0_0 {
    pub ErrorCode: ::windows::core::HRESULT,
}
impl ::core::marker::Copy for WINBIO_EVENT_0_0 {}
impl ::core::clone::Clone for WINBIO_EVENT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_EVENT_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_EVENT_0_0").field("ErrorCode", &self.ErrorCode).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_EVENT_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_EVENT_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.ErrorCode == other.ErrorCode
    }
}
impl ::core::cmp::Eq for WINBIO_EVENT_0_0 {}
impl ::core::default::Default for WINBIO_EVENT_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_EVENT_0_1 {
    pub UnitId: u32,
    pub Identity: WINBIO_IDENTITY,
    pub SubFactor: u8,
    pub RejectDetail: u32,
}
impl ::core::marker::Copy for WINBIO_EVENT_0_1 {}
impl ::core::clone::Clone for WINBIO_EVENT_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WINBIO_EVENT_0_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WINBIO_EVENT_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_EVENT_0_2 {
    pub UnitId: u32,
    pub RejectDetail: u32,
}
impl ::core::marker::Copy for WINBIO_EVENT_0_2 {}
impl ::core::clone::Clone for WINBIO_EVENT_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_EVENT_0_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_EVENT_0_2").field("UnitId", &self.UnitId).field("RejectDetail", &self.RejectDetail).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_EVENT_0_2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_EVENT_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.UnitId == other.UnitId && self.RejectDetail == other.RejectDetail
    }
}
impl ::core::cmp::Eq for WINBIO_EVENT_0_2 {}
impl ::core::default::Default for WINBIO_EVENT_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_EXTENDED_ENGINE_INFO {
    pub GenericEngineCapabilities: u32,
    pub Factor: u32,
    pub Specific: WINBIO_EXTENDED_ENGINE_INFO_0,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_ENGINE_INFO {}
impl ::core::clone::Clone for WINBIO_EXTENDED_ENGINE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WINBIO_EXTENDED_ENGINE_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WINBIO_EXTENDED_ENGINE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub union WINBIO_EXTENDED_ENGINE_INFO_0 {
    pub Null: u32,
    pub FacialFeatures: WINBIO_EXTENDED_ENGINE_INFO_0_0,
    pub Fingerprint: WINBIO_EXTENDED_ENGINE_INFO_0_1,
    pub Iris: WINBIO_EXTENDED_ENGINE_INFO_0_2,
    pub Voice: WINBIO_EXTENDED_ENGINE_INFO_0_3,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_ENGINE_INFO_0 {}
impl ::core::clone::Clone for WINBIO_EXTENDED_ENGINE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WINBIO_EXTENDED_ENGINE_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WINBIO_EXTENDED_ENGINE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_0 {
    pub Capabilities: u32,
    pub EnrollmentRequirements: WINBIO_EXTENDED_ENGINE_INFO_0_0_0,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_ENGINE_INFO_0_0 {}
impl ::core::clone::Clone for WINBIO_EXTENDED_ENGINE_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_EXTENDED_ENGINE_INFO_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_EXTENDED_ENGINE_INFO_0_0").field("Capabilities", &self.Capabilities).field("EnrollmentRequirements", &self.EnrollmentRequirements).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_EXTENDED_ENGINE_INFO_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENGINE_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Capabilities == other.Capabilities && self.EnrollmentRequirements == other.EnrollmentRequirements
    }
}
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENGINE_INFO_0_0 {}
impl ::core::default::Default for WINBIO_EXTENDED_ENGINE_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_0_0 {
    pub Null: u32,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_ENGINE_INFO_0_0_0 {}
impl ::core::clone::Clone for WINBIO_EXTENDED_ENGINE_INFO_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_EXTENDED_ENGINE_INFO_0_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_EXTENDED_ENGINE_INFO_0_0_0").field("Null", &self.Null).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_EXTENDED_ENGINE_INFO_0_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENGINE_INFO_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Null == other.Null
    }
}
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENGINE_INFO_0_0_0 {}
impl ::core::default::Default for WINBIO_EXTENDED_ENGINE_INFO_0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_1 {
    pub Capabilities: u32,
    pub EnrollmentRequirements: WINBIO_EXTENDED_ENGINE_INFO_0_1_0,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_ENGINE_INFO_0_1 {}
impl ::core::clone::Clone for WINBIO_EXTENDED_ENGINE_INFO_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_EXTENDED_ENGINE_INFO_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_EXTENDED_ENGINE_INFO_0_1").field("Capabilities", &self.Capabilities).field("EnrollmentRequirements", &self.EnrollmentRequirements).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_EXTENDED_ENGINE_INFO_0_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENGINE_INFO_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Capabilities == other.Capabilities && self.EnrollmentRequirements == other.EnrollmentRequirements
    }
}
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENGINE_INFO_0_1 {}
impl ::core::default::Default for WINBIO_EXTENDED_ENGINE_INFO_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_1_0 {
    pub GeneralSamples: u32,
    pub Center: u32,
    pub TopEdge: u32,
    pub BottomEdge: u32,
    pub LeftEdge: u32,
    pub RightEdge: u32,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_ENGINE_INFO_0_1_0 {}
impl ::core::clone::Clone for WINBIO_EXTENDED_ENGINE_INFO_0_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_EXTENDED_ENGINE_INFO_0_1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_EXTENDED_ENGINE_INFO_0_1_0").field("GeneralSamples", &self.GeneralSamples).field("Center", &self.Center).field("TopEdge", &self.TopEdge).field("BottomEdge", &self.BottomEdge).field("LeftEdge", &self.LeftEdge).field("RightEdge", &self.RightEdge).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_EXTENDED_ENGINE_INFO_0_1_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENGINE_INFO_0_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.GeneralSamples == other.GeneralSamples && self.Center == other.Center && self.TopEdge == other.TopEdge && self.BottomEdge == other.BottomEdge && self.LeftEdge == other.LeftEdge && self.RightEdge == other.RightEdge
    }
}
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENGINE_INFO_0_1_0 {}
impl ::core::default::Default for WINBIO_EXTENDED_ENGINE_INFO_0_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_2 {
    pub Capabilities: u32,
    pub EnrollmentRequirements: WINBIO_EXTENDED_ENGINE_INFO_0_2_0,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_ENGINE_INFO_0_2 {}
impl ::core::clone::Clone for WINBIO_EXTENDED_ENGINE_INFO_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_EXTENDED_ENGINE_INFO_0_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_EXTENDED_ENGINE_INFO_0_2").field("Capabilities", &self.Capabilities).field("EnrollmentRequirements", &self.EnrollmentRequirements).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_EXTENDED_ENGINE_INFO_0_2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENGINE_INFO_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.Capabilities == other.Capabilities && self.EnrollmentRequirements == other.EnrollmentRequirements
    }
}
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENGINE_INFO_0_2 {}
impl ::core::default::Default for WINBIO_EXTENDED_ENGINE_INFO_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_2_0 {
    pub Null: u32,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_ENGINE_INFO_0_2_0 {}
impl ::core::clone::Clone for WINBIO_EXTENDED_ENGINE_INFO_0_2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_EXTENDED_ENGINE_INFO_0_2_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_EXTENDED_ENGINE_INFO_0_2_0").field("Null", &self.Null).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_EXTENDED_ENGINE_INFO_0_2_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENGINE_INFO_0_2_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Null == other.Null
    }
}
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENGINE_INFO_0_2_0 {}
impl ::core::default::Default for WINBIO_EXTENDED_ENGINE_INFO_0_2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_3 {
    pub Capabilities: u32,
    pub EnrollmentRequirements: WINBIO_EXTENDED_ENGINE_INFO_0_3_0,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_ENGINE_INFO_0_3 {}
impl ::core::clone::Clone for WINBIO_EXTENDED_ENGINE_INFO_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_EXTENDED_ENGINE_INFO_0_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_EXTENDED_ENGINE_INFO_0_3").field("Capabilities", &self.Capabilities).field("EnrollmentRequirements", &self.EnrollmentRequirements).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_EXTENDED_ENGINE_INFO_0_3 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENGINE_INFO_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.Capabilities == other.Capabilities && self.EnrollmentRequirements == other.EnrollmentRequirements
    }
}
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENGINE_INFO_0_3 {}
impl ::core::default::Default for WINBIO_EXTENDED_ENGINE_INFO_0_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_3_0 {
    pub Null: u32,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_ENGINE_INFO_0_3_0 {}
impl ::core::clone::Clone for WINBIO_EXTENDED_ENGINE_INFO_0_3_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_EXTENDED_ENGINE_INFO_0_3_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_EXTENDED_ENGINE_INFO_0_3_0").field("Null", &self.Null).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_EXTENDED_ENGINE_INFO_0_3_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENGINE_INFO_0_3_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Null == other.Null
    }
}
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENGINE_INFO_0_3_0 {}
impl ::core::default::Default for WINBIO_EXTENDED_ENGINE_INFO_0_3_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_EXTENDED_ENROLLMENT_PARAMETERS {
    pub Size: usize,
    pub SubFactor: u8,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_ENROLLMENT_PARAMETERS {}
impl ::core::clone::Clone for WINBIO_EXTENDED_ENROLLMENT_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_EXTENDED_ENROLLMENT_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_EXTENDED_ENROLLMENT_PARAMETERS").field("Size", &self.Size).field("SubFactor", &self.SubFactor).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_EXTENDED_ENROLLMENT_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENROLLMENT_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.SubFactor == other.SubFactor
    }
}
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENROLLMENT_PARAMETERS {}
impl ::core::default::Default for WINBIO_EXTENDED_ENROLLMENT_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS {
    pub TemplateStatus: ::windows::core::HRESULT,
    pub RejectDetail: u32,
    pub PercentComplete: u32,
    pub Factor: u32,
    pub SubFactor: u8,
    pub Specific: WINBIO_EXTENDED_ENROLLMENT_STATUS_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_EXTENDED_ENROLLMENT_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_EXTENDED_ENROLLMENT_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_EXTENDED_ENROLLMENT_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_EXTENDED_ENROLLMENT_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union WINBIO_EXTENDED_ENROLLMENT_STATUS_0 {
    pub Null: u32,
    pub FacialFeatures: WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0,
    pub Fingerprint: WINBIO_EXTENDED_ENROLLMENT_STATUS_0_1,
    pub Iris: WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2,
    pub Voice: WINBIO_EXTENDED_ENROLLMENT_STATUS_0_3,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_EXTENDED_ENROLLMENT_STATUS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_EXTENDED_ENROLLMENT_STATUS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_EXTENDED_ENROLLMENT_STATUS_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_EXTENDED_ENROLLMENT_STATUS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0 {
    pub BoundingBox: super::super::Foundation::RECT,
    pub Distance: i32,
    pub OpaqueEngineData: WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0").field("BoundingBox", &self.BoundingBox).field("Distance", &self.Distance).field("OpaqueEngineData", &self.OpaqueEngineData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.BoundingBox == other.BoundingBox && self.Distance == other.Distance && self.OpaqueEngineData == other.OpaqueEngineData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0_0 {
    pub AdapterId: ::windows::core::GUID,
    pub Data: [u32; 78],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0_0").field("AdapterId", &self.AdapterId).field("Data", &self.Data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.AdapterId == other.AdapterId && self.Data == other.Data
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS_0_1 {
    pub GeneralSamples: u32,
    pub Center: u32,
    pub TopEdge: u32,
    pub BottomEdge: u32,
    pub LeftEdge: u32,
    pub RightEdge: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_EXTENDED_ENROLLMENT_STATUS_0_1").field("GeneralSamples", &self.GeneralSamples).field("Center", &self.Center).field("TopEdge", &self.TopEdge).field("BottomEdge", &self.BottomEdge).field("LeftEdge", &self.LeftEdge).field("RightEdge", &self.RightEdge).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_1 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.GeneralSamples == other.GeneralSamples && self.Center == other.Center && self.TopEdge == other.TopEdge && self.BottomEdge == other.BottomEdge && self.LeftEdge == other.LeftEdge && self.RightEdge == other.RightEdge
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2 {
    pub EyeBoundingBox_1: super::super::Foundation::RECT,
    pub EyeBoundingBox_2: super::super::Foundation::RECT,
    pub PupilCenter_1: super::super::Foundation::POINT,
    pub PupilCenter_2: super::super::Foundation::POINT,
    pub Distance: i32,
    pub GridPointCompletionPercent: u32,
    pub GridPointIndex: u16,
    pub Point3D: WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2_0,
    pub StopCaptureAndShowCriticalFeedback: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2")
            .field("EyeBoundingBox_1", &self.EyeBoundingBox_1)
            .field("EyeBoundingBox_2", &self.EyeBoundingBox_2)
            .field("PupilCenter_1", &self.PupilCenter_1)
            .field("PupilCenter_2", &self.PupilCenter_2)
            .field("Distance", &self.Distance)
            .field("GridPointCompletionPercent", &self.GridPointCompletionPercent)
            .field("GridPointIndex", &self.GridPointIndex)
            .field("Point3D", &self.Point3D)
            .field("StopCaptureAndShowCriticalFeedback", &self.StopCaptureAndShowCriticalFeedback)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.EyeBoundingBox_1 == other.EyeBoundingBox_1 && self.EyeBoundingBox_2 == other.EyeBoundingBox_2 && self.PupilCenter_1 == other.PupilCenter_1 && self.PupilCenter_2 == other.PupilCenter_2 && self.Distance == other.Distance && self.GridPointCompletionPercent == other.GridPointCompletionPercent && self.GridPointIndex == other.GridPointIndex && self.Point3D == other.Point3D && self.StopCaptureAndShowCriticalFeedback == other.StopCaptureAndShowCriticalFeedback
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2_0 {
    pub X: f64,
    pub Y: f64,
    pub Z: f64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2_0").field("X", &self.X).field("Y", &self.Y).field("Z", &self.Z).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2_0 {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Z == other.Z
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS_0_3 {
    pub Reserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_EXTENDED_ENROLLMENT_STATUS_0_3").field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_3 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_EXTENDED_SENSOR_INFO {
    pub GenericSensorCapabilities: u32,
    pub Factor: u32,
    pub Specific: WINBIO_EXTENDED_SENSOR_INFO_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_EXTENDED_SENSOR_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_EXTENDED_SENSOR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_EXTENDED_SENSOR_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_EXTENDED_SENSOR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union WINBIO_EXTENDED_SENSOR_INFO_0 {
    pub Null: u32,
    pub FacialFeatures: WINBIO_EXTENDED_SENSOR_INFO_0_0,
    pub Fingerprint: WINBIO_EXTENDED_SENSOR_INFO_0_1,
    pub Iris: WINBIO_EXTENDED_SENSOR_INFO_0_2,
    pub Voice: WINBIO_EXTENDED_SENSOR_INFO_0_3,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_EXTENDED_SENSOR_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_EXTENDED_SENSOR_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_EXTENDED_SENSOR_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_EXTENDED_SENSOR_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_EXTENDED_SENSOR_INFO_0_0 {
    pub FrameSize: super::super::Foundation::RECT,
    pub FrameOffset: super::super::Foundation::POINT,
    pub MandatoryOrientation: u32,
    pub HardwareInfo: WINBIO_EXTENDED_SENSOR_INFO_0_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_EXTENDED_SENSOR_INFO_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_EXTENDED_SENSOR_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINBIO_EXTENDED_SENSOR_INFO_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_EXTENDED_SENSOR_INFO_0_0").field("FrameSize", &self.FrameSize).field("FrameOffset", &self.FrameOffset).field("MandatoryOrientation", &self.MandatoryOrientation).field("HardwareInfo", &self.HardwareInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_EXTENDED_SENSOR_INFO_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_SENSOR_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.FrameSize == other.FrameSize && self.FrameOffset == other.FrameOffset && self.MandatoryOrientation == other.MandatoryOrientation && self.HardwareInfo == other.HardwareInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_EXTENDED_SENSOR_INFO_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_EXTENDED_SENSOR_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_EXTENDED_SENSOR_INFO_0_0_0 {
    pub ColorSensorId: [u16; 260],
    pub InfraredSensorId: [u16; 260],
    pub InfraredSensorRotationAngle: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_EXTENDED_SENSOR_INFO_0_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_EXTENDED_SENSOR_INFO_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINBIO_EXTENDED_SENSOR_INFO_0_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_EXTENDED_SENSOR_INFO_0_0_0").field("ColorSensorId", &self.ColorSensorId).field("InfraredSensorId", &self.InfraredSensorId).field("InfraredSensorRotationAngle", &self.InfraredSensorRotationAngle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_EXTENDED_SENSOR_INFO_0_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_SENSOR_INFO_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.ColorSensorId == other.ColorSensorId && self.InfraredSensorId == other.InfraredSensorId && self.InfraredSensorRotationAngle == other.InfraredSensorRotationAngle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_EXTENDED_SENSOR_INFO_0_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_EXTENDED_SENSOR_INFO_0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_EXTENDED_SENSOR_INFO_0_1 {
    pub Reserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_EXTENDED_SENSOR_INFO_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_EXTENDED_SENSOR_INFO_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINBIO_EXTENDED_SENSOR_INFO_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_EXTENDED_SENSOR_INFO_0_1").field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_EXTENDED_SENSOR_INFO_0_1 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_SENSOR_INFO_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_EXTENDED_SENSOR_INFO_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_EXTENDED_SENSOR_INFO_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_EXTENDED_SENSOR_INFO_0_2 {
    pub FrameSize: super::super::Foundation::RECT,
    pub FrameOffset: super::super::Foundation::POINT,
    pub MandatoryOrientation: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_EXTENDED_SENSOR_INFO_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_EXTENDED_SENSOR_INFO_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINBIO_EXTENDED_SENSOR_INFO_0_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_EXTENDED_SENSOR_INFO_0_2").field("FrameSize", &self.FrameSize).field("FrameOffset", &self.FrameOffset).field("MandatoryOrientation", &self.MandatoryOrientation).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_EXTENDED_SENSOR_INFO_0_2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_SENSOR_INFO_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.FrameSize == other.FrameSize && self.FrameOffset == other.FrameOffset && self.MandatoryOrientation == other.MandatoryOrientation
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_EXTENDED_SENSOR_INFO_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_EXTENDED_SENSOR_INFO_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_EXTENDED_SENSOR_INFO_0_3 {
    pub Reserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_EXTENDED_SENSOR_INFO_0_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_EXTENDED_SENSOR_INFO_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINBIO_EXTENDED_SENSOR_INFO_0_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_EXTENDED_SENSOR_INFO_0_3").field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_EXTENDED_SENSOR_INFO_0_3 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_SENSOR_INFO_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_EXTENDED_SENSOR_INFO_0_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_EXTENDED_SENSOR_INFO_0_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_EXTENDED_STORAGE_INFO {
    pub GenericStorageCapabilities: u32,
    pub Factor: u32,
    pub Specific: WINBIO_EXTENDED_STORAGE_INFO_0,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_STORAGE_INFO {}
impl ::core::clone::Clone for WINBIO_EXTENDED_STORAGE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WINBIO_EXTENDED_STORAGE_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WINBIO_EXTENDED_STORAGE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub union WINBIO_EXTENDED_STORAGE_INFO_0 {
    pub Null: u32,
    pub FacialFeatures: WINBIO_EXTENDED_STORAGE_INFO_0_0,
    pub Fingerprint: WINBIO_EXTENDED_STORAGE_INFO_0_1,
    pub Iris: WINBIO_EXTENDED_STORAGE_INFO_0_2,
    pub Voice: WINBIO_EXTENDED_STORAGE_INFO_0_3,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_STORAGE_INFO_0 {}
impl ::core::clone::Clone for WINBIO_EXTENDED_STORAGE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WINBIO_EXTENDED_STORAGE_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WINBIO_EXTENDED_STORAGE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_EXTENDED_STORAGE_INFO_0_0 {
    pub Capabilities: u32,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_STORAGE_INFO_0_0 {}
impl ::core::clone::Clone for WINBIO_EXTENDED_STORAGE_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_EXTENDED_STORAGE_INFO_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_EXTENDED_STORAGE_INFO_0_0").field("Capabilities", &self.Capabilities).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_EXTENDED_STORAGE_INFO_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_STORAGE_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Capabilities == other.Capabilities
    }
}
impl ::core::cmp::Eq for WINBIO_EXTENDED_STORAGE_INFO_0_0 {}
impl ::core::default::Default for WINBIO_EXTENDED_STORAGE_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_EXTENDED_STORAGE_INFO_0_1 {
    pub Capabilities: u32,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_STORAGE_INFO_0_1 {}
impl ::core::clone::Clone for WINBIO_EXTENDED_STORAGE_INFO_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_EXTENDED_STORAGE_INFO_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_EXTENDED_STORAGE_INFO_0_1").field("Capabilities", &self.Capabilities).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_EXTENDED_STORAGE_INFO_0_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_STORAGE_INFO_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Capabilities == other.Capabilities
    }
}
impl ::core::cmp::Eq for WINBIO_EXTENDED_STORAGE_INFO_0_1 {}
impl ::core::default::Default for WINBIO_EXTENDED_STORAGE_INFO_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_EXTENDED_STORAGE_INFO_0_2 {
    pub Capabilities: u32,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_STORAGE_INFO_0_2 {}
impl ::core::clone::Clone for WINBIO_EXTENDED_STORAGE_INFO_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_EXTENDED_STORAGE_INFO_0_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_EXTENDED_STORAGE_INFO_0_2").field("Capabilities", &self.Capabilities).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_EXTENDED_STORAGE_INFO_0_2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_STORAGE_INFO_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.Capabilities == other.Capabilities
    }
}
impl ::core::cmp::Eq for WINBIO_EXTENDED_STORAGE_INFO_0_2 {}
impl ::core::default::Default for WINBIO_EXTENDED_STORAGE_INFO_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_EXTENDED_STORAGE_INFO_0_3 {
    pub Capabilities: u32,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_STORAGE_INFO_0_3 {}
impl ::core::clone::Clone for WINBIO_EXTENDED_STORAGE_INFO_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_EXTENDED_STORAGE_INFO_0_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_EXTENDED_STORAGE_INFO_0_3").field("Capabilities", &self.Capabilities).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_EXTENDED_STORAGE_INFO_0_3 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_STORAGE_INFO_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.Capabilities == other.Capabilities
    }
}
impl ::core::cmp::Eq for WINBIO_EXTENDED_STORAGE_INFO_0_3 {}
impl ::core::default::Default for WINBIO_EXTENDED_STORAGE_INFO_0_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_EXTENDED_UNIT_STATUS {
    pub Availability: u32,
    pub ReasonCode: u32,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_UNIT_STATUS {}
impl ::core::clone::Clone for WINBIO_EXTENDED_UNIT_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_EXTENDED_UNIT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_EXTENDED_UNIT_STATUS").field("Availability", &self.Availability).field("ReasonCode", &self.ReasonCode).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_EXTENDED_UNIT_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_UNIT_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.Availability == other.Availability && self.ReasonCode == other.ReasonCode
    }
}
impl ::core::cmp::Eq for WINBIO_EXTENDED_UNIT_STATUS {}
impl ::core::default::Default for WINBIO_EXTENDED_UNIT_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_FP_BU_STATE {
    pub SensorAttached: super::super::Foundation::BOOL,
    pub CreationResult: ::windows::core::HRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_FP_BU_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_FP_BU_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINBIO_FP_BU_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_FP_BU_STATE").field("SensorAttached", &self.SensorAttached).field("CreationResult", &self.CreationResult).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_FP_BU_STATE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_FP_BU_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.SensorAttached == other.SensorAttached && self.CreationResult == other.CreationResult
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_FP_BU_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_FP_BU_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub struct WINBIO_FRAMEWORK_INTERFACE {
    pub Version: WINBIO_ADAPTER_INTERFACE_VERSION,
    pub Type: u32,
    pub Size: usize,
    pub AdapterId: ::windows::core::GUID,
    pub SetUnitStatus: PIBIO_FRAMEWORK_SET_UNIT_STATUS_FN,
    pub VsmStorageAttach: PIBIO_STORAGE_ATTACH_FN,
    pub VsmStorageDetach: PIBIO_STORAGE_DETACH_FN,
    pub VsmStorageClearContext: PIBIO_STORAGE_CLEAR_CONTEXT_FN,
    pub VsmStorageCreateDatabase: PIBIO_STORAGE_CREATE_DATABASE_FN,
    pub VsmStorageOpenDatabase: PIBIO_STORAGE_OPEN_DATABASE_FN,
    pub VsmStorageCloseDatabase: PIBIO_STORAGE_CLOSE_DATABASE_FN,
    pub VsmStorageDeleteRecord: PIBIO_STORAGE_DELETE_RECORD_FN,
    pub VsmStorageNotifyPowerChange: PIBIO_STORAGE_NOTIFY_POWER_CHANGE_FN,
    pub VsmStoragePipelineInit: PIBIO_STORAGE_PIPELINE_INIT_FN,
    pub VsmStoragePipelineCleanup: PIBIO_STORAGE_PIPELINE_CLEANUP_FN,
    pub VsmStorageActivate: PIBIO_STORAGE_ACTIVATE_FN,
    pub VsmStorageDeactivate: PIBIO_STORAGE_DEACTIVATE_FN,
    pub VsmStorageQueryExtendedInfo: PIBIO_STORAGE_QUERY_EXTENDED_INFO_FN,
    pub VsmStorageCacheClear: PIBIO_FRAMEWORK_VSM_CACHE_CLEAR_FN,
    pub VsmStorageCacheImportBegin: PIBIO_FRAMEWORK_VSM_CACHE_IMPORT_BEGIN_FN,
    pub VsmStorageCacheImportNext: PIBIO_FRAMEWORK_VSM_CACHE_IMPORT_NEXT_FN,
    pub VsmStorageCacheImportEnd: PIBIO_FRAMEWORK_VSM_CACHE_IMPORT_END_FN,
    pub VsmStorageCacheExportBegin: PIBIO_FRAMEWORK_VSM_CACHE_EXPORT_BEGIN_FN,
    pub VsmStorageCacheExportNext: PIBIO_FRAMEWORK_VSM_CACHE_EXPORT_NEXT_FN,
    pub VsmStorageCacheExportEnd: PIBIO_FRAMEWORK_VSM_CACHE_EXPORT_END_FN,
    pub VsmSensorAttach: PIBIO_SENSOR_ATTACH_FN,
    pub VsmSensorDetach: PIBIO_SENSOR_DETACH_FN,
    pub VsmSensorClearContext: PIBIO_SENSOR_CLEAR_CONTEXT_FN,
    pub VsmSensorPushDataToEngine: PIBIO_SENSOR_PUSH_DATA_TO_ENGINE_FN,
    pub VsmSensorNotifyPowerChange: PIBIO_SENSOR_NOTIFY_POWER_CHANGE_FN,
    pub VsmSensorPipelineInit: PIBIO_SENSOR_PIPELINE_INIT_FN,
    pub VsmSensorPipelineCleanup: PIBIO_SENSOR_PIPELINE_CLEANUP_FN,
    pub VsmSensorActivate: PIBIO_SENSOR_ACTIVATE_FN,
    pub VsmSensorDeactivate: PIBIO_SENSOR_DEACTIVATE_FN,
    pub VsmSensorAsyncImportRawBuffer: PIBIO_SENSOR_ASYNC_IMPORT_RAW_BUFFER_FN,
    pub VsmSensorAsyncImportSecureBuffer: PIBIO_SENSOR_ASYNC_IMPORT_SECURE_BUFFER_FN,
    pub Reserved1: PIBIO_FRAMEWORK_VSM_STORAGE_RESERVED_1_FN,
    pub Reserved2: PIBIO_FRAMEWORK_VSM_STORAGE_RESERVED_2_FN,
    pub Reserved3: PIBIO_FRAMEWORK_VSM_STORAGE_RESERVED_3_FN,
    pub Reserved4: PIBIO_STORAGE_RESERVED_1_FN,
    pub Reserved5: PIBIO_STORAGE_RESERVED_2_FN,
    pub AllocateMemory: PIBIO_FRAMEWORK_ALLOCATE_MEMORY_FN,
    pub FreeMemory: PIBIO_FRAMEWORK_FREE_MEMORY_FN,
    pub GetProperty: PIBIO_FRAMEWORK_GET_PROPERTY_FN,
    pub LockAndValidateSecureBuffer: PIBIO_FRAMEWORK_LOCK_AND_VALIDATE_SECURE_BUFFER_FN,
    pub ReleaseSecureBuffer: PIBIO_FRAMEWORK_RELEASE_SECURE_BUFFER_FN,
    pub QueryAuthorizedEnrollments: PIBIO_FRAMEWORK_VSM_QUERY_AUTHORIZED_ENROLLMENTS_FN,
    pub DecryptSample: PIBIO_FRAMEWORK_VSM_DECRYPT_SAMPLE_FN,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::marker::Copy for WINBIO_FRAMEWORK_INTERFACE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::clone::Clone for WINBIO_FRAMEWORK_INTERFACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::fmt::Debug for WINBIO_FRAMEWORK_INTERFACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_FRAMEWORK_INTERFACE").field("Version", &self.Version).field("Type", &self.Type).field("Size", &self.Size).field("AdapterId", &self.AdapterId).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::windows::core::TypeKind for WINBIO_FRAMEWORK_INTERFACE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::default::Default for WINBIO_FRAMEWORK_INTERFACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_GESTURE_METADATA {
    pub Size: usize,
    pub BiometricType: u32,
    pub MatchType: u32,
    pub ProtectionType: u32,
}
impl ::core::marker::Copy for WINBIO_GESTURE_METADATA {}
impl ::core::clone::Clone for WINBIO_GESTURE_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_GESTURE_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_GESTURE_METADATA").field("Size", &self.Size).field("BiometricType", &self.BiometricType).field("MatchType", &self.MatchType).field("ProtectionType", &self.ProtectionType).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_GESTURE_METADATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_GESTURE_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.BiometricType == other.BiometricType && self.MatchType == other.MatchType && self.ProtectionType == other.ProtectionType
    }
}
impl ::core::cmp::Eq for WINBIO_GESTURE_METADATA {}
impl ::core::default::Default for WINBIO_GESTURE_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_GET_INDICATOR {
    pub PayloadSize: u32,
    pub WinBioHresult: ::windows::core::HRESULT,
    pub IndicatorStatus: u32,
}
impl ::core::marker::Copy for WINBIO_GET_INDICATOR {}
impl ::core::clone::Clone for WINBIO_GET_INDICATOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_GET_INDICATOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_GET_INDICATOR").field("PayloadSize", &self.PayloadSize).field("WinBioHresult", &self.WinBioHresult).field("IndicatorStatus", &self.IndicatorStatus).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_GET_INDICATOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_GET_INDICATOR {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.WinBioHresult == other.WinBioHresult && self.IndicatorStatus == other.IndicatorStatus
    }
}
impl ::core::cmp::Eq for WINBIO_GET_INDICATOR {}
impl ::core::default::Default for WINBIO_GET_INDICATOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_IDENTITY {
    pub Type: u32,
    pub Value: WINBIO_IDENTITY_0,
}
impl ::core::marker::Copy for WINBIO_IDENTITY {}
impl ::core::clone::Clone for WINBIO_IDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WINBIO_IDENTITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WINBIO_IDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub union WINBIO_IDENTITY_0 {
    pub Null: u32,
    pub Wildcard: u32,
    pub TemplateGuid: ::windows::core::GUID,
    pub AccountSid: WINBIO_IDENTITY_0_0,
    pub SecureId: [u8; 32],
}
impl ::core::marker::Copy for WINBIO_IDENTITY_0 {}
impl ::core::clone::Clone for WINBIO_IDENTITY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WINBIO_IDENTITY_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WINBIO_IDENTITY_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_IDENTITY_0_0 {
    pub Size: u32,
    pub Data: [u8; 68],
}
impl ::core::marker::Copy for WINBIO_IDENTITY_0_0 {}
impl ::core::clone::Clone for WINBIO_IDENTITY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_IDENTITY_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_IDENTITY_0_0").field("Size", &self.Size).field("Data", &self.Data).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_IDENTITY_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_IDENTITY_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for WINBIO_IDENTITY_0_0 {}
impl ::core::default::Default for WINBIO_IDENTITY_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_NOTIFY_WAKE {
    pub PayloadSize: u32,
    pub WinBioHresult: ::windows::core::HRESULT,
    pub Reason: u32,
}
impl ::core::marker::Copy for WINBIO_NOTIFY_WAKE {}
impl ::core::clone::Clone for WINBIO_NOTIFY_WAKE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_NOTIFY_WAKE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_NOTIFY_WAKE").field("PayloadSize", &self.PayloadSize).field("WinBioHresult", &self.WinBioHresult).field("Reason", &self.Reason).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_NOTIFY_WAKE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_NOTIFY_WAKE {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.WinBioHresult == other.WinBioHresult && self.Reason == other.Reason
    }
}
impl ::core::cmp::Eq for WINBIO_NOTIFY_WAKE {}
impl ::core::default::Default for WINBIO_NOTIFY_WAKE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub struct WINBIO_PIPELINE {
    pub SensorHandle: super::super::Foundation::HANDLE,
    pub EngineHandle: super::super::Foundation::HANDLE,
    pub StorageHandle: super::super::Foundation::HANDLE,
    pub SensorInterface: *mut WINBIO_SENSOR_INTERFACE,
    pub EngineInterface: *mut WINBIO_ENGINE_INTERFACE,
    pub StorageInterface: *mut WINBIO_STORAGE_INTERFACE,
    pub SensorContext: *mut _WINIBIO_SENSOR_CONTEXT,
    pub EngineContext: *mut _WINIBIO_ENGINE_CONTEXT,
    pub StorageContext: *mut _WINIBIO_STORAGE_CONTEXT,
    pub FrameworkInterface: *mut WINBIO_FRAMEWORK_INTERFACE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::marker::Copy for WINBIO_PIPELINE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::clone::Clone for WINBIO_PIPELINE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::fmt::Debug for WINBIO_PIPELINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_PIPELINE")
            .field("SensorHandle", &self.SensorHandle)
            .field("EngineHandle", &self.EngineHandle)
            .field("StorageHandle", &self.StorageHandle)
            .field("SensorInterface", &self.SensorInterface)
            .field("EngineInterface", &self.EngineInterface)
            .field("StorageInterface", &self.StorageInterface)
            .field("SensorContext", &self.SensorContext)
            .field("EngineContext", &self.EngineContext)
            .field("StorageContext", &self.StorageContext)
            .field("FrameworkInterface", &self.FrameworkInterface)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::windows::core::TypeKind for WINBIO_PIPELINE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::cmp::PartialEq for WINBIO_PIPELINE {
    fn eq(&self, other: &Self) -> bool {
        self.SensorHandle == other.SensorHandle && self.EngineHandle == other.EngineHandle && self.StorageHandle == other.StorageHandle && self.SensorInterface == other.SensorInterface && self.EngineInterface == other.EngineInterface && self.StorageInterface == other.StorageInterface && self.SensorContext == other.SensorContext && self.EngineContext == other.EngineContext && self.StorageContext == other.StorageContext && self.FrameworkInterface == other.FrameworkInterface
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::cmp::Eq for WINBIO_PIPELINE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::default::Default for WINBIO_PIPELINE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_PRESENCE {
    pub Factor: u32,
    pub SubFactor: u8,
    pub Status: ::windows::core::HRESULT,
    pub RejectDetail: u32,
    pub Identity: WINBIO_IDENTITY,
    pub TrackingId: u64,
    pub Ticket: u64,
    pub Properties: WINBIO_PRESENCE_PROPERTIES,
    pub Authorization: WINBIO_PRESENCE_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_PRESENCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_PRESENCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_PRESENCE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_PRESENCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_PRESENCE_0 {
    pub Size: u32,
    pub Data: [u8; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_PRESENCE_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_PRESENCE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINBIO_PRESENCE_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_PRESENCE_0").field("Size", &self.Size).field("Data", &self.Data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_PRESENCE_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_PRESENCE_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Data == other.Data
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_PRESENCE_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_PRESENCE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union WINBIO_PRESENCE_PROPERTIES {
    pub FacialFeatures: WINBIO_PRESENCE_PROPERTIES_0,
    pub Iris: WINBIO_PRESENCE_PROPERTIES_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_PRESENCE_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_PRESENCE_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_PRESENCE_PROPERTIES {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_PRESENCE_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_PRESENCE_PROPERTIES_0 {
    pub BoundingBox: super::super::Foundation::RECT,
    pub Distance: i32,
    pub OpaqueEngineData: WINBIO_PRESENCE_PROPERTIES_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_PRESENCE_PROPERTIES_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_PRESENCE_PROPERTIES_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINBIO_PRESENCE_PROPERTIES_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_PRESENCE_PROPERTIES_0").field("BoundingBox", &self.BoundingBox).field("Distance", &self.Distance).field("OpaqueEngineData", &self.OpaqueEngineData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_PRESENCE_PROPERTIES_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_PRESENCE_PROPERTIES_0 {
    fn eq(&self, other: &Self) -> bool {
        self.BoundingBox == other.BoundingBox && self.Distance == other.Distance && self.OpaqueEngineData == other.OpaqueEngineData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_PRESENCE_PROPERTIES_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_PRESENCE_PROPERTIES_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_PRESENCE_PROPERTIES_0_0 {
    pub AdapterId: ::windows::core::GUID,
    pub Data: [u32; 78],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_PRESENCE_PROPERTIES_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_PRESENCE_PROPERTIES_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINBIO_PRESENCE_PROPERTIES_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_PRESENCE_PROPERTIES_0_0").field("AdapterId", &self.AdapterId).field("Data", &self.Data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_PRESENCE_PROPERTIES_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_PRESENCE_PROPERTIES_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.AdapterId == other.AdapterId && self.Data == other.Data
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_PRESENCE_PROPERTIES_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_PRESENCE_PROPERTIES_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_PRESENCE_PROPERTIES_1 {
    pub EyeBoundingBox_1: super::super::Foundation::RECT,
    pub EyeBoundingBox_2: super::super::Foundation::RECT,
    pub PupilCenter_1: super::super::Foundation::POINT,
    pub PupilCenter_2: super::super::Foundation::POINT,
    pub Distance: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_PRESENCE_PROPERTIES_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_PRESENCE_PROPERTIES_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINBIO_PRESENCE_PROPERTIES_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_PRESENCE_PROPERTIES_1").field("EyeBoundingBox_1", &self.EyeBoundingBox_1).field("EyeBoundingBox_2", &self.EyeBoundingBox_2).field("PupilCenter_1", &self.PupilCenter_1).field("PupilCenter_2", &self.PupilCenter_2).field("Distance", &self.Distance).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINBIO_PRESENCE_PROPERTIES_1 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_PRESENCE_PROPERTIES_1 {
    fn eq(&self, other: &Self) -> bool {
        self.EyeBoundingBox_1 == other.EyeBoundingBox_1 && self.EyeBoundingBox_2 == other.EyeBoundingBox_2 && self.PupilCenter_1 == other.PupilCenter_1 && self.PupilCenter_2 == other.PupilCenter_2 && self.Distance == other.Distance
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_PRESENCE_PROPERTIES_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_PRESENCE_PROPERTIES_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_PRIVATE_SENSOR_TYPE_INFO {
    pub PayloadSize: u32,
    pub WinBioHresult: ::windows::core::HRESULT,
    pub PrivateSensorTypeInfo: WINBIO_DATA,
}
impl ::core::marker::Copy for WINBIO_PRIVATE_SENSOR_TYPE_INFO {}
impl ::core::clone::Clone for WINBIO_PRIVATE_SENSOR_TYPE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_PRIVATE_SENSOR_TYPE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_PRIVATE_SENSOR_TYPE_INFO").field("PayloadSize", &self.PayloadSize).field("WinBioHresult", &self.WinBioHresult).field("PrivateSensorTypeInfo", &self.PrivateSensorTypeInfo).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_PRIVATE_SENSOR_TYPE_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_PRIVATE_SENSOR_TYPE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.WinBioHresult == other.WinBioHresult && self.PrivateSensorTypeInfo == other.PrivateSensorTypeInfo
    }
}
impl ::core::cmp::Eq for WINBIO_PRIVATE_SENSOR_TYPE_INFO {}
impl ::core::default::Default for WINBIO_PRIVATE_SENSOR_TYPE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_PROTECTION_POLICY {
    pub Version: u32,
    pub Identity: WINBIO_IDENTITY,
    pub DatabaseId: ::windows::core::GUID,
    pub UserState: u64,
    pub PolicySize: usize,
    pub Policy: [u8; 128],
}
impl ::core::marker::Copy for WINBIO_PROTECTION_POLICY {}
impl ::core::clone::Clone for WINBIO_PROTECTION_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WINBIO_PROTECTION_POLICY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WINBIO_PROTECTION_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_REGISTERED_FORMAT {
    pub Owner: u16,
    pub Type: u16,
}
impl ::core::marker::Copy for WINBIO_REGISTERED_FORMAT {}
impl ::core::clone::Clone for WINBIO_REGISTERED_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_REGISTERED_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_REGISTERED_FORMAT").field("Owner", &self.Owner).field("Type", &self.Type).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_REGISTERED_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_REGISTERED_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.Owner == other.Owner && self.Type == other.Type
    }
}
impl ::core::cmp::Eq for WINBIO_REGISTERED_FORMAT {}
impl ::core::default::Default for WINBIO_REGISTERED_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_SECURE_BUFFER_HEADER_V1 {
    pub Type: u32,
    pub Size: u32,
    pub Flags: u32,
    pub ValidationTag: u64,
}
impl ::core::marker::Copy for WINBIO_SECURE_BUFFER_HEADER_V1 {}
impl ::core::clone::Clone for WINBIO_SECURE_BUFFER_HEADER_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_SECURE_BUFFER_HEADER_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_SECURE_BUFFER_HEADER_V1").field("Type", &self.Type).field("Size", &self.Size).field("Flags", &self.Flags).field("ValidationTag", &self.ValidationTag).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_SECURE_BUFFER_HEADER_V1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_SECURE_BUFFER_HEADER_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Size == other.Size && self.Flags == other.Flags && self.ValidationTag == other.ValidationTag
    }
}
impl ::core::cmp::Eq for WINBIO_SECURE_BUFFER_HEADER_V1 {}
impl ::core::default::Default for WINBIO_SECURE_BUFFER_HEADER_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_SECURE_CONNECTION_DATA {
    pub Size: u32,
    pub Version: u16,
    pub Flags: u16,
    pub ModelCertificateSize: u32,
    pub IntermediateCA1Size: u32,
    pub IntermediateCA2Size: u32,
}
impl ::core::marker::Copy for WINBIO_SECURE_CONNECTION_DATA {}
impl ::core::clone::Clone for WINBIO_SECURE_CONNECTION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_SECURE_CONNECTION_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_SECURE_CONNECTION_DATA").field("Size", &self.Size).field("Version", &self.Version).field("Flags", &self.Flags).field("ModelCertificateSize", &self.ModelCertificateSize).field("IntermediateCA1Size", &self.IntermediateCA1Size).field("IntermediateCA2Size", &self.IntermediateCA2Size).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_SECURE_CONNECTION_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_SECURE_CONNECTION_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.Flags == other.Flags && self.ModelCertificateSize == other.ModelCertificateSize && self.IntermediateCA1Size == other.IntermediateCA1Size && self.IntermediateCA2Size == other.IntermediateCA2Size
    }
}
impl ::core::cmp::Eq for WINBIO_SECURE_CONNECTION_DATA {}
impl ::core::default::Default for WINBIO_SECURE_CONNECTION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_SECURE_CONNECTION_PARAMS {
    pub PayloadSize: u32,
    pub Version: u16,
    pub Flags: u16,
}
impl ::core::marker::Copy for WINBIO_SECURE_CONNECTION_PARAMS {}
impl ::core::clone::Clone for WINBIO_SECURE_CONNECTION_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_SECURE_CONNECTION_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_SECURE_CONNECTION_PARAMS").field("PayloadSize", &self.PayloadSize).field("Version", &self.Version).field("Flags", &self.Flags).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_SECURE_CONNECTION_PARAMS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_SECURE_CONNECTION_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.Version == other.Version && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for WINBIO_SECURE_CONNECTION_PARAMS {}
impl ::core::default::Default for WINBIO_SECURE_CONNECTION_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_SENSOR_ATTRIBUTES {
    pub PayloadSize: u32,
    pub WinBioHresult: ::windows::core::HRESULT,
    pub WinBioVersion: WINBIO_VERSION,
    pub SensorType: u32,
    pub SensorSubType: u32,
    pub Capabilities: u32,
    pub ManufacturerName: [u16; 256],
    pub ModelName: [u16; 256],
    pub SerialNumber: [u16; 256],
    pub FirmwareVersion: WINBIO_VERSION,
    pub SupportedFormatEntries: u32,
    pub SupportedFormat: [WINBIO_REGISTERED_FORMAT; 1],
}
impl ::core::marker::Copy for WINBIO_SENSOR_ATTRIBUTES {}
impl ::core::clone::Clone for WINBIO_SENSOR_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_SENSOR_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_SENSOR_ATTRIBUTES")
            .field("PayloadSize", &self.PayloadSize)
            .field("WinBioHresult", &self.WinBioHresult)
            .field("WinBioVersion", &self.WinBioVersion)
            .field("SensorType", &self.SensorType)
            .field("SensorSubType", &self.SensorSubType)
            .field("Capabilities", &self.Capabilities)
            .field("ManufacturerName", &self.ManufacturerName)
            .field("ModelName", &self.ModelName)
            .field("SerialNumber", &self.SerialNumber)
            .field("FirmwareVersion", &self.FirmwareVersion)
            .field("SupportedFormatEntries", &self.SupportedFormatEntries)
            .field("SupportedFormat", &self.SupportedFormat)
            .finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_SENSOR_ATTRIBUTES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_SENSOR_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.WinBioHresult == other.WinBioHresult && self.WinBioVersion == other.WinBioVersion && self.SensorType == other.SensorType && self.SensorSubType == other.SensorSubType && self.Capabilities == other.Capabilities && self.ManufacturerName == other.ManufacturerName && self.ModelName == other.ModelName && self.SerialNumber == other.SerialNumber && self.FirmwareVersion == other.FirmwareVersion && self.SupportedFormatEntries == other.SupportedFormatEntries && self.SupportedFormat == other.SupportedFormat
    }
}
impl ::core::cmp::Eq for WINBIO_SENSOR_ATTRIBUTES {}
impl ::core::default::Default for WINBIO_SENSOR_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub struct WINBIO_SENSOR_INTERFACE {
    pub Version: WINBIO_ADAPTER_INTERFACE_VERSION,
    pub Type: u32,
    pub Size: usize,
    pub AdapterId: ::windows::core::GUID,
    pub Attach: PIBIO_SENSOR_ATTACH_FN,
    pub Detach: PIBIO_SENSOR_DETACH_FN,
    pub ClearContext: PIBIO_SENSOR_CLEAR_CONTEXT_FN,
    pub QueryStatus: PIBIO_SENSOR_QUERY_STATUS_FN,
    pub Reset: PIBIO_SENSOR_RESET_FN,
    pub SetMode: PIBIO_SENSOR_SET_MODE_FN,
    pub SetIndicatorStatus: PIBIO_SENSOR_SET_INDICATOR_STATUS_FN,
    pub GetIndicatorStatus: PIBIO_SENSOR_GET_INDICATOR_STATUS_FN,
    pub StartCapture: PIBIO_SENSOR_START_CAPTURE_FN,
    pub FinishCapture: PIBIO_SENSOR_FINISH_CAPTURE_FN,
    pub ExportSensorData: PIBIO_SENSOR_EXPORT_SENSOR_DATA_FN,
    pub Cancel: PIBIO_SENSOR_CANCEL_FN,
    pub PushDataToEngine: PIBIO_SENSOR_PUSH_DATA_TO_ENGINE_FN,
    pub ControlUnit: PIBIO_SENSOR_CONTROL_UNIT_FN,
    pub ControlUnitPrivileged: PIBIO_SENSOR_CONTROL_UNIT_PRIVILEGED_FN,
    pub NotifyPowerChange: PIBIO_SENSOR_NOTIFY_POWER_CHANGE_FN,
    pub PipelineInit: PIBIO_SENSOR_PIPELINE_INIT_FN,
    pub PipelineCleanup: PIBIO_SENSOR_PIPELINE_CLEANUP_FN,
    pub Activate: PIBIO_SENSOR_ACTIVATE_FN,
    pub Deactivate: PIBIO_SENSOR_DEACTIVATE_FN,
    pub QueryExtendedInfo: PIBIO_SENSOR_QUERY_EXTENDED_INFO_FN,
    pub QueryCalibrationFormats: PIBIO_SENSOR_QUERY_CALIBRATION_FORMATS_FN,
    pub SetCalibrationFormat: PIBIO_SENSOR_SET_CALIBRATION_FORMAT_FN,
    pub AcceptCalibrationData: PIBIO_SENSOR_ACCEPT_CALIBRATION_DATA_FN,
    pub AsyncImportRawBuffer: PIBIO_SENSOR_ASYNC_IMPORT_RAW_BUFFER_FN,
    pub AsyncImportSecureBuffer: PIBIO_SENSOR_ASYNC_IMPORT_SECURE_BUFFER_FN,
    pub QueryPrivateSensorType: PIBIO_SENSOR_QUERY_PRIVATE_SENSOR_TYPE_FN,
    pub ConnectSecure: PIBIO_SENSOR_CONNECT_SECURE_FN,
    pub StartCaptureEx: PIBIO_SENSOR_START_CAPTURE_EX_FN,
    pub StartNotifyWake: PIBIO_SENSOR_START_NOTIFY_WAKE_FN,
    pub FinishNotifyWake: PIBIO_SENSOR_FINISH_NOTIFY_WAKE_FN,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::marker::Copy for WINBIO_SENSOR_INTERFACE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::clone::Clone for WINBIO_SENSOR_INTERFACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::fmt::Debug for WINBIO_SENSOR_INTERFACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_SENSOR_INTERFACE").field("Version", &self.Version).field("Type", &self.Type).field("Size", &self.Size).field("AdapterId", &self.AdapterId).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::windows::core::TypeKind for WINBIO_SENSOR_INTERFACE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::default::Default for WINBIO_SENSOR_INTERFACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_SET_INDICATOR {
    pub PayloadSize: u32,
    pub IndicatorStatus: u32,
}
impl ::core::marker::Copy for WINBIO_SET_INDICATOR {}
impl ::core::clone::Clone for WINBIO_SET_INDICATOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_SET_INDICATOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_SET_INDICATOR").field("PayloadSize", &self.PayloadSize).field("IndicatorStatus", &self.IndicatorStatus).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_SET_INDICATOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_SET_INDICATOR {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.IndicatorStatus == other.IndicatorStatus
    }
}
impl ::core::cmp::Eq for WINBIO_SET_INDICATOR {}
impl ::core::default::Default for WINBIO_SET_INDICATOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub struct WINBIO_STORAGE_INTERFACE {
    pub Version: WINBIO_ADAPTER_INTERFACE_VERSION,
    pub Type: u32,
    pub Size: usize,
    pub AdapterId: ::windows::core::GUID,
    pub Attach: PIBIO_STORAGE_ATTACH_FN,
    pub Detach: PIBIO_STORAGE_DETACH_FN,
    pub ClearContext: PIBIO_STORAGE_CLEAR_CONTEXT_FN,
    pub CreateDatabase: PIBIO_STORAGE_CREATE_DATABASE_FN,
    pub EraseDatabase: PIBIO_STORAGE_ERASE_DATABASE_FN,
    pub OpenDatabase: PIBIO_STORAGE_OPEN_DATABASE_FN,
    pub CloseDatabase: PIBIO_STORAGE_CLOSE_DATABASE_FN,
    pub GetDataFormat: PIBIO_STORAGE_GET_DATA_FORMAT_FN,
    pub GetDatabaseSize: PIBIO_STORAGE_GET_DATABASE_SIZE_FN,
    pub AddRecord: PIBIO_STORAGE_ADD_RECORD_FN,
    pub DeleteRecord: PIBIO_STORAGE_DELETE_RECORD_FN,
    pub QueryBySubject: PIBIO_STORAGE_QUERY_BY_SUBJECT_FN,
    pub QueryByContent: PIBIO_STORAGE_QUERY_BY_CONTENT_FN,
    pub GetRecordCount: PIBIO_STORAGE_GET_RECORD_COUNT_FN,
    pub FirstRecord: PIBIO_STORAGE_FIRST_RECORD_FN,
    pub NextRecord: PIBIO_STORAGE_NEXT_RECORD_FN,
    pub GetCurrentRecord: PIBIO_STORAGE_GET_CURRENT_RECORD_FN,
    pub ControlUnit: PIBIO_STORAGE_CONTROL_UNIT_FN,
    pub ControlUnitPrivileged: PIBIO_STORAGE_CONTROL_UNIT_PRIVILEGED_FN,
    pub NotifyPowerChange: PIBIO_STORAGE_NOTIFY_POWER_CHANGE_FN,
    pub PipelineInit: PIBIO_STORAGE_PIPELINE_INIT_FN,
    pub PipelineCleanup: PIBIO_STORAGE_PIPELINE_CLEANUP_FN,
    pub Activate: PIBIO_STORAGE_ACTIVATE_FN,
    pub Deactivate: PIBIO_STORAGE_DEACTIVATE_FN,
    pub QueryExtendedInfo: PIBIO_STORAGE_QUERY_EXTENDED_INFO_FN,
    pub NotifyDatabaseChange: PIBIO_STORAGE_NOTIFY_DATABASE_CHANGE_FN,
    pub Reserved1: PIBIO_STORAGE_RESERVED_1_FN,
    pub Reserved2: PIBIO_STORAGE_RESERVED_2_FN,
    pub UpdateRecordBegin: PIBIO_STORAGE_UPDATE_RECORD_BEGIN_FN,
    pub UpdateRecordCommit: PIBIO_STORAGE_UPDATE_RECORD_COMMIT_FN,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::marker::Copy for WINBIO_STORAGE_INTERFACE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::clone::Clone for WINBIO_STORAGE_INTERFACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::fmt::Debug for WINBIO_STORAGE_INTERFACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_STORAGE_INTERFACE").field("Version", &self.Version).field("Type", &self.Type).field("Size", &self.Size).field("AdapterId", &self.AdapterId).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::windows::core::TypeKind for WINBIO_STORAGE_INTERFACE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::default::Default for WINBIO_STORAGE_INTERFACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_STORAGE_RECORD {
    pub Identity: *mut WINBIO_IDENTITY,
    pub SubFactor: u8,
    pub IndexVector: *mut u32,
    pub IndexElementCount: usize,
    pub TemplateBlob: *mut u8,
    pub TemplateBlobSize: usize,
    pub PayloadBlob: *mut u8,
    pub PayloadBlobSize: usize,
}
impl ::core::marker::Copy for WINBIO_STORAGE_RECORD {}
impl ::core::clone::Clone for WINBIO_STORAGE_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_STORAGE_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_STORAGE_RECORD").field("Identity", &self.Identity).field("SubFactor", &self.SubFactor).field("IndexVector", &self.IndexVector).field("IndexElementCount", &self.IndexElementCount).field("TemplateBlob", &self.TemplateBlob).field("TemplateBlobSize", &self.TemplateBlobSize).field("PayloadBlob", &self.PayloadBlob).field("PayloadBlobSize", &self.PayloadBlobSize).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_STORAGE_RECORD {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_STORAGE_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.Identity == other.Identity && self.SubFactor == other.SubFactor && self.IndexVector == other.IndexVector && self.IndexElementCount == other.IndexElementCount && self.TemplateBlob == other.TemplateBlob && self.TemplateBlobSize == other.TemplateBlobSize && self.PayloadBlob == other.PayloadBlob && self.PayloadBlobSize == other.PayloadBlobSize
    }
}
impl ::core::cmp::Eq for WINBIO_STORAGE_RECORD {}
impl ::core::default::Default for WINBIO_STORAGE_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_STORAGE_SCHEMA {
    pub BiometricFactor: u32,
    pub DatabaseId: ::windows::core::GUID,
    pub DataFormat: ::windows::core::GUID,
    pub Attributes: u32,
    pub FilePath: [u16; 256],
    pub ConnectionString: [u16; 256],
}
impl ::core::marker::Copy for WINBIO_STORAGE_SCHEMA {}
impl ::core::clone::Clone for WINBIO_STORAGE_SCHEMA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_STORAGE_SCHEMA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_STORAGE_SCHEMA").field("BiometricFactor", &self.BiometricFactor).field("DatabaseId", &self.DatabaseId).field("DataFormat", &self.DataFormat).field("Attributes", &self.Attributes).field("FilePath", &self.FilePath).field("ConnectionString", &self.ConnectionString).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_STORAGE_SCHEMA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_STORAGE_SCHEMA {
    fn eq(&self, other: &Self) -> bool {
        self.BiometricFactor == other.BiometricFactor && self.DatabaseId == other.DatabaseId && self.DataFormat == other.DataFormat && self.Attributes == other.Attributes && self.FilePath == other.FilePath && self.ConnectionString == other.ConnectionString
    }
}
impl ::core::cmp::Eq for WINBIO_STORAGE_SCHEMA {}
impl ::core::default::Default for WINBIO_STORAGE_SCHEMA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_SUPPORTED_ALGORITHMS {
    pub PayloadSize: u32,
    pub WinBioHresult: ::windows::core::HRESULT,
    pub NumberOfAlgorithms: u32,
    pub AlgorithmData: WINBIO_DATA,
}
impl ::core::marker::Copy for WINBIO_SUPPORTED_ALGORITHMS {}
impl ::core::clone::Clone for WINBIO_SUPPORTED_ALGORITHMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_SUPPORTED_ALGORITHMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_SUPPORTED_ALGORITHMS").field("PayloadSize", &self.PayloadSize).field("WinBioHresult", &self.WinBioHresult).field("NumberOfAlgorithms", &self.NumberOfAlgorithms).field("AlgorithmData", &self.AlgorithmData).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_SUPPORTED_ALGORITHMS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_SUPPORTED_ALGORITHMS {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.WinBioHresult == other.WinBioHresult && self.NumberOfAlgorithms == other.NumberOfAlgorithms && self.AlgorithmData == other.AlgorithmData
    }
}
impl ::core::cmp::Eq for WINBIO_SUPPORTED_ALGORITHMS {}
impl ::core::default::Default for WINBIO_SUPPORTED_ALGORITHMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_UNIT_SCHEMA {
    pub UnitId: u32,
    pub PoolType: u32,
    pub BiometricFactor: u32,
    pub SensorSubType: u32,
    pub Capabilities: u32,
    pub DeviceInstanceId: [u16; 256],
    pub Description: [u16; 256],
    pub Manufacturer: [u16; 256],
    pub Model: [u16; 256],
    pub SerialNumber: [u16; 256],
    pub FirmwareVersion: WINBIO_VERSION,
}
impl ::core::marker::Copy for WINBIO_UNIT_SCHEMA {}
impl ::core::clone::Clone for WINBIO_UNIT_SCHEMA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_UNIT_SCHEMA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_UNIT_SCHEMA")
            .field("UnitId", &self.UnitId)
            .field("PoolType", &self.PoolType)
            .field("BiometricFactor", &self.BiometricFactor)
            .field("SensorSubType", &self.SensorSubType)
            .field("Capabilities", &self.Capabilities)
            .field("DeviceInstanceId", &self.DeviceInstanceId)
            .field("Description", &self.Description)
            .field("Manufacturer", &self.Manufacturer)
            .field("Model", &self.Model)
            .field("SerialNumber", &self.SerialNumber)
            .field("FirmwareVersion", &self.FirmwareVersion)
            .finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_UNIT_SCHEMA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_UNIT_SCHEMA {
    fn eq(&self, other: &Self) -> bool {
        self.UnitId == other.UnitId && self.PoolType == other.PoolType && self.BiometricFactor == other.BiometricFactor && self.SensorSubType == other.SensorSubType && self.Capabilities == other.Capabilities && self.DeviceInstanceId == other.DeviceInstanceId && self.Description == other.Description && self.Manufacturer == other.Manufacturer && self.Model == other.Model && self.SerialNumber == other.SerialNumber && self.FirmwareVersion == other.FirmwareVersion
    }
}
impl ::core::cmp::Eq for WINBIO_UNIT_SCHEMA {}
impl ::core::default::Default for WINBIO_UNIT_SCHEMA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_UPDATE_FIRMWARE {
    pub PayloadSize: u32,
    pub FirmwareData: WINBIO_DATA,
}
impl ::core::marker::Copy for WINBIO_UPDATE_FIRMWARE {}
impl ::core::clone::Clone for WINBIO_UPDATE_FIRMWARE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_UPDATE_FIRMWARE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_UPDATE_FIRMWARE").field("PayloadSize", &self.PayloadSize).field("FirmwareData", &self.FirmwareData).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_UPDATE_FIRMWARE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_UPDATE_FIRMWARE {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.FirmwareData == other.FirmwareData
    }
}
impl ::core::cmp::Eq for WINBIO_UPDATE_FIRMWARE {}
impl ::core::default::Default for WINBIO_UPDATE_FIRMWARE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub struct WINBIO_VERSION {
    pub MajorVersion: u32,
    pub MinorVersion: u32,
}
impl ::core::marker::Copy for WINBIO_VERSION {}
impl ::core::clone::Clone for WINBIO_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINBIO_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINBIO_VERSION").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).finish()
    }
}
impl ::windows::core::TypeKind for WINBIO_VERSION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINBIO_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion
    }
}
impl ::core::cmp::Eq for WINBIO_VERSION {}
impl ::core::default::Default for WINBIO_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct _WINIBIO_ENGINE_CONTEXT(pub u8);
#[repr(C)]
pub struct _WINIBIO_SENSOR_CONTEXT(pub u8);
#[repr(C)]
pub struct _WINIBIO_STORAGE_CONTEXT(pub u8);
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_ACCEPT_PRIVATE_SENSOR_TYPE_INFO_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, typeinfobufferaddress: *const u8, typeinfobuffersize: usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_ACCEPT_SAMPLE_DATA_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, samplebuffer: *const WINBIO_BIR, samplesize: usize, purpose: u8, rejectdetail: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_ACTIVATE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_ATTACH_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_CHECK_FOR_DUPLICATE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *mut WINBIO_IDENTITY, subfactor: *mut u8, duplicate: *mut super::super::Foundation::BOOLEAN) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_CLEAR_CONTEXT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_COMMIT_ENROLLMENT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *const WINBIO_IDENTITY, subfactor: u8, payloadblob: *const u8, payloadblobsize: usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_CONTROL_UNIT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_CONTROL_UNIT_PRIVILEGED_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_CREATE_ENROLLMENT_AUTHENTICATED_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, nonce: *mut *mut u8, noncesize: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_CREATE_ENROLLMENT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_CREATE_KEY_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, key: *const u8, keysize: usize, keyidentifier: *mut u8, keyidentifiersize: usize, resultsize: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_DEACTIVATE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_DETACH_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_DISCARD_ENROLLMENT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_EXPORT_ENGINE_DATA_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, flags: u8, samplebuffer: *mut *mut WINBIO_BIR, samplesize: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_GET_ENROLLMENT_HASH_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, hashvalue: *mut *mut u8, hashsize: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_GET_ENROLLMENT_STATUS_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, rejectdetail: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_IDENTIFY_ALL_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, presencecount: *mut usize, presencearray: *mut *mut WINBIO_PRESENCE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_IDENTIFY_FEATURE_SET_AUTHENTICATED_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, nonce: *const u8, noncesize: usize, identity: *mut WINBIO_IDENTITY, subfactor: *mut u8, rejectdetail: *mut u32, authentication: *mut *mut u8, authenticationsize: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_IDENTIFY_FEATURE_SET_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *mut WINBIO_IDENTITY, subfactor: *mut u8, payloadblob: *mut *mut u8, payloadblobsize: *mut usize, hashvalue: *mut *mut u8, hashsize: *mut usize, rejectdetail: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_IDENTIFY_FEATURE_SET_SECURE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, nonce: *const u8, noncesize: usize, keyidentifier: *const u8, keyidentifiersize: usize, identity: *mut WINBIO_IDENTITY, subfactor: *mut u8, rejectdetail: *mut u32, authorization: *mut *mut u8, authorizationsize: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_NOTIFY_POWER_CHANGE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, powereventtype: u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_PIPELINE_CLEANUP_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_PIPELINE_INIT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_QUERY_CALIBRATION_DATA_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, discardandrepeatcapture: *mut super::super::Foundation::BOOLEAN, calibrationbuffer: *mut u8, calibrationbuffersize: *mut usize, maxbuffersize: usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_QUERY_EXTENDED_ENROLLMENT_STATUS_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, enrollmentstatus: *mut WINBIO_EXTENDED_ENROLLMENT_STATUS, enrollmentstatussize: usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_QUERY_EXTENDED_INFO_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, engineinfo: *mut WINBIO_EXTENDED_ENGINE_INFO, engineinfosize: usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_QUERY_HASH_ALGORITHMS_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, algorithmcount: *mut usize, algorithmbuffersize: *mut usize, algorithmbuffer: *mut *mut u8) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_QUERY_INDEX_VECTOR_SIZE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, indexelementcount: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_QUERY_PREFERRED_FORMAT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, standardformat: *mut WINBIO_REGISTERED_FORMAT, vendorformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_QUERY_SAMPLE_HINT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, samplehint: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_REFRESH_CACHE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_RESERVED_1_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *mut WINBIO_IDENTITY) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_SELECT_CALIBRATION_FORMAT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, formatarray: *const ::windows::core::GUID, formatcount: usize, selectedformat: *mut ::windows::core::GUID, maxbuffersize: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_SET_ACCOUNT_POLICY_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, policyitemarray: *const WINBIO_ACCOUNT_POLICY, policyitemcount: usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_SET_ENROLLMENT_PARAMETERS_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, parameters: *const WINBIO_EXTENDED_ENROLLMENT_PARAMETERS) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_SET_ENROLLMENT_SELECTOR_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, selectorvalue: u64) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_SET_HASH_ALGORITHM_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, algorithmbuffersize: usize, algorithmbuffer: *const u8) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_UPDATE_ENROLLMENT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, rejectdetail: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_VERIFY_FEATURE_SET_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *const WINBIO_IDENTITY, subfactor: u8, r#match: *mut super::super::Foundation::BOOLEAN, payloadblob: *mut *mut u8, payloadblobsize: *mut usize, hashvalue: *mut *mut u8, hashsize: *mut usize, rejectdetail: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_ALLOCATE_MEMORY_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, allocationsize: usize, address: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_FREE_MEMORY_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, address: *const ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_GET_PROPERTY_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, propertytype: u32, propertyid: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, propertybuffer: *mut *mut ::core::ffi::c_void, propertybuffersize: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_LOCK_AND_VALIDATE_SECURE_BUFFER_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, securebufferidentifier: ::windows::core::GUID, securebufferaddress: *mut *mut ::core::ffi::c_void, securebuffersize: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_RELEASE_SECURE_BUFFER_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, securebufferidentifier: ::windows::core::GUID) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_SET_UNIT_STATUS_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, extendedstatus: *const WINBIO_EXTENDED_UNIT_STATUS, extendedstatussize: usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_CACHE_CLEAR_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_CACHE_EXPORT_BEGIN_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, requiredcapacity: *mut usize, maxbuffersize: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_CACHE_EXPORT_END_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_CACHE_EXPORT_NEXT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, bufferaddress: *mut u8, buffersize: usize, returneddatasize: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_CACHE_IMPORT_BEGIN_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, requiredcapacity: usize, maxbuffersize: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_CACHE_IMPORT_END_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_CACHE_IMPORT_NEXT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, bufferaddress: *const u8, buffersize: usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_DECRYPT_SAMPLE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, authentication: *const u8, authenticationsize: usize, iv: *const u8, ivsize: usize, encrypteddata: *mut u8, encrypteddatasize: usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_QUERY_AUTHORIZED_ENROLLMENTS_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *const WINBIO_IDENTITY, secureidentitycount: *mut usize, secureidentities: *mut *mut WINBIO_IDENTITY) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_STORAGE_RESERVED_1_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *const WINBIO_PIPELINE, reserved1: usize, reserved2: *const usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_STORAGE_RESERVED_2_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *const WINBIO_PIPELINE, reserved1: *const u8, reserved2: usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_STORAGE_RESERVED_3_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *const WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_ACCEPT_CALIBRATION_DATA_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, calibrationbuffer: *const u8, calibrationbuffersize: usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_ACTIVATE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_ASYNC_IMPORT_RAW_BUFFER_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, rawbufferaddress: *const u8, rawbuffersize: usize, resultbufferaddress: *mut *mut u8, resultbuffersize: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_ASYNC_IMPORT_SECURE_BUFFER_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, securebufferidentifier: ::windows::core::GUID, metadatabufferaddress: *const u8, metadatabuffersize: usize, resultbufferaddress: *mut *mut u8, resultbuffersize: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_ATTACH_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_CANCEL_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_CLEAR_CONTEXT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_CONNECT_SECURE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, connectionparams: *const WINBIO_SECURE_CONNECTION_PARAMS, connectiondata: *mut *mut WINBIO_SECURE_CONNECTION_DATA) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_CONTROL_UNIT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_CONTROL_UNIT_PRIVILEGED_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_DEACTIVATE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_DETACH_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_EXPORT_SENSOR_DATA_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, samplebuffer: *mut *mut WINBIO_BIR, samplesize: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_FINISH_CAPTURE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, rejectdetail: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_FINISH_NOTIFY_WAKE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, reason: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_GET_INDICATOR_STATUS_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, indicatorstatus: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_NOTIFY_POWER_CHANGE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, powereventtype: u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_PIPELINE_CLEANUP_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_PIPELINE_INIT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_PUSH_DATA_TO_ENGINE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, purpose: u8, flags: u8, rejectdetail: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_QUERY_CALIBRATION_FORMATS_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, formatarray: *mut ::windows::core::GUID, formatarraysize: usize, formatcount: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_QUERY_EXTENDED_INFO_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, sensorinfo: *mut WINBIO_EXTENDED_SENSOR_INFO, sensorinfosize: usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_QUERY_PRIVATE_SENSOR_TYPE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, typeinfobufferaddress: *mut u8, typeinfobuffersize: usize, typeinfodatasize: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_QUERY_STATUS_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, status: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_RESET_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_SET_CALIBRATION_FORMAT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, format: *const ::windows::core::GUID) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_SET_INDICATOR_STATUS_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, indicatorstatus: u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_SET_MODE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, mode: u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_START_CAPTURE_EX_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, purpose: u8, nonce: *const u8, noncesize: usize, flags: u8, overlapped: *mut *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_START_CAPTURE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, purpose: u8, overlapped: *mut *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_START_NOTIFY_WAKE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, overlapped: *mut *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_ACTIVATE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_ADD_RECORD_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, recordcontents: *const WINBIO_STORAGE_RECORD) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_ATTACH_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_CLEAR_CONTEXT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_CLOSE_DATABASE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_CONTROL_UNIT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_CONTROL_UNIT_PRIVILEGED_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_CREATE_DATABASE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, databaseid: *const ::windows::core::GUID, factor: u32, format: *const ::windows::core::GUID, filepath: ::windows::core::PCWSTR, connectstring: ::windows::core::PCWSTR, indexelementcount: usize, initialsize: usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_DEACTIVATE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_DELETE_RECORD_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *const WINBIO_IDENTITY, subfactor: u8) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_DETACH_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_ERASE_DATABASE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, databaseid: *const ::windows::core::GUID, filepath: ::windows::core::PCWSTR, connectstring: ::windows::core::PCWSTR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_FIRST_RECORD_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_GET_CURRENT_RECORD_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, recordcontents: *mut WINBIO_STORAGE_RECORD) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_GET_DATABASE_SIZE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, availablerecordcount: *mut usize, totalrecordcount: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_GET_DATA_FORMAT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, format: *mut ::windows::core::GUID, version: *mut WINBIO_VERSION) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_GET_RECORD_COUNT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, recordcount: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_NEXT_RECORD_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_NOTIFY_DATABASE_CHANGE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, recordsadded: super::super::Foundation::BOOLEAN) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_NOTIFY_POWER_CHANGE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, powereventtype: u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_OPEN_DATABASE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, databaseid: *const ::windows::core::GUID, filepath: ::windows::core::PCWSTR, connectstring: ::windows::core::PCWSTR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_PIPELINE_CLEANUP_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_PIPELINE_INIT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_QUERY_BY_CONTENT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, subfactor: u8, indexvector: *const u32, indexelementcount: usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_QUERY_BY_SUBJECT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *const WINBIO_IDENTITY, subfactor: u8) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_QUERY_EXTENDED_INFO_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, storageinfo: *mut WINBIO_EXTENDED_STORAGE_INFO, storageinfosize: usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_RESERVED_1_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *const WINBIO_PIPELINE, identity: *const WINBIO_IDENTITY, reserved1: *const u64, reserved2: *const u64) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_RESERVED_2_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *const WINBIO_PIPELINE, identity: *const WINBIO_IDENTITY) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_UPDATE_RECORD_BEGIN_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *const WINBIO_IDENTITY, subfactor: u8, recordcontents: *mut WINBIO_STORAGE_RECORD) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_UPDATE_RECORD_COMMIT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, recordcontents: *const WINBIO_STORAGE_RECORD) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PWINBIO_ASYNC_COMPLETION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(asyncresult: *const WINBIO_ASYNC_RESULT) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub type PWINBIO_CAPTURE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(capturecallbackcontext: *const ::core::ffi::c_void, operationstatus: ::windows::core::HRESULT, unitid: u32, sample: *const WINBIO_BIR, samplesize: usize, rejectdetail: u32) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub type PWINBIO_ENROLL_CAPTURE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(enrollcallbackcontext: *const ::core::ffi::c_void, operationstatus: ::windows::core::HRESULT, rejectdetail: u32) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub type PWINBIO_EVENT_CALLBACK = ::core::option::Option<unsafe extern "system" fn(eventcallbackcontext: *const ::core::ffi::c_void, operationstatus: ::windows::core::HRESULT, event: *const WINBIO_EVENT) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub type PWINBIO_IDENTIFY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(identifycallbackcontext: *const ::core::ffi::c_void, operationstatus: ::windows::core::HRESULT, unitid: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, rejectdetail: u32) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`*"]
pub type PWINBIO_LOCATE_SENSOR_CALLBACK = ::core::option::Option<unsafe extern "system" fn(locatecallbackcontext: *const ::core::ffi::c_void, operationstatus: ::windows::core::HRESULT, unitid: u32) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PWINBIO_QUERY_ENGINE_INTERFACE_FN = ::core::option::Option<unsafe extern "system" fn(engineinterface: *mut *mut WINBIO_ENGINE_INTERFACE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PWINBIO_QUERY_SENSOR_INTERFACE_FN = ::core::option::Option<unsafe extern "system" fn(sensorinterface: *mut *mut WINBIO_SENSOR_INTERFACE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PWINBIO_QUERY_STORAGE_INTERFACE_FN = ::core::option::Option<unsafe extern "system" fn(storageinterface: *mut *mut WINBIO_STORAGE_INTERFACE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_BiometricFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PWINBIO_VERIFY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(verifycallbackcontext: *const ::core::ffi::c_void, operationstatus: ::windows::core::HRESULT, unitid: u32, r#match: super::super::Foundation::BOOLEAN, rejectdetail: u32) -> ()>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
