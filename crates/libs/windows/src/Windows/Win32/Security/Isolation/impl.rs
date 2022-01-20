#[cfg(feature = "Win32_Foundation")]
pub trait IIsolatedAppLauncher_Impl: Sized {
    fn Launch(&mut self, appusermodelid: super::super::Foundation::PWSTR, arguments: super::super::Foundation::PWSTR, telemetryparameters: *const IsolatedAppLauncherTelemetryParameters) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IIsolatedAppLauncher_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIsolatedAppLauncher_Impl, const OFFSET: isize>() -> IIsolatedAppLauncher_Vtbl {
        unsafe extern "system" fn Launch<Identity: ::windows::core::IUnknownImpl, Impl: IIsolatedAppLauncher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appusermodelid: super::super::Foundation::PWSTR, arguments: super::super::Foundation::PWSTR, telemetryparameters: *const IsolatedAppLauncherTelemetryParameters) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Launch(::core::mem::transmute_copy(&appusermodelid), ::core::mem::transmute_copy(&arguments), ::core::mem::transmute_copy(&telemetryparameters)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Launch: Launch::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIsolatedAppLauncher as ::windows::core::Interface>::IID
    }
}
