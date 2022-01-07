#[cfg(feature = "implement_exclusive")]
pub trait IClassicAppManagerStaticsImpl: Sized {
    fn FindInstalledApp(&self, appuninstallkey: &::windows::core::HSTRING) -> ::windows::core::Result<InstalledClassicAppInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IClassicAppManagerStatics {
    const NAME: &'static str = "Windows.Management.Deployment.Preview.IClassicAppManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IClassicAppManagerStaticsVtbl {
    pub const fn new<Impl: IClassicAppManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IClassicAppManagerStaticsVtbl {
        unsafe extern "system" fn FindInstalledApp<Impl: IClassicAppManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appuninstallkey: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindInstalledApp(&*(&appuninstallkey as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IClassicAppManagerStatics>, base.5, FindInstalledApp::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInstalledClassicAppInfoImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInstalledClassicAppInfo {
    const NAME: &'static str = "Windows.Management.Deployment.Preview.IInstalledClassicAppInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IInstalledClassicAppInfoVtbl {
    pub const fn new<Impl: IInstalledClassicAppInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInstalledClassicAppInfoVtbl {
        unsafe extern "system" fn DisplayName<Impl: IInstalledClassicAppInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayVersion<Impl: IInstalledClassicAppInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInstalledClassicAppInfo>, base.5, DisplayName::<Impl, OFFSET>, DisplayVersion::<Impl, OFFSET>)
    }
}
