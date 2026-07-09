windows_link::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoActivateInstance(activatableclassid : windows_sys::core::HSTRING, instance : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoGetActivationFactory(activatableclassid : windows_sys::core::HSTRING, iid : *const windows_sys::core::GUID, factory : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoGetApartmentIdentifier(apartmentidentifier : *mut u64) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoInitialize(inittype : RO_INIT_TYPE) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_activation")]
windows_link::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoRegisterActivationFactories(activatableclassids : *const windows_sys::core::HSTRING, activationfactorycallbacks : *const PFNGETACTIVATIONFACTORY, count : u32, cookie : *mut RO_REGISTRATION_COOKIE) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidl")]
windows_link::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoRegisterForApartmentShutdown(callbackobject : *mut core::ffi::c_void, apartmentidentifier : *mut u64, regcookie : *mut APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoRevokeActivationFactories(cookie : *const _RO_REGISTRATION_COOKIE));
windows_link::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoUninitialize());
windows_link::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoUnregisterForApartmentShutdown(regcookie : APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> windows_sys::core::HRESULT);
pub type APARTMENT_SHUTDOWN_REGISTRATION_COOKIE = *mut core::ffi::c_void;
#[cfg(feature = "Win32_activation")]
pub type PFNGETACTIVATIONFACTORY = Option<unsafe extern "system" fn(param0: windows_sys::core::HSTRING, param1: *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
pub const RO_INIT_MULTITHREADED: RO_INIT_TYPE = 1;
pub const RO_INIT_SINGLETHREADED: RO_INIT_TYPE = 0;
pub type RO_INIT_TYPE = i32;
pub type RO_REGISTRATION_COOKIE = *mut _RO_REGISTRATION_COOKIE;
#[repr(C, align(1))]
#[derive(Clone, Copy, Default)]
pub struct _RO_REGISTRATION_COOKIE(pub u8);
