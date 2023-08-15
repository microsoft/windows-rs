#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IIsolatedAppLauncher_Impl: Sized {
    fn Launch(&self, appusermodelid: &::windows_core::PCWSTR, arguments: &::windows_core::PCWSTR, telemetryparameters: *const IsolatedAppLauncherTelemetryParameters) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IIsolatedAppLauncher {}
#[cfg(feature = "Win32_Foundation")]
impl IIsolatedAppLauncher_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIsolatedAppLauncher_Impl, const OFFSET: isize>() -> IIsolatedAppLauncher_Vtbl {
        unsafe extern "system" fn Launch<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIsolatedAppLauncher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appusermodelid: ::windows_core::PCWSTR, arguments: ::windows_core::PCWSTR, telemetryparameters: *const IsolatedAppLauncherTelemetryParameters) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Launch(::core::mem::transmute(&appusermodelid), ::core::mem::transmute(&arguments), ::core::mem::transmute_copy(&telemetryparameters)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Launch: Launch::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IIsolatedAppLauncher as ::windows_core::ComInterface>::IID
    }
}
