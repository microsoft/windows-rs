pub trait AsyncIAdviseSinkImpl: Sized {
    fn Begin_OnDataChange();
    fn Finish_OnDataChange();
    fn Begin_OnViewChange();
    fn Finish_OnViewChange();
    fn Begin_OnRename();
    fn Finish_OnRename();
    fn Begin_OnSave();
    fn Finish_OnSave();
    fn Begin_OnClose();
    fn Finish_OnClose();
}
impl ::windows::core::RuntimeName for AsyncIAdviseSink {
    const NAME: &'static str = "Windows.Win32.System.Com.AsyncIAdviseSink";
}
impl AsyncIAdviseSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIAdviseSinkImpl, const OFFSET: isize>() -> AsyncIAdviseSinkVtbl {
        unsafe extern "system" fn Begin_OnDataChange<Impl: AsyncIAdviseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_OnDataChange(&*(&pformatetc as *const <FORMATETC as ::windows::core::Abi>::Abi as *const <FORMATETC as ::windows::core::DefaultType>::DefaultType), &*(&pstgmed as *const <STGMEDIUM as ::windows::core::Abi>::Abi as *const <STGMEDIUM as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Finish_OnDataChange<Impl: AsyncIAdviseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish_OnDataChange().into()
        }
        unsafe extern "system" fn Begin_OnViewChange<Impl: AsyncIAdviseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: u32, lindex: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_OnViewChange(dwaspect, lindex).into()
        }
        unsafe extern "system" fn Finish_OnViewChange<Impl: AsyncIAdviseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish_OnViewChange().into()
        }
        unsafe extern "system" fn Begin_OnRename<Impl: AsyncIAdviseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_OnRename(&*(&pmk as *const <IMoniker as ::windows::core::Abi>::Abi as *const <IMoniker as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Finish_OnRename<Impl: AsyncIAdviseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish_OnRename().into()
        }
        unsafe extern "system" fn Begin_OnSave<Impl: AsyncIAdviseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_OnSave().into()
        }
        unsafe extern "system" fn Finish_OnSave<Impl: AsyncIAdviseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish_OnSave().into()
        }
        unsafe extern "system" fn Begin_OnClose<Impl: AsyncIAdviseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_OnClose().into()
        }
        unsafe extern "system" fn Finish_OnClose<Impl: AsyncIAdviseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish_OnClose().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<AsyncIAdviseSink>,
            ::windows::core::GetTrustLevel,
            Begin_OnDataChange::<Impl, OFFSET>,
            Finish_OnDataChange::<Impl, OFFSET>,
            Begin_OnViewChange::<Impl, OFFSET>,
            Finish_OnViewChange::<Impl, OFFSET>,
            Begin_OnRename::<Impl, OFFSET>,
            Finish_OnRename::<Impl, OFFSET>,
            Begin_OnSave::<Impl, OFFSET>,
            Finish_OnSave::<Impl, OFFSET>,
            Begin_OnClose::<Impl, OFFSET>,
            Finish_OnClose::<Impl, OFFSET>,
        )
    }
}
pub trait AsyncIAdviseSink2Impl: Sized + AsyncIAdviseSinkImpl {
    fn Begin_OnLinkSrcChange();
    fn Finish_OnLinkSrcChange();
}
impl ::windows::core::RuntimeName for AsyncIAdviseSink2 {
    const NAME: &'static str = "Windows.Win32.System.Com.AsyncIAdviseSink2";
}
impl AsyncIAdviseSink2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIAdviseSink2Impl, const OFFSET: isize>() -> AsyncIAdviseSink2Vtbl {
        unsafe extern "system" fn Begin_OnLinkSrcChange<Impl: AsyncIAdviseSink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin_OnLinkSrcChange(&*(&pmk as *const <IMoniker as ::windows::core::Abi>::Abi as *const <IMoniker as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Finish_OnLinkSrcChange<Impl: AsyncIAdviseSink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish_OnLinkSrcChange().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<AsyncIAdviseSink2>, ::windows::core::GetTrustLevel, Begin_OnLinkSrcChange::<Impl, OFFSET>, Finish_OnLinkSrcChange::<Impl, OFFSET>)
    }
}
pub trait AsyncIMultiQIImpl: Sized {
    fn Begin_QueryMultipleInterfaces();
    fn Finish_QueryMultipleInterfaces();
}
impl ::windows::core::RuntimeName for AsyncIMultiQI {
    const NAME: &'static str = "Windows.Win32.System.Com.AsyncIMultiQI";
}
impl AsyncIMultiQIVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIMultiQIImpl, const OFFSET: isize>() -> AsyncIMultiQIVtbl {
        unsafe extern "system" fn Begin_QueryMultipleInterfaces<Impl: AsyncIMultiQIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cmqis: u32, pmqis: *mut MULTI_QI) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_QueryMultipleInterfaces(cmqis, &*(&pmqis as *const <MULTI_QI as ::windows::core::Abi>::Abi as *const <MULTI_QI as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_QueryMultipleInterfaces<Impl: AsyncIMultiQIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmqis: *mut MULTI_QI) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_QueryMultipleInterfaces(&*(&pmqis as *const <MULTI_QI as ::windows::core::Abi>::Abi as *const <MULTI_QI as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<AsyncIMultiQI>, ::windows::core::GetTrustLevel, Begin_QueryMultipleInterfaces::<Impl, OFFSET>, Finish_QueryMultipleInterfaces::<Impl, OFFSET>)
    }
}
pub trait AsyncIPipeByteImpl: Sized {
    fn Begin_Pull();
    fn Finish_Pull();
    fn Begin_Push();
    fn Finish_Push();
}
impl ::windows::core::RuntimeName for AsyncIPipeByte {
    const NAME: &'static str = "Windows.Win32.System.Com.AsyncIPipeByte";
}
impl AsyncIPipeByteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIPipeByteImpl, const OFFSET: isize>() -> AsyncIPipeByteVtbl {
        unsafe extern "system" fn Begin_Pull<Impl: AsyncIPipeByteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crequest: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_Pull(crequest) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_Pull<Impl: AsyncIPipeByteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *mut u8, pcreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_Pull(::core::mem::transmute_copy(&buf), ::core::mem::transmute_copy(&pcreturned)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_Push<Impl: AsyncIPipeByteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *const u8, csent: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_Push(buf, csent) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_Push<Impl: AsyncIPipeByteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_Push() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<AsyncIPipeByte>, ::windows::core::GetTrustLevel, Begin_Pull::<Impl, OFFSET>, Finish_Pull::<Impl, OFFSET>, Begin_Push::<Impl, OFFSET>, Finish_Push::<Impl, OFFSET>)
    }
}
pub trait AsyncIPipeDoubleImpl: Sized {
    fn Begin_Pull();
    fn Finish_Pull();
    fn Begin_Push();
    fn Finish_Push();
}
impl ::windows::core::RuntimeName for AsyncIPipeDouble {
    const NAME: &'static str = "Windows.Win32.System.Com.AsyncIPipeDouble";
}
impl AsyncIPipeDoubleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIPipeDoubleImpl, const OFFSET: isize>() -> AsyncIPipeDoubleVtbl {
        unsafe extern "system" fn Begin_Pull<Impl: AsyncIPipeDoubleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crequest: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_Pull(crequest) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_Pull<Impl: AsyncIPipeDoubleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *mut f64, pcreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_Pull(::core::mem::transmute_copy(&buf), ::core::mem::transmute_copy(&pcreturned)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_Push<Impl: AsyncIPipeDoubleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *const f64, csent: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_Push(buf, csent) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_Push<Impl: AsyncIPipeDoubleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_Push() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<AsyncIPipeDouble>, ::windows::core::GetTrustLevel, Begin_Pull::<Impl, OFFSET>, Finish_Pull::<Impl, OFFSET>, Begin_Push::<Impl, OFFSET>, Finish_Push::<Impl, OFFSET>)
    }
}
pub trait AsyncIPipeLongImpl: Sized {
    fn Begin_Pull();
    fn Finish_Pull();
    fn Begin_Push();
    fn Finish_Push();
}
impl ::windows::core::RuntimeName for AsyncIPipeLong {
    const NAME: &'static str = "Windows.Win32.System.Com.AsyncIPipeLong";
}
impl AsyncIPipeLongVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIPipeLongImpl, const OFFSET: isize>() -> AsyncIPipeLongVtbl {
        unsafe extern "system" fn Begin_Pull<Impl: AsyncIPipeLongImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crequest: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_Pull(crequest) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_Pull<Impl: AsyncIPipeLongImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *mut i32, pcreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_Pull(::core::mem::transmute_copy(&buf), ::core::mem::transmute_copy(&pcreturned)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_Push<Impl: AsyncIPipeLongImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *const i32, csent: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_Push(buf, csent) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_Push<Impl: AsyncIPipeLongImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_Push() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<AsyncIPipeLong>, ::windows::core::GetTrustLevel, Begin_Pull::<Impl, OFFSET>, Finish_Pull::<Impl, OFFSET>, Begin_Push::<Impl, OFFSET>, Finish_Push::<Impl, OFFSET>)
    }
}
pub trait AsyncIUnknownImpl: Sized {
    fn Begin_QueryInterface();
    fn Finish_QueryInterface();
    fn Begin_AddRef();
    fn Finish_AddRef();
    fn Begin_Release();
    fn Finish_Release();
}
impl ::windows::core::RuntimeName for AsyncIUnknown {
    const NAME: &'static str = "Windows.Win32.System.Com.AsyncIUnknown";
}
impl AsyncIUnknownVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIUnknownImpl, const OFFSET: isize>() -> AsyncIUnknownVtbl {
        unsafe extern "system" fn Begin_QueryInterface<Impl: AsyncIUnknownImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_QueryInterface(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_QueryInterface<Impl: AsyncIUnknownImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_QueryInterface(::core::mem::transmute_copy(&ppvobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_AddRef<Impl: AsyncIUnknownImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_AddRef() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_AddRef<Impl: AsyncIUnknownImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_AddRef() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_Release<Impl: AsyncIUnknownImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin_Release() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish_Release<Impl: AsyncIUnknownImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finish_Release() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<AsyncIUnknown>, ::windows::core::GetTrustLevel, Begin_QueryInterface::<Impl, OFFSET>, Finish_QueryInterface::<Impl, OFFSET>, Begin_AddRef::<Impl, OFFSET>, Finish_AddRef::<Impl, OFFSET>, Begin_Release::<Impl, OFFSET>, Finish_Release::<Impl, OFFSET>)
    }
}
pub trait IActivationFilterImpl: Sized {
    fn HandleActivation();
}
impl ::windows::core::RuntimeName for IActivationFilter {
    const NAME: &'static str = "Windows.Win32.System.Com.IActivationFilter";
}
impl IActivationFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivationFilterImpl, const OFFSET: isize>() -> IActivationFilterVtbl {
        unsafe extern "system" fn HandleActivation<Impl: IActivationFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwactivationtype: u32, rclsid: *const ::windows::core::GUID, preplacementclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HandleActivation(dwactivationtype, &*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&preplacementclsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IActivationFilter>, ::windows::core::GetTrustLevel, HandleActivation::<Impl, OFFSET>)
    }
}
pub trait IAddrExclusionControlImpl: Sized {
    fn GetCurrentAddrExclusionList();
    fn UpdateAddrExclusionList();
}
impl ::windows::core::RuntimeName for IAddrExclusionControl {
    const NAME: &'static str = "Windows.Win32.System.Com.IAddrExclusionControl";
}
impl IAddrExclusionControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAddrExclusionControlImpl, const OFFSET: isize>() -> IAddrExclusionControlVtbl {
        unsafe extern "system" fn GetCurrentAddrExclusionList<Impl: IAddrExclusionControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentAddrExclusionList(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppenumerator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateAddrExclusionList<Impl: IAddrExclusionControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penumerator: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateAddrExclusionList(&*(&penumerator as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAddrExclusionControl>, ::windows::core::GetTrustLevel, GetCurrentAddrExclusionList::<Impl, OFFSET>, UpdateAddrExclusionList::<Impl, OFFSET>)
    }
}
pub trait IAddrTrackingControlImpl: Sized {
    fn EnableCOMDynamicAddrTracking();
    fn DisableCOMDynamicAddrTracking();
}
impl ::windows::core::RuntimeName for IAddrTrackingControl {
    const NAME: &'static str = "Windows.Win32.System.Com.IAddrTrackingControl";
}
impl IAddrTrackingControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAddrTrackingControlImpl, const OFFSET: isize>() -> IAddrTrackingControlVtbl {
        unsafe extern "system" fn EnableCOMDynamicAddrTracking<Impl: IAddrTrackingControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableCOMDynamicAddrTracking() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableCOMDynamicAddrTracking<Impl: IAddrTrackingControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisableCOMDynamicAddrTracking() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAddrTrackingControl>, ::windows::core::GetTrustLevel, EnableCOMDynamicAddrTracking::<Impl, OFFSET>, DisableCOMDynamicAddrTracking::<Impl, OFFSET>)
    }
}
pub trait IAdviseSinkImpl: Sized {
    fn OnDataChange();
    fn OnViewChange();
    fn OnRename();
    fn OnSave();
    fn OnClose();
}
impl ::windows::core::RuntimeName for IAdviseSink {
    const NAME: &'static str = "Windows.Win32.System.Com.IAdviseSink";
}
impl IAdviseSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdviseSinkImpl, const OFFSET: isize>() -> IAdviseSinkVtbl {
        unsafe extern "system" fn OnDataChange<Impl: IAdviseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDataChange(&*(&pformatetc as *const <FORMATETC as ::windows::core::Abi>::Abi as *const <FORMATETC as ::windows::core::DefaultType>::DefaultType), &*(&pstgmed as *const <STGMEDIUM as ::windows::core::Abi>::Abi as *const <STGMEDIUM as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnViewChange<Impl: IAdviseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: u32, lindex: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnViewChange(dwaspect, lindex).into()
        }
        unsafe extern "system" fn OnRename<Impl: IAdviseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnRename(&*(&pmk as *const <IMoniker as ::windows::core::Abi>::Abi as *const <IMoniker as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnSave<Impl: IAdviseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSave().into()
        }
        unsafe extern "system" fn OnClose<Impl: IAdviseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnClose().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAdviseSink>, ::windows::core::GetTrustLevel, OnDataChange::<Impl, OFFSET>, OnViewChange::<Impl, OFFSET>, OnRename::<Impl, OFFSET>, OnSave::<Impl, OFFSET>, OnClose::<Impl, OFFSET>)
    }
}
pub trait IAdviseSink2Impl: Sized + IAdviseSinkImpl {
    fn OnLinkSrcChange();
}
impl ::windows::core::RuntimeName for IAdviseSink2 {
    const NAME: &'static str = "Windows.Win32.System.Com.IAdviseSink2";
}
impl IAdviseSink2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdviseSink2Impl, const OFFSET: isize>() -> IAdviseSink2Vtbl {
        unsafe extern "system" fn OnLinkSrcChange<Impl: IAdviseSink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnLinkSrcChange(&*(&pmk as *const <IMoniker as ::windows::core::Abi>::Abi as *const <IMoniker as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAdviseSink2>, ::windows::core::GetTrustLevel, OnLinkSrcChange::<Impl, OFFSET>)
    }
}
pub trait IAgileObjectImpl: Sized {}
impl ::windows::core::RuntimeName for IAgileObject {
    const NAME: &'static str = "Windows.Win32.System.Com.IAgileObject";
}
impl IAgileObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAgileObjectImpl, const OFFSET: isize>() -> IAgileObjectVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAgileObject>, ::windows::core::GetTrustLevel)
    }
}
pub trait IAsyncManagerImpl: Sized {
    fn CompleteCall();
    fn GetCallContext();
    fn GetState();
}
impl ::windows::core::RuntimeName for IAsyncManager {
    const NAME: &'static str = "Windows.Win32.System.Com.IAsyncManager";
}
impl IAsyncManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncManagerImpl, const OFFSET: isize>() -> IAsyncManagerVtbl {
        unsafe extern "system" fn CompleteCall<Impl: IAsyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompleteCall(result) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCallContext<Impl: IAsyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCallContext(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetState<Impl: IAsyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulstateflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetState(::core::mem::transmute_copy(&pulstateflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAsyncManager>, ::windows::core::GetTrustLevel, CompleteCall::<Impl, OFFSET>, GetCallContext::<Impl, OFFSET>, GetState::<Impl, OFFSET>)
    }
}
pub trait IAsyncRpcChannelBufferImpl: Sized + IRpcChannelBuffer2Impl + IRpcChannelBufferImpl {
    fn Send();
    fn Receive();
    fn GetDestCtxEx();
}
impl ::windows::core::RuntimeName for IAsyncRpcChannelBuffer {
    const NAME: &'static str = "Windows.Win32.System.Com.IAsyncRpcChannelBuffer";
}
impl IAsyncRpcChannelBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncRpcChannelBufferImpl, const OFFSET: isize>() -> IAsyncRpcChannelBufferVtbl {
        unsafe extern "system" fn Send<Impl: IAsyncRpcChannelBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, psync: ::windows::core::RawPtr, pulstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Send(&*(&pmsg as *const <RPCOLEMESSAGE as ::windows::core::Abi>::Abi as *const <RPCOLEMESSAGE as ::windows::core::DefaultType>::DefaultType), &*(&psync as *const <ISynchronize as ::windows::core::Abi>::Abi as *const <ISynchronize as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pulstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Receive<Impl: IAsyncRpcChannelBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Receive(&*(&pmsg as *const <RPCOLEMESSAGE as ::windows::core::Abi>::Abi as *const <RPCOLEMESSAGE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pulstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestCtxEx<Impl: IAsyncRpcChannelBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDestCtxEx(&*(&pmsg as *const <RPCOLEMESSAGE as ::windows::core::Abi>::Abi as *const <RPCOLEMESSAGE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwdestcontext), ::core::mem::transmute_copy(&ppvdestcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAsyncRpcChannelBuffer>, ::windows::core::GetTrustLevel, Send::<Impl, OFFSET>, Receive::<Impl, OFFSET>, GetDestCtxEx::<Impl, OFFSET>)
    }
}
pub trait IAuthenticateImpl: Sized {
    fn Authenticate();
}
impl ::windows::core::RuntimeName for IAuthenticate {
    const NAME: &'static str = "Windows.Win32.System.Com.IAuthenticate";
}
impl IAuthenticateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAuthenticateImpl, const OFFSET: isize>() -> IAuthenticateVtbl {
        unsafe extern "system" fn Authenticate<Impl: IAuthenticateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut super::super::Foundation::PWSTR, pszpassword: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Authenticate(::core::mem::transmute_copy(&phwnd), ::core::mem::transmute_copy(&pszusername), ::core::mem::transmute_copy(&pszpassword)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAuthenticate>, ::windows::core::GetTrustLevel, Authenticate::<Impl, OFFSET>)
    }
}
pub trait IAuthenticateExImpl: Sized + IAuthenticateImpl {
    fn AuthenticateEx();
}
impl ::windows::core::RuntimeName for IAuthenticateEx {
    const NAME: &'static str = "Windows.Win32.System.Com.IAuthenticateEx";
}
impl IAuthenticateExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAuthenticateExImpl, const OFFSET: isize>() -> IAuthenticateExVtbl {
        unsafe extern "system" fn AuthenticateEx<Impl: IAuthenticateExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut super::super::Foundation::PWSTR, pszpassword: *mut super::super::Foundation::PWSTR, pauthinfo: *const AUTHENTICATEINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticateEx(::core::mem::transmute_copy(&phwnd), ::core::mem::transmute_copy(&pszusername), ::core::mem::transmute_copy(&pszpassword), &*(&pauthinfo as *const <AUTHENTICATEINFO as ::windows::core::Abi>::Abi as *const <AUTHENTICATEINFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAuthenticateEx>, ::windows::core::GetTrustLevel, AuthenticateEx::<Impl, OFFSET>)
    }
}
pub trait IBindCtxImpl: Sized {
    fn RegisterObjectBound();
    fn RevokeObjectBound();
    fn ReleaseBoundObjects();
    fn SetBindOptions();
    fn GetBindOptions();
    fn GetRunningObjectTable();
    fn RegisterObjectParam();
    fn GetObjectParam();
    fn EnumObjectParam();
    fn RevokeObjectParam();
}
impl ::windows::core::RuntimeName for IBindCtx {
    const NAME: &'static str = "Windows.Win32.System.Com.IBindCtx";
}
impl IBindCtxVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBindCtxImpl, const OFFSET: isize>() -> IBindCtxVtbl {
        unsafe extern "system" fn RegisterObjectBound<Impl: IBindCtxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterObjectBound(&*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RevokeObjectBound<Impl: IBindCtxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RevokeObjectBound(&*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseBoundObjects<Impl: IBindCtxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseBoundObjects() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBindOptions<Impl: IBindCtxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbindopts: *const BIND_OPTS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBindOptions(&*(&pbindopts as *const <BIND_OPTS as ::windows::core::Abi>::Abi as *const <BIND_OPTS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBindOptions<Impl: IBindCtxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbindopts: *mut BIND_OPTS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBindOptions(&*(&pbindopts as *const <BIND_OPTS as ::windows::core::Abi>::Abi as *const <BIND_OPTS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRunningObjectTable<Impl: IBindCtxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprot: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRunningObjectTable(::core::mem::transmute_copy(&pprot)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterObjectParam<Impl: IBindCtxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszkey: super::super::Foundation::PWSTR, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterObjectParam(&*(&pszkey as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectParam<Impl: IBindCtxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszkey: super::super::Foundation::PWSTR, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObjectParam(&*(&pszkey as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumObjectParam<Impl: IBindCtxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumObjectParam(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RevokeObjectParam<Impl: IBindCtxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszkey: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RevokeObjectParam(&*(&pszkey as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IBindCtx>,
            ::windows::core::GetTrustLevel,
            RegisterObjectBound::<Impl, OFFSET>,
            RevokeObjectBound::<Impl, OFFSET>,
            ReleaseBoundObjects::<Impl, OFFSET>,
            SetBindOptions::<Impl, OFFSET>,
            GetBindOptions::<Impl, OFFSET>,
            GetRunningObjectTable::<Impl, OFFSET>,
            RegisterObjectParam::<Impl, OFFSET>,
            GetObjectParam::<Impl, OFFSET>,
            EnumObjectParam::<Impl, OFFSET>,
            RevokeObjectParam::<Impl, OFFSET>,
        )
    }
}
pub trait IBindHostImpl: Sized {
    fn CreateMoniker();
    fn MonikerBindToStorage();
    fn MonikerBindToObject();
}
impl ::windows::core::RuntimeName for IBindHost {
    const NAME: &'static str = "Windows.Win32.System.Com.IBindHost";
}
impl IBindHostVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBindHostImpl, const OFFSET: isize>() -> IBindHostVtbl {
        unsafe extern "system" fn CreateMoniker<Impl: IBindHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: super::super::Foundation::PWSTR, pbc: ::windows::core::RawPtr, ppmk: *mut ::windows::core::RawPtr, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMoniker(&*(&szname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pbc as *const <IBindCtx as ::windows::core::Abi>::Abi as *const <IBindCtx as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppmk), dwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MonikerBindToStorage<Impl: IBindHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr, pbc: ::windows::core::RawPtr, pbsc: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MonikerBindToStorage(
                &*(&pmk as *const <IMoniker as ::windows::core::Abi>::Abi as *const <IMoniker as ::windows::core::DefaultType>::DefaultType),
                &*(&pbc as *const <IBindCtx as ::windows::core::Abi>::Abi as *const <IBindCtx as ::windows::core::DefaultType>::DefaultType),
                &*(&pbsc as *const <IBindStatusCallback as ::windows::core::Abi>::Abi as *const <IBindStatusCallback as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppvobj),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MonikerBindToObject<Impl: IBindHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr, pbc: ::windows::core::RawPtr, pbsc: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MonikerBindToObject(
                &*(&pmk as *const <IMoniker as ::windows::core::Abi>::Abi as *const <IMoniker as ::windows::core::DefaultType>::DefaultType),
                &*(&pbc as *const <IBindCtx as ::windows::core::Abi>::Abi as *const <IBindCtx as ::windows::core::DefaultType>::DefaultType),
                &*(&pbsc as *const <IBindStatusCallback as ::windows::core::Abi>::Abi as *const <IBindStatusCallback as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppvobj),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBindHost>, ::windows::core::GetTrustLevel, CreateMoniker::<Impl, OFFSET>, MonikerBindToStorage::<Impl, OFFSET>, MonikerBindToObject::<Impl, OFFSET>)
    }
}
pub trait IBindStatusCallbackImpl: Sized {
    fn OnStartBinding();
    fn GetPriority();
    fn OnLowResource();
    fn OnProgress();
    fn OnStopBinding();
    fn GetBindInfo();
    fn OnDataAvailable();
    fn OnObjectAvailable();
}
impl ::windows::core::RuntimeName for IBindStatusCallback {
    const NAME: &'static str = "Windows.Win32.System.Com.IBindStatusCallback";
}
impl IBindStatusCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBindStatusCallbackImpl, const OFFSET: isize>() -> IBindStatusCallbackVtbl {
        unsafe extern "system" fn OnStartBinding<Impl: IBindStatusCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32, pib: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnStartBinding(dwreserved, &*(&pib as *const <IBinding as ::windows::core::Abi>::Abi as *const <IBinding as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPriority<Impl: IBindStatusCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPriority(::core::mem::transmute_copy(&pnpriority)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnLowResource<Impl: IBindStatusCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnLowResource(reserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnProgress<Impl: IBindStatusCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnProgress(ulprogress, ulprogressmax, ulstatuscode, &*(&szstatustext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnStopBinding<Impl: IBindStatusCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT, szerror: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnStopBinding(hresult, &*(&szerror as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBindInfo<Impl: IBindStatusCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBindInfo(::core::mem::transmute_copy(&grfbindf), &*(&pbindinfo as *const <BINDINFO as ::windows::core::Abi>::Abi as *const <BINDINFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDataAvailable<Impl: IBindStatusCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnDataAvailable(grfbscf, dwsize, &*(&pformatetc as *const <FORMATETC as ::windows::core::Abi>::Abi as *const <FORMATETC as ::windows::core::DefaultType>::DefaultType), &*(&pstgmed as *const <STGMEDIUM as ::windows::core::Abi>::Abi as *const <STGMEDIUM as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnObjectAvailable<Impl: IBindStatusCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnObjectAvailable(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IBindStatusCallback>,
            ::windows::core::GetTrustLevel,
            OnStartBinding::<Impl, OFFSET>,
            GetPriority::<Impl, OFFSET>,
            OnLowResource::<Impl, OFFSET>,
            OnProgress::<Impl, OFFSET>,
            OnStopBinding::<Impl, OFFSET>,
            GetBindInfo::<Impl, OFFSET>,
            OnDataAvailable::<Impl, OFFSET>,
            OnObjectAvailable::<Impl, OFFSET>,
        )
    }
}
pub trait IBindStatusCallbackExImpl: Sized + IBindStatusCallbackImpl {
    fn GetBindInfoEx();
}
impl ::windows::core::RuntimeName for IBindStatusCallbackEx {
    const NAME: &'static str = "Windows.Win32.System.Com.IBindStatusCallbackEx";
}
impl IBindStatusCallbackExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBindStatusCallbackExImpl, const OFFSET: isize>() -> IBindStatusCallbackExVtbl {
        unsafe extern "system" fn GetBindInfoEx<Impl: IBindStatusCallbackExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBindInfoEx(::core::mem::transmute_copy(&grfbindf), &*(&pbindinfo as *const <BINDINFO as ::windows::core::Abi>::Abi as *const <BINDINFO as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&grfbindf2), ::core::mem::transmute_copy(&pdwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBindStatusCallbackEx>, ::windows::core::GetTrustLevel, GetBindInfoEx::<Impl, OFFSET>)
    }
}
pub trait IBindingImpl: Sized {
    fn Abort();
    fn Suspend();
    fn Resume();
    fn SetPriority();
    fn GetPriority();
    fn GetBindResult();
}
impl ::windows::core::RuntimeName for IBinding {
    const NAME: &'static str = "Windows.Win32.System.Com.IBinding";
}
impl IBindingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBindingImpl, const OFFSET: isize>() -> IBindingVtbl {
        unsafe extern "system" fn Abort<Impl: IBindingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Abort() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Suspend<Impl: IBindingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Suspend() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resume<Impl: IBindingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Resume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Impl: IBindingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, npriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPriority(npriority) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPriority<Impl: IBindingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPriority(::core::mem::transmute_copy(&pnpriority)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBindResult<Impl: IBindingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsidprotocol: *mut ::windows::core::GUID, pdwresult: *mut u32, pszresult: *mut super::super::Foundation::PWSTR, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBindResult(::core::mem::transmute_copy(&pclsidprotocol), ::core::mem::transmute_copy(&pdwresult), ::core::mem::transmute_copy(&pszresult), pdwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBinding>, ::windows::core::GetTrustLevel, Abort::<Impl, OFFSET>, Suspend::<Impl, OFFSET>, Resume::<Impl, OFFSET>, SetPriority::<Impl, OFFSET>, GetPriority::<Impl, OFFSET>, GetBindResult::<Impl, OFFSET>)
    }
}
pub trait IBlockingLockImpl: Sized {
    fn Lock();
    fn Unlock();
}
impl ::windows::core::RuntimeName for IBlockingLock {
    const NAME: &'static str = "Windows.Win32.System.Com.IBlockingLock";
}
impl IBlockingLockVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBlockingLockImpl, const OFFSET: isize>() -> IBlockingLockVtbl {
        unsafe extern "system" fn Lock<Impl: IBlockingLockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Lock(dwtimeout) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unlock<Impl: IBlockingLockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unlock() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBlockingLock>, ::windows::core::GetTrustLevel, Lock::<Impl, OFFSET>, Unlock::<Impl, OFFSET>)
    }
}
pub trait ICallFactoryImpl: Sized {
    fn CreateCall();
}
impl ::windows::core::RuntimeName for ICallFactory {
    const NAME: &'static str = "Windows.Win32.System.Com.ICallFactory";
}
impl ICallFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICallFactoryImpl, const OFFSET: isize>() -> ICallFactoryVtbl {
        unsafe extern "system" fn CreateCall<Impl: ICallFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pctrlunk: *mut ::core::ffi::c_void, riid2: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCall(
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pctrlunk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&riid2 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppv),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICallFactory>, ::windows::core::GetTrustLevel, CreateCall::<Impl, OFFSET>)
    }
}
pub trait ICancelMethodCallsImpl: Sized {
    fn Cancel();
    fn TestCancel();
}
impl ::windows::core::RuntimeName for ICancelMethodCalls {
    const NAME: &'static str = "Windows.Win32.System.Com.ICancelMethodCalls";
}
impl ICancelMethodCallsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICancelMethodCallsImpl, const OFFSET: isize>() -> ICancelMethodCallsVtbl {
        unsafe extern "system" fn Cancel<Impl: ICancelMethodCallsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulseconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cancel(ulseconds) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TestCancel<Impl: ICancelMethodCallsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TestCancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICancelMethodCalls>, ::windows::core::GetTrustLevel, Cancel::<Impl, OFFSET>, TestCancel::<Impl, OFFSET>)
    }
}
pub trait ICatInformationImpl: Sized {
    fn EnumCategories();
    fn GetCategoryDesc();
    fn EnumClassesOfCategories();
    fn IsClassOfCategories();
    fn EnumImplCategoriesOfClass();
    fn EnumReqCategoriesOfClass();
}
impl ::windows::core::RuntimeName for ICatInformation {
    const NAME: &'static str = "Windows.Win32.System.Com.ICatInformation";
}
impl ICatInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICatInformationImpl, const OFFSET: isize>() -> ICatInformationVtbl {
        unsafe extern "system" fn EnumCategories<Impl: ICatInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: u32, ppenumcategoryinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumCategories(lcid, ::core::mem::transmute_copy(&ppenumcategoryinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCategoryDesc<Impl: ICatInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rcatid: *const ::windows::core::GUID, lcid: u32, pszdesc: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCategoryDesc(&*(&rcatid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), lcid, ::core::mem::transmute_copy(&pszdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumClassesOfCategories<Impl: ICatInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cimplemented: u32, rgcatidimpl: *const ::windows::core::GUID, crequired: u32, rgcatidreq: *const ::windows::core::GUID, ppenumclsid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumClassesOfCategories(cimplemented, &*(&rgcatidimpl as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), crequired, &*(&rgcatidreq as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppenumclsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsClassOfCategories<Impl: ICatInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, cimplemented: u32, rgcatidimpl: *const ::windows::core::GUID, crequired: u32, rgcatidreq: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsClassOfCategories(
                &*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                cimplemented,
                &*(&rgcatidimpl as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                crequired,
                &*(&rgcatidreq as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumImplCategoriesOfClass<Impl: ICatInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ppenumcatid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumImplCategoriesOfClass(&*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppenumcatid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumReqCategoriesOfClass<Impl: ICatInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ppenumcatid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumReqCategoriesOfClass(&*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppenumcatid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ICatInformation>,
            ::windows::core::GetTrustLevel,
            EnumCategories::<Impl, OFFSET>,
            GetCategoryDesc::<Impl, OFFSET>,
            EnumClassesOfCategories::<Impl, OFFSET>,
            IsClassOfCategories::<Impl, OFFSET>,
            EnumImplCategoriesOfClass::<Impl, OFFSET>,
            EnumReqCategoriesOfClass::<Impl, OFFSET>,
        )
    }
}
pub trait ICatRegisterImpl: Sized {
    fn RegisterCategories();
    fn UnRegisterCategories();
    fn RegisterClassImplCategories();
    fn UnRegisterClassImplCategories();
    fn RegisterClassReqCategories();
    fn UnRegisterClassReqCategories();
}
impl ::windows::core::RuntimeName for ICatRegister {
    const NAME: &'static str = "Windows.Win32.System.Com.ICatRegister";
}
impl ICatRegisterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICatRegisterImpl, const OFFSET: isize>() -> ICatRegisterVtbl {
        unsafe extern "system" fn RegisterCategories<Impl: ICatRegisterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccategories: u32, rgcategoryinfo: *const CATEGORYINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterCategories(ccategories, &*(&rgcategoryinfo as *const <CATEGORYINFO as ::windows::core::Abi>::Abi as *const <CATEGORYINFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnRegisterCategories<Impl: ICatRegisterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnRegisterCategories(ccategories, &*(&rgcatid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterClassImplCategories<Impl: ICatRegisterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterClassImplCategories(&*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ccategories, &*(&rgcatid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnRegisterClassImplCategories<Impl: ICatRegisterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnRegisterClassImplCategories(&*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ccategories, &*(&rgcatid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterClassReqCategories<Impl: ICatRegisterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterClassReqCategories(&*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ccategories, &*(&rgcatid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnRegisterClassReqCategories<Impl: ICatRegisterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnRegisterClassReqCategories(&*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ccategories, &*(&rgcatid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ICatRegister>,
            ::windows::core::GetTrustLevel,
            RegisterCategories::<Impl, OFFSET>,
            UnRegisterCategories::<Impl, OFFSET>,
            RegisterClassImplCategories::<Impl, OFFSET>,
            UnRegisterClassImplCategories::<Impl, OFFSET>,
            RegisterClassReqCategories::<Impl, OFFSET>,
            UnRegisterClassReqCategories::<Impl, OFFSET>,
        )
    }
}
pub trait IChannelHookImpl: Sized {
    fn ClientGetSize();
    fn ClientFillBuffer();
    fn ClientNotify();
    fn ServerNotify();
    fn ServerGetSize();
    fn ServerFillBuffer();
}
impl ::windows::core::RuntimeName for IChannelHook {
    const NAME: &'static str = "Windows.Win32.System.Com.IChannelHook";
}
impl IChannelHookVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChannelHookImpl, const OFFSET: isize>() -> IChannelHookVtbl {
        unsafe extern "system" fn ClientGetSize<Impl: IChannelHookImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClientGetSize(&*(&uextent as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdatasize)).into()
        }
        unsafe extern "system" fn ClientFillBuffer<Impl: IChannelHookImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .ClientFillBuffer(&*(&uextent as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), pdatasize, &*(&pdatabuffer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType))
                .into()
        }
        unsafe extern "system" fn ClientNotify<Impl: IChannelHookImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32, hrfault: ::windows::core::HRESULT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .ClientNotify(
                    &*(&uextent as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                    &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                    cbdatasize,
                    &*(&pdatabuffer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                    ldatarep,
                    hrfault,
                )
                .into()
        }
        unsafe extern "system" fn ServerNotify<Impl: IChannelHookImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .ServerNotify(
                    &*(&uextent as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                    &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                    cbdatasize,
                    &*(&pdatabuffer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                    ldatarep,
                )
                .into()
        }
        unsafe extern "system" fn ServerGetSize<Impl: IChannelHookImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, hrfault: ::windows::core::HRESULT, pdatasize: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ServerGetSize(&*(&uextent as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), hrfault, ::core::mem::transmute_copy(&pdatasize)).into()
        }
        unsafe extern "system" fn ServerFillBuffer<Impl: IChannelHookImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void, hrfault: ::windows::core::HRESULT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .ServerFillBuffer(
                    &*(&uextent as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                    &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                    pdatasize,
                    &*(&pdatabuffer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                    hrfault,
                )
                .into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IChannelHook>, ::windows::core::GetTrustLevel, ClientGetSize::<Impl, OFFSET>, ClientFillBuffer::<Impl, OFFSET>, ClientNotify::<Impl, OFFSET>, ServerNotify::<Impl, OFFSET>, ServerGetSize::<Impl, OFFSET>, ServerFillBuffer::<Impl, OFFSET>)
    }
}
pub trait IClassActivatorImpl: Sized {
    fn GetClassObject();
}
impl ::windows::core::RuntimeName for IClassActivator {
    const NAME: &'static str = "Windows.Win32.System.Com.IClassActivator";
}
impl IClassActivatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClassActivatorImpl, const OFFSET: isize>() -> IClassActivatorVtbl {
        unsafe extern "system" fn GetClassObject<Impl: IClassActivatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, dwclasscontext: u32, locale: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClassObject(&*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwclasscontext, locale, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IClassActivator>, ::windows::core::GetTrustLevel, GetClassObject::<Impl, OFFSET>)
    }
}
pub trait IClassFactoryImpl: Sized {
    fn CreateInstance();
    fn LockServer();
}
impl ::windows::core::RuntimeName for IClassFactory {
    const NAME: &'static str = "Windows.Win32.System.Com.IClassFactory";
}
impl IClassFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClassFactoryImpl, const OFFSET: isize>() -> IClassFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IClassFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockServer<Impl: IClassFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flock: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LockServer(&*(&flock as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IClassFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>, LockServer::<Impl, OFFSET>)
    }
}
pub trait IClientSecurityImpl: Sized {
    fn QueryBlanket();
    fn SetBlanket();
    fn CopyProxy();
}
impl ::windows::core::RuntimeName for IClientSecurity {
    const NAME: &'static str = "Windows.Win32.System.Com.IClientSecurity";
}
impl IClientSecurityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClientSecurityImpl, const OFFSET: isize>() -> IClientSecurityVtbl {
        unsafe extern "system" fn QueryBlanket<Impl: IClientSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproxy: *mut ::core::ffi::c_void, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut RPC_C_AUTHN_LEVEL, pimplevel: *mut RPC_C_IMP_LEVEL, pauthinfo: *mut *mut ::core::ffi::c_void, pcapabilites: *mut EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryBlanket(&*(&pproxy as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pauthnsvc), ::core::mem::transmute_copy(&pauthzsvc), ::core::mem::transmute_copy(&pserverprincname), ::core::mem::transmute_copy(&pauthnlevel), ::core::mem::transmute_copy(&pimplevel), ::core::mem::transmute_copy(&pauthinfo), ::core::mem::transmute_copy(&pcapabilites)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlanket<Impl: IClientSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproxy: *mut ::core::ffi::c_void, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: super::super::Foundation::PWSTR, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBlanket(
                &*(&pproxy as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                dwauthnsvc,
                dwauthzsvc,
                &*(&pserverprincname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                dwauthnlevel,
                dwimplevel,
                &*(&pauthinfo as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                dwcapabilities,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyProxy<Impl: IClientSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproxy: *mut ::core::ffi::c_void, ppcopy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyProxy(&*(&pproxy as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcopy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IClientSecurity>, ::windows::core::GetTrustLevel, QueryBlanket::<Impl, OFFSET>, SetBlanket::<Impl, OFFSET>, CopyProxy::<Impl, OFFSET>)
    }
}
pub trait IComThreadingInfoImpl: Sized {
    fn GetCurrentApartmentType();
    fn GetCurrentThreadType();
    fn GetCurrentLogicalThreadId();
    fn SetCurrentLogicalThreadId();
}
impl ::windows::core::RuntimeName for IComThreadingInfo {
    const NAME: &'static str = "Windows.Win32.System.Com.IComThreadingInfo";
}
impl IComThreadingInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComThreadingInfoImpl, const OFFSET: isize>() -> IComThreadingInfoVtbl {
        unsafe extern "system" fn GetCurrentApartmentType<Impl: IComThreadingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, papttype: *mut APTTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentApartmentType(::core::mem::transmute_copy(&papttype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentThreadType<Impl: IComThreadingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pthreadtype: *mut THDTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentThreadType(::core::mem::transmute_copy(&pthreadtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentLogicalThreadId<Impl: IComThreadingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidlogicalthreadid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentLogicalThreadId(::core::mem::transmute_copy(&pguidlogicalthreadid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentLogicalThreadId<Impl: IComThreadingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCurrentLogicalThreadId(&*(&rguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComThreadingInfo>, ::windows::core::GetTrustLevel, GetCurrentApartmentType::<Impl, OFFSET>, GetCurrentThreadType::<Impl, OFFSET>, GetCurrentLogicalThreadId::<Impl, OFFSET>, SetCurrentLogicalThreadId::<Impl, OFFSET>)
    }
}
pub trait IConnectionPointImpl: Sized {
    fn GetConnectionInterface();
    fn GetConnectionPointContainer();
    fn Advise();
    fn Unadvise();
    fn EnumConnections();
}
impl ::windows::core::RuntimeName for IConnectionPoint {
    const NAME: &'static str = "Windows.Win32.System.Com.IConnectionPoint";
}
impl IConnectionPointVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectionPointImpl, const OFFSET: isize>() -> IConnectionPointVtbl {
        unsafe extern "system" fn GetConnectionInterface<Impl: IConnectionPointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectionInterface(::core::mem::transmute_copy(&piid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectionPointContainer<Impl: IConnectionPointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcpc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectionPointContainer(::core::mem::transmute_copy(&ppcpc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Advise<Impl: IConnectionPointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punksink: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Advise(&*(&punksink as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Impl: IConnectionPointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unadvise(dwcookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumConnections<Impl: IConnectionPointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumConnections(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConnectionPoint>, ::windows::core::GetTrustLevel, GetConnectionInterface::<Impl, OFFSET>, GetConnectionPointContainer::<Impl, OFFSET>, Advise::<Impl, OFFSET>, Unadvise::<Impl, OFFSET>, EnumConnections::<Impl, OFFSET>)
    }
}
pub trait IConnectionPointContainerImpl: Sized {
    fn EnumConnectionPoints();
    fn FindConnectionPoint();
}
impl ::windows::core::RuntimeName for IConnectionPointContainer {
    const NAME: &'static str = "Windows.Win32.System.Com.IConnectionPointContainer";
}
impl IConnectionPointContainerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectionPointContainerImpl, const OFFSET: isize>() -> IConnectionPointContainerVtbl {
        unsafe extern "system" fn EnumConnectionPoints<Impl: IConnectionPointContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumConnectionPoints(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindConnectionPoint<Impl: IConnectionPointContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppcp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindConnectionPoint(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConnectionPointContainer>, ::windows::core::GetTrustLevel, EnumConnectionPoints::<Impl, OFFSET>, FindConnectionPoint::<Impl, OFFSET>)
    }
}
pub trait IContextCallbackImpl: Sized {
    fn ContextCallback();
}
impl ::windows::core::RuntimeName for IContextCallback {
    const NAME: &'static str = "Windows.Win32.System.Com.IContextCallback";
}
impl IContextCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContextCallbackImpl, const OFFSET: isize>() -> IContextCallbackVtbl {
        unsafe extern "system" fn ContextCallback<Impl: IContextCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfncallback: ::windows::core::RawPtr, pparam: *const ComCallData, riid: *const ::windows::core::GUID, imethod: i32, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContextCallback(
                &*(&pfncallback as *const <PFNCONTEXTCALL as ::windows::core::Abi>::Abi as *const <PFNCONTEXTCALL as ::windows::core::DefaultType>::DefaultType),
                &*(&pparam as *const <ComCallData as ::windows::core::Abi>::Abi as *const <ComCallData as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                imethod,
                &*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContextCallback>, ::windows::core::GetTrustLevel, ContextCallback::<Impl, OFFSET>)
    }
}
pub trait IDataAdviseHolderImpl: Sized {
    fn Advise();
    fn Unadvise();
    fn EnumAdvise();
    fn SendOnDataChange();
}
impl ::windows::core::RuntimeName for IDataAdviseHolder {
    const NAME: &'static str = "Windows.Win32.System.Com.IDataAdviseHolder";
}
impl IDataAdviseHolderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataAdviseHolderImpl, const OFFSET: isize>() -> IDataAdviseHolderVtbl {
        unsafe extern "system" fn Advise<Impl: IDataAdviseHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobject: ::windows::core::RawPtr, pfetc: *const FORMATETC, advf: u32, padvise: ::windows::core::RawPtr, pdwconnection: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Advise(&*(&pdataobject as *const <IDataObject as ::windows::core::Abi>::Abi as *const <IDataObject as ::windows::core::DefaultType>::DefaultType), &*(&pfetc as *const <FORMATETC as ::windows::core::Abi>::Abi as *const <FORMATETC as ::windows::core::DefaultType>::DefaultType), advf, &*(&padvise as *const <IAdviseSink as ::windows::core::Abi>::Abi as *const <IAdviseSink as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwconnection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Impl: IDataAdviseHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unadvise(dwconnection) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumAdvise<Impl: IDataAdviseHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumadvise: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumAdvise(::core::mem::transmute_copy(&ppenumadvise)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendOnDataChange<Impl: IDataAdviseHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobject: ::windows::core::RawPtr, dwreserved: u32, advf: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendOnDataChange(&*(&pdataobject as *const <IDataObject as ::windows::core::Abi>::Abi as *const <IDataObject as ::windows::core::DefaultType>::DefaultType), dwreserved, advf) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDataAdviseHolder>, ::windows::core::GetTrustLevel, Advise::<Impl, OFFSET>, Unadvise::<Impl, OFFSET>, EnumAdvise::<Impl, OFFSET>, SendOnDataChange::<Impl, OFFSET>)
    }
}
pub trait IDataObjectImpl: Sized {
    fn GetData();
    fn GetDataHere();
    fn QueryGetData();
    fn GetCanonicalFormatEtc();
    fn SetData();
    fn EnumFormatEtc();
    fn DAdvise();
    fn DUnadvise();
    fn EnumDAdvise();
}
impl ::windows::core::RuntimeName for IDataObject {
    const NAME: &'static str = "Windows.Win32.System.Com.IDataObject";
}
impl IDataObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataObjectImpl, const OFFSET: isize>() -> IDataObjectVtbl {
        unsafe extern "system" fn GetData<Impl: IDataObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetcin: *const FORMATETC, pmedium: *mut STGMEDIUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetData(&*(&pformatetcin as *const <FORMATETC as ::windows::core::Abi>::Abi as *const <FORMATETC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pmedium)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataHere<Impl: IDataObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pmedium: *mut STGMEDIUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataHere(&*(&pformatetc as *const <FORMATETC as ::windows::core::Abi>::Abi as *const <FORMATETC as ::windows::core::DefaultType>::DefaultType), &*(&pmedium as *const <STGMEDIUM as ::windows::core::Abi>::Abi as *const <STGMEDIUM as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryGetData<Impl: IDataObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryGetData(&*(&pformatetc as *const <FORMATETC as ::windows::core::Abi>::Abi as *const <FORMATETC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCanonicalFormatEtc<Impl: IDataObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatectin: *const FORMATETC, pformatetcout: *mut FORMATETC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCanonicalFormatEtc(&*(&pformatectin as *const <FORMATETC as ::windows::core::Abi>::Abi as *const <FORMATETC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pformatetcout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Impl: IDataObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pmedium: *const STGMEDIUM, frelease: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetData(&*(&pformatetc as *const <FORMATETC as ::windows::core::Abi>::Abi as *const <FORMATETC as ::windows::core::DefaultType>::DefaultType), &*(&pmedium as *const <STGMEDIUM as ::windows::core::Abi>::Abi as *const <STGMEDIUM as ::windows::core::DefaultType>::DefaultType), &*(&frelease as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumFormatEtc<Impl: IDataObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdirection: u32, ppenumformatetc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumFormatEtc(dwdirection, ::core::mem::transmute_copy(&ppenumformatetc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DAdvise<Impl: IDataObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, advf: u32, padvsink: ::windows::core::RawPtr, pdwconnection: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DAdvise(&*(&pformatetc as *const <FORMATETC as ::windows::core::Abi>::Abi as *const <FORMATETC as ::windows::core::DefaultType>::DefaultType), advf, &*(&padvsink as *const <IAdviseSink as ::windows::core::Abi>::Abi as *const <IAdviseSink as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwconnection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DUnadvise<Impl: IDataObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DUnadvise(dwconnection) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDAdvise<Impl: IDataObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumadvise: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumDAdvise(::core::mem::transmute_copy(&ppenumadvise)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDataObject>,
            ::windows::core::GetTrustLevel,
            GetData::<Impl, OFFSET>,
            GetDataHere::<Impl, OFFSET>,
            QueryGetData::<Impl, OFFSET>,
            GetCanonicalFormatEtc::<Impl, OFFSET>,
            SetData::<Impl, OFFSET>,
            EnumFormatEtc::<Impl, OFFSET>,
            DAdvise::<Impl, OFFSET>,
            DUnadvise::<Impl, OFFSET>,
            EnumDAdvise::<Impl, OFFSET>,
        )
    }
}
pub trait IDispatchImpl: Sized {
    fn GetTypeInfoCount();
    fn GetTypeInfo();
    fn GetIDsOfNames();
    fn Invoke();
}
impl ::windows::core::RuntimeName for IDispatch {
    const NAME: &'static str = "Windows.Win32.System.Com.IDispatch";
}
impl IDispatchVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispatchImpl, const OFFSET: isize>() -> IDispatchVtbl {
        unsafe extern "system" fn GetTypeInfoCount<Impl: IDispatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTypeInfoCount(::core::mem::transmute_copy(&pctinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeInfo<Impl: IDispatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTypeInfo(itinfo, lcid, ::core::mem::transmute_copy(&pptinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIDsOfNames<Impl: IDispatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIDsOfNames(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&rgsznames as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cnames, lcid, ::core::mem::transmute_copy(&rgdispid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Invoke<Impl: IDispatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Invoke(dispidmember, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), lcid, wflags, &*(&pdispparams as *const <DISPPARAMS as ::windows::core::Abi>::Abi as *const <DISPPARAMS as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarresult), ::core::mem::transmute_copy(&pexcepinfo), ::core::mem::transmute_copy(&puargerr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDispatch>, ::windows::core::GetTrustLevel, GetTypeInfoCount::<Impl, OFFSET>, GetTypeInfo::<Impl, OFFSET>, GetIDsOfNames::<Impl, OFFSET>, Invoke::<Impl, OFFSET>)
    }
}
pub trait IEnumCATEGORYINFOImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumCATEGORYINFO {
    const NAME: &'static str = "Windows.Win32.System.Com.IEnumCATEGORYINFO";
}
impl IEnumCATEGORYINFOVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCATEGORYINFOImpl, const OFFSET: isize>() -> IEnumCATEGORYINFOVtbl {
        unsafe extern "system" fn Next<Impl: IEnumCATEGORYINFOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut CATEGORYINFO, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumCATEGORYINFOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumCATEGORYINFOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumCATEGORYINFOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumCATEGORYINFO>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumConnectionPointsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumConnectionPoints {
    const NAME: &'static str = "Windows.Win32.System.Com.IEnumConnectionPoints";
}
impl IEnumConnectionPointsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumConnectionPointsImpl, const OFFSET: isize>() -> IEnumConnectionPointsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumConnectionPointsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cconnections: u32, ppcp: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(cconnections, ::core::mem::transmute_copy(&ppcp), ::core::mem::transmute_copy(&pcfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumConnectionPointsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cconnections: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(cconnections) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumConnectionPointsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumConnectionPointsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumConnectionPoints>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumConnectionsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumConnections {
    const NAME: &'static str = "Windows.Win32.System.Com.IEnumConnections";
}
impl IEnumConnectionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumConnectionsImpl, const OFFSET: isize>() -> IEnumConnectionsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumConnectionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cconnections: u32, rgcd: *mut CONNECTDATA, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(cconnections, ::core::mem::transmute_copy(&rgcd), ::core::mem::transmute_copy(&pcfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumConnectionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cconnections: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(cconnections) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumConnectionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumConnectionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumConnections>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumFORMATETCImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumFORMATETC {
    const NAME: &'static str = "Windows.Win32.System.Com.IEnumFORMATETC";
}
impl IEnumFORMATETCVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumFORMATETCImpl, const OFFSET: isize>() -> IEnumFORMATETCVtbl {
        unsafe extern "system" fn Next<Impl: IEnumFORMATETCImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut FORMATETC, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumFORMATETCImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumFORMATETCImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumFORMATETCImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumFORMATETC>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumGUIDImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumGUID {
    const NAME: &'static str = "Windows.Win32.System.Com.IEnumGUID";
}
impl IEnumGUIDVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumGUIDImpl, const OFFSET: isize>() -> IEnumGUIDVtbl {
        unsafe extern "system" fn Next<Impl: IEnumGUIDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumGUIDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumGUIDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumGUIDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumGUID>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumMonikerImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumMoniker {
    const NAME: &'static str = "Windows.Win32.System.Com.IEnumMoniker";
}
impl IEnumMonikerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumMonikerImpl, const OFFSET: isize>() -> IEnumMonikerVtbl {
        unsafe extern "system" fn Next<Impl: IEnumMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumMoniker>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumSTATDATAImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumSTATDATA {
    const NAME: &'static str = "Windows.Win32.System.Com.IEnumSTATDATA";
}
impl IEnumSTATDATAVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSTATDATAImpl, const OFFSET: isize>() -> IEnumSTATDATAVtbl {
        unsafe extern "system" fn Next<Impl: IEnumSTATDATAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut STATDATA, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumSTATDATAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumSTATDATAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumSTATDATAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumSTATDATA>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumStringImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumString {
    const NAME: &'static str = "Windows.Win32.System.Com.IEnumString";
}
impl IEnumStringVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumStringImpl, const OFFSET: isize>() -> IEnumStringVtbl {
        unsafe extern "system" fn Next<Impl: IEnumStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut super::super::Foundation::PWSTR, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumString>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumUnknownImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumUnknown {
    const NAME: &'static str = "Windows.Win32.System.Com.IEnumUnknown";
}
impl IEnumUnknownVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumUnknownImpl, const OFFSET: isize>() -> IEnumUnknownVtbl {
        unsafe extern "system" fn Next<Impl: IEnumUnknownImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumUnknownImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumUnknownImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumUnknownImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumUnknown>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IErrorInfoImpl: Sized {
    fn GetGUID();
    fn GetSource();
    fn GetDescription();
    fn GetHelpFile();
    fn GetHelpContext();
}
impl ::windows::core::RuntimeName for IErrorInfo {
    const NAME: &'static str = "Windows.Win32.System.Com.IErrorInfo";
}
impl IErrorInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IErrorInfoImpl, const OFFSET: isize>() -> IErrorInfoVtbl {
        unsafe extern "system" fn GetGUID<Impl: IErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGUID(::core::mem::transmute_copy(&pguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSource<Impl: IErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSource(::core::mem::transmute_copy(&pbstrsource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Impl: IErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescription(::core::mem::transmute_copy(&pbstrdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHelpFile<Impl: IErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrhelpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHelpFile(::core::mem::transmute_copy(&pbstrhelpfile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHelpContext<Impl: IErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwhelpcontext: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHelpContext(::core::mem::transmute_copy(&pdwhelpcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IErrorInfo>, ::windows::core::GetTrustLevel, GetGUID::<Impl, OFFSET>, GetSource::<Impl, OFFSET>, GetDescription::<Impl, OFFSET>, GetHelpFile::<Impl, OFFSET>, GetHelpContext::<Impl, OFFSET>)
    }
}
pub trait IErrorLogImpl: Sized {
    fn AddError();
}
impl ::windows::core::RuntimeName for IErrorLog {
    const NAME: &'static str = "Windows.Win32.System.Com.IErrorLog";
}
impl IErrorLogVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IErrorLogImpl, const OFFSET: isize>() -> IErrorLogVtbl {
        unsafe extern "system" fn AddError<Impl: IErrorLogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropname: super::super::Foundation::PWSTR, pexcepinfo: *const EXCEPINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddError(&*(&pszpropname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pexcepinfo as *const <EXCEPINFO as ::windows::core::Abi>::Abi as *const <EXCEPINFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IErrorLog>, ::windows::core::GetTrustLevel, AddError::<Impl, OFFSET>)
    }
}
pub trait IExternalConnectionImpl: Sized {
    fn AddConnection();
    fn ReleaseConnection();
}
impl ::windows::core::RuntimeName for IExternalConnection {
    const NAME: &'static str = "Windows.Win32.System.Com.IExternalConnection";
}
impl IExternalConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExternalConnectionImpl, const OFFSET: isize>() -> IExternalConnectionVtbl {
        unsafe extern "system" fn AddConnection<Impl: IExternalConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extconn: u32, reserved: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddConnection(extconn, reserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseConnection<Impl: IExternalConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extconn: u32, reserved: u32, flastreleasecloses: super::super::Foundation::BOOL) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseConnection(extconn, reserved, &*(&flastreleasecloses as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IExternalConnection>, ::windows::core::GetTrustLevel, AddConnection::<Impl, OFFSET>, ReleaseConnection::<Impl, OFFSET>)
    }
}
pub trait IFastRundownImpl: Sized {}
impl ::windows::core::RuntimeName for IFastRundown {
    const NAME: &'static str = "Windows.Win32.System.Com.IFastRundown";
}
impl IFastRundownVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFastRundownImpl, const OFFSET: isize>() -> IFastRundownVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFastRundown>, ::windows::core::GetTrustLevel)
    }
}
pub trait IForegroundTransferImpl: Sized {
    fn AllowForegroundTransfer();
}
impl ::windows::core::RuntimeName for IForegroundTransfer {
    const NAME: &'static str = "Windows.Win32.System.Com.IForegroundTransfer";
}
impl IForegroundTransferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IForegroundTransferImpl, const OFFSET: isize>() -> IForegroundTransferVtbl {
        unsafe extern "system" fn AllowForegroundTransfer<Impl: IForegroundTransferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpvreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowForegroundTransfer(&*(&lpvreserved as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IForegroundTransfer>, ::windows::core::GetTrustLevel, AllowForegroundTransfer::<Impl, OFFSET>)
    }
}
pub trait IGlobalInterfaceTableImpl: Sized {
    fn RegisterInterfaceInGlobal();
    fn RevokeInterfaceFromGlobal();
    fn GetInterfaceFromGlobal();
}
impl ::windows::core::RuntimeName for IGlobalInterfaceTable {
    const NAME: &'static str = "Windows.Win32.System.Com.IGlobalInterfaceTable";
}
impl IGlobalInterfaceTableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGlobalInterfaceTableImpl, const OFFSET: isize>() -> IGlobalInterfaceTableVtbl {
        unsafe extern "system" fn RegisterInterfaceInGlobal<Impl: IGlobalInterfaceTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterInterfaceInGlobal(&*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RevokeInterfaceFromGlobal<Impl: IGlobalInterfaceTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RevokeInterfaceFromGlobal(dwcookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInterfaceFromGlobal<Impl: IGlobalInterfaceTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInterfaceFromGlobal(dwcookie, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGlobalInterfaceTable>, ::windows::core::GetTrustLevel, RegisterInterfaceInGlobal::<Impl, OFFSET>, RevokeInterfaceFromGlobal::<Impl, OFFSET>, GetInterfaceFromGlobal::<Impl, OFFSET>)
    }
}
pub trait IGlobalOptionsImpl: Sized {
    fn Set();
    fn Query();
}
impl ::windows::core::RuntimeName for IGlobalOptions {
    const NAME: &'static str = "Windows.Win32.System.Com.IGlobalOptions";
}
impl IGlobalOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGlobalOptionsImpl, const OFFSET: isize>() -> IGlobalOptionsVtbl {
        unsafe extern "system" fn Set<Impl: IGlobalOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwproperty: GLOBALOPT_PROPERTIES, dwvalue: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Set(dwproperty, dwvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Query<Impl: IGlobalOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwproperty: GLOBALOPT_PROPERTIES, pdwvalue: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Query(dwproperty, ::core::mem::transmute_copy(&pdwvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGlobalOptions>, ::windows::core::GetTrustLevel, Set::<Impl, OFFSET>, Query::<Impl, OFFSET>)
    }
}
pub trait IInitializeSpyImpl: Sized {
    fn PreInitialize();
    fn PostInitialize();
    fn PreUninitialize();
    fn PostUninitialize();
}
impl ::windows::core::RuntimeName for IInitializeSpy {
    const NAME: &'static str = "Windows.Win32.System.Com.IInitializeSpy";
}
impl IInitializeSpyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInitializeSpyImpl, const OFFSET: isize>() -> IInitializeSpyVtbl {
        unsafe extern "system" fn PreInitialize<Impl: IInitializeSpyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcoinit: u32, dwcurthreadaptrefs: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreInitialize(dwcoinit, dwcurthreadaptrefs) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PostInitialize<Impl: IInitializeSpyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrcoinit: ::windows::core::HRESULT, dwcoinit: u32, dwnewthreadaptrefs: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PostInitialize(hrcoinit, dwcoinit, dwnewthreadaptrefs) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreUninitialize<Impl: IInitializeSpyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcurthreadaptrefs: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreUninitialize(dwcurthreadaptrefs) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PostUninitialize<Impl: IInitializeSpyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwnewthreadaptrefs: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PostUninitialize(dwnewthreadaptrefs) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInitializeSpy>, ::windows::core::GetTrustLevel, PreInitialize::<Impl, OFFSET>, PostInitialize::<Impl, OFFSET>, PreUninitialize::<Impl, OFFSET>, PostUninitialize::<Impl, OFFSET>)
    }
}
pub trait IInternalUnknownImpl: Sized {
    fn QueryInternalInterface();
}
impl ::windows::core::RuntimeName for IInternalUnknown {
    const NAME: &'static str = "Windows.Win32.System.Com.IInternalUnknown";
}
impl IInternalUnknownVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternalUnknownImpl, const OFFSET: isize>() -> IInternalUnknownVtbl {
        unsafe extern "system" fn QueryInternalInterface<Impl: IInternalUnknownImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryInternalInterface(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInternalUnknown>, ::windows::core::GetTrustLevel, QueryInternalInterface::<Impl, OFFSET>)
    }
}
pub trait IMachineGlobalObjectTableImpl: Sized {
    fn RegisterObject();
    fn GetObject();
    fn RevokeObject();
}
impl ::windows::core::RuntimeName for IMachineGlobalObjectTable {
    const NAME: &'static str = "Windows.Win32.System.Com.IMachineGlobalObjectTable";
}
impl IMachineGlobalObjectTableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMachineGlobalObjectTableImpl, const OFFSET: isize>() -> IMachineGlobalObjectTableVtbl {
        unsafe extern "system" fn RegisterObject<Impl: IMachineGlobalObjectTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, identifier: super::super::Foundation::PWSTR, object: *mut ::core::ffi::c_void, token: *mut *mut MachineGlobalObjectTableRegistrationToken__) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterObject(
                &*(&clsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&identifier as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&object as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&token),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObject<Impl: IMachineGlobalObjectTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, identifier: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObject(
                &*(&clsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&identifier as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppv),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RevokeObject<Impl: IMachineGlobalObjectTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: *const MachineGlobalObjectTableRegistrationToken__) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RevokeObject(&*(&token as *const <MachineGlobalObjectTableRegistrationToken__ as ::windows::core::Abi>::Abi as *const <MachineGlobalObjectTableRegistrationToken__ as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMachineGlobalObjectTable>, ::windows::core::GetTrustLevel, RegisterObject::<Impl, OFFSET>, GetObject::<Impl, OFFSET>, RevokeObject::<Impl, OFFSET>)
    }
}
pub trait IMallocImpl: Sized {
    fn Alloc();
    fn Realloc();
    fn Free();
    fn GetSize();
    fn DidAlloc();
    fn HeapMinimize();
}
impl ::windows::core::RuntimeName for IMalloc {
    const NAME: &'static str = "Windows.Win32.System.Com.IMalloc";
}
impl IMallocVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMallocImpl, const OFFSET: isize>() -> IMallocVtbl {
        unsafe extern "system" fn Alloc<Impl: IMallocImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Alloc(cb) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Realloc<Impl: IMallocImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Realloc(&*(&pv as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), cb) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Free<Impl: IMallocImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Free(&*(&pv as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetSize<Impl: IMallocImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void) -> usize {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSize(&*(&pv as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DidAlloc<Impl: IMallocImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DidAlloc(&*(&pv as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeapMinimize<Impl: IMallocImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HeapMinimize().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMalloc>, ::windows::core::GetTrustLevel, Alloc::<Impl, OFFSET>, Realloc::<Impl, OFFSET>, Free::<Impl, OFFSET>, GetSize::<Impl, OFFSET>, DidAlloc::<Impl, OFFSET>, HeapMinimize::<Impl, OFFSET>)
    }
}
pub trait IMallocSpyImpl: Sized {
    fn PreAlloc();
    fn PostAlloc();
    fn PreFree();
    fn PostFree();
    fn PreRealloc();
    fn PostRealloc();
    fn PreGetSize();
    fn PostGetSize();
    fn PreDidAlloc();
    fn PostDidAlloc();
    fn PreHeapMinimize();
    fn PostHeapMinimize();
}
impl ::windows::core::RuntimeName for IMallocSpy {
    const NAME: &'static str = "Windows.Win32.System.Com.IMallocSpy";
}
impl IMallocSpyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMallocSpyImpl, const OFFSET: isize>() -> IMallocSpyVtbl {
        unsafe extern "system" fn PreAlloc<Impl: IMallocSpyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbrequest: usize) -> usize {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreAlloc(cbrequest) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PostAlloc<Impl: IMallocSpyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pactual: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PostAlloc(&*(&pactual as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreFree<Impl: IMallocSpyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreFree(&*(&prequest as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&fspyed as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PostFree<Impl: IMallocSpyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PostFree(&*(&fspyed as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PreRealloc<Impl: IMallocSpyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, cbrequest: usize, ppnewrequest: *mut *mut ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> usize {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreRealloc(&*(&prequest as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), cbrequest, ::core::mem::transmute_copy(&ppnewrequest), &*(&fspyed as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PostRealloc<Impl: IMallocSpyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pactual: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PostRealloc(&*(&pactual as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&fspyed as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreGetSize<Impl: IMallocSpyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreGetSize(&*(&prequest as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&fspyed as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PostGetSize<Impl: IMallocSpyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbactual: usize, fspyed: super::super::Foundation::BOOL) -> usize {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PostGetSize(cbactual, &*(&fspyed as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreDidAlloc<Impl: IMallocSpyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreDidAlloc(&*(&prequest as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&fspyed as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PostDidAlloc<Impl: IMallocSpyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL, factual: i32) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PostDidAlloc(&*(&prequest as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&fspyed as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), factual) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreHeapMinimize<Impl: IMallocSpyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PreHeapMinimize().into()
        }
        unsafe extern "system" fn PostHeapMinimize<Impl: IMallocSpyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PostHeapMinimize().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMallocSpy>,
            ::windows::core::GetTrustLevel,
            PreAlloc::<Impl, OFFSET>,
            PostAlloc::<Impl, OFFSET>,
            PreFree::<Impl, OFFSET>,
            PostFree::<Impl, OFFSET>,
            PreRealloc::<Impl, OFFSET>,
            PostRealloc::<Impl, OFFSET>,
            PreGetSize::<Impl, OFFSET>,
            PostGetSize::<Impl, OFFSET>,
            PreDidAlloc::<Impl, OFFSET>,
            PostDidAlloc::<Impl, OFFSET>,
            PreHeapMinimize::<Impl, OFFSET>,
            PostHeapMinimize::<Impl, OFFSET>,
        )
    }
}
pub trait IMonikerImpl: Sized + IPersistStreamImpl + IPersistImpl {
    fn BindToObject();
    fn BindToStorage();
    fn Reduce();
    fn ComposeWith();
    fn Enum();
    fn IsEqual();
    fn Hash();
    fn IsRunning();
    fn GetTimeOfLastChange();
    fn Inverse();
    fn CommonPrefixWith();
    fn RelativePathTo();
    fn GetDisplayName();
    fn ParseDisplayName();
    fn IsSystemMoniker();
}
impl ::windows::core::RuntimeName for IMoniker {
    const NAME: &'static str = "Windows.Win32.System.Com.IMoniker";
}
impl IMonikerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMonikerImpl, const OFFSET: isize>() -> IMonikerVtbl {
        unsafe extern "system" fn BindToObject<Impl: IMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, pmktoleft: ::windows::core::RawPtr, riidresult: *const ::windows::core::GUID, ppvresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BindToObject(&*(&pbc as *const <IBindCtx as ::windows::core::Abi>::Abi as *const <IBindCtx as ::windows::core::DefaultType>::DefaultType), &*(&pmktoleft as *const <IMoniker as ::windows::core::Abi>::Abi as *const <IMoniker as ::windows::core::DefaultType>::DefaultType), &*(&riidresult as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindToStorage<Impl: IMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, pmktoleft: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BindToStorage(&*(&pbc as *const <IBindCtx as ::windows::core::Abi>::Abi as *const <IBindCtx as ::windows::core::DefaultType>::DefaultType), &*(&pmktoleft as *const <IMoniker as ::windows::core::Abi>::Abi as *const <IMoniker as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvobj)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reduce<Impl: IMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, dwreducehowfar: u32, ppmktoleft: *mut ::windows::core::RawPtr, ppmkreduced: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reduce(&*(&pbc as *const <IBindCtx as ::windows::core::Abi>::Abi as *const <IBindCtx as ::windows::core::DefaultType>::DefaultType), dwreducehowfar, &*(&ppmktoleft as *const <IMoniker as ::windows::core::Abi>::Abi as *const <IMoniker as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppmkreduced)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComposeWith<Impl: IMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmkright: ::windows::core::RawPtr, fonlyifnotgeneric: super::super::Foundation::BOOL, ppmkcomposite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComposeWith(&*(&pmkright as *const <IMoniker as ::windows::core::Abi>::Abi as *const <IMoniker as ::windows::core::DefaultType>::DefaultType), &*(&fonlyifnotgeneric as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppmkcomposite)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enum<Impl: IMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fforward: super::super::Foundation::BOOL, ppenummoniker: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enum(&*(&fforward as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppenummoniker)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Impl: IMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmkothermoniker: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqual(&*(&pmkothermoniker as *const <IMoniker as ::windows::core::Abi>::Abi as *const <IMoniker as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Hash<Impl: IMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwhash: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Hash(::core::mem::transmute_copy(&pdwhash)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRunning<Impl: IMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, pmktoleft: ::windows::core::RawPtr, pmknewlyrunning: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRunning(&*(&pbc as *const <IBindCtx as ::windows::core::Abi>::Abi as *const <IBindCtx as ::windows::core::DefaultType>::DefaultType), &*(&pmktoleft as *const <IMoniker as ::windows::core::Abi>::Abi as *const <IMoniker as ::windows::core::DefaultType>::DefaultType), &*(&pmknewlyrunning as *const <IMoniker as ::windows::core::Abi>::Abi as *const <IMoniker as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTimeOfLastChange<Impl: IMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, pmktoleft: ::windows::core::RawPtr, pfiletime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTimeOfLastChange(&*(&pbc as *const <IBindCtx as ::windows::core::Abi>::Abi as *const <IBindCtx as ::windows::core::DefaultType>::DefaultType), &*(&pmktoleft as *const <IMoniker as ::windows::core::Abi>::Abi as *const <IMoniker as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfiletime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Inverse<Impl: IMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Inverse(::core::mem::transmute_copy(&ppmk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommonPrefixWith<Impl: IMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmkother: ::windows::core::RawPtr, ppmkprefix: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommonPrefixWith(&*(&pmkother as *const <IMoniker as ::windows::core::Abi>::Abi as *const <IMoniker as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppmkprefix)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RelativePathTo<Impl: IMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmkother: ::windows::core::RawPtr, ppmkrelpath: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RelativePathTo(&*(&pmkother as *const <IMoniker as ::windows::core::Abi>::Abi as *const <IMoniker as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppmkrelpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayName<Impl: IMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, pmktoleft: ::windows::core::RawPtr, ppszdisplayname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplayName(&*(&pbc as *const <IBindCtx as ::windows::core::Abi>::Abi as *const <IBindCtx as ::windows::core::DefaultType>::DefaultType), &*(&pmktoleft as *const <IMoniker as ::windows::core::Abi>::Abi as *const <IMoniker as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppszdisplayname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParseDisplayName<Impl: IMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, pmktoleft: ::windows::core::RawPtr, pszdisplayname: super::super::Foundation::PWSTR, pcheaten: *mut u32, ppmkout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ParseDisplayName(
                &*(&pbc as *const <IBindCtx as ::windows::core::Abi>::Abi as *const <IBindCtx as ::windows::core::DefaultType>::DefaultType),
                &*(&pmktoleft as *const <IMoniker as ::windows::core::Abi>::Abi as *const <IMoniker as ::windows::core::DefaultType>::DefaultType),
                &*(&pszdisplayname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pcheaten),
                ::core::mem::transmute_copy(&ppmkout),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSystemMoniker<Impl: IMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmksys: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSystemMoniker(::core::mem::transmute_copy(&pdwmksys)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMoniker>,
            ::windows::core::GetTrustLevel,
            BindToObject::<Impl, OFFSET>,
            BindToStorage::<Impl, OFFSET>,
            Reduce::<Impl, OFFSET>,
            ComposeWith::<Impl, OFFSET>,
            Enum::<Impl, OFFSET>,
            IsEqual::<Impl, OFFSET>,
            Hash::<Impl, OFFSET>,
            IsRunning::<Impl, OFFSET>,
            GetTimeOfLastChange::<Impl, OFFSET>,
            Inverse::<Impl, OFFSET>,
            CommonPrefixWith::<Impl, OFFSET>,
            RelativePathTo::<Impl, OFFSET>,
            GetDisplayName::<Impl, OFFSET>,
            ParseDisplayName::<Impl, OFFSET>,
            IsSystemMoniker::<Impl, OFFSET>,
        )
    }
}
pub trait IMultiQIImpl: Sized {
    fn QueryMultipleInterfaces();
}
impl ::windows::core::RuntimeName for IMultiQI {
    const NAME: &'static str = "Windows.Win32.System.Com.IMultiQI";
}
impl IMultiQIVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultiQIImpl, const OFFSET: isize>() -> IMultiQIVtbl {
        unsafe extern "system" fn QueryMultipleInterfaces<Impl: IMultiQIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cmqis: u32, pmqis: *mut MULTI_QI) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryMultipleInterfaces(cmqis, &*(&pmqis as *const <MULTI_QI as ::windows::core::Abi>::Abi as *const <MULTI_QI as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMultiQI>, ::windows::core::GetTrustLevel, QueryMultipleInterfaces::<Impl, OFFSET>)
    }
}
pub trait INoMarshalImpl: Sized {}
impl ::windows::core::RuntimeName for INoMarshal {
    const NAME: &'static str = "Windows.Win32.System.Com.INoMarshal";
}
impl INoMarshalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INoMarshalImpl, const OFFSET: isize>() -> INoMarshalVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INoMarshal>, ::windows::core::GetTrustLevel)
    }
}
pub trait IOplockStorageImpl: Sized {
    fn CreateStorageEx();
    fn OpenStorageEx();
}
impl ::windows::core::RuntimeName for IOplockStorage {
    const NAME: &'static str = "Windows.Win32.System.Com.IOplockStorage";
}
impl IOplockStorageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOplockStorageImpl, const OFFSET: isize>() -> IOplockStorageVtbl {
        unsafe extern "system" fn CreateStorageEx<Impl: IOplockStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsname: super::super::Foundation::PWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const ::windows::core::GUID, ppstgopen: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStorageEx(&*(&pwcsname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), grfmode, stgfmt, grfattrs, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppstgopen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenStorageEx<Impl: IOplockStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsname: super::super::Foundation::PWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const ::windows::core::GUID, ppstgopen: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenStorageEx(&*(&pwcsname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), grfmode, stgfmt, grfattrs, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppstgopen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOplockStorage>, ::windows::core::GetTrustLevel, CreateStorageEx::<Impl, OFFSET>, OpenStorageEx::<Impl, OFFSET>)
    }
}
pub trait IPSFactoryBufferImpl: Sized {
    fn CreateProxy();
    fn CreateStub();
}
impl ::windows::core::RuntimeName for IPSFactoryBuffer {
    const NAME: &'static str = "Windows.Win32.System.Com.IPSFactoryBuffer";
}
impl IPSFactoryBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPSFactoryBufferImpl, const OFFSET: isize>() -> IPSFactoryBufferVtbl {
        unsafe extern "system" fn CreateProxy<Impl: IPSFactoryBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppproxy: *mut ::windows::core::RawPtr, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateProxy(&*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppproxy), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStub<Impl: IPSFactoryBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punkserver: *mut ::core::ffi::c_void, ppstub: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStub(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&punkserver as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppstub)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPSFactoryBuffer>, ::windows::core::GetTrustLevel, CreateProxy::<Impl, OFFSET>, CreateStub::<Impl, OFFSET>)
    }
}
pub trait IPersistImpl: Sized {
    fn GetClassID();
}
impl ::windows::core::RuntimeName for IPersist {
    const NAME: &'static str = "Windows.Win32.System.Com.IPersist";
}
impl IPersistVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersistImpl, const OFFSET: isize>() -> IPersistVtbl {
        unsafe extern "system" fn GetClassID<Impl: IPersistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclassid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClassID(::core::mem::transmute_copy(&pclassid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPersist>, ::windows::core::GetTrustLevel, GetClassID::<Impl, OFFSET>)
    }
}
pub trait IPersistFileImpl: Sized + IPersistImpl {
    fn IsDirty();
    fn Load();
    fn Save();
    fn SaveCompleted();
    fn GetCurFile();
}
impl ::windows::core::RuntimeName for IPersistFile {
    const NAME: &'static str = "Windows.Win32.System.Com.IPersistFile";
}
impl IPersistFileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersistFileImpl, const OFFSET: isize>() -> IPersistFileVtbl {
        unsafe extern "system" fn IsDirty<Impl: IPersistFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDirty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Load<Impl: IPersistFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilename: super::super::Foundation::PWSTR, dwmode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Load(&*(&pszfilename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Save<Impl: IPersistFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilename: super::super::Foundation::PWSTR, fremember: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Save(&*(&pszfilename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&fremember as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveCompleted<Impl: IPersistFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveCompleted(&*(&pszfilename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurFile<Impl: IPersistFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszfilename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurFile(::core::mem::transmute_copy(&ppszfilename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPersistFile>, ::windows::core::GetTrustLevel, IsDirty::<Impl, OFFSET>, Load::<Impl, OFFSET>, Save::<Impl, OFFSET>, SaveCompleted::<Impl, OFFSET>, GetCurFile::<Impl, OFFSET>)
    }
}
pub trait IPersistMemoryImpl: Sized + IPersistImpl {
    fn IsDirty();
    fn Load();
    fn Save();
    fn GetSizeMax();
    fn InitNew();
}
impl ::windows::core::RuntimeName for IPersistMemory {
    const NAME: &'static str = "Windows.Win32.System.Com.IPersistMemory";
}
impl IPersistMemoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersistMemoryImpl, const OFFSET: isize>() -> IPersistMemoryVtbl {
        unsafe extern "system" fn IsDirty<Impl: IPersistMemoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDirty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Load<Impl: IPersistMemoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmem: *const ::core::ffi::c_void, cbsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Load(&*(&pmem as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), cbsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Save<Impl: IPersistMemoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmem: *mut ::core::ffi::c_void, fcleardirty: super::super::Foundation::BOOL, cbsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Save(::core::mem::transmute_copy(&pmem), &*(&fcleardirty as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), cbsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSizeMax<Impl: IPersistMemoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSizeMax(::core::mem::transmute_copy(&pcbsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitNew<Impl: IPersistMemoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitNew() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPersistMemory>, ::windows::core::GetTrustLevel, IsDirty::<Impl, OFFSET>, Load::<Impl, OFFSET>, Save::<Impl, OFFSET>, GetSizeMax::<Impl, OFFSET>, InitNew::<Impl, OFFSET>)
    }
}
pub trait IPersistStreamImpl: Sized + IPersistImpl {
    fn IsDirty();
    fn Load();
    fn Save();
    fn GetSizeMax();
}
impl ::windows::core::RuntimeName for IPersistStream {
    const NAME: &'static str = "Windows.Win32.System.Com.IPersistStream";
}
impl IPersistStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersistStreamImpl, const OFFSET: isize>() -> IPersistStreamVtbl {
        unsafe extern "system" fn IsDirty<Impl: IPersistStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDirty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Load<Impl: IPersistStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Load(&*(&pstm as *const <IStream as ::windows::core::Abi>::Abi as *const <IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Save<Impl: IPersistStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr, fcleardirty: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Save(&*(&pstm as *const <IStream as ::windows::core::Abi>::Abi as *const <IStream as ::windows::core::DefaultType>::DefaultType), &*(&fcleardirty as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSizeMax<Impl: IPersistStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbsize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSizeMax(::core::mem::transmute_copy(&pcbsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPersistStream>, ::windows::core::GetTrustLevel, IsDirty::<Impl, OFFSET>, Load::<Impl, OFFSET>, Save::<Impl, OFFSET>, GetSizeMax::<Impl, OFFSET>)
    }
}
pub trait IPersistStreamInitImpl: Sized + IPersistImpl {
    fn IsDirty();
    fn Load();
    fn Save();
    fn GetSizeMax();
    fn InitNew();
}
impl ::windows::core::RuntimeName for IPersistStreamInit {
    const NAME: &'static str = "Windows.Win32.System.Com.IPersistStreamInit";
}
impl IPersistStreamInitVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersistStreamInitImpl, const OFFSET: isize>() -> IPersistStreamInitVtbl {
        unsafe extern "system" fn IsDirty<Impl: IPersistStreamInitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDirty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Load<Impl: IPersistStreamInitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Load(&*(&pstm as *const <IStream as ::windows::core::Abi>::Abi as *const <IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Save<Impl: IPersistStreamInitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr, fcleardirty: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Save(&*(&pstm as *const <IStream as ::windows::core::Abi>::Abi as *const <IStream as ::windows::core::DefaultType>::DefaultType), &*(&fcleardirty as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSizeMax<Impl: IPersistStreamInitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbsize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSizeMax(::core::mem::transmute_copy(&pcbsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitNew<Impl: IPersistStreamInitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitNew() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPersistStreamInit>, ::windows::core::GetTrustLevel, IsDirty::<Impl, OFFSET>, Load::<Impl, OFFSET>, Save::<Impl, OFFSET>, GetSizeMax::<Impl, OFFSET>, InitNew::<Impl, OFFSET>)
    }
}
pub trait IPipeByteImpl: Sized {
    fn Pull();
    fn Push();
}
impl ::windows::core::RuntimeName for IPipeByte {
    const NAME: &'static str = "Windows.Win32.System.Com.IPipeByte";
}
impl IPipeByteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPipeByteImpl, const OFFSET: isize>() -> IPipeByteVtbl {
        unsafe extern "system" fn Pull<Impl: IPipeByteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *mut u8, crequest: u32, pcreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pull(::core::mem::transmute_copy(&buf), crequest, ::core::mem::transmute_copy(&pcreturned)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Push<Impl: IPipeByteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *const u8, csent: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Push(buf, csent) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPipeByte>, ::windows::core::GetTrustLevel, Pull::<Impl, OFFSET>, Push::<Impl, OFFSET>)
    }
}
pub trait IPipeDoubleImpl: Sized {
    fn Pull();
    fn Push();
}
impl ::windows::core::RuntimeName for IPipeDouble {
    const NAME: &'static str = "Windows.Win32.System.Com.IPipeDouble";
}
impl IPipeDoubleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPipeDoubleImpl, const OFFSET: isize>() -> IPipeDoubleVtbl {
        unsafe extern "system" fn Pull<Impl: IPipeDoubleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *mut f64, crequest: u32, pcreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pull(::core::mem::transmute_copy(&buf), crequest, ::core::mem::transmute_copy(&pcreturned)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Push<Impl: IPipeDoubleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *const f64, csent: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Push(buf, csent) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPipeDouble>, ::windows::core::GetTrustLevel, Pull::<Impl, OFFSET>, Push::<Impl, OFFSET>)
    }
}
pub trait IPipeLongImpl: Sized {
    fn Pull();
    fn Push();
}
impl ::windows::core::RuntimeName for IPipeLong {
    const NAME: &'static str = "Windows.Win32.System.Com.IPipeLong";
}
impl IPipeLongVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPipeLongImpl, const OFFSET: isize>() -> IPipeLongVtbl {
        unsafe extern "system" fn Pull<Impl: IPipeLongImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *mut i32, crequest: u32, pcreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pull(::core::mem::transmute_copy(&buf), crequest, ::core::mem::transmute_copy(&pcreturned)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Push<Impl: IPipeLongImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *const i32, csent: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Push(buf, csent) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPipeLong>, ::windows::core::GetTrustLevel, Pull::<Impl, OFFSET>, Push::<Impl, OFFSET>)
    }
}
pub trait IProcessInitControlImpl: Sized {
    fn ResetInitializerTimeout();
}
impl ::windows::core::RuntimeName for IProcessInitControl {
    const NAME: &'static str = "Windows.Win32.System.Com.IProcessInitControl";
}
impl IProcessInitControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessInitControlImpl, const OFFSET: isize>() -> IProcessInitControlVtbl {
        unsafe extern "system" fn ResetInitializerTimeout<Impl: IProcessInitControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsecondsremaining: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResetInitializerTimeout(dwsecondsremaining) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProcessInitControl>, ::windows::core::GetTrustLevel, ResetInitializerTimeout::<Impl, OFFSET>)
    }
}
pub trait IProcessLockImpl: Sized {
    fn AddRefOnProcess();
    fn ReleaseRefOnProcess();
}
impl ::windows::core::RuntimeName for IProcessLock {
    const NAME: &'static str = "Windows.Win32.System.Com.IProcessLock";
}
impl IProcessLockVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessLockImpl, const OFFSET: isize>() -> IProcessLockVtbl {
        unsafe extern "system" fn AddRefOnProcess<Impl: IProcessLockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddRefOnProcess() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseRefOnProcess<Impl: IProcessLockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseRefOnProcess() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProcessLock>, ::windows::core::GetTrustLevel, AddRefOnProcess::<Impl, OFFSET>, ReleaseRefOnProcess::<Impl, OFFSET>)
    }
}
pub trait IProgressNotifyImpl: Sized {
    fn OnProgress();
}
impl ::windows::core::RuntimeName for IProgressNotify {
    const NAME: &'static str = "Windows.Win32.System.Com.IProgressNotify";
}
impl IProgressNotifyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProgressNotifyImpl, const OFFSET: isize>() -> IProgressNotifyVtbl {
        unsafe extern "system" fn OnProgress<Impl: IProgressNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprogresscurrent: u32, dwprogressmaximum: u32, faccurate: super::super::Foundation::BOOL, fowner: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnProgress(dwprogresscurrent, dwprogressmaximum, &*(&faccurate as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), &*(&fowner as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProgressNotify>, ::windows::core::GetTrustLevel, OnProgress::<Impl, OFFSET>)
    }
}
pub trait IROTDataImpl: Sized {
    fn GetComparisonData();
}
impl ::windows::core::RuntimeName for IROTData {
    const NAME: &'static str = "Windows.Win32.System.Com.IROTData";
}
impl IROTDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IROTDataImpl, const OFFSET: isize>() -> IROTDataVtbl {
        unsafe extern "system" fn GetComparisonData<Impl: IROTDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdata: *mut u8, cbmax: u32, pcbdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetComparisonData(::core::mem::transmute_copy(&pbdata), cbmax, ::core::mem::transmute_copy(&pcbdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IROTData>, ::windows::core::GetTrustLevel, GetComparisonData::<Impl, OFFSET>)
    }
}
pub trait IReleaseMarshalBuffersImpl: Sized {
    fn ReleaseMarshalBuffer();
}
impl ::windows::core::RuntimeName for IReleaseMarshalBuffers {
    const NAME: &'static str = "Windows.Win32.System.Com.IReleaseMarshalBuffers";
}
impl IReleaseMarshalBuffersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReleaseMarshalBuffersImpl, const OFFSET: isize>() -> IReleaseMarshalBuffersVtbl {
        unsafe extern "system" fn ReleaseMarshalBuffer<Impl: IReleaseMarshalBuffersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, dwflags: u32, pchnl: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseMarshalBuffer(&*(&pmsg as *const <RPCOLEMESSAGE as ::windows::core::Abi>::Abi as *const <RPCOLEMESSAGE as ::windows::core::DefaultType>::DefaultType), dwflags, &*(&pchnl as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IReleaseMarshalBuffers>, ::windows::core::GetTrustLevel, ReleaseMarshalBuffer::<Impl, OFFSET>)
    }
}
pub trait IRpcChannelBufferImpl: Sized {
    fn GetBuffer();
    fn SendReceive();
    fn FreeBuffer();
    fn GetDestCtx();
    fn IsConnected();
}
impl ::windows::core::RuntimeName for IRpcChannelBuffer {
    const NAME: &'static str = "Windows.Win32.System.Com.IRpcChannelBuffer";
}
impl IRpcChannelBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRpcChannelBufferImpl, const OFFSET: isize>() -> IRpcChannelBufferVtbl {
        unsafe extern "system" fn GetBuffer<Impl: IRpcChannelBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBuffer(&*(&pmessage as *const <RPCOLEMESSAGE as ::windows::core::Abi>::Abi as *const <RPCOLEMESSAGE as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendReceive<Impl: IRpcChannelBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendReceive(&*(&pmessage as *const <RPCOLEMESSAGE as ::windows::core::Abi>::Abi as *const <RPCOLEMESSAGE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeBuffer<Impl: IRpcChannelBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FreeBuffer(&*(&pmessage as *const <RPCOLEMESSAGE as ::windows::core::Abi>::Abi as *const <RPCOLEMESSAGE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestCtx<Impl: IRpcChannelBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDestCtx(::core::mem::transmute_copy(&pdwdestcontext), ::core::mem::transmute_copy(&ppvdestcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnected<Impl: IRpcChannelBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsConnected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRpcChannelBuffer>, ::windows::core::GetTrustLevel, GetBuffer::<Impl, OFFSET>, SendReceive::<Impl, OFFSET>, FreeBuffer::<Impl, OFFSET>, GetDestCtx::<Impl, OFFSET>, IsConnected::<Impl, OFFSET>)
    }
}
pub trait IRpcChannelBuffer2Impl: Sized + IRpcChannelBufferImpl {
    fn GetProtocolVersion();
}
impl ::windows::core::RuntimeName for IRpcChannelBuffer2 {
    const NAME: &'static str = "Windows.Win32.System.Com.IRpcChannelBuffer2";
}
impl IRpcChannelBuffer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRpcChannelBuffer2Impl, const OFFSET: isize>() -> IRpcChannelBuffer2Vtbl {
        unsafe extern "system" fn GetProtocolVersion<Impl: IRpcChannelBuffer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProtocolVersion(::core::mem::transmute_copy(&pdwversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRpcChannelBuffer2>, ::windows::core::GetTrustLevel, GetProtocolVersion::<Impl, OFFSET>)
    }
}
pub trait IRpcChannelBuffer3Impl: Sized + IRpcChannelBuffer2Impl + IRpcChannelBufferImpl {
    fn Send();
    fn Receive();
    fn Cancel();
    fn GetCallContext();
    fn GetDestCtxEx();
    fn GetState();
    fn RegisterAsync();
}
impl ::windows::core::RuntimeName for IRpcChannelBuffer3 {
    const NAME: &'static str = "Windows.Win32.System.Com.IRpcChannelBuffer3";
}
impl IRpcChannelBuffer3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRpcChannelBuffer3Impl, const OFFSET: isize>() -> IRpcChannelBuffer3Vtbl {
        unsafe extern "system" fn Send<Impl: IRpcChannelBuffer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Send(&*(&pmsg as *const <RPCOLEMESSAGE as ::windows::core::Abi>::Abi as *const <RPCOLEMESSAGE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pulstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Receive<Impl: IRpcChannelBuffer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, ulsize: u32, pulstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Receive(&*(&pmsg as *const <RPCOLEMESSAGE as ::windows::core::Abi>::Abi as *const <RPCOLEMESSAGE as ::windows::core::DefaultType>::DefaultType), ulsize, ::core::mem::transmute_copy(&pulstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IRpcChannelBuffer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cancel(&*(&pmsg as *const <RPCOLEMESSAGE as ::windows::core::Abi>::Abi as *const <RPCOLEMESSAGE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCallContext<Impl: IRpcChannelBuffer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCallContext(&*(&pmsg as *const <RPCOLEMESSAGE as ::windows::core::Abi>::Abi as *const <RPCOLEMESSAGE as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestCtxEx<Impl: IRpcChannelBuffer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDestCtxEx(&*(&pmsg as *const <RPCOLEMESSAGE as ::windows::core::Abi>::Abi as *const <RPCOLEMESSAGE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwdestcontext), ::core::mem::transmute_copy(&ppvdestcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetState<Impl: IRpcChannelBuffer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetState(&*(&pmsg as *const <RPCOLEMESSAGE as ::windows::core::Abi>::Abi as *const <RPCOLEMESSAGE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterAsync<Impl: IRpcChannelBuffer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pasyncmgr: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterAsync(&*(&pmsg as *const <RPCOLEMESSAGE as ::windows::core::Abi>::Abi as *const <RPCOLEMESSAGE as ::windows::core::DefaultType>::DefaultType), &*(&pasyncmgr as *const <IAsyncManager as ::windows::core::Abi>::Abi as *const <IAsyncManager as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRpcChannelBuffer3>, ::windows::core::GetTrustLevel, Send::<Impl, OFFSET>, Receive::<Impl, OFFSET>, Cancel::<Impl, OFFSET>, GetCallContext::<Impl, OFFSET>, GetDestCtxEx::<Impl, OFFSET>, GetState::<Impl, OFFSET>, RegisterAsync::<Impl, OFFSET>)
    }
}
pub trait IRpcHelperImpl: Sized {
    fn GetDCOMProtocolVersion();
    fn GetIIDFromOBJREF();
}
impl ::windows::core::RuntimeName for IRpcHelper {
    const NAME: &'static str = "Windows.Win32.System.Com.IRpcHelper";
}
impl IRpcHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRpcHelperImpl, const OFFSET: isize>() -> IRpcHelperVtbl {
        unsafe extern "system" fn GetDCOMProtocolVersion<Impl: IRpcHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcomversion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDCOMProtocolVersion(::core::mem::transmute_copy(&pcomversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIIDFromOBJREF<Impl: IRpcHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjref: *const ::core::ffi::c_void, piid: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIIDFromOBJREF(&*(&pobjref as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&piid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRpcHelper>, ::windows::core::GetTrustLevel, GetDCOMProtocolVersion::<Impl, OFFSET>, GetIIDFromOBJREF::<Impl, OFFSET>)
    }
}
pub trait IRpcOptionsImpl: Sized {
    fn Set();
    fn Query();
}
impl ::windows::core::RuntimeName for IRpcOptions {
    const NAME: &'static str = "Windows.Win32.System.Com.IRpcOptions";
}
impl IRpcOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRpcOptionsImpl, const OFFSET: isize>() -> IRpcOptionsVtbl {
        unsafe extern "system" fn Set<Impl: IRpcOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprx: *mut ::core::ffi::c_void, dwproperty: RPCOPT_PROPERTIES, dwvalue: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Set(&*(&pprx as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), dwproperty, dwvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Query<Impl: IRpcOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprx: *mut ::core::ffi::c_void, dwproperty: RPCOPT_PROPERTIES, pdwvalue: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Query(&*(&pprx as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), dwproperty, ::core::mem::transmute_copy(&pdwvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRpcOptions>, ::windows::core::GetTrustLevel, Set::<Impl, OFFSET>, Query::<Impl, OFFSET>)
    }
}
pub trait IRpcProxyBufferImpl: Sized {
    fn Connect();
    fn Disconnect();
}
impl ::windows::core::RuntimeName for IRpcProxyBuffer {
    const NAME: &'static str = "Windows.Win32.System.Com.IRpcProxyBuffer";
}
impl IRpcProxyBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRpcProxyBufferImpl, const OFFSET: isize>() -> IRpcProxyBufferVtbl {
        unsafe extern "system" fn Connect<Impl: IRpcProxyBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prpcchannelbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Connect(&*(&prpcchannelbuffer as *const <IRpcChannelBuffer as ::windows::core::Abi>::Abi as *const <IRpcChannelBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disconnect<Impl: IRpcProxyBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disconnect().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRpcProxyBuffer>, ::windows::core::GetTrustLevel, Connect::<Impl, OFFSET>, Disconnect::<Impl, OFFSET>)
    }
}
pub trait IRpcStubBufferImpl: Sized {
    fn Connect();
    fn Disconnect();
    fn Invoke();
    fn IsIIDSupported();
    fn CountRefs();
    fn DebugServerQueryInterface();
    fn DebugServerRelease();
}
impl ::windows::core::RuntimeName for IRpcStubBuffer {
    const NAME: &'static str = "Windows.Win32.System.Com.IRpcStubBuffer";
}
impl IRpcStubBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRpcStubBufferImpl, const OFFSET: isize>() -> IRpcStubBufferVtbl {
        unsafe extern "system" fn Connect<Impl: IRpcStubBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkserver: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Connect(&*(&punkserver as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disconnect<Impl: IRpcStubBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disconnect().into()
        }
        unsafe extern "system" fn Invoke<Impl: IRpcStubBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _prpcmsg: *mut RPCOLEMESSAGE, _prpcchannelbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Invoke(&*(&_prpcmsg as *const <RPCOLEMESSAGE as ::windows::core::Abi>::Abi as *const <RPCOLEMESSAGE as ::windows::core::DefaultType>::DefaultType), &*(&_prpcchannelbuffer as *const <IRpcChannelBuffer as ::windows::core::Abi>::Abi as *const <IRpcChannelBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsIIDSupported<Impl: IRpcStubBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID) -> ::core::option::Option<IRpcStubBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsIIDSupported(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CountRefs<Impl: IRpcStubBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CountRefs() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DebugServerQueryInterface<Impl: IRpcStubBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DebugServerQueryInterface(::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DebugServerRelease<Impl: IRpcStubBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DebugServerRelease(&*(&pv as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRpcStubBuffer>, ::windows::core::GetTrustLevel, Connect::<Impl, OFFSET>, Disconnect::<Impl, OFFSET>, Invoke::<Impl, OFFSET>, IsIIDSupported::<Impl, OFFSET>, CountRefs::<Impl, OFFSET>, DebugServerQueryInterface::<Impl, OFFSET>, DebugServerRelease::<Impl, OFFSET>)
    }
}
pub trait IRpcSyntaxNegotiateImpl: Sized {
    fn NegotiateSyntax();
}
impl ::windows::core::RuntimeName for IRpcSyntaxNegotiate {
    const NAME: &'static str = "Windows.Win32.System.Com.IRpcSyntaxNegotiate";
}
impl IRpcSyntaxNegotiateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRpcSyntaxNegotiateImpl, const OFFSET: isize>() -> IRpcSyntaxNegotiateVtbl {
        unsafe extern "system" fn NegotiateSyntax<Impl: IRpcSyntaxNegotiateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NegotiateSyntax(&*(&pmsg as *const <RPCOLEMESSAGE as ::windows::core::Abi>::Abi as *const <RPCOLEMESSAGE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRpcSyntaxNegotiate>, ::windows::core::GetTrustLevel, NegotiateSyntax::<Impl, OFFSET>)
    }
}
pub trait IRunnableObjectImpl: Sized {
    fn GetRunningClass();
    fn Run();
    fn IsRunning();
    fn LockRunning();
    fn SetContainedObject();
}
impl ::windows::core::RuntimeName for IRunnableObject {
    const NAME: &'static str = "Windows.Win32.System.Com.IRunnableObject";
}
impl IRunnableObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRunnableObjectImpl, const OFFSET: isize>() -> IRunnableObjectVtbl {
        unsafe extern "system" fn GetRunningClass<Impl: IRunnableObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRunningClass(::core::mem::transmute_copy(&lpclsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Run<Impl: IRunnableObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Run(&*(&pbc as *const <IBindCtx as ::windows::core::Abi>::Abi as *const <IBindCtx as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRunning<Impl: IRunnableObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRunning() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockRunning<Impl: IRunnableObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flock: super::super::Foundation::BOOL, flastunlockcloses: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LockRunning(&*(&flock as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), &*(&flastunlockcloses as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContainedObject<Impl: IRunnableObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fcontained: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetContainedObject(&*(&fcontained as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRunnableObject>, ::windows::core::GetTrustLevel, GetRunningClass::<Impl, OFFSET>, Run::<Impl, OFFSET>, IsRunning::<Impl, OFFSET>, LockRunning::<Impl, OFFSET>, SetContainedObject::<Impl, OFFSET>)
    }
}
pub trait IRunningObjectTableImpl: Sized {
    fn Register();
    fn Revoke();
    fn IsRunning();
    fn GetObject();
    fn NoteChangeTime();
    fn GetTimeOfLastChange();
    fn EnumRunning();
}
impl ::windows::core::RuntimeName for IRunningObjectTable {
    const NAME: &'static str = "Windows.Win32.System.Com.IRunningObjectTable";
}
impl IRunningObjectTableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRunningObjectTableImpl, const OFFSET: isize>() -> IRunningObjectTableVtbl {
        unsafe extern "system" fn Register<Impl: IRunningObjectTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfflags: u32, punkobject: *mut ::core::ffi::c_void, pmkobjectname: ::windows::core::RawPtr, pdwregister: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Register(grfflags, &*(&punkobject as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), &*(&pmkobjectname as *const <IMoniker as ::windows::core::Abi>::Abi as *const <IMoniker as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwregister)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Revoke<Impl: IRunningObjectTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwregister: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Revoke(dwregister) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRunning<Impl: IRunningObjectTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmkobjectname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRunning(&*(&pmkobjectname as *const <IMoniker as ::windows::core::Abi>::Abi as *const <IMoniker as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObject<Impl: IRunningObjectTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmkobjectname: ::windows::core::RawPtr, ppunkobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObject(&*(&pmkobjectname as *const <IMoniker as ::windows::core::Abi>::Abi as *const <IMoniker as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppunkobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NoteChangeTime<Impl: IRunningObjectTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwregister: u32, pfiletime: *const super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NoteChangeTime(dwregister, &*(&pfiletime as *const <super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi as *const <super::super::Foundation::FILETIME as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTimeOfLastChange<Impl: IRunningObjectTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmkobjectname: ::windows::core::RawPtr, pfiletime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTimeOfLastChange(&*(&pmkobjectname as *const <IMoniker as ::windows::core::Abi>::Abi as *const <IMoniker as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfiletime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumRunning<Impl: IRunningObjectTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenummoniker: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumRunning(::core::mem::transmute_copy(&ppenummoniker)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRunningObjectTable>, ::windows::core::GetTrustLevel, Register::<Impl, OFFSET>, Revoke::<Impl, OFFSET>, IsRunning::<Impl, OFFSET>, GetObject::<Impl, OFFSET>, NoteChangeTime::<Impl, OFFSET>, GetTimeOfLastChange::<Impl, OFFSET>, EnumRunning::<Impl, OFFSET>)
    }
}
pub trait ISequentialStreamImpl: Sized {
    fn Read();
    fn Write();
}
impl ::windows::core::RuntimeName for ISequentialStream {
    const NAME: &'static str = "Windows.Win32.System.Com.ISequentialStream";
}
impl ISequentialStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISequentialStreamImpl, const OFFSET: isize>() -> ISequentialStreamVtbl {
        unsafe extern "system" fn Read<Impl: ISequentialStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Read(::core::mem::transmute_copy(&pv), cb, ::core::mem::transmute_copy(&pcbread)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Write<Impl: ISequentialStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Write(&*(&pv as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), cb, ::core::mem::transmute_copy(&pcbwritten)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISequentialStream>, ::windows::core::GetTrustLevel, Read::<Impl, OFFSET>, Write::<Impl, OFFSET>)
    }
}
pub trait IServerSecurityImpl: Sized {
    fn QueryBlanket();
    fn ImpersonateClient();
    fn RevertToSelf();
    fn IsImpersonating();
}
impl ::windows::core::RuntimeName for IServerSecurity {
    const NAME: &'static str = "Windows.Win32.System.Com.IServerSecurity";
}
impl IServerSecurityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServerSecurityImpl, const OFFSET: isize>() -> IServerSecurityVtbl {
        unsafe extern "system" fn QueryBlanket<Impl: IServerSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::core::ffi::c_void, pcapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryBlanket(::core::mem::transmute_copy(&pauthnsvc), ::core::mem::transmute_copy(&pauthzsvc), ::core::mem::transmute_copy(&pserverprincname), ::core::mem::transmute_copy(&pauthnlevel), ::core::mem::transmute_copy(&pimplevel), ::core::mem::transmute_copy(&pprivs), pcapabilities) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImpersonateClient<Impl: IServerSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImpersonateClient() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RevertToSelf<Impl: IServerSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RevertToSelf() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsImpersonating<Impl: IServerSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsImpersonating() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IServerSecurity>, ::windows::core::GetTrustLevel, QueryBlanket::<Impl, OFFSET>, ImpersonateClient::<Impl, OFFSET>, RevertToSelf::<Impl, OFFSET>, IsImpersonating::<Impl, OFFSET>)
    }
}
pub trait IServiceProviderImpl: Sized {
    fn QueryService();
}
impl ::windows::core::RuntimeName for IServiceProvider {
    const NAME: &'static str = "Windows.Win32.System.Com.IServiceProvider";
}
impl IServiceProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceProviderImpl, const OFFSET: isize>() -> IServiceProviderVtbl {
        unsafe extern "system" fn QueryService<Impl: IServiceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryService(&*(&guidservice as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IServiceProvider>, ::windows::core::GetTrustLevel, QueryService::<Impl, OFFSET>)
    }
}
pub trait IStdMarshalInfoImpl: Sized {
    fn GetClassForHandler();
}
impl ::windows::core::RuntimeName for IStdMarshalInfo {
    const NAME: &'static str = "Windows.Win32.System.Com.IStdMarshalInfo";
}
impl IStdMarshalInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStdMarshalInfoImpl, const OFFSET: isize>() -> IStdMarshalInfoVtbl {
        unsafe extern "system" fn GetClassForHandler<Impl: IStdMarshalInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClassForHandler(dwdestcontext, &*(&pvdestcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pclsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStdMarshalInfo>, ::windows::core::GetTrustLevel, GetClassForHandler::<Impl, OFFSET>)
    }
}
pub trait IStreamImpl: Sized + ISequentialStreamImpl {
    fn Seek();
    fn SetSize();
    fn CopyTo();
    fn Commit();
    fn Revert();
    fn LockRegion();
    fn UnlockRegion();
    fn Stat();
    fn Clone();
}
impl ::windows::core::RuntimeName for IStream {
    const NAME: &'static str = "Windows.Win32.System.Com.IStream";
}
impl IStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamImpl, const OFFSET: isize>() -> IStreamVtbl {
        unsafe extern "system" fn Seek<Impl: IStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dlibmove: i64, dworigin: STREAM_SEEK, plibnewposition: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Seek(dlibmove, dworigin, ::core::mem::transmute_copy(&plibnewposition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Impl: IStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, libnewsize: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSize(libnewsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyTo<Impl: IStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyTo(&*(&pstm as *const <IStream as ::windows::core::Abi>::Abi as *const <IStream as ::windows::core::DefaultType>::DefaultType), cb, ::core::mem::transmute_copy(&pcbread), ::core::mem::transmute_copy(&pcbwritten)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Impl: IStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfcommitflags: StructuredStorage::STGC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Commit(grfcommitflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Revert<Impl: IStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Revert() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockRegion<Impl: IStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LockRegion(liboffset, cb, dwlocktype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnlockRegion<Impl: IStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnlockRegion(liboffset, cb, dwlocktype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stat<Impl: IStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatstg: *mut STATSTG, grfstatflag: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stat(::core::mem::transmute_copy(&pstatstg), grfstatflag) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppstm)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStream>, ::windows::core::GetTrustLevel, Seek::<Impl, OFFSET>, SetSize::<Impl, OFFSET>, CopyTo::<Impl, OFFSET>, Commit::<Impl, OFFSET>, Revert::<Impl, OFFSET>, LockRegion::<Impl, OFFSET>, UnlockRegion::<Impl, OFFSET>, Stat::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait ISupportErrorInfoImpl: Sized {
    fn InterfaceSupportsErrorInfo();
}
impl ::windows::core::RuntimeName for ISupportErrorInfo {
    const NAME: &'static str = "Windows.Win32.System.Com.ISupportErrorInfo";
}
impl ISupportErrorInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISupportErrorInfoImpl, const OFFSET: isize>() -> ISupportErrorInfoVtbl {
        unsafe extern "system" fn InterfaceSupportsErrorInfo<Impl: ISupportErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InterfaceSupportsErrorInfo(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISupportErrorInfo>, ::windows::core::GetTrustLevel, InterfaceSupportsErrorInfo::<Impl, OFFSET>)
    }
}
pub trait ISurrogateImpl: Sized {
    fn LoadDllServer();
    fn FreeSurrogate();
}
impl ::windows::core::RuntimeName for ISurrogate {
    const NAME: &'static str = "Windows.Win32.System.Com.ISurrogate";
}
impl ISurrogateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISurrogateImpl, const OFFSET: isize>() -> ISurrogateVtbl {
        unsafe extern "system" fn LoadDllServer<Impl: ISurrogateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadDllServer(&*(&clsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeSurrogate<Impl: ISurrogateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FreeSurrogate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISurrogate>, ::windows::core::GetTrustLevel, LoadDllServer::<Impl, OFFSET>, FreeSurrogate::<Impl, OFFSET>)
    }
}
pub trait ISurrogateServiceImpl: Sized {
    fn Init();
    fn ApplicationLaunch();
    fn ApplicationFree();
    fn CatalogRefresh();
    fn ProcessShutdown();
}
impl ::windows::core::RuntimeName for ISurrogateService {
    const NAME: &'static str = "Windows.Win32.System.Com.ISurrogateService";
}
impl ISurrogateServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISurrogateServiceImpl, const OFFSET: isize>() -> ISurrogateServiceVtbl {
        unsafe extern "system" fn Init<Impl: ISurrogateServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidprocessid: *const ::windows::core::GUID, pprocesslock: ::windows::core::RawPtr, pfapplicationaware: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Init(&*(&rguidprocessid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pprocesslock as *const <IProcessLock as ::windows::core::Abi>::Abi as *const <IProcessLock as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfapplicationaware)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationLaunch<Impl: ISurrogateServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidapplid: *const ::windows::core::GUID, apptype: ApplicationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationLaunch(&*(&rguidapplid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), apptype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationFree<Impl: ISurrogateServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidapplid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationFree(&*(&rguidapplid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CatalogRefresh<Impl: ISurrogateServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CatalogRefresh(ulreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessShutdown<Impl: ISurrogateServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shutdowntype: ShutdownType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessShutdown(shutdowntype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISurrogateService>, ::windows::core::GetTrustLevel, Init::<Impl, OFFSET>, ApplicationLaunch::<Impl, OFFSET>, ApplicationFree::<Impl, OFFSET>, CatalogRefresh::<Impl, OFFSET>, ProcessShutdown::<Impl, OFFSET>)
    }
}
pub trait ISynchronizeImpl: Sized {
    fn Wait();
    fn Signal();
    fn Reset();
}
impl ::windows::core::RuntimeName for ISynchronize {
    const NAME: &'static str = "Windows.Win32.System.Com.ISynchronize";
}
impl ISynchronizeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISynchronizeImpl, const OFFSET: isize>() -> ISynchronizeVtbl {
        unsafe extern "system" fn Wait<Impl: ISynchronizeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, dwmilliseconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Wait(dwflags, dwmilliseconds) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Signal<Impl: ISynchronizeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Signal() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: ISynchronizeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISynchronize>, ::windows::core::GetTrustLevel, Wait::<Impl, OFFSET>, Signal::<Impl, OFFSET>, Reset::<Impl, OFFSET>)
    }
}
pub trait ISynchronizeContainerImpl: Sized {
    fn AddSynchronize();
    fn WaitMultiple();
}
impl ::windows::core::RuntimeName for ISynchronizeContainer {
    const NAME: &'static str = "Windows.Win32.System.Com.ISynchronizeContainer";
}
impl ISynchronizeContainerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISynchronizeContainerImpl, const OFFSET: isize>() -> ISynchronizeContainerVtbl {
        unsafe extern "system" fn AddSynchronize<Impl: ISynchronizeContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psync: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddSynchronize(&*(&psync as *const <ISynchronize as ::windows::core::Abi>::Abi as *const <ISynchronize as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WaitMultiple<Impl: ISynchronizeContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, dwtimeout: u32, ppsync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WaitMultiple(dwflags, dwtimeout, ::core::mem::transmute_copy(&ppsync)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISynchronizeContainer>, ::windows::core::GetTrustLevel, AddSynchronize::<Impl, OFFSET>, WaitMultiple::<Impl, OFFSET>)
    }
}
pub trait ISynchronizeEventImpl: Sized + ISynchronizeHandleImpl {
    fn SetEventHandle();
}
impl ::windows::core::RuntimeName for ISynchronizeEvent {
    const NAME: &'static str = "Windows.Win32.System.Com.ISynchronizeEvent";
}
impl ISynchronizeEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISynchronizeEventImpl, const OFFSET: isize>() -> ISynchronizeEventVtbl {
        unsafe extern "system" fn SetEventHandle<Impl: ISynchronizeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ph: *const super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEventHandle(&*(&ph as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISynchronizeEvent>, ::windows::core::GetTrustLevel, SetEventHandle::<Impl, OFFSET>)
    }
}
pub trait ISynchronizeHandleImpl: Sized {
    fn GetHandle();
}
impl ::windows::core::RuntimeName for ISynchronizeHandle {
    const NAME: &'static str = "Windows.Win32.System.Com.ISynchronizeHandle";
}
impl ISynchronizeHandleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISynchronizeHandleImpl, const OFFSET: isize>() -> ISynchronizeHandleVtbl {
        unsafe extern "system" fn GetHandle<Impl: ISynchronizeHandleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ph: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHandle(::core::mem::transmute_copy(&ph)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISynchronizeHandle>, ::windows::core::GetTrustLevel, GetHandle::<Impl, OFFSET>)
    }
}
pub trait ISynchronizeMutexImpl: Sized + ISynchronizeImpl {
    fn ReleaseMutex();
}
impl ::windows::core::RuntimeName for ISynchronizeMutex {
    const NAME: &'static str = "Windows.Win32.System.Com.ISynchronizeMutex";
}
impl ISynchronizeMutexVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISynchronizeMutexImpl, const OFFSET: isize>() -> ISynchronizeMutexVtbl {
        unsafe extern "system" fn ReleaseMutex<Impl: ISynchronizeMutexImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseMutex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISynchronizeMutex>, ::windows::core::GetTrustLevel, ReleaseMutex::<Impl, OFFSET>)
    }
}
pub trait ITimeAndNoticeControlImpl: Sized {
    fn SuppressChanges();
}
impl ::windows::core::RuntimeName for ITimeAndNoticeControl {
    const NAME: &'static str = "Windows.Win32.System.Com.ITimeAndNoticeControl";
}
impl ITimeAndNoticeControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimeAndNoticeControlImpl, const OFFSET: isize>() -> ITimeAndNoticeControlVtbl {
        unsafe extern "system" fn SuppressChanges<Impl: ITimeAndNoticeControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, res1: u32, res2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuppressChanges(res1, res2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITimeAndNoticeControl>, ::windows::core::GetTrustLevel, SuppressChanges::<Impl, OFFSET>)
    }
}
pub trait ITypeCompImpl: Sized {
    fn Bind();
    fn BindType();
}
impl ::windows::core::RuntimeName for ITypeComp {
    const NAME: &'static str = "Windows.Win32.System.Com.ITypeComp";
}
impl ITypeCompVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITypeCompImpl, const OFFSET: isize>() -> ITypeCompVtbl {
        unsafe extern "system" fn Bind<Impl: ITypeCompImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: super::super::Foundation::PWSTR, lhashval: u32, wflags: u16, pptinfo: *mut ::windows::core::RawPtr, pdesckind: *mut DESCKIND, pbindptr: *mut BINDPTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bind(&*(&szname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), lhashval, wflags, ::core::mem::transmute_copy(&pptinfo), ::core::mem::transmute_copy(&pdesckind), ::core::mem::transmute_copy(&pbindptr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindType<Impl: ITypeCompImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: super::super::Foundation::PWSTR, lhashval: u32, pptinfo: *mut ::windows::core::RawPtr, pptcomp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BindType(&*(&szname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), lhashval, ::core::mem::transmute_copy(&pptinfo), ::core::mem::transmute_copy(&pptcomp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITypeComp>, ::windows::core::GetTrustLevel, Bind::<Impl, OFFSET>, BindType::<Impl, OFFSET>)
    }
}
pub trait ITypeInfoImpl: Sized {
    fn GetTypeAttr();
    fn GetTypeComp();
    fn GetFuncDesc();
    fn GetVarDesc();
    fn GetNames();
    fn GetRefTypeOfImplType();
    fn GetImplTypeFlags();
    fn GetIDsOfNames();
    fn Invoke();
    fn GetDocumentation();
    fn GetDllEntry();
    fn GetRefTypeInfo();
    fn AddressOfMember();
    fn CreateInstance();
    fn GetMops();
    fn GetContainingTypeLib();
    fn ReleaseTypeAttr();
    fn ReleaseFuncDesc();
    fn ReleaseVarDesc();
}
impl ::windows::core::RuntimeName for ITypeInfo {
    const NAME: &'static str = "Windows.Win32.System.Com.ITypeInfo";
}
impl ITypeInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITypeInfoImpl, const OFFSET: isize>() -> ITypeInfoVtbl {
        unsafe extern "system" fn GetTypeAttr<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptypeattr: *mut *mut TYPEATTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTypeAttr(::core::mem::transmute_copy(&pptypeattr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeComp<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptcomp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTypeComp(::core::mem::transmute_copy(&pptcomp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFuncDesc<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppfuncdesc: *mut *mut FUNCDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFuncDesc(index, ::core::mem::transmute_copy(&ppfuncdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVarDesc<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppvardesc: *mut *mut VARDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVarDesc(index, ::core::mem::transmute_copy(&ppvardesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNames<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memid: i32, rgbstrnames: *mut super::super::Foundation::BSTR, cmaxnames: u32, pcnames: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNames(memid, ::core::mem::transmute_copy(&rgbstrnames), cmaxnames, ::core::mem::transmute_copy(&pcnames)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRefTypeOfImplType<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, preftype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRefTypeOfImplType(index, ::core::mem::transmute_copy(&preftype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImplTypeFlags<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pimpltypeflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetImplTypeFlags(index, ::core::mem::transmute_copy(&pimpltypeflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIDsOfNames<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, pmemid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIDsOfNames(&*(&rgsznames as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cnames, ::core::mem::transmute_copy(&pmemid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Invoke<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvinstance: *const ::core::ffi::c_void, memid: i32, wflags: u16, pdispparams: *mut DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Invoke(&*(&pvinstance as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), memid, wflags, &*(&pdispparams as *const <DISPPARAMS as ::windows::core::Abi>::Abi as *const <DISPPARAMS as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarresult), ::core::mem::transmute_copy(&pexcepinfo), ::core::mem::transmute_copy(&puargerr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentation<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memid: i32, pbstrname: *mut super::super::Foundation::BSTR, pbstrdocstring: *mut super::super::Foundation::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentation(memid, ::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&pbstrdocstring), ::core::mem::transmute_copy(&pdwhelpcontext), ::core::mem::transmute_copy(&pbstrhelpfile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDllEntry<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memid: i32, invkind: INVOKEKIND, pbstrdllname: *mut super::super::Foundation::BSTR, pbstrname: *mut super::super::Foundation::BSTR, pwordinal: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDllEntry(memid, invkind, ::core::mem::transmute_copy(&pbstrdllname), ::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&pwordinal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRefTypeInfo<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreftype: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRefTypeInfo(hreftype, ::core::mem::transmute_copy(&pptinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddressOfMember<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memid: i32, invkind: INVOKEKIND, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddressOfMember(memid, invkind, ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstance<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvobj)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMops<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memid: i32, pbstrmops: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMops(memid, ::core::mem::transmute_copy(&pbstrmops)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContainingTypeLib<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptlib: *mut ::windows::core::RawPtr, pindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContainingTypeLib(::core::mem::transmute_copy(&pptlib), ::core::mem::transmute_copy(&pindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseTypeAttr<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptypeattr: *const TYPEATTR) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseTypeAttr(&*(&ptypeattr as *const <TYPEATTR as ::windows::core::Abi>::Abi as *const <TYPEATTR as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ReleaseFuncDesc<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfuncdesc: *const FUNCDESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseFuncDesc(&*(&pfuncdesc as *const <FUNCDESC as ::windows::core::Abi>::Abi as *const <FUNCDESC as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ReleaseVarDesc<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardesc: *const VARDESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseVarDesc(&*(&pvardesc as *const <VARDESC as ::windows::core::Abi>::Abi as *const <VARDESC as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITypeInfo>,
            ::windows::core::GetTrustLevel,
            GetTypeAttr::<Impl, OFFSET>,
            GetTypeComp::<Impl, OFFSET>,
            GetFuncDesc::<Impl, OFFSET>,
            GetVarDesc::<Impl, OFFSET>,
            GetNames::<Impl, OFFSET>,
            GetRefTypeOfImplType::<Impl, OFFSET>,
            GetImplTypeFlags::<Impl, OFFSET>,
            GetIDsOfNames::<Impl, OFFSET>,
            Invoke::<Impl, OFFSET>,
            GetDocumentation::<Impl, OFFSET>,
            GetDllEntry::<Impl, OFFSET>,
            GetRefTypeInfo::<Impl, OFFSET>,
            AddressOfMember::<Impl, OFFSET>,
            CreateInstance::<Impl, OFFSET>,
            GetMops::<Impl, OFFSET>,
            GetContainingTypeLib::<Impl, OFFSET>,
            ReleaseTypeAttr::<Impl, OFFSET>,
            ReleaseFuncDesc::<Impl, OFFSET>,
            ReleaseVarDesc::<Impl, OFFSET>,
        )
    }
}
pub trait ITypeInfo2Impl: Sized + ITypeInfoImpl {
    fn GetTypeKind();
    fn GetTypeFlags();
    fn GetFuncIndexOfMemId();
    fn GetVarIndexOfMemId();
    fn GetCustData();
    fn GetFuncCustData();
    fn GetParamCustData();
    fn GetVarCustData();
    fn GetImplTypeCustData();
    fn GetDocumentation2();
    fn GetAllCustData();
    fn GetAllFuncCustData();
    fn GetAllParamCustData();
    fn GetAllVarCustData();
    fn GetAllImplTypeCustData();
}
impl ::windows::core::RuntimeName for ITypeInfo2 {
    const NAME: &'static str = "Windows.Win32.System.Com.ITypeInfo2";
}
impl ITypeInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITypeInfo2Impl, const OFFSET: isize>() -> ITypeInfo2Vtbl {
        unsafe extern "system" fn GetTypeKind<Impl: ITypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptypekind: *mut TYPEKIND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTypeKind(::core::mem::transmute_copy(&ptypekind)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeFlags<Impl: ITypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptypeflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTypeFlags(::core::mem::transmute_copy(&ptypeflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFuncIndexOfMemId<Impl: ITypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memid: i32, invkind: INVOKEKIND, pfuncindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFuncIndexOfMemId(memid, invkind, ::core::mem::transmute_copy(&pfuncindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVarIndexOfMemId<Impl: ITypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memid: i32, pvarindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVarIndexOfMemId(memid, ::core::mem::transmute_copy(&pvarindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustData<Impl: ITypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCustData(&*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFuncCustData<Impl: ITypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFuncCustData(index, &*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParamCustData<Impl: ITypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexfunc: u32, indexparam: u32, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParamCustData(indexfunc, indexparam, &*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVarCustData<Impl: ITypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVarCustData(index, &*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImplTypeCustData<Impl: ITypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetImplTypeCustData(index, &*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentation2<Impl: ITypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memid: i32, lcid: u32, pbstrhelpstring: *mut super::super::Foundation::BSTR, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentation2(memid, lcid, ::core::mem::transmute_copy(&pbstrhelpstring), ::core::mem::transmute_copy(&pdwhelpstringcontext), ::core::mem::transmute_copy(&pbstrhelpstringdll)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllCustData<Impl: ITypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllCustData(::core::mem::transmute_copy(&pcustdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllFuncCustData<Impl: ITypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllFuncCustData(index, ::core::mem::transmute_copy(&pcustdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllParamCustData<Impl: ITypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexfunc: u32, indexparam: u32, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllParamCustData(indexfunc, indexparam, ::core::mem::transmute_copy(&pcustdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllVarCustData<Impl: ITypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllVarCustData(index, ::core::mem::transmute_copy(&pcustdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllImplTypeCustData<Impl: ITypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllImplTypeCustData(index, ::core::mem::transmute_copy(&pcustdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITypeInfo2>,
            ::windows::core::GetTrustLevel,
            GetTypeKind::<Impl, OFFSET>,
            GetTypeFlags::<Impl, OFFSET>,
            GetFuncIndexOfMemId::<Impl, OFFSET>,
            GetVarIndexOfMemId::<Impl, OFFSET>,
            GetCustData::<Impl, OFFSET>,
            GetFuncCustData::<Impl, OFFSET>,
            GetParamCustData::<Impl, OFFSET>,
            GetVarCustData::<Impl, OFFSET>,
            GetImplTypeCustData::<Impl, OFFSET>,
            GetDocumentation2::<Impl, OFFSET>,
            GetAllCustData::<Impl, OFFSET>,
            GetAllFuncCustData::<Impl, OFFSET>,
            GetAllParamCustData::<Impl, OFFSET>,
            GetAllVarCustData::<Impl, OFFSET>,
            GetAllImplTypeCustData::<Impl, OFFSET>,
        )
    }
}
pub trait ITypeLibImpl: Sized {
    fn GetTypeInfoCount();
    fn GetTypeInfo();
    fn GetTypeInfoType();
    fn GetTypeInfoOfGuid();
    fn GetLibAttr();
    fn GetTypeComp();
    fn GetDocumentation();
    fn IsName();
    fn FindName();
    fn ReleaseTLibAttr();
}
impl ::windows::core::RuntimeName for ITypeLib {
    const NAME: &'static str = "Windows.Win32.System.Com.ITypeLib";
}
impl ITypeLibVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITypeLibImpl, const OFFSET: isize>() -> ITypeLibVtbl {
        unsafe extern "system" fn GetTypeInfoCount<Impl: ITypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTypeInfoCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeInfo<Impl: ITypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTypeInfo(index, ::core::mem::transmute_copy(&pptinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeInfoType<Impl: ITypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ptkind: *mut TYPEKIND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTypeInfoType(index, ::core::mem::transmute_copy(&ptkind)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeInfoOfGuid<Impl: ITypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTypeInfoOfGuid(&*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pptinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLibAttr<Impl: ITypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptlibattr: *mut *mut TLIBATTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLibAttr(::core::mem::transmute_copy(&pptlibattr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeComp<Impl: ITypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptcomp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTypeComp(::core::mem::transmute_copy(&pptcomp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentation<Impl: ITypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pbstrname: *mut super::super::Foundation::BSTR, pbstrdocstring: *mut super::super::Foundation::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentation(index, ::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&pbstrdocstring), ::core::mem::transmute_copy(&pdwhelpcontext), ::core::mem::transmute_copy(&pbstrhelpfile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsName<Impl: ITypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sznamebuf: super::super::Foundation::PWSTR, lhashval: u32, pfname: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsName(&*(&sznamebuf as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), lhashval, ::core::mem::transmute_copy(&pfname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindName<Impl: ITypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sznamebuf: super::super::Foundation::PWSTR, lhashval: u32, pptinfo: *mut ::windows::core::RawPtr, rgmemid: *mut i32, pcfound: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindName(&*(&sznamebuf as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), lhashval, ::core::mem::transmute_copy(&pptinfo), ::core::mem::transmute_copy(&rgmemid), pcfound) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseTLibAttr<Impl: ITypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptlibattr: *const TLIBATTR) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseTLibAttr(&*(&ptlibattr as *const <TLIBATTR as ::windows::core::Abi>::Abi as *const <TLIBATTR as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITypeLib>,
            ::windows::core::GetTrustLevel,
            GetTypeInfoCount::<Impl, OFFSET>,
            GetTypeInfo::<Impl, OFFSET>,
            GetTypeInfoType::<Impl, OFFSET>,
            GetTypeInfoOfGuid::<Impl, OFFSET>,
            GetLibAttr::<Impl, OFFSET>,
            GetTypeComp::<Impl, OFFSET>,
            GetDocumentation::<Impl, OFFSET>,
            IsName::<Impl, OFFSET>,
            FindName::<Impl, OFFSET>,
            ReleaseTLibAttr::<Impl, OFFSET>,
        )
    }
}
pub trait ITypeLib2Impl: Sized + ITypeLibImpl {
    fn GetCustData();
    fn GetLibStatistics();
    fn GetDocumentation2();
    fn GetAllCustData();
}
impl ::windows::core::RuntimeName for ITypeLib2 {
    const NAME: &'static str = "Windows.Win32.System.Com.ITypeLib2";
}
impl ITypeLib2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITypeLib2Impl, const OFFSET: isize>() -> ITypeLib2Vtbl {
        unsafe extern "system" fn GetCustData<Impl: ITypeLib2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCustData(&*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLibStatistics<Impl: ITypeLib2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcuniquenames: *mut u32, pcchuniquenames: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLibStatistics(::core::mem::transmute_copy(&pcuniquenames), ::core::mem::transmute_copy(&pcchuniquenames)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentation2<Impl: ITypeLib2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, lcid: u32, pbstrhelpstring: *mut super::super::Foundation::BSTR, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentation2(index, lcid, ::core::mem::transmute_copy(&pbstrhelpstring), ::core::mem::transmute_copy(&pdwhelpstringcontext), ::core::mem::transmute_copy(&pbstrhelpstringdll)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllCustData<Impl: ITypeLib2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllCustData(::core::mem::transmute_copy(&pcustdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITypeLib2>, ::windows::core::GetTrustLevel, GetCustData::<Impl, OFFSET>, GetLibStatistics::<Impl, OFFSET>, GetDocumentation2::<Impl, OFFSET>, GetAllCustData::<Impl, OFFSET>)
    }
}
pub trait ITypeLibRegistrationImpl: Sized {
    fn GetGuid();
    fn GetVersion();
    fn GetLcid();
    fn GetWin32Path();
    fn GetWin64Path();
    fn GetDisplayName();
    fn GetFlags();
    fn GetHelpDir();
}
impl ::windows::core::RuntimeName for ITypeLibRegistration {
    const NAME: &'static str = "Windows.Win32.System.Com.ITypeLibRegistration";
}
impl ITypeLibRegistrationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITypeLibRegistrationImpl, const OFFSET: isize>() -> ITypeLibRegistrationVtbl {
        unsafe extern "system" fn GetGuid<Impl: ITypeLibRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGuid(::core::mem::transmute_copy(&pguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVersion<Impl: ITypeLibRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pversion: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVersion(::core::mem::transmute_copy(&pversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLcid<Impl: ITypeLibRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLcid(::core::mem::transmute_copy(&plcid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWin32Path<Impl: ITypeLibRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwin32path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWin32Path(::core::mem::transmute_copy(&pwin32path)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWin64Path<Impl: ITypeLibRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwin64path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWin64Path(::core::mem::transmute_copy(&pwin64path)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayName<Impl: ITypeLibRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplayName(::core::mem::transmute_copy(&pdisplayname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlags<Impl: ITypeLibRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFlags(::core::mem::transmute_copy(&pflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHelpDir<Impl: ITypeLibRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phelpdir: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHelpDir(::core::mem::transmute_copy(&phelpdir)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITypeLibRegistration>,
            ::windows::core::GetTrustLevel,
            GetGuid::<Impl, OFFSET>,
            GetVersion::<Impl, OFFSET>,
            GetLcid::<Impl, OFFSET>,
            GetWin32Path::<Impl, OFFSET>,
            GetWin64Path::<Impl, OFFSET>,
            GetDisplayName::<Impl, OFFSET>,
            GetFlags::<Impl, OFFSET>,
            GetHelpDir::<Impl, OFFSET>,
        )
    }
}
pub trait ITypeLibRegistrationReaderImpl: Sized {
    fn EnumTypeLibRegistrations();
}
impl ::windows::core::RuntimeName for ITypeLibRegistrationReader {
    const NAME: &'static str = "Windows.Win32.System.Com.ITypeLibRegistrationReader";
}
impl ITypeLibRegistrationReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITypeLibRegistrationReaderImpl, const OFFSET: isize>() -> ITypeLibRegistrationReaderVtbl {
        unsafe extern "system" fn EnumTypeLibRegistrations<Impl: ITypeLibRegistrationReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumunknown: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumTypeLibRegistrations(::core::mem::transmute_copy(&ppenumunknown)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITypeLibRegistrationReader>, ::windows::core::GetTrustLevel, EnumTypeLibRegistrations::<Impl, OFFSET>)
    }
}
pub trait IUriImpl: Sized {
    fn GetPropertyBSTR();
    fn GetPropertyLength();
    fn GetPropertyDWORD();
    fn HasProperty();
    fn GetAbsoluteUri();
    fn GetAuthority();
    fn GetDisplayUri();
    fn GetDomain();
    fn GetExtension();
    fn GetFragment();
    fn GetHost();
    fn GetPassword();
    fn GetPath();
    fn GetPathAndQuery();
    fn GetQuery();
    fn GetRawUri();
    fn GetSchemeName();
    fn GetUserInfo();
    fn GetUserName();
    fn GetHostType();
    fn GetPort();
    fn GetScheme();
    fn GetZone();
    fn GetProperties();
    fn IsEqual();
}
impl ::windows::core::RuntimeName for IUri {
    const NAME: &'static str = "Windows.Win32.System.Com.IUri";
}
impl IUriVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUriImpl, const OFFSET: isize>() -> IUriVtbl {
        unsafe extern "system" fn GetPropertyBSTR<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uriprop: Uri_PROPERTY, pbstrproperty: *mut super::super::Foundation::BSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyBSTR(uriprop, ::core::mem::transmute_copy(&pbstrproperty), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyLength<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uriprop: Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyLength(uriprop, ::core::mem::transmute_copy(&pcchproperty), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyDWORD<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uriprop: Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyDWORD(uriprop, ::core::mem::transmute_copy(&pdwproperty), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasProperty<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uriprop: Uri_PROPERTY, pfhasproperty: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasProperty(uriprop, ::core::mem::transmute_copy(&pfhasproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAbsoluteUri<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrabsoluteuri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAbsoluteUri(::core::mem::transmute_copy(&pbstrabsoluteuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAuthority<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrauthority: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAuthority(::core::mem::transmute_copy(&pbstrauthority)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayUri<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdisplaystring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplayUri(::core::mem::transmute_copy(&pbstrdisplaystring)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDomain<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdomain: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDomain(::core::mem::transmute_copy(&pbstrdomain)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExtension<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrextension: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetExtension(::core::mem::transmute_copy(&pbstrextension)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFragment<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfragment: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFragment(::core::mem::transmute_copy(&pbstrfragment)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHost<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrhost: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHost(::core::mem::transmute_copy(&pbstrhost)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPassword<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpassword: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPassword(::core::mem::transmute_copy(&pbstrpassword)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPath<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPath(::core::mem::transmute_copy(&pbstrpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPathAndQuery<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathandquery: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPathAndQuery(::core::mem::transmute_copy(&pbstrpathandquery)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQuery<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrquery: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetQuery(::core::mem::transmute_copy(&pbstrquery)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRawUri<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrrawuri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRawUri(::core::mem::transmute_copy(&pbstrrawuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSchemeName<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrschemename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSchemeName(::core::mem::transmute_copy(&pbstrschemename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserInfo<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstruserinfo: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUserInfo(::core::mem::transmute_copy(&pbstruserinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserName<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUserName(::core::mem::transmute_copy(&pbstrusername)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHostType<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwhosttype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHostType(::core::mem::transmute_copy(&pdwhosttype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPort<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPort(::core::mem::transmute_copy(&pdwport)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScheme<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwscheme: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScheme(::core::mem::transmute_copy(&pdwscheme)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetZone<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwzone: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetZone(::core::mem::transmute_copy(&pdwzone)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperties<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties(::core::mem::transmute_copy(&pdwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puri: ::windows::core::RawPtr, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqual(&*(&puri as *const <IUri as ::windows::core::Abi>::Abi as *const <IUri as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfequal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUri>,
            ::windows::core::GetTrustLevel,
            GetPropertyBSTR::<Impl, OFFSET>,
            GetPropertyLength::<Impl, OFFSET>,
            GetPropertyDWORD::<Impl, OFFSET>,
            HasProperty::<Impl, OFFSET>,
            GetAbsoluteUri::<Impl, OFFSET>,
            GetAuthority::<Impl, OFFSET>,
            GetDisplayUri::<Impl, OFFSET>,
            GetDomain::<Impl, OFFSET>,
            GetExtension::<Impl, OFFSET>,
            GetFragment::<Impl, OFFSET>,
            GetHost::<Impl, OFFSET>,
            GetPassword::<Impl, OFFSET>,
            GetPath::<Impl, OFFSET>,
            GetPathAndQuery::<Impl, OFFSET>,
            GetQuery::<Impl, OFFSET>,
            GetRawUri::<Impl, OFFSET>,
            GetSchemeName::<Impl, OFFSET>,
            GetUserInfo::<Impl, OFFSET>,
            GetUserName::<Impl, OFFSET>,
            GetHostType::<Impl, OFFSET>,
            GetPort::<Impl, OFFSET>,
            GetScheme::<Impl, OFFSET>,
            GetZone::<Impl, OFFSET>,
            GetProperties::<Impl, OFFSET>,
            IsEqual::<Impl, OFFSET>,
        )
    }
}
pub trait IUriBuilderImpl: Sized {
    fn CreateUriSimple();
    fn CreateUri();
    fn CreateUriWithFlags();
    fn GetIUri();
    fn SetIUri();
    fn GetFragment();
    fn GetHost();
    fn GetPassword();
    fn GetPath();
    fn GetPort();
    fn GetQuery();
    fn GetSchemeName();
    fn GetUserName();
    fn SetFragment();
    fn SetHost();
    fn SetPassword();
    fn SetPath();
    fn SetPort();
    fn SetQuery();
    fn SetSchemeName();
    fn SetUserName();
    fn RemoveProperties();
    fn HasBeenModified();
}
impl ::windows::core::RuntimeName for IUriBuilder {
    const NAME: &'static str = "Windows.Win32.System.Com.IUriBuilder";
}
impl IUriBuilderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUriBuilderImpl, const OFFSET: isize>() -> IUriBuilderVtbl {
        unsafe extern "system" fn CreateUriSimple<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUriSimple(dwallowencodingpropertymask, dwreserved, ::core::mem::transmute_copy(&ppiuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUri<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcreateflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUri(dwcreateflags, dwallowencodingpropertymask, dwreserved, ::core::mem::transmute_copy(&ppiuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUriWithFlags<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcreateflags: u32, dwuribuilderflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUriWithFlags(dwcreateflags, dwuribuilderflags, dwallowencodingpropertymask, dwreserved, ::core::mem::transmute_copy(&ppiuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIUri<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIUri(::core::mem::transmute_copy(&ppiuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIUri<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piuri: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIUri(&*(&piuri as *const <IUri as ::windows::core::Abi>::Abi as *const <IUri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFragment<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchfragment: *mut u32, ppwzfragment: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFragment(::core::mem::transmute_copy(&pcchfragment), ::core::mem::transmute_copy(&ppwzfragment)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHost<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchhost: *mut u32, ppwzhost: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHost(::core::mem::transmute_copy(&pcchhost), ::core::mem::transmute_copy(&ppwzhost)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPassword<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchpassword: *mut u32, ppwzpassword: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPassword(::core::mem::transmute_copy(&pcchpassword), ::core::mem::transmute_copy(&ppwzpassword)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPath<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchpath: *mut u32, ppwzpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPath(::core::mem::transmute_copy(&pcchpath), ::core::mem::transmute_copy(&ppwzpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPort<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfhasport: *mut super::super::Foundation::BOOL, pdwport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPort(::core::mem::transmute_copy(&pfhasport), ::core::mem::transmute_copy(&pdwport)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQuery<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchquery: *mut u32, ppwzquery: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetQuery(::core::mem::transmute_copy(&pcchquery), ::core::mem::transmute_copy(&ppwzquery)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSchemeName<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchschemename: *mut u32, ppwzschemename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSchemeName(::core::mem::transmute_copy(&pcchschemename), ::core::mem::transmute_copy(&ppwzschemename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserName<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchusername: *mut u32, ppwzusername: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUserName(::core::mem::transmute_copy(&pcchusername), ::core::mem::transmute_copy(&ppwzusername)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFragment<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFragment(&*(&pwznewvalue as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHost<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHost(&*(&pwznewvalue as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPassword<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPassword(&*(&pwznewvalue as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPath(&*(&pwznewvalue as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPort<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fhasport: super::super::Foundation::BOOL, dwnewvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPort(&*(&fhasport as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), dwnewvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuery<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetQuery(&*(&pwznewvalue as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSchemeName<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSchemeName(&*(&pwznewvalue as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserName<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetUserName(&*(&pwznewvalue as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveProperties<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwpropertymask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveProperties(dwpropertymask) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasBeenModified<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfmodified: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasBeenModified(::core::mem::transmute_copy(&pfmodified)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUriBuilder>,
            ::windows::core::GetTrustLevel,
            CreateUriSimple::<Impl, OFFSET>,
            CreateUri::<Impl, OFFSET>,
            CreateUriWithFlags::<Impl, OFFSET>,
            GetIUri::<Impl, OFFSET>,
            SetIUri::<Impl, OFFSET>,
            GetFragment::<Impl, OFFSET>,
            GetHost::<Impl, OFFSET>,
            GetPassword::<Impl, OFFSET>,
            GetPath::<Impl, OFFSET>,
            GetPort::<Impl, OFFSET>,
            GetQuery::<Impl, OFFSET>,
            GetSchemeName::<Impl, OFFSET>,
            GetUserName::<Impl, OFFSET>,
            SetFragment::<Impl, OFFSET>,
            SetHost::<Impl, OFFSET>,
            SetPassword::<Impl, OFFSET>,
            SetPath::<Impl, OFFSET>,
            SetPort::<Impl, OFFSET>,
            SetQuery::<Impl, OFFSET>,
            SetSchemeName::<Impl, OFFSET>,
            SetUserName::<Impl, OFFSET>,
            RemoveProperties::<Impl, OFFSET>,
            HasBeenModified::<Impl, OFFSET>,
        )
    }
}
pub trait IUrlMonImpl: Sized {
    fn AsyncGetClassBits();
}
impl ::windows::core::RuntimeName for IUrlMon {
    const NAME: &'static str = "Windows.Win32.System.Com.IUrlMon";
}
impl IUrlMonVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUrlMonImpl, const OFFSET: isize>() -> IUrlMonVtbl {
        unsafe extern "system" fn AsyncGetClassBits<Impl: IUrlMonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, psztype: super::super::Foundation::PWSTR, pszext: super::super::Foundation::PWSTR, dwfileversionms: u32, dwfileversionls: u32, pszcodebase: super::super::Foundation::PWSTR, pbc: ::windows::core::RawPtr, dwclasscontext: u32, riid: *const ::windows::core::GUID, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsyncGetClassBits(
                &*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&psztype as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                dwfileversionms,
                dwfileversionls,
                &*(&pszcodebase as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pbc as *const <IBindCtx as ::windows::core::Abi>::Abi as *const <IBindCtx as ::windows::core::DefaultType>::DefaultType),
                dwclasscontext,
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                flags,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUrlMon>, ::windows::core::GetTrustLevel, AsyncGetClassBits::<Impl, OFFSET>)
    }
}
pub trait IWaitMultipleImpl: Sized {
    fn WaitMultiple();
    fn AddSynchronize();
}
impl ::windows::core::RuntimeName for IWaitMultiple {
    const NAME: &'static str = "Windows.Win32.System.Com.IWaitMultiple";
}
impl IWaitMultipleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWaitMultipleImpl, const OFFSET: isize>() -> IWaitMultipleVtbl {
        unsafe extern "system" fn WaitMultiple<Impl: IWaitMultipleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: u32, psync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WaitMultiple(timeout, ::core::mem::transmute_copy(&psync)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddSynchronize<Impl: IWaitMultipleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psync: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddSynchronize(&*(&psync as *const <ISynchronize as ::windows::core::Abi>::Abi as *const <ISynchronize as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWaitMultiple>, ::windows::core::GetTrustLevel, WaitMultiple::<Impl, OFFSET>, AddSynchronize::<Impl, OFFSET>)
    }
}
