windows_link::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn GetRestrictedErrorInfo(pprestrictederrorinfo : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn IsErrorPropagationEnabled() -> windows_sys::core::BOOL);
#[cfg(all(feature = "guiddef", feature = "minwindef"))]
windows_link::link!("rometadata.dll" "system" fn MetaDataGetDispenser(rclsid : *const super::IID, riid : *const super::IID, ppv : *mut super::LPVOID) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoActivateInstance(activatableclassid : windows_sys::core::HSTRING, instance : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoCaptureErrorContext(hr : windows_sys::core::HRESULT) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoClearError());
windows_link::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoFailFastWithErrorContext(hrerror : windows_sys::core::HRESULT));
windows_link::link!("api-ms-win-core-winrt-roparameterizediid-l1-1-0.dll" "system" fn RoFreeParameterizedTypeExtra(extra : ROPARAMIIDHANDLE));
#[cfg(feature = "guiddef")]
windows_link::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoGetActivationFactory(activatableclassid : windows_sys::core::HSTRING, iid : *const super::IID, factory : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoGetApartmentIdentifier(apartmentidentifier : *mut super::UINT64) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("api-ms-win-core-winrt-robuffer-l1-1-0.dll" "system" fn RoGetBufferMarshaler(buffermarshaler : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoGetErrorReportingFlags(pflags : *mut super::UINT32) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoGetMatchingRestrictedErrorInfo(hrin : windows_sys::core::HRESULT, pprestrictederrorinfo : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-core-winrt-registration-l1-1-0.dll" "system" fn RoGetServerActivatableClasses(servername : windows_sys::core::HSTRING, activatableclassids : *mut *mut windows_sys::core::HSTRING, count : *mut super::DWORD) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoInitialize(inittype : RO_INIT_TYPE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoInspectCapturedStackBackTrace(targeterrorinfoaddress : super::UINT_PTR, machine : super::USHORT, readmemorycallback : PINSPECT_MEMORY_CALLBACK, context : super::PVOID, framecount : *mut super::UINT32, targetbacktraceaddress : *mut super::UINT_PTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoInspectThreadErrorInfo(targettebaddress : super::UINT_PTR, machine : super::USHORT, readmemorycallback : PINSPECT_MEMORY_CALLBACK, context : super::PVOID, targeterrorinfoaddress : *mut super::UINT_PTR) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoOriginateError(error : windows_sys::core::HRESULT, message : windows_sys::core::HSTRING) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoOriginateErrorW(error : windows_sys::core::HRESULT, cchmax : super::UINT, message : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoOriginateLanguageException(error : windows_sys::core::HRESULT, message : windows_sys::core::HSTRING, languageexception : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("api-ms-win-core-winrt-roparameterizediid-l1-1-0.dll" "system" fn RoParameterizedTypeExtraGetTypeSignature(extra : ROPARAMIIDHANDLE) -> windows_sys::core::PCSTR);
#[cfg(all(feature = "activation", feature = "shared"))]
windows_link::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoRegisterActivationFactories(activatableclassids : *const windows_sys::core::HSTRING, activationfactorycallbacks : *const PFNGETACTIVATIONFACTORY, count : super::UINT32, cookie : *mut RO_REGISTRATION_COOKIE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "shared"))]
windows_link::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoRegisterForApartmentShutdown(callbackobject : *mut core::ffi::c_void, apartmentidentifier : *mut super::UINT64, regcookie : *mut APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoReportFailedDelegate(punkdelegate : *mut core::ffi::c_void, prestrictederrorinfo : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoReportUnhandledError(prestrictederrorinfo : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoResolveRestrictedErrorInfoReference(reference : windows_sys::core::PCWSTR, pprestrictederrorinfo : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoRevokeActivationFactories(cookie : RO_REGISTRATION_COOKIE));
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoSetErrorReportingFlags(flags : super::UINT32) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoTransformError(olderror : windows_sys::core::HRESULT, newerror : windows_sys::core::HRESULT, message : windows_sys::core::HSTRING) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoTransformErrorW(olderror : windows_sys::core::HRESULT, newerror : windows_sys::core::HRESULT, cchmax : super::UINT, message : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoUninitialize());
windows_link::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoUnregisterForApartmentShutdown(regcookie : APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn SetRestrictedErrorInfo(prestrictederrorinfo : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
pub type APARTMENT_SHUTDOWN_REGISTRATION_COOKIE = *mut APARTMENT_SHUTDOWN_REGISTRATION_COOKIE__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct APARTMENT_SHUTDOWN_REGISTRATION_COOKIE__ {
    pub unused: i32,
}
#[cfg(feature = "shared")]
pub const EXCEPTION_RO_ORIGINATEERROR: super::DWORD = 1074266625;
#[cfg(feature = "shared")]
pub const EXCEPTION_RO_TRANSFORMERROR: super::DWORD = 1074266626;
pub const MAX_ERROR_MESSAGE_CHARS: i32 = 512;
#[cfg(feature = "activation")]
pub type PFNGETACTIVATIONFACTORY = Option<unsafe extern "system" fn(param0: windows_sys::core::HSTRING, param1: *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
#[cfg(feature = "shared")]
pub type PINSPECT_MEMORY_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, readaddress: super::UINT_PTR, length: super::UINT32, buffer: *mut super::BYTE) -> windows_sys::core::HRESULT>;
pub type ROPARAMIIDHANDLE = *mut ROPARAMIIDHANDLE__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ROPARAMIIDHANDLE__ {
    pub unused: i32,
}
pub type RO_ERROR_REPORTING_FLAGS = i32;
pub const RO_ERROR_REPORTING_FORCEEXCEPTIONS: RO_ERROR_REPORTING_FLAGS = 2;
pub const RO_ERROR_REPORTING_NONE: RO_ERROR_REPORTING_FLAGS = 0;
pub const RO_ERROR_REPORTING_SUPPRESSEXCEPTIONS: RO_ERROR_REPORTING_FLAGS = 1;
pub const RO_ERROR_REPORTING_SUPPRESSSETERRORINFO: RO_ERROR_REPORTING_FLAGS = 8;
pub const RO_ERROR_REPORTING_USESETERRORINFO: RO_ERROR_REPORTING_FLAGS = 4;
pub const RO_INIT_MULTITHREADED: RO_INIT_TYPE = 1;
pub const RO_INIT_SINGLETHREADED: RO_INIT_TYPE = 0;
pub type RO_INIT_TYPE = i32;
pub type RO_REGISTRATION_COOKIE = *mut _RO_REGISTRATION_COOKIE;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _RO_REGISTRATION_COOKIE(pub u8);
