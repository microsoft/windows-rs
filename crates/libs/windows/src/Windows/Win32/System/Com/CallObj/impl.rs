#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait ICallFrame_Impl: Sized {
    fn GetInfo(&mut self) -> ::windows::core::Result<CALLFRAMEINFO>;
    fn GetIIDAndMethod(&mut self, piid: *mut ::windows::core::GUID, pimethod: *mut u32) -> ::windows::core::Result<()>;
    fn GetNames(&mut self, pwszinterface: *mut super::super::super::Foundation::PWSTR, pwszmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetStackLocation(&mut self) -> *mut ::core::ffi::c_void;
    fn SetStackLocation(&mut self, pvstack: *const ::core::ffi::c_void);
    fn SetReturnValue(&mut self, hr: ::windows::core::HRESULT);
    fn GetReturnValue(&mut self) -> ::windows::core::Result<()>;
    fn GetParamInfo(&mut self, iparam: u32) -> ::windows::core::Result<CALLFRAMEPARAMINFO>;
    fn SetParam(&mut self, iparam: u32, pvar: *const super::VARIANT) -> ::windows::core::Result<()>;
    fn GetParam(&mut self, iparam: u32) -> ::windows::core::Result<super::VARIANT>;
    fn Copy(&mut self, copycontrol: CALLFRAME_COPY, pwalker: &::core::option::Option<ICallFrameWalker>) -> ::windows::core::Result<ICallFrame>;
    fn Free(&mut self, pframeargsdest: &::core::option::Option<ICallFrame>, pwalkerdestfree: &::core::option::Option<ICallFrameWalker>, pwalkercopy: &::core::option::Option<ICallFrameWalker>, freeflags: u32, pwalkerfree: &::core::option::Option<ICallFrameWalker>, nullflags: u32) -> ::windows::core::Result<()>;
    fn FreeParam(&mut self, iparam: u32, freeflags: u32, pwalkerfree: &::core::option::Option<ICallFrameWalker>, nullflags: u32) -> ::windows::core::Result<()>;
    fn WalkFrame(&mut self, walkwhat: u32, pwalker: &::core::option::Option<ICallFrameWalker>) -> ::windows::core::Result<()>;
    fn GetMarshalSizeMax(&mut self, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS) -> ::windows::core::Result<u32>;
    fn Marshal(&mut self, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, pcbbufferused: *mut u32, pdatarep: *mut u32, prpcflags: *mut u32) -> ::windows::core::Result<()>;
    fn Unmarshal(&mut self, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows::core::Result<u32>;
    fn ReleaseMarshalData(&mut self, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows::core::Result<()>;
    fn Invoke(&mut self, pvreceiver: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ICallFrame_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICallFrame_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICallFrame_Vtbl {
        unsafe extern "system" fn GetInfo<Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut CALLFRAMEINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *pinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIIDAndMethod<Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piid: *mut ::windows::core::GUID, pimethod: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetIIDAndMethod(::core::mem::transmute_copy(&piid), ::core::mem::transmute_copy(&pimethod)).into()
        }
        unsafe extern "system" fn GetNames<Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszinterface: *mut super::super::super::Foundation::PWSTR, pwszmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNames(::core::mem::transmute_copy(&pwszinterface), ::core::mem::transmute_copy(&pwszmethod)).into()
        }
        unsafe extern "system" fn GetStackLocation<Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStackLocation()
        }
        unsafe extern "system" fn SetStackLocation<Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvstack: *const ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStackLocation(::core::mem::transmute_copy(&pvstack))
        }
        unsafe extern "system" fn SetReturnValue<Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReturnValue(::core::mem::transmute_copy(&hr))
        }
        unsafe extern "system" fn GetReturnValue<Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetReturnValue().into()
        }
        unsafe extern "system" fn GetParamInfo<Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iparam: u32, pinfo: *mut CALLFRAMEPARAMINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParamInfo(::core::mem::transmute_copy(&iparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *pinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParam<Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iparam: u32, pvar: *const super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParam(::core::mem::transmute_copy(&iparam), ::core::mem::transmute_copy(&pvar)).into()
        }
        unsafe extern "system" fn GetParam<Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iparam: u32, pvar: *mut super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParam(::core::mem::transmute_copy(&iparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvar = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Copy<Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copycontrol: CALLFRAME_COPY, pwalker: ::windows::core::RawPtr, ppframe: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Copy(::core::mem::transmute_copy(&copycontrol), ::core::mem::transmute(&pwalker)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppframe = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Free<Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pframeargsdest: ::windows::core::RawPtr, pwalkerdestfree: ::windows::core::RawPtr, pwalkercopy: ::windows::core::RawPtr, freeflags: u32, pwalkerfree: ::windows::core::RawPtr, nullflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Free(::core::mem::transmute(&pframeargsdest), ::core::mem::transmute(&pwalkerdestfree), ::core::mem::transmute(&pwalkercopy), ::core::mem::transmute_copy(&freeflags), ::core::mem::transmute(&pwalkerfree), ::core::mem::transmute_copy(&nullflags)).into()
        }
        unsafe extern "system" fn FreeParam<Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iparam: u32, freeflags: u32, pwalkerfree: ::windows::core::RawPtr, nullflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FreeParam(::core::mem::transmute_copy(&iparam), ::core::mem::transmute_copy(&freeflags), ::core::mem::transmute(&pwalkerfree), ::core::mem::transmute_copy(&nullflags)).into()
        }
        unsafe extern "system" fn WalkFrame<Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, walkwhat: u32, pwalker: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WalkFrame(::core::mem::transmute_copy(&walkwhat), ::core::mem::transmute(&pwalker)).into()
        }
        unsafe extern "system" fn GetMarshalSizeMax<Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS, pcbbufferneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMarshalSizeMax(::core::mem::transmute_copy(&pmshlcontext), ::core::mem::transmute_copy(&mshlflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcbbufferneeded = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Marshal<Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, pcbbufferused: *mut u32, pdatarep: *mut u32, prpcflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Marshal(::core::mem::transmute_copy(&pmshlcontext), ::core::mem::transmute_copy(&mshlflags), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&pcbbufferused), ::core::mem::transmute_copy(&pdatarep), ::core::mem::transmute_copy(&prpcflags)).into()
        }
        unsafe extern "system" fn Unmarshal<Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT, pcbunmarshalled: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unmarshal(::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&datarep), ::core::mem::transmute_copy(&pcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcbunmarshalled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseMarshalData<Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseMarshalData(::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&ibfirstrelease), ::core::mem::transmute_copy(&datarep), ::core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn Invoke<Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvreceiver: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Invoke(::core::mem::transmute_copy(&pvreceiver)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetInfo: GetInfo::<Impl, IMPL_OFFSET>,
            GetIIDAndMethod: GetIIDAndMethod::<Impl, IMPL_OFFSET>,
            GetNames: GetNames::<Impl, IMPL_OFFSET>,
            GetStackLocation: GetStackLocation::<Impl, IMPL_OFFSET>,
            SetStackLocation: SetStackLocation::<Impl, IMPL_OFFSET>,
            SetReturnValue: SetReturnValue::<Impl, IMPL_OFFSET>,
            GetReturnValue: GetReturnValue::<Impl, IMPL_OFFSET>,
            GetParamInfo: GetParamInfo::<Impl, IMPL_OFFSET>,
            SetParam: SetParam::<Impl, IMPL_OFFSET>,
            GetParam: GetParam::<Impl, IMPL_OFFSET>,
            Copy: Copy::<Impl, IMPL_OFFSET>,
            Free: Free::<Impl, IMPL_OFFSET>,
            FreeParam: FreeParam::<Impl, IMPL_OFFSET>,
            WalkFrame: WalkFrame::<Impl, IMPL_OFFSET>,
            GetMarshalSizeMax: GetMarshalSizeMax::<Impl, IMPL_OFFSET>,
            Marshal: Marshal::<Impl, IMPL_OFFSET>,
            Unmarshal: Unmarshal::<Impl, IMPL_OFFSET>,
            ReleaseMarshalData: ReleaseMarshalData::<Impl, IMPL_OFFSET>,
            Invoke: Invoke::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICallFrame as ::windows::core::Interface>::IID
    }
}
pub trait ICallFrameEvents_Impl: Sized {
    fn OnCall(&mut self, pframe: &::core::option::Option<ICallFrame>) -> ::windows::core::Result<()>;
}
impl ICallFrameEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICallFrameEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICallFrameEvents_Vtbl {
        unsafe extern "system" fn OnCall<Impl: ICallFrameEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pframe: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCall(::core::mem::transmute(&pframe)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnCall: OnCall::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICallFrameEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ICallFrameWalker_Impl: Sized {
    fn OnWalkInterface(&mut self, iid: *const ::windows::core::GUID, ppvinterface: *const *const ::core::ffi::c_void, fin: super::super::super::Foundation::BOOL, fout: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ICallFrameWalker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICallFrameWalker_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICallFrameWalker_Vtbl {
        unsafe extern "system" fn OnWalkInterface<Impl: ICallFrameWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, ppvinterface: *const *const ::core::ffi::c_void, fin: super::super::super::Foundation::BOOL, fout: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnWalkInterface(::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&ppvinterface), ::core::mem::transmute_copy(&fin), ::core::mem::transmute_copy(&fout)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnWalkInterface: OnWalkInterface::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICallFrameWalker as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ICallIndirect_Impl: Sized {
    fn CallIndirect(&mut self, phrreturn: *mut ::windows::core::HRESULT, imethod: u32, pvargs: *const ::core::ffi::c_void, cbargs: *mut u32) -> ::windows::core::Result<()>;
    fn GetMethodInfo(&mut self, imethod: u32, pinfo: *mut CALLFRAMEINFO, pwszmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetStackSize(&mut self, imethod: u32) -> ::windows::core::Result<u32>;
    fn GetIID(&mut self, piid: *mut ::windows::core::GUID, pfderivesfromidispatch: *mut super::super::super::Foundation::BOOL, pcmethod: *mut u32, pwszinterface: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ICallIndirect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICallIndirect_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICallIndirect_Vtbl {
        unsafe extern "system" fn CallIndirect<Impl: ICallIndirect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrreturn: *mut ::windows::core::HRESULT, imethod: u32, pvargs: *const ::core::ffi::c_void, cbargs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CallIndirect(::core::mem::transmute_copy(&phrreturn), ::core::mem::transmute_copy(&imethod), ::core::mem::transmute_copy(&pvargs), ::core::mem::transmute_copy(&cbargs)).into()
        }
        unsafe extern "system" fn GetMethodInfo<Impl: ICallIndirect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imethod: u32, pinfo: *mut CALLFRAMEINFO, pwszmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMethodInfo(::core::mem::transmute_copy(&imethod), ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&pwszmethod)).into()
        }
        unsafe extern "system" fn GetStackSize<Impl: ICallIndirect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imethod: u32, cbargs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStackSize(::core::mem::transmute_copy(&imethod)) {
                ::core::result::Result::Ok(ok__) => {
                    *cbargs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIID<Impl: ICallIndirect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piid: *mut ::windows::core::GUID, pfderivesfromidispatch: *mut super::super::super::Foundation::BOOL, pcmethod: *mut u32, pwszinterface: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetIID(::core::mem::transmute_copy(&piid), ::core::mem::transmute_copy(&pfderivesfromidispatch), ::core::mem::transmute_copy(&pcmethod), ::core::mem::transmute_copy(&pwszinterface)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CallIndirect: CallIndirect::<Impl, IMPL_OFFSET>,
            GetMethodInfo: GetMethodInfo::<Impl, IMPL_OFFSET>,
            GetStackSize: GetStackSize::<Impl, IMPL_OFFSET>,
            GetIID: GetIID::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICallIndirect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ICallInterceptor_Impl: Sized + ICallIndirect_Impl {
    fn RegisterSink(&mut self, psink: &::core::option::Option<ICallFrameEvents>) -> ::windows::core::Result<()>;
    fn GetRegisteredSink(&mut self) -> ::windows::core::Result<ICallFrameEvents>;
}
#[cfg(feature = "Win32_Foundation")]
impl ICallInterceptor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICallInterceptor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICallInterceptor_Vtbl {
        unsafe extern "system" fn RegisterSink<Impl: ICallInterceptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterSink(::core::mem::transmute(&psink)).into()
        }
        unsafe extern "system" fn GetRegisteredSink<Impl: ICallInterceptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRegisteredSink() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsink = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ICallIndirect_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RegisterSink: RegisterSink::<Impl, IMPL_OFFSET>,
            GetRegisteredSink: GetRegisteredSink::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICallInterceptor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ICallUnmarshal_Impl: Sized {
    fn Unmarshal(&mut self, imethod: u32, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, fforcebuffercopy: super::super::super::Foundation::BOOL, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT, pcbunmarshalled: *mut u32, ppframe: *mut ::core::option::Option<ICallFrame>) -> ::windows::core::Result<()>;
    fn ReleaseMarshalData(&mut self, imethod: u32, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ICallUnmarshal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICallUnmarshal_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICallUnmarshal_Vtbl {
        unsafe extern "system" fn Unmarshal<Impl: ICallUnmarshal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imethod: u32, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, fforcebuffercopy: super::super::super::Foundation::BOOL, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT, pcbunmarshalled: *mut u32, ppframe: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unmarshal(::core::mem::transmute_copy(&imethod), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&fforcebuffercopy), ::core::mem::transmute_copy(&datarep), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&pcbunmarshalled), ::core::mem::transmute_copy(&ppframe)).into()
        }
        unsafe extern "system" fn ReleaseMarshalData<Impl: ICallUnmarshal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imethod: u32, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseMarshalData(::core::mem::transmute_copy(&imethod), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&ibfirstrelease), ::core::mem::transmute_copy(&datarep), ::core::mem::transmute_copy(&pcontext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Unmarshal: Unmarshal::<Impl, IMPL_OFFSET>,
            ReleaseMarshalData: ReleaseMarshalData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICallUnmarshal as ::windows::core::Interface>::IID
    }
}
pub trait IInterfaceRelated_Impl: Sized {
    fn SetIID(&mut self, iid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetIID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
impl IInterfaceRelated_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInterfaceRelated_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInterfaceRelated_Vtbl {
        unsafe extern "system" fn SetIID<Impl: IInterfaceRelated_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIID(::core::mem::transmute_copy(&iid)).into()
        }
        unsafe extern "system" fn GetIID<Impl: IInterfaceRelated_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIID() {
                ::core::result::Result::Ok(ok__) => {
                    *piid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetIID: SetIID::<Impl, IMPL_OFFSET>, GetIID: GetIID::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInterfaceRelated as ::windows::core::Interface>::IID
    }
}
