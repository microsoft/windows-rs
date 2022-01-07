pub trait IIsolatedAppLauncherImpl: Sized {
    fn Launch();
}
impl ::windows::core::RuntimeName for IIsolatedAppLauncher {
    const NAME: &'static str = "Windows.Win32.Security.Isolation.IIsolatedAppLauncher";
}
impl IIsolatedAppLauncherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIsolatedAppLauncherImpl, const OFFSET: isize>() -> IIsolatedAppLauncherVtbl {
        unsafe extern "system" fn Launch<Impl: IIsolatedAppLauncherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appusermodelid: super::super::Foundation::PWSTR, arguments: super::super::Foundation::PWSTR, telemetryparameters: *const IsolatedAppLauncherTelemetryParameters) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Launch(
                &*(&appusermodelid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&arguments as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&telemetryparameters as *const <IsolatedAppLauncherTelemetryParameters as ::windows::core::Abi>::Abi as *const <IsolatedAppLauncherTelemetryParameters as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IIsolatedAppLauncher>, ::windows::core::GetTrustLevel, Launch::<Impl, OFFSET>)
    }
}
