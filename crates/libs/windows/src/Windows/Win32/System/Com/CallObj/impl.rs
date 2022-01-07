pub trait ICallFrameImpl: Sized {
    fn GetInfo();
    fn GetIIDAndMethod();
    fn GetNames();
    fn GetStackLocation();
    fn SetStackLocation();
    fn SetReturnValue();
    fn GetReturnValue();
    fn GetParamInfo();
    fn SetParam();
    fn GetParam();
    fn Copy();
    fn Free();
    fn FreeParam();
    fn WalkFrame();
    fn GetMarshalSizeMax();
    fn Marshal();
    fn Unmarshal();
    fn ReleaseMarshalData();
    fn Invoke();
}
impl ::windows::core::RuntimeName for ICallFrame {
    const NAME: &'static str = "Windows.Win32.System.Com.CallObj.ICallFrame";
}
impl ICallFrameVtbl {
    pub const fn new<Impl: ICallFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICallFrameVtbl {
        unsafe extern "system" fn GetInfo<Impl: ICallFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *mut CALLFRAMEINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInfo(::core::mem::transmute_copy(&pinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIIDAndMethod<Impl: ICallFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piid: *mut ::windows::core::GUID, pimethod: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIIDAndMethod(::core::mem::transmute_copy(&piid), ::core::mem::transmute_copy(&pimethod)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNames<Impl: ICallFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszinterface: *mut super::super::super::Foundation::PWSTR, pwszmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNames(::core::mem::transmute_copy(&pwszinterface), ::core::mem::transmute_copy(&pwszmethod)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStackLocation<Impl: ICallFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStackLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStackLocation<Impl: ICallFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvstack: *const ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStackLocation(&*(&pvstack as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetReturnValue<Impl: ICallFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetReturnValue(hr).into()
        }
        unsafe extern "system" fn GetReturnValue<Impl: ICallFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetReturnValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParamInfo<Impl: ICallFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iparam: u32, pinfo: *mut CALLFRAMEPARAMINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetParamInfo(iparam, ::core::mem::transmute_copy(&pinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParam<Impl: ICallFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iparam: u32, pvar: *const super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetParam(iparam, &*(&pvar as *const <super::VARIANT as ::windows::core::Abi>::Abi as *const <super::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParam<Impl: ICallFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iparam: u32, pvar: *mut super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetParam(iparam, ::core::mem::transmute_copy(&pvar)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Copy<Impl: ICallFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, copycontrol: CALLFRAME_COPY, pwalker: ::windows::core::RawPtr, ppframe: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Copy(copycontrol, &*(&pwalker as *const <ICallFrameWalker as ::windows::core::Abi>::Abi as *const <ICallFrameWalker as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppframe)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Free<Impl: ICallFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pframeargsdest: ::windows::core::RawPtr, pwalkerdestfree: ::windows::core::RawPtr, pwalkercopy: ::windows::core::RawPtr, freeflags: u32, pwalkerfree: ::windows::core::RawPtr, nullflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Free(
                &*(&pframeargsdest as *const <ICallFrame as ::windows::core::Abi>::Abi as *const <ICallFrame as ::windows::core::DefaultType>::DefaultType),
                &*(&pwalkerdestfree as *const <ICallFrameWalker as ::windows::core::Abi>::Abi as *const <ICallFrameWalker as ::windows::core::DefaultType>::DefaultType),
                &*(&pwalkercopy as *const <ICallFrameWalker as ::windows::core::Abi>::Abi as *const <ICallFrameWalker as ::windows::core::DefaultType>::DefaultType),
                freeflags,
                &*(&pwalkerfree as *const <ICallFrameWalker as ::windows::core::Abi>::Abi as *const <ICallFrameWalker as ::windows::core::DefaultType>::DefaultType),
                nullflags,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeParam<Impl: ICallFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iparam: u32, freeflags: u32, pwalkerfree: ::windows::core::RawPtr, nullflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FreeParam(iparam, freeflags, &*(&pwalkerfree as *const <ICallFrameWalker as ::windows::core::Abi>::Abi as *const <ICallFrameWalker as ::windows::core::DefaultType>::DefaultType), nullflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WalkFrame<Impl: ICallFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, walkwhat: u32, pwalker: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WalkFrame(walkwhat, &*(&pwalker as *const <ICallFrameWalker as ::windows::core::Abi>::Abi as *const <ICallFrameWalker as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMarshalSizeMax<Impl: ICallFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS, pcbbufferneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMarshalSizeMax(&*(&pmshlcontext as *const <CALLFRAME_MARSHALCONTEXT as ::windows::core::Abi>::Abi as *const <CALLFRAME_MARSHALCONTEXT as ::windows::core::DefaultType>::DefaultType), mshlflags, ::core::mem::transmute_copy(&pcbbufferneeded)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Marshal<Impl: ICallFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, pcbbufferused: *mut u32, pdatarep: *mut u32, prpcflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Marshal(&*(&pmshlcontext as *const <CALLFRAME_MARSHALCONTEXT as ::windows::core::Abi>::Abi as *const <CALLFRAME_MARSHALCONTEXT as ::windows::core::DefaultType>::DefaultType), mshlflags, &*(&pbuffer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), cbbuffer, ::core::mem::transmute_copy(&pcbbufferused), ::core::mem::transmute_copy(&pdatarep), ::core::mem::transmute_copy(&prpcflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unmarshal<Impl: ICallFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT, pcbunmarshalled: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Unmarshal(&*(&pbuffer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), cbbuffer, datarep, &*(&pcontext as *const <CALLFRAME_MARSHALCONTEXT as ::windows::core::Abi>::Abi as *const <CALLFRAME_MARSHALCONTEXT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcbunmarshalled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseMarshalData<Impl: ICallFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseMarshalData(&*(&pbuffer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), cbbuffer, ibfirstrelease, datarep, &*(&pcontext as *const <CALLFRAME_MARSHALCONTEXT as ::windows::core::Abi>::Abi as *const <CALLFRAME_MARSHALCONTEXT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Invoke<Impl: ICallFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvreceiver: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Invoke(&*(&pvreceiver as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ICallFrame>,
            base.5,
            GetInfo::<Impl, OFFSET>,
            GetIIDAndMethod::<Impl, OFFSET>,
            GetNames::<Impl, OFFSET>,
            GetStackLocation::<Impl, OFFSET>,
            SetStackLocation::<Impl, OFFSET>,
            SetReturnValue::<Impl, OFFSET>,
            GetReturnValue::<Impl, OFFSET>,
            GetParamInfo::<Impl, OFFSET>,
            SetParam::<Impl, OFFSET>,
            GetParam::<Impl, OFFSET>,
            Copy::<Impl, OFFSET>,
            Free::<Impl, OFFSET>,
            FreeParam::<Impl, OFFSET>,
            WalkFrame::<Impl, OFFSET>,
            GetMarshalSizeMax::<Impl, OFFSET>,
            Marshal::<Impl, OFFSET>,
            Unmarshal::<Impl, OFFSET>,
            ReleaseMarshalData::<Impl, OFFSET>,
            Invoke::<Impl, OFFSET>,
        )
    }
}
pub trait ICallFrameEventsImpl: Sized {
    fn OnCall();
}
impl ::windows::core::RuntimeName for ICallFrameEvents {
    const NAME: &'static str = "Windows.Win32.System.Com.CallObj.ICallFrameEvents";
}
impl ICallFrameEventsVtbl {
    pub const fn new<Impl: ICallFrameEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICallFrameEventsVtbl {
        unsafe extern "system" fn OnCall<Impl: ICallFrameEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pframe: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnCall(&*(&pframe as *const <ICallFrame as ::windows::core::Abi>::Abi as *const <ICallFrame as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICallFrameEvents>, base.5, OnCall::<Impl, OFFSET>)
    }
}
pub trait ICallFrameWalkerImpl: Sized {
    fn OnWalkInterface();
}
impl ::windows::core::RuntimeName for ICallFrameWalker {
    const NAME: &'static str = "Windows.Win32.System.Com.CallObj.ICallFrameWalker";
}
impl ICallFrameWalkerVtbl {
    pub const fn new<Impl: ICallFrameWalkerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICallFrameWalkerVtbl {
        unsafe extern "system" fn OnWalkInterface<Impl: ICallFrameWalkerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, ppvinterface: *const *const ::core::ffi::c_void, fin: super::super::super::Foundation::BOOL, fout: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnWalkInterface(
                &*(&iid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&ppvinterface as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                &*(&fin as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&fout as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICallFrameWalker>, base.5, OnWalkInterface::<Impl, OFFSET>)
    }
}
pub trait ICallIndirectImpl: Sized {
    fn CallIndirect();
    fn GetMethodInfo();
    fn GetStackSize();
    fn GetIID();
}
impl ::windows::core::RuntimeName for ICallIndirect {
    const NAME: &'static str = "Windows.Win32.System.Com.CallObj.ICallIndirect";
}
impl ICallIndirectVtbl {
    pub const fn new<Impl: ICallIndirectImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICallIndirectVtbl {
        unsafe extern "system" fn CallIndirect<Impl: ICallIndirectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phrreturn: *mut ::windows::core::HRESULT, imethod: u32, pvargs: *const ::core::ffi::c_void, cbargs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CallIndirect(::core::mem::transmute_copy(&phrreturn), imethod, &*(&pvargs as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&cbargs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMethodInfo<Impl: ICallIndirectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imethod: u32, pinfo: *mut CALLFRAMEINFO, pwszmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMethodInfo(imethod, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&pwszmethod)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStackSize<Impl: ICallIndirectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imethod: u32, cbargs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStackSize(imethod, ::core::mem::transmute_copy(&cbargs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIID<Impl: ICallIndirectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piid: *mut ::windows::core::GUID, pfderivesfromidispatch: *mut super::super::super::Foundation::BOOL, pcmethod: *mut u32, pwszinterface: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIID(::core::mem::transmute_copy(&piid), ::core::mem::transmute_copy(&pfderivesfromidispatch), ::core::mem::transmute_copy(&pcmethod), ::core::mem::transmute_copy(&pwszinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICallIndirect>, base.5, CallIndirect::<Impl, OFFSET>, GetMethodInfo::<Impl, OFFSET>, GetStackSize::<Impl, OFFSET>, GetIID::<Impl, OFFSET>)
    }
}
pub trait ICallInterceptorImpl: Sized + ICallIndirectImpl {
    fn RegisterSink();
    fn GetRegisteredSink();
}
impl ::windows::core::RuntimeName for ICallInterceptor {
    const NAME: &'static str = "Windows.Win32.System.Com.CallObj.ICallInterceptor";
}
impl ICallInterceptorVtbl {
    pub const fn new<Impl: ICallInterceptorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICallInterceptorVtbl {
        unsafe extern "system" fn RegisterSink<Impl: ICallInterceptorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterSink(&*(&psink as *const <ICallFrameEvents as ::windows::core::Abi>::Abi as *const <ICallFrameEvents as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegisteredSink<Impl: ICallInterceptorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRegisteredSink(::core::mem::transmute_copy(&ppsink)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICallInterceptor>, base.5, RegisterSink::<Impl, OFFSET>, GetRegisteredSink::<Impl, OFFSET>)
    }
}
pub trait ICallUnmarshalImpl: Sized {
    fn Unmarshal();
    fn ReleaseMarshalData();
}
impl ::windows::core::RuntimeName for ICallUnmarshal {
    const NAME: &'static str = "Windows.Win32.System.Com.CallObj.ICallUnmarshal";
}
impl ICallUnmarshalVtbl {
    pub const fn new<Impl: ICallUnmarshalImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICallUnmarshalVtbl {
        unsafe extern "system" fn Unmarshal<Impl: ICallUnmarshalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imethod: u32, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, fforcebuffercopy: super::super::super::Foundation::BOOL, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT, pcbunmarshalled: *mut u32, ppframe: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Unmarshal(
                imethod,
                &*(&pbuffer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                cbbuffer,
                &*(&fforcebuffercopy as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                datarep,
                &*(&pcontext as *const <CALLFRAME_MARSHALCONTEXT as ::windows::core::Abi>::Abi as *const <CALLFRAME_MARSHALCONTEXT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pcbunmarshalled),
                ::core::mem::transmute_copy(&ppframe),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseMarshalData<Impl: ICallUnmarshalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imethod: u32, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseMarshalData(imethod, &*(&pbuffer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), cbbuffer, ibfirstrelease, datarep, &*(&pcontext as *const <CALLFRAME_MARSHALCONTEXT as ::windows::core::Abi>::Abi as *const <CALLFRAME_MARSHALCONTEXT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICallUnmarshal>, base.5, Unmarshal::<Impl, OFFSET>, ReleaseMarshalData::<Impl, OFFSET>)
    }
}
pub trait IInterfaceRelatedImpl: Sized {
    fn SetIID();
    fn GetIID();
}
impl ::windows::core::RuntimeName for IInterfaceRelated {
    const NAME: &'static str = "Windows.Win32.System.Com.CallObj.IInterfaceRelated";
}
impl IInterfaceRelatedVtbl {
    pub const fn new<Impl: IInterfaceRelatedImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInterfaceRelatedVtbl {
        unsafe extern "system" fn SetIID<Impl: IInterfaceRelatedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetIID(&*(&iid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIID<Impl: IInterfaceRelatedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIID(::core::mem::transmute_copy(&piid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInterfaceRelated>, base.5, SetIID::<Impl, OFFSET>, GetIID::<Impl, OFFSET>)
    }
}
