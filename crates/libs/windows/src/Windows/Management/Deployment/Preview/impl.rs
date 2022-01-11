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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClassicAppManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClassicAppManagerStaticsVtbl {
        unsafe extern "system" fn FindInstalledApp<Impl: IClassicAppManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appuninstallkey: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindInstalledApp(&*(&appuninstallkey as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IClassicAppManagerStatics>, ::windows::core::GetTrustLevel, FindInstalledApp::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClassicAppManagerStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInstalledClassicAppInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInstalledClassicAppInfoVtbl {
        unsafe extern "system" fn DisplayName<Impl: IInstalledClassicAppInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayVersion<Impl: IInstalledClassicAppInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInstalledClassicAppInfo>, ::windows::core::GetTrustLevel, DisplayName::<Impl, IMPL_OFFSET>, DisplayVersion::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInstalledClassicAppInfo as ::windows::core::Interface>::IID
    }
}
