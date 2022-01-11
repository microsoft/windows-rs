#[cfg(feature = "Win32_Foundation")]
pub trait IIsolatedAppLauncherImpl: Sized {
    fn Launch();
}
#[cfg(feature = "Win32_Foundation")]
impl IIsolatedAppLauncherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIsolatedAppLauncherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIsolatedAppLauncherVtbl {
        unsafe extern "system" fn Launch<Impl: IIsolatedAppLauncherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appusermodelid: super::super::Foundation::PWSTR, arguments: super::super::Foundation::PWSTR, telemetryparameters: *const IsolatedAppLauncherTelemetryParameters) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Launch: Launch::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIsolatedAppLauncher as ::windows::core::Interface>::IID
    }
}
