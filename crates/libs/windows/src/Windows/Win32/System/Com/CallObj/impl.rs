#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ICallFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICallFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICallFrameVtbl {
        unsafe extern "system" fn GetInfo<Impl: ICallFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut CALLFRAMEINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIIDAndMethod<Impl: ICallFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piid: *mut ::windows::core::GUID, pimethod: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNames<Impl: ICallFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszinterface: *mut super::super::super::Foundation::PWSTR, pwszmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStackLocation<Impl: ICallFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStackLocation<Impl: ICallFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvstack: *const ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetReturnValue<Impl: ICallFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetReturnValue<Impl: ICallFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetParamInfo<Impl: ICallFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iparam: u32, pinfo: *mut CALLFRAMEPARAMINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetParam<Impl: ICallFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iparam: u32, pvar: *const super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetParam<Impl: ICallFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iparam: u32, pvar: *mut super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Copy<Impl: ICallFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copycontrol: CALLFRAME_COPY, pwalker: ::windows::core::RawPtr, ppframe: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Free<Impl: ICallFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pframeargsdest: ::windows::core::RawPtr, pwalkerdestfree: ::windows::core::RawPtr, pwalkercopy: ::windows::core::RawPtr, freeflags: u32, pwalkerfree: ::windows::core::RawPtr, nullflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FreeParam<Impl: ICallFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iparam: u32, freeflags: u32, pwalkerfree: ::windows::core::RawPtr, nullflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WalkFrame<Impl: ICallFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, walkwhat: u32, pwalker: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMarshalSizeMax<Impl: ICallFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS, pcbbufferneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Marshal<Impl: ICallFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, pcbbufferused: *mut u32, pdatarep: *mut u32, prpcflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unmarshal<Impl: ICallFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT, pcbunmarshalled: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseMarshalData<Impl: ICallFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Invoke<Impl: ICallFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvreceiver: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetInfo::<Impl, IMPL_OFFSET>,
            GetIIDAndMethod::<Impl, IMPL_OFFSET>,
            GetNames::<Impl, IMPL_OFFSET>,
            GetStackLocation::<Impl, IMPL_OFFSET>,
            SetStackLocation::<Impl, IMPL_OFFSET>,
            SetReturnValue::<Impl, IMPL_OFFSET>,
            GetReturnValue::<Impl, IMPL_OFFSET>,
            GetParamInfo::<Impl, IMPL_OFFSET>,
            SetParam::<Impl, IMPL_OFFSET>,
            GetParam::<Impl, IMPL_OFFSET>,
            Copy::<Impl, IMPL_OFFSET>,
            Free::<Impl, IMPL_OFFSET>,
            FreeParam::<Impl, IMPL_OFFSET>,
            WalkFrame::<Impl, IMPL_OFFSET>,
            GetMarshalSizeMax::<Impl, IMPL_OFFSET>,
            Marshal::<Impl, IMPL_OFFSET>,
            Unmarshal::<Impl, IMPL_OFFSET>,
            ReleaseMarshalData::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICallFrame as ::windows::core::Interface>::IID
    }
}
pub trait ICallFrameEventsImpl: Sized {
    fn OnCall();
}
impl ICallFrameEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICallFrameEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICallFrameEventsVtbl {
        unsafe extern "system" fn OnCall<Impl: ICallFrameEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pframe: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnCall::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICallFrameEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ICallFrameWalkerImpl: Sized {
    fn OnWalkInterface();
}
#[cfg(feature = "Win32_Foundation")]
impl ICallFrameWalkerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICallFrameWalkerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICallFrameWalkerVtbl {
        unsafe extern "system" fn OnWalkInterface<Impl: ICallFrameWalkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, ppvinterface: *const *const ::core::ffi::c_void, fin: super::super::super::Foundation::BOOL, fout: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnWalkInterface::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICallFrameWalker as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ICallIndirectImpl: Sized {
    fn CallIndirect();
    fn GetMethodInfo();
    fn GetStackSize();
    fn GetIID();
}
#[cfg(feature = "Win32_Foundation")]
impl ICallIndirectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICallIndirectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICallIndirectVtbl {
        unsafe extern "system" fn CallIndirect<Impl: ICallIndirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrreturn: *mut ::windows::core::HRESULT, imethod: u32, pvargs: *const ::core::ffi::c_void, cbargs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMethodInfo<Impl: ICallIndirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imethod: u32, pinfo: *mut CALLFRAMEINFO, pwszmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStackSize<Impl: ICallIndirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imethod: u32, cbargs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIID<Impl: ICallIndirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piid: *mut ::windows::core::GUID, pfderivesfromidispatch: *mut super::super::super::Foundation::BOOL, pcmethod: *mut u32, pwszinterface: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CallIndirect::<Impl, IMPL_OFFSET>, GetMethodInfo::<Impl, IMPL_OFFSET>, GetStackSize::<Impl, IMPL_OFFSET>, GetIID::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICallIndirect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ICallInterceptorImpl: Sized + ICallIndirectImpl {
    fn RegisterSink();
    fn GetRegisteredSink();
}
#[cfg(feature = "Win32_Foundation")]
impl ICallInterceptorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICallInterceptorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICallInterceptorVtbl {
        unsafe extern "system" fn RegisterSink<Impl: ICallInterceptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRegisteredSink<Impl: ICallInterceptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CallIndirect::<Impl, IMPL_OFFSET>, GetMethodInfo::<Impl, IMPL_OFFSET>, GetStackSize::<Impl, IMPL_OFFSET>, GetIID::<Impl, IMPL_OFFSET>, RegisterSink::<Impl, IMPL_OFFSET>, GetRegisteredSink::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICallInterceptor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ICallUnmarshalImpl: Sized {
    fn Unmarshal();
    fn ReleaseMarshalData();
}
#[cfg(feature = "Win32_Foundation")]
impl ICallUnmarshalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICallUnmarshalImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICallUnmarshalVtbl {
        unsafe extern "system" fn Unmarshal<Impl: ICallUnmarshalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imethod: u32, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, fforcebuffercopy: super::super::super::Foundation::BOOL, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT, pcbunmarshalled: *mut u32, ppframe: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseMarshalData<Impl: ICallUnmarshalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imethod: u32, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Unmarshal::<Impl, IMPL_OFFSET>, ReleaseMarshalData::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICallUnmarshal as ::windows::core::Interface>::IID
    }
}
pub trait IInterfaceRelatedImpl: Sized {
    fn SetIID();
    fn GetIID();
}
impl IInterfaceRelatedVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInterfaceRelatedImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInterfaceRelatedVtbl {
        unsafe extern "system" fn SetIID<Impl: IInterfaceRelatedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIID<Impl: IInterfaceRelatedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetIID::<Impl, IMPL_OFFSET>, GetIID::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInterfaceRelated as ::windows::core::Interface>::IID
    }
}
