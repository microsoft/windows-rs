#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBarcodeScannerDisableScannerRequestImpl: Sized {
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarcodeScannerDisableScannerRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.IBarcodeScannerDisableScannerRequest";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBarcodeScannerDisableScannerRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerDisableScannerRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerDisableScannerRequestVtbl {
        unsafe extern "system" fn ReportCompletedAsync<Impl: IBarcodeScannerDisableScannerRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportCompletedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedAsync<Impl: IBarcodeScannerDisableScannerRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarcodeScannerDisableScannerRequest>, ::windows::core::GetTrustLevel, ReportCompletedAsync::<Impl, IMPL_OFFSET>, ReportFailedAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerDisableScannerRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBarcodeScannerDisableScannerRequest2Impl: Sized {
    fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedWithFailedReasonAndDescriptionAsync(&self, reason: i32, failedreasondescription: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarcodeScannerDisableScannerRequest2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.IBarcodeScannerDisableScannerRequest2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBarcodeScannerDisableScannerRequest2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerDisableScannerRequest2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerDisableScannerRequest2Vtbl {
        unsafe extern "system" fn ReportFailedWithFailedReasonAsync<Impl: IBarcodeScannerDisableScannerRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedWithFailedReasonAsync(reason) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedWithFailedReasonAndDescriptionAsync<Impl: IBarcodeScannerDisableScannerRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: i32, failedreasondescription: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedWithFailedReasonAndDescriptionAsync(reason, &*(&failedreasondescription as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarcodeScannerDisableScannerRequest2>, ::windows::core::GetTrustLevel, ReportFailedWithFailedReasonAsync::<Impl, IMPL_OFFSET>, ReportFailedWithFailedReasonAndDescriptionAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerDisableScannerRequest2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBarcodeScannerDisableScannerRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<BarcodeScannerDisableScannerRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarcodeScannerDisableScannerRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.IBarcodeScannerDisableScannerRequestEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBarcodeScannerDisableScannerRequestEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerDisableScannerRequestEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerDisableScannerRequestEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IBarcodeScannerDisableScannerRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IBarcodeScannerDisableScannerRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarcodeScannerDisableScannerRequestEventArgs>, ::windows::core::GetTrustLevel, Request::<Impl, IMPL_OFFSET>, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerDisableScannerRequestEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBarcodeScannerEnableScannerRequestImpl: Sized {
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarcodeScannerEnableScannerRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.IBarcodeScannerEnableScannerRequest";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBarcodeScannerEnableScannerRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerEnableScannerRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerEnableScannerRequestVtbl {
        unsafe extern "system" fn ReportCompletedAsync<Impl: IBarcodeScannerEnableScannerRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportCompletedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedAsync<Impl: IBarcodeScannerEnableScannerRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarcodeScannerEnableScannerRequest>, ::windows::core::GetTrustLevel, ReportCompletedAsync::<Impl, IMPL_OFFSET>, ReportFailedAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerEnableScannerRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBarcodeScannerEnableScannerRequest2Impl: Sized {
    fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedWithFailedReasonAndDescriptionAsync(&self, reason: i32, failedreasondescription: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarcodeScannerEnableScannerRequest2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.IBarcodeScannerEnableScannerRequest2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBarcodeScannerEnableScannerRequest2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerEnableScannerRequest2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerEnableScannerRequest2Vtbl {
        unsafe extern "system" fn ReportFailedWithFailedReasonAsync<Impl: IBarcodeScannerEnableScannerRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedWithFailedReasonAsync(reason) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedWithFailedReasonAndDescriptionAsync<Impl: IBarcodeScannerEnableScannerRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: i32, failedreasondescription: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedWithFailedReasonAndDescriptionAsync(reason, &*(&failedreasondescription as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarcodeScannerEnableScannerRequest2>, ::windows::core::GetTrustLevel, ReportFailedWithFailedReasonAsync::<Impl, IMPL_OFFSET>, ReportFailedWithFailedReasonAndDescriptionAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerEnableScannerRequest2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBarcodeScannerEnableScannerRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<BarcodeScannerEnableScannerRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarcodeScannerEnableScannerRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.IBarcodeScannerEnableScannerRequestEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBarcodeScannerEnableScannerRequestEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerEnableScannerRequestEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerEnableScannerRequestEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IBarcodeScannerEnableScannerRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IBarcodeScannerEnableScannerRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarcodeScannerEnableScannerRequestEventArgs>, ::windows::core::GetTrustLevel, Request::<Impl, IMPL_OFFSET>, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerEnableScannerRequestEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBarcodeScannerFrameReaderImpl: Sized {
    fn StartAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
    fn StopAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn TryAcquireLatestFrameAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<BarcodeScannerVideoFrame>>;
    fn Connection(&self) -> ::windows::core::Result<BarcodeScannerProviderConnection>;
    fn FrameArrived(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<BarcodeScannerFrameReader, BarcodeScannerFrameReaderFrameArrivedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameArrived(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarcodeScannerFrameReader {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.IBarcodeScannerFrameReader";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBarcodeScannerFrameReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerFrameReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerFrameReaderVtbl {
        unsafe extern "system" fn StartAsync<Impl: IBarcodeScannerFrameReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopAsync<Impl: IBarcodeScannerFrameReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryAcquireLatestFrameAsync<Impl: IBarcodeScannerFrameReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryAcquireLatestFrameAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connection<Impl: IBarcodeScannerFrameReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Connection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FrameArrived<Impl: IBarcodeScannerFrameReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameArrived(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<BarcodeScannerFrameReader, BarcodeScannerFrameReaderFrameArrivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<BarcodeScannerFrameReader, BarcodeScannerFrameReaderFrameArrivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFrameArrived<Impl: IBarcodeScannerFrameReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFrameArrived(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IBarcodeScannerFrameReader>,
            ::windows::core::GetTrustLevel,
            StartAsync::<Impl, IMPL_OFFSET>,
            StopAsync::<Impl, IMPL_OFFSET>,
            TryAcquireLatestFrameAsync::<Impl, IMPL_OFFSET>,
            Connection::<Impl, IMPL_OFFSET>,
            FrameArrived::<Impl, IMPL_OFFSET>,
            RemoveFrameArrived::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerFrameReader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBarcodeScannerFrameReaderFrameArrivedEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarcodeScannerFrameReaderFrameArrivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.IBarcodeScannerFrameReaderFrameArrivedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBarcodeScannerFrameReaderFrameArrivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerFrameReaderFrameArrivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerFrameReaderFrameArrivedEventArgsVtbl {
        unsafe extern "system" fn GetDeferral<Impl: IBarcodeScannerFrameReaderFrameArrivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarcodeScannerFrameReaderFrameArrivedEventArgs>, ::windows::core::GetTrustLevel, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerFrameReaderFrameArrivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBarcodeScannerGetSymbologyAttributesRequestImpl: Sized {
    fn Symbology(&self) -> ::windows::core::Result<u32>;
    fn ReportCompletedAsync(&self, attributes: &::core::option::Option<super::BarcodeSymbologyAttributes>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarcodeScannerGetSymbologyAttributesRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.IBarcodeScannerGetSymbologyAttributesRequest";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBarcodeScannerGetSymbologyAttributesRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerGetSymbologyAttributesRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerGetSymbologyAttributesRequestVtbl {
        unsafe extern "system" fn Symbology<Impl: IBarcodeScannerGetSymbologyAttributesRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Symbology() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCompletedAsync<Impl: IBarcodeScannerGetSymbologyAttributesRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportCompletedAsync(&*(&attributes as *const <super::BarcodeSymbologyAttributes as ::windows::core::Abi>::Abi as *const <super::BarcodeSymbologyAttributes as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedAsync<Impl: IBarcodeScannerGetSymbologyAttributesRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarcodeScannerGetSymbologyAttributesRequest>, ::windows::core::GetTrustLevel, Symbology::<Impl, IMPL_OFFSET>, ReportCompletedAsync::<Impl, IMPL_OFFSET>, ReportFailedAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerGetSymbologyAttributesRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBarcodeScannerGetSymbologyAttributesRequest2Impl: Sized {
    fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedWithFailedReasonAndDescriptionAsync(&self, reason: i32, failedreasondescription: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarcodeScannerGetSymbologyAttributesRequest2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.IBarcodeScannerGetSymbologyAttributesRequest2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBarcodeScannerGetSymbologyAttributesRequest2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerGetSymbologyAttributesRequest2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerGetSymbologyAttributesRequest2Vtbl {
        unsafe extern "system" fn ReportFailedWithFailedReasonAsync<Impl: IBarcodeScannerGetSymbologyAttributesRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedWithFailedReasonAsync(reason) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedWithFailedReasonAndDescriptionAsync<Impl: IBarcodeScannerGetSymbologyAttributesRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: i32, failedreasondescription: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedWithFailedReasonAndDescriptionAsync(reason, &*(&failedreasondescription as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarcodeScannerGetSymbologyAttributesRequest2>, ::windows::core::GetTrustLevel, ReportFailedWithFailedReasonAsync::<Impl, IMPL_OFFSET>, ReportFailedWithFailedReasonAndDescriptionAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerGetSymbologyAttributesRequest2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBarcodeScannerGetSymbologyAttributesRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<BarcodeScannerGetSymbologyAttributesRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarcodeScannerGetSymbologyAttributesRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.IBarcodeScannerGetSymbologyAttributesRequestEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBarcodeScannerGetSymbologyAttributesRequestEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerGetSymbologyAttributesRequestEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerGetSymbologyAttributesRequestEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IBarcodeScannerGetSymbologyAttributesRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IBarcodeScannerGetSymbologyAttributesRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarcodeScannerGetSymbologyAttributesRequestEventArgs>, ::windows::core::GetTrustLevel, Request::<Impl, IMPL_OFFSET>, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerGetSymbologyAttributesRequestEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBarcodeScannerHideVideoPreviewRequestImpl: Sized {
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarcodeScannerHideVideoPreviewRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.IBarcodeScannerHideVideoPreviewRequest";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBarcodeScannerHideVideoPreviewRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerHideVideoPreviewRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerHideVideoPreviewRequestVtbl {
        unsafe extern "system" fn ReportCompletedAsync<Impl: IBarcodeScannerHideVideoPreviewRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportCompletedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedAsync<Impl: IBarcodeScannerHideVideoPreviewRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarcodeScannerHideVideoPreviewRequest>, ::windows::core::GetTrustLevel, ReportCompletedAsync::<Impl, IMPL_OFFSET>, ReportFailedAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerHideVideoPreviewRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBarcodeScannerHideVideoPreviewRequest2Impl: Sized {
    fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedWithFailedReasonAndDescriptionAsync(&self, reason: i32, failedreasondescription: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarcodeScannerHideVideoPreviewRequest2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.IBarcodeScannerHideVideoPreviewRequest2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBarcodeScannerHideVideoPreviewRequest2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerHideVideoPreviewRequest2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerHideVideoPreviewRequest2Vtbl {
        unsafe extern "system" fn ReportFailedWithFailedReasonAsync<Impl: IBarcodeScannerHideVideoPreviewRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedWithFailedReasonAsync(reason) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedWithFailedReasonAndDescriptionAsync<Impl: IBarcodeScannerHideVideoPreviewRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: i32, failedreasondescription: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedWithFailedReasonAndDescriptionAsync(reason, &*(&failedreasondescription as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarcodeScannerHideVideoPreviewRequest2>, ::windows::core::GetTrustLevel, ReportFailedWithFailedReasonAsync::<Impl, IMPL_OFFSET>, ReportFailedWithFailedReasonAndDescriptionAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerHideVideoPreviewRequest2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBarcodeScannerHideVideoPreviewRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<BarcodeScannerHideVideoPreviewRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarcodeScannerHideVideoPreviewRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.IBarcodeScannerHideVideoPreviewRequestEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBarcodeScannerHideVideoPreviewRequestEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerHideVideoPreviewRequestEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerHideVideoPreviewRequestEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IBarcodeScannerHideVideoPreviewRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IBarcodeScannerHideVideoPreviewRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarcodeScannerHideVideoPreviewRequestEventArgs>, ::windows::core::GetTrustLevel, Request::<Impl, IMPL_OFFSET>, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerHideVideoPreviewRequestEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IBarcodeScannerProviderConnectionImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SupportedSymbologies(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<u32>>;
    fn CompanyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCompanyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Version(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetVersion(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn ReportScannedDataAsync(&self, report: &::core::option::Option<super::BarcodeScannerReport>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportTriggerStateAsync(&self, state: BarcodeScannerTriggerState) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportErrorAsync(&self, errordata: &::core::option::Option<super::UnifiedPosErrorData>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportErrorAsyncWithScanReport(&self, errordata: &::core::option::Option<super::UnifiedPosErrorData>, isretriable: bool, scanreport: &::core::option::Option<super::BarcodeScannerReport>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn EnableScannerRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerEnableScannerRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnableScannerRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DisableScannerRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerDisableScannerRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDisableScannerRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetActiveSymbologiesRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerSetActiveSymbologiesRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSetActiveSymbologiesRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StartSoftwareTriggerRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerStartSoftwareTriggerRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStartSoftwareTriggerRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StopSoftwareTriggerRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerStopSoftwareTriggerRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStopSoftwareTriggerRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetBarcodeSymbologyAttributesRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerGetSymbologyAttributesRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGetBarcodeSymbologyAttributesRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetBarcodeSymbologyAttributesRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerSetSymbologyAttributesRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSetBarcodeSymbologyAttributesRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn HideVideoPreviewRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerHideVideoPreviewRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveHideVideoPreviewRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarcodeScannerProviderConnection {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.IBarcodeScannerProviderConnection";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IBarcodeScannerProviderConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerProviderConnectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerProviderConnectionVtbl {
        unsafe extern "system" fn Id<Impl: IBarcodeScannerProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoDeviceId<Impl: IBarcodeScannerProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoDeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedSymbologies<Impl: IBarcodeScannerProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedSymbologies() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompanyName<Impl: IBarcodeScannerProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompanyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompanyName<Impl: IBarcodeScannerProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompanyName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Name<Impl: IBarcodeScannerProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IBarcodeScannerProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Version<Impl: IBarcodeScannerProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Version() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVersion<Impl: IBarcodeScannerProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVersion(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Start<Impl: IBarcodeScannerProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn ReportScannedDataAsync<Impl: IBarcodeScannerProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, report: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportScannedDataAsync(&*(&report as *const <super::BarcodeScannerReport as ::windows::core::Abi>::Abi as *const <super::BarcodeScannerReport as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportTriggerStateAsync<Impl: IBarcodeScannerProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: BarcodeScannerTriggerState, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportTriggerStateAsync(state) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportErrorAsync<Impl: IBarcodeScannerProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errordata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportErrorAsync(&*(&errordata as *const <super::UnifiedPosErrorData as ::windows::core::Abi>::Abi as *const <super::UnifiedPosErrorData as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportErrorAsyncWithScanReport<Impl: IBarcodeScannerProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errordata: ::windows::core::RawPtr, isretriable: bool, scanreport: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportErrorAsyncWithScanReport(&*(&errordata as *const <super::UnifiedPosErrorData as ::windows::core::Abi>::Abi as *const <super::UnifiedPosErrorData as ::windows::core::DefaultType>::DefaultType), isretriable, &*(&scanreport as *const <super::BarcodeScannerReport as ::windows::core::Abi>::Abi as *const <super::BarcodeScannerReport as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableScannerRequested<Impl: IBarcodeScannerProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableScannerRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerEnableScannerRequestEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerEnableScannerRequestEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnableScannerRequested<Impl: IBarcodeScannerProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEnableScannerRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisableScannerRequested<Impl: IBarcodeScannerProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisableScannerRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerDisableScannerRequestEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerDisableScannerRequestEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDisableScannerRequested<Impl: IBarcodeScannerProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDisableScannerRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetActiveSymbologiesRequested<Impl: IBarcodeScannerProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetActiveSymbologiesRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerSetActiveSymbologiesRequestEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerSetActiveSymbologiesRequestEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSetActiveSymbologiesRequested<Impl: IBarcodeScannerProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSetActiveSymbologiesRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartSoftwareTriggerRequested<Impl: IBarcodeScannerProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartSoftwareTriggerRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerStartSoftwareTriggerRequestEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerStartSoftwareTriggerRequestEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStartSoftwareTriggerRequested<Impl: IBarcodeScannerProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStartSoftwareTriggerRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StopSoftwareTriggerRequested<Impl: IBarcodeScannerProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopSoftwareTriggerRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerStopSoftwareTriggerRequestEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerStopSoftwareTriggerRequestEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStopSoftwareTriggerRequested<Impl: IBarcodeScannerProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStopSoftwareTriggerRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetBarcodeSymbologyAttributesRequested<Impl: IBarcodeScannerProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBarcodeSymbologyAttributesRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerGetSymbologyAttributesRequestEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerGetSymbologyAttributesRequestEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGetBarcodeSymbologyAttributesRequested<Impl: IBarcodeScannerProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGetBarcodeSymbologyAttributesRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetBarcodeSymbologyAttributesRequested<Impl: IBarcodeScannerProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBarcodeSymbologyAttributesRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerSetSymbologyAttributesRequestEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerSetSymbologyAttributesRequestEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSetBarcodeSymbologyAttributesRequested<Impl: IBarcodeScannerProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSetBarcodeSymbologyAttributesRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HideVideoPreviewRequested<Impl: IBarcodeScannerProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HideVideoPreviewRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerHideVideoPreviewRequestEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerHideVideoPreviewRequestEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHideVideoPreviewRequested<Impl: IBarcodeScannerProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveHideVideoPreviewRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IBarcodeScannerProviderConnection>,
            ::windows::core::GetTrustLevel,
            Id::<Impl, IMPL_OFFSET>,
            VideoDeviceId::<Impl, IMPL_OFFSET>,
            SupportedSymbologies::<Impl, IMPL_OFFSET>,
            CompanyName::<Impl, IMPL_OFFSET>,
            SetCompanyName::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            Version::<Impl, IMPL_OFFSET>,
            SetVersion::<Impl, IMPL_OFFSET>,
            Start::<Impl, IMPL_OFFSET>,
            ReportScannedDataAsync::<Impl, IMPL_OFFSET>,
            ReportTriggerStateAsync::<Impl, IMPL_OFFSET>,
            ReportErrorAsync::<Impl, IMPL_OFFSET>,
            ReportErrorAsyncWithScanReport::<Impl, IMPL_OFFSET>,
            EnableScannerRequested::<Impl, IMPL_OFFSET>,
            RemoveEnableScannerRequested::<Impl, IMPL_OFFSET>,
            DisableScannerRequested::<Impl, IMPL_OFFSET>,
            RemoveDisableScannerRequested::<Impl, IMPL_OFFSET>,
            SetActiveSymbologiesRequested::<Impl, IMPL_OFFSET>,
            RemoveSetActiveSymbologiesRequested::<Impl, IMPL_OFFSET>,
            StartSoftwareTriggerRequested::<Impl, IMPL_OFFSET>,
            RemoveStartSoftwareTriggerRequested::<Impl, IMPL_OFFSET>,
            StopSoftwareTriggerRequested::<Impl, IMPL_OFFSET>,
            RemoveStopSoftwareTriggerRequested::<Impl, IMPL_OFFSET>,
            GetBarcodeSymbologyAttributesRequested::<Impl, IMPL_OFFSET>,
            RemoveGetBarcodeSymbologyAttributesRequested::<Impl, IMPL_OFFSET>,
            SetBarcodeSymbologyAttributesRequested::<Impl, IMPL_OFFSET>,
            RemoveSetBarcodeSymbologyAttributesRequested::<Impl, IMPL_OFFSET>,
            HideVideoPreviewRequested::<Impl, IMPL_OFFSET>,
            RemoveHideVideoPreviewRequested::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerProviderConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
pub trait IBarcodeScannerProviderConnection2Impl: Sized {
    fn CreateFrameReaderAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<BarcodeScannerFrameReader>>;
    fn CreateFrameReaderWithFormatAsync(&self, preferredformat: super::super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<BarcodeScannerFrameReader>>;
    fn CreateFrameReaderWithFormatAndSizeAsync(&self, preferredformat: super::super::super::Graphics::Imaging::BitmapPixelFormat, preferredsize: &super::super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<BarcodeScannerFrameReader>>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarcodeScannerProviderConnection2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.IBarcodeScannerProviderConnection2";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl IBarcodeScannerProviderConnection2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerProviderConnection2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerProviderConnection2Vtbl {
        unsafe extern "system" fn CreateFrameReaderAsync<Impl: IBarcodeScannerProviderConnection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFrameReaderAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFrameReaderWithFormatAsync<Impl: IBarcodeScannerProviderConnection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preferredformat: super::super::super::Graphics::Imaging::BitmapPixelFormat, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFrameReaderWithFormatAsync(preferredformat) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFrameReaderWithFormatAndSizeAsync<Impl: IBarcodeScannerProviderConnection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preferredformat: super::super::super::Graphics::Imaging::BitmapPixelFormat, preferredsize: super::super::super::Graphics::Imaging::BitmapSize, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFrameReaderWithFormatAndSizeAsync(preferredformat, &*(&preferredsize as *const <super::super::super::Graphics::Imaging::BitmapSize as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Imaging::BitmapSize as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarcodeScannerProviderConnection2>, ::windows::core::GetTrustLevel, CreateFrameReaderAsync::<Impl, IMPL_OFFSET>, CreateFrameReaderWithFormatAsync::<Impl, IMPL_OFFSET>, CreateFrameReaderWithFormatAndSizeAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerProviderConnection2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerProviderTriggerDetailsImpl: Sized {
    fn Connection(&self) -> ::windows::core::Result<BarcodeScannerProviderConnection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarcodeScannerProviderTriggerDetails {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.IBarcodeScannerProviderTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IBarcodeScannerProviderTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerProviderTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerProviderTriggerDetailsVtbl {
        unsafe extern "system" fn Connection<Impl: IBarcodeScannerProviderTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Connection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarcodeScannerProviderTriggerDetails>, ::windows::core::GetTrustLevel, Connection::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerProviderTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IBarcodeScannerSetActiveSymbologiesRequestImpl: Sized {
    fn Symbologies(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<u32>>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarcodeScannerSetActiveSymbologiesRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.IBarcodeScannerSetActiveSymbologiesRequest";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IBarcodeScannerSetActiveSymbologiesRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerSetActiveSymbologiesRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerSetActiveSymbologiesRequestVtbl {
        unsafe extern "system" fn Symbologies<Impl: IBarcodeScannerSetActiveSymbologiesRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Symbologies() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCompletedAsync<Impl: IBarcodeScannerSetActiveSymbologiesRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportCompletedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedAsync<Impl: IBarcodeScannerSetActiveSymbologiesRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarcodeScannerSetActiveSymbologiesRequest>, ::windows::core::GetTrustLevel, Symbologies::<Impl, IMPL_OFFSET>, ReportCompletedAsync::<Impl, IMPL_OFFSET>, ReportFailedAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerSetActiveSymbologiesRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBarcodeScannerSetActiveSymbologiesRequest2Impl: Sized {
    fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedWithFailedReasonAndDescriptionAsync(&self, reason: i32, failedreasondescription: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarcodeScannerSetActiveSymbologiesRequest2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.IBarcodeScannerSetActiveSymbologiesRequest2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBarcodeScannerSetActiveSymbologiesRequest2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerSetActiveSymbologiesRequest2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerSetActiveSymbologiesRequest2Vtbl {
        unsafe extern "system" fn ReportFailedWithFailedReasonAsync<Impl: IBarcodeScannerSetActiveSymbologiesRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedWithFailedReasonAsync(reason) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedWithFailedReasonAndDescriptionAsync<Impl: IBarcodeScannerSetActiveSymbologiesRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: i32, failedreasondescription: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedWithFailedReasonAndDescriptionAsync(reason, &*(&failedreasondescription as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarcodeScannerSetActiveSymbologiesRequest2>, ::windows::core::GetTrustLevel, ReportFailedWithFailedReasonAsync::<Impl, IMPL_OFFSET>, ReportFailedWithFailedReasonAndDescriptionAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerSetActiveSymbologiesRequest2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBarcodeScannerSetActiveSymbologiesRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<BarcodeScannerSetActiveSymbologiesRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarcodeScannerSetActiveSymbologiesRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.IBarcodeScannerSetActiveSymbologiesRequestEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBarcodeScannerSetActiveSymbologiesRequestEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerSetActiveSymbologiesRequestEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerSetActiveSymbologiesRequestEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IBarcodeScannerSetActiveSymbologiesRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IBarcodeScannerSetActiveSymbologiesRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarcodeScannerSetActiveSymbologiesRequestEventArgs>, ::windows::core::GetTrustLevel, Request::<Impl, IMPL_OFFSET>, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerSetActiveSymbologiesRequestEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBarcodeScannerSetSymbologyAttributesRequestImpl: Sized {
    fn Symbology(&self) -> ::windows::core::Result<u32>;
    fn Attributes(&self) -> ::windows::core::Result<super::BarcodeSymbologyAttributes>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarcodeScannerSetSymbologyAttributesRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.IBarcodeScannerSetSymbologyAttributesRequest";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBarcodeScannerSetSymbologyAttributesRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerSetSymbologyAttributesRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerSetSymbologyAttributesRequestVtbl {
        unsafe extern "system" fn Symbology<Impl: IBarcodeScannerSetSymbologyAttributesRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Symbology() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Attributes<Impl: IBarcodeScannerSetSymbologyAttributesRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Attributes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCompletedAsync<Impl: IBarcodeScannerSetSymbologyAttributesRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportCompletedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedAsync<Impl: IBarcodeScannerSetSymbologyAttributesRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarcodeScannerSetSymbologyAttributesRequest>, ::windows::core::GetTrustLevel, Symbology::<Impl, IMPL_OFFSET>, Attributes::<Impl, IMPL_OFFSET>, ReportCompletedAsync::<Impl, IMPL_OFFSET>, ReportFailedAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerSetSymbologyAttributesRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBarcodeScannerSetSymbologyAttributesRequest2Impl: Sized {
    fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedWithFailedReasonAndDescriptionAsync(&self, reason: i32, failedreasondescription: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarcodeScannerSetSymbologyAttributesRequest2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.IBarcodeScannerSetSymbologyAttributesRequest2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBarcodeScannerSetSymbologyAttributesRequest2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerSetSymbologyAttributesRequest2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerSetSymbologyAttributesRequest2Vtbl {
        unsafe extern "system" fn ReportFailedWithFailedReasonAsync<Impl: IBarcodeScannerSetSymbologyAttributesRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedWithFailedReasonAsync(reason) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedWithFailedReasonAndDescriptionAsync<Impl: IBarcodeScannerSetSymbologyAttributesRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: i32, failedreasondescription: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedWithFailedReasonAndDescriptionAsync(reason, &*(&failedreasondescription as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarcodeScannerSetSymbologyAttributesRequest2>, ::windows::core::GetTrustLevel, ReportFailedWithFailedReasonAsync::<Impl, IMPL_OFFSET>, ReportFailedWithFailedReasonAndDescriptionAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerSetSymbologyAttributesRequest2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBarcodeScannerSetSymbologyAttributesRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<BarcodeScannerSetSymbologyAttributesRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarcodeScannerSetSymbologyAttributesRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.IBarcodeScannerSetSymbologyAttributesRequestEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBarcodeScannerSetSymbologyAttributesRequestEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerSetSymbologyAttributesRequestEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerSetSymbologyAttributesRequestEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IBarcodeScannerSetSymbologyAttributesRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IBarcodeScannerSetSymbologyAttributesRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarcodeScannerSetSymbologyAttributesRequestEventArgs>, ::windows::core::GetTrustLevel, Request::<Impl, IMPL_OFFSET>, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerSetSymbologyAttributesRequestEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBarcodeScannerStartSoftwareTriggerRequestImpl: Sized {
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarcodeScannerStartSoftwareTriggerRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.IBarcodeScannerStartSoftwareTriggerRequest";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBarcodeScannerStartSoftwareTriggerRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerStartSoftwareTriggerRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerStartSoftwareTriggerRequestVtbl {
        unsafe extern "system" fn ReportCompletedAsync<Impl: IBarcodeScannerStartSoftwareTriggerRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportCompletedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedAsync<Impl: IBarcodeScannerStartSoftwareTriggerRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarcodeScannerStartSoftwareTriggerRequest>, ::windows::core::GetTrustLevel, ReportCompletedAsync::<Impl, IMPL_OFFSET>, ReportFailedAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerStartSoftwareTriggerRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBarcodeScannerStartSoftwareTriggerRequest2Impl: Sized {
    fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedWithFailedReasonAndDescriptionAsync(&self, reason: i32, failedreasondescription: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarcodeScannerStartSoftwareTriggerRequest2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.IBarcodeScannerStartSoftwareTriggerRequest2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBarcodeScannerStartSoftwareTriggerRequest2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerStartSoftwareTriggerRequest2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerStartSoftwareTriggerRequest2Vtbl {
        unsafe extern "system" fn ReportFailedWithFailedReasonAsync<Impl: IBarcodeScannerStartSoftwareTriggerRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedWithFailedReasonAsync(reason) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedWithFailedReasonAndDescriptionAsync<Impl: IBarcodeScannerStartSoftwareTriggerRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: i32, failedreasondescription: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedWithFailedReasonAndDescriptionAsync(reason, &*(&failedreasondescription as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarcodeScannerStartSoftwareTriggerRequest2>, ::windows::core::GetTrustLevel, ReportFailedWithFailedReasonAsync::<Impl, IMPL_OFFSET>, ReportFailedWithFailedReasonAndDescriptionAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerStartSoftwareTriggerRequest2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBarcodeScannerStartSoftwareTriggerRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<BarcodeScannerStartSoftwareTriggerRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarcodeScannerStartSoftwareTriggerRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.IBarcodeScannerStartSoftwareTriggerRequestEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBarcodeScannerStartSoftwareTriggerRequestEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerStartSoftwareTriggerRequestEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerStartSoftwareTriggerRequestEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IBarcodeScannerStartSoftwareTriggerRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IBarcodeScannerStartSoftwareTriggerRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarcodeScannerStartSoftwareTriggerRequestEventArgs>, ::windows::core::GetTrustLevel, Request::<Impl, IMPL_OFFSET>, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerStartSoftwareTriggerRequestEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBarcodeScannerStopSoftwareTriggerRequestImpl: Sized {
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarcodeScannerStopSoftwareTriggerRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.IBarcodeScannerStopSoftwareTriggerRequest";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBarcodeScannerStopSoftwareTriggerRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerStopSoftwareTriggerRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerStopSoftwareTriggerRequestVtbl {
        unsafe extern "system" fn ReportCompletedAsync<Impl: IBarcodeScannerStopSoftwareTriggerRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportCompletedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedAsync<Impl: IBarcodeScannerStopSoftwareTriggerRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarcodeScannerStopSoftwareTriggerRequest>, ::windows::core::GetTrustLevel, ReportCompletedAsync::<Impl, IMPL_OFFSET>, ReportFailedAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerStopSoftwareTriggerRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBarcodeScannerStopSoftwareTriggerRequest2Impl: Sized {
    fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedWithFailedReasonAndDescriptionAsync(&self, reason: i32, failedreasondescription: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarcodeScannerStopSoftwareTriggerRequest2 {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.IBarcodeScannerStopSoftwareTriggerRequest2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBarcodeScannerStopSoftwareTriggerRequest2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerStopSoftwareTriggerRequest2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerStopSoftwareTriggerRequest2Vtbl {
        unsafe extern "system" fn ReportFailedWithFailedReasonAsync<Impl: IBarcodeScannerStopSoftwareTriggerRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedWithFailedReasonAsync(reason) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedWithFailedReasonAndDescriptionAsync<Impl: IBarcodeScannerStopSoftwareTriggerRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: i32, failedreasondescription: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedWithFailedReasonAndDescriptionAsync(reason, &*(&failedreasondescription as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarcodeScannerStopSoftwareTriggerRequest2>, ::windows::core::GetTrustLevel, ReportFailedWithFailedReasonAsync::<Impl, IMPL_OFFSET>, ReportFailedWithFailedReasonAndDescriptionAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerStopSoftwareTriggerRequest2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBarcodeScannerStopSoftwareTriggerRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<BarcodeScannerStopSoftwareTriggerRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarcodeScannerStopSoftwareTriggerRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.IBarcodeScannerStopSoftwareTriggerRequestEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBarcodeScannerStopSoftwareTriggerRequestEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerStopSoftwareTriggerRequestEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerStopSoftwareTriggerRequestEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IBarcodeScannerStopSoftwareTriggerRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IBarcodeScannerStopSoftwareTriggerRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarcodeScannerStopSoftwareTriggerRequestEventArgs>, ::windows::core::GetTrustLevel, Request::<Impl, IMPL_OFFSET>, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerStopSoftwareTriggerRequestEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_Imaging", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IBarcodeScannerVideoFrameImpl: Sized {
    fn Format(&self) -> ::windows::core::Result<super::super::super::Graphics::Imaging::BitmapPixelFormat>;
    fn Width(&self) -> ::windows::core::Result<u32>;
    fn Height(&self) -> ::windows::core::Result<u32>;
    fn PixelData(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Graphics_Imaging", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBarcodeScannerVideoFrame {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.IBarcodeScannerVideoFrame";
}
#[cfg(all(feature = "Graphics_Imaging", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IBarcodeScannerVideoFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeScannerVideoFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeScannerVideoFrameVtbl {
        unsafe extern "system" fn Format<Impl: IBarcodeScannerVideoFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Format() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Impl: IBarcodeScannerVideoFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Impl: IBarcodeScannerVideoFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Height() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PixelData<Impl: IBarcodeScannerVideoFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PixelData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBarcodeScannerVideoFrame>, ::windows::core::GetTrustLevel, Format::<Impl, IMPL_OFFSET>, Width::<Impl, IMPL_OFFSET>, Height::<Impl, IMPL_OFFSET>, PixelData::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeScannerVideoFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeSymbologyAttributesBuilderImpl: Sized {
    fn IsCheckDigitValidationSupported(&self) -> ::windows::core::Result<bool>;
    fn SetIsCheckDigitValidationSupported(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsCheckDigitTransmissionSupported(&self) -> ::windows::core::Result<bool>;
    fn SetIsCheckDigitTransmissionSupported(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsDecodeLengthSupported(&self) -> ::windows::core::Result<bool>;
    fn SetIsDecodeLengthSupported(&self, value: bool) -> ::windows::core::Result<()>;
    fn CreateAttributes(&self) -> ::windows::core::Result<super::BarcodeSymbologyAttributes>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBarcodeSymbologyAttributesBuilder {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.IBarcodeSymbologyAttributesBuilder";
}
#[cfg(feature = "implement_exclusive")]
impl IBarcodeSymbologyAttributesBuilderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBarcodeSymbologyAttributesBuilderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBarcodeSymbologyAttributesBuilderVtbl {
        unsafe extern "system" fn IsCheckDigitValidationSupported<Impl: IBarcodeSymbologyAttributesBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCheckDigitValidationSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsCheckDigitValidationSupported<Impl: IBarcodeSymbologyAttributesBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsCheckDigitValidationSupported(value).into()
        }
        unsafe extern "system" fn IsCheckDigitTransmissionSupported<Impl: IBarcodeSymbologyAttributesBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCheckDigitTransmissionSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsCheckDigitTransmissionSupported<Impl: IBarcodeSymbologyAttributesBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsCheckDigitTransmissionSupported(value).into()
        }
        unsafe extern "system" fn IsDecodeLengthSupported<Impl: IBarcodeSymbologyAttributesBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDecodeLengthSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsDecodeLengthSupported<Impl: IBarcodeSymbologyAttributesBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsDecodeLengthSupported(value).into()
        }
        unsafe extern "system" fn CreateAttributes<Impl: IBarcodeSymbologyAttributesBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IBarcodeSymbologyAttributesBuilder>,
            ::windows::core::GetTrustLevel,
            IsCheckDigitValidationSupported::<Impl, IMPL_OFFSET>,
            SetIsCheckDigitValidationSupported::<Impl, IMPL_OFFSET>,
            IsCheckDigitTransmissionSupported::<Impl, IMPL_OFFSET>,
            SetIsCheckDigitTransmissionSupported::<Impl, IMPL_OFFSET>,
            IsDecodeLengthSupported::<Impl, IMPL_OFFSET>,
            SetIsDecodeLengthSupported::<Impl, IMPL_OFFSET>,
            CreateAttributes::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBarcodeSymbologyAttributesBuilder as ::windows::core::Interface>::IID
    }
}
