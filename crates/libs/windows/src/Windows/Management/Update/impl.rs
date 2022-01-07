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
    pub const fn new<Impl: IPreviewBuildsManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPreviewBuildsManagerVtbl {
        unsafe extern "system" fn ArePreviewBuildsAllowed<Impl: IPreviewBuildsManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ArePreviewBuildsAllowed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetArePreviewBuildsAllowed<Impl: IPreviewBuildsManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetArePreviewBuildsAllowed(value).into()
        }
        unsafe extern "system" fn GetCurrentState<Impl: IPreviewBuildsManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrentState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SyncAsync<Impl: IPreviewBuildsManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SyncAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPreviewBuildsManager>, base.5, ArePreviewBuildsAllowed::<Impl, OFFSET>, SetArePreviewBuildsAllowed::<Impl, OFFSET>, GetCurrentState::<Impl, OFFSET>, SyncAsync::<Impl, OFFSET>)
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
    pub const fn new<Impl: IPreviewBuildsManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPreviewBuildsManagerStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IPreviewBuildsManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSupported<Impl: IPreviewBuildsManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPreviewBuildsManagerStatics>, base.5, GetDefault::<Impl, OFFSET>, IsSupported::<Impl, OFFSET>)
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
    pub const fn new<Impl: IPreviewBuildsStateImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPreviewBuildsStateVtbl {
        unsafe extern "system" fn Properties<Impl: IPreviewBuildsStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPreviewBuildsState>, base.5, Properties::<Impl, OFFSET>)
    }
}
