#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPreviewBuildsManagerImpl: Sized {
    fn ArePreviewBuildsAllowed(&mut self) -> ::windows::core::Result<bool>;
    fn SetArePreviewBuildsAllowed(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn GetCurrentState(&mut self) -> ::windows::core::Result<PreviewBuildsState>;
    fn SyncAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPreviewBuildsManager {
    const NAME: &'static str = "Windows.Management.Update.IPreviewBuildsManager";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPreviewBuildsManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPreviewBuildsManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPreviewBuildsManagerVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPreviewBuildsManager, BASE_OFFSET>(),
            ArePreviewBuildsAllowed: ArePreviewBuildsAllowed::<Impl, IMPL_OFFSET>,
            SetArePreviewBuildsAllowed: SetArePreviewBuildsAllowed::<Impl, IMPL_OFFSET>,
            GetCurrentState: GetCurrentState::<Impl, IMPL_OFFSET>,
            SyncAsync: SyncAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPreviewBuildsManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPreviewBuildsManagerStaticsImpl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<PreviewBuildsManager>;
    fn IsSupported(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPreviewBuildsManagerStatics {
    const NAME: &'static str = "Windows.Management.Update.IPreviewBuildsManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPreviewBuildsManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPreviewBuildsManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPreviewBuildsManagerStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPreviewBuildsManagerStatics, BASE_OFFSET>(),
            GetDefault: GetDefault::<Impl, IMPL_OFFSET>,
            IsSupported: IsSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPreviewBuildsManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPreviewBuildsStateImpl: Sized {
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPreviewBuildsState {
    const NAME: &'static str = "Windows.Management.Update.IPreviewBuildsState";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPreviewBuildsStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPreviewBuildsStateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPreviewBuildsStateVtbl {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPreviewBuildsState, BASE_OFFSET>(), Properties: Properties::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPreviewBuildsState as ::windows::core::Interface>::IID
    }
}
