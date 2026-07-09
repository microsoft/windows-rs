#[inline]
pub unsafe fn RoActivateInstance(activatableclassid: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoActivateInstance(activatableclassid : *mut core::ffi::c_void, instance : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RoActivateInstance(core::mem::transmute_copy(activatableclassid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn RoGetActivationFactory<T>(activatableclassid: &windows_core::HSTRING) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoGetActivationFactory(activatableclassid : *mut core::ffi::c_void, iid : *const windows_core::GUID, factory : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { RoGetActivationFactory(core::mem::transmute_copy(activatableclassid), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn RoGetApartmentIdentifier() -> windows_core::Result<u64> {
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoGetApartmentIdentifier(apartmentidentifier : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RoGetApartmentIdentifier(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn RoInitialize(inittype: RO_INIT_TYPE) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoInitialize(inittype : RO_INIT_TYPE) -> windows_core::HRESULT);
    unsafe { RoInitialize(inittype) }
}
#[cfg(feature = "Win32_activation")]
#[inline]
pub unsafe fn RoRegisterActivationFactories(activatableclassids: *const windows_core::HSTRING, activationfactorycallbacks: *const PFNGETACTIVATIONFACTORY, count: u32) -> windows_core::Result<RO_REGISTRATION_COOKIE> {
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoRegisterActivationFactories(activatableclassids : *const *mut core::ffi::c_void, activationfactorycallbacks : *const PFNGETACTIVATIONFACTORY, count : u32, cookie : *mut RO_REGISTRATION_COOKIE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RoRegisterActivationFactories(core::mem::transmute(activatableclassids), activationfactorycallbacks, count, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_objidl")]
#[inline]
pub unsafe fn RoRegisterForApartmentShutdown<P0>(callbackobject: P0, apartmentidentifier: *mut u64, regcookie: *mut APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::objidl::IApartmentShutdown>,
{
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoRegisterForApartmentShutdown(callbackobject : *mut core::ffi::c_void, apartmentidentifier : *mut u64, regcookie : *mut APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> windows_core::HRESULT);
    unsafe { RoRegisterForApartmentShutdown(callbackobject.param().abi(), apartmentidentifier as _, regcookie as _) }
}
#[inline]
pub unsafe fn RoRevokeActivationFactories(cookie: *const _RO_REGISTRATION_COOKIE) {
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoRevokeActivationFactories(cookie : *const _RO_REGISTRATION_COOKIE));
    unsafe { RoRevokeActivationFactories(cookie) }
}
#[inline]
pub unsafe fn RoUninitialize() {
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoUninitialize());
    unsafe { RoUninitialize() }
}
#[inline]
pub unsafe fn RoUnregisterForApartmentShutdown(regcookie: APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoUnregisterForApartmentShutdown(regcookie : APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> windows_core::HRESULT);
    unsafe { RoUnregisterForApartmentShutdown(regcookie) }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct APARTMENT_SHUTDOWN_REGISTRATION_COOKIE(pub *mut core::ffi::c_void);
impl APARTMENT_SHUTDOWN_REGISTRATION_COOKIE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for APARTMENT_SHUTDOWN_REGISTRATION_COOKIE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_activation")]
pub type PFNGETACTIVATIONFACTORY = Option<unsafe extern "system" fn(param0: windows_core::Ref<windows_core::HSTRING>, param1: windows_core::OutRef<super::activation::IActivationFactory>) -> windows_core::HRESULT>;
pub const RO_INIT_MULTITHREADED: RO_INIT_TYPE = 1;
pub const RO_INIT_SINGLETHREADED: RO_INIT_TYPE = 0;
pub type RO_INIT_TYPE = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RO_REGISTRATION_COOKIE(pub *mut _RO_REGISTRATION_COOKIE);
impl RO_REGISTRATION_COOKIE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for RO_REGISTRATION_COOKIE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, align(1))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _RO_REGISTRATION_COOKIE(pub u8);
