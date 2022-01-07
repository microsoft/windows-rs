#[cfg(feature = "implement_exclusive")]
pub trait IPreviewBuildsManagerImpl: Sized {
    fn ArePreviewBuildsAllowed(&self) -> ::windows::core::Result<bool>;
    fn SetArePreviewBuildsAllowed(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetCurrentState(&self) -> ::windows::core::Result<PreviewBuildsState>;
    fn SyncAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPreviewBuildsManager {
    const NAME: &'static str = "Windows.Management.Update.IPreviewBuildsManager";
}
#[cfg(feature = "implement_exclusive")]
impl IPreviewBuildsManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPreviewBuildsManagerImpl, const OFFSET: isize>() -> IPreviewBuildsManagerVtbl {
        unsafe extern "system" fn ArePreviewBuildsAllowed<Impl: IPreviewBuildsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArePreviewBuildsAllowed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetArePreviewBuildsAllowed<Impl: IPreviewBuildsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetArePreviewBuildsAllowed(value).into()
        }
        unsafe extern "system" fn GetCurrentState<Impl: IPreviewBuildsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SyncAsync<Impl: IPreviewBuildsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SyncAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPreviewBuildsManager>, ::windows::core::GetTrustLevel, ArePreviewBuildsAllowed::<Impl, OFFSET>, SetArePreviewBuildsAllowed::<Impl, OFFSET>, GetCurrentState::<Impl, OFFSET>, SyncAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPreviewBuildsManagerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<PreviewBuildsManager>;
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPreviewBuildsManagerStatics {
    const NAME: &'static str = "Windows.Management.Update.IPreviewBuildsManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPreviewBuildsManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPreviewBuildsManagerStaticsImpl, const OFFSET: isize>() -> IPreviewBuildsManagerStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IPreviewBuildsManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSupported<Impl: IPreviewBuildsManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPreviewBuildsManagerStatics>, ::windows::core::GetTrustLevel, GetDefault::<Impl, OFFSET>, IsSupported::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPreviewBuildsStateImpl: Sized {
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPreviewBuildsState {
    const NAME: &'static str = "Windows.Management.Update.IPreviewBuildsState";
}
#[cfg(feature = "implement_exclusive")]
impl IPreviewBuildsStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPreviewBuildsStateImpl, const OFFSET: isize>() -> IPreviewBuildsStateVtbl {
        unsafe extern "system" fn Properties<Impl: IPreviewBuildsStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPreviewBuildsState>, ::windows::core::GetTrustLevel, Properties::<Impl, OFFSET>)
    }
}
