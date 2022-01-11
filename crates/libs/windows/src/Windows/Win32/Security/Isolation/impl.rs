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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Launch::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIsolatedAppLauncher as ::windows::core::Interface>::IID
    }
}
