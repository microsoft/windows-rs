#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait ICallFrame_Impl: Sized {
    fn GetInfo(&self, pinfo: *mut CALLFRAMEINFO) -> ::windows::core::Result<()>;
    fn GetIIDAndMethod(&self, piid: *mut ::windows::core::GUID, pimethod: *mut u32) -> ::windows::core::Result<()>;
    fn GetNames(&self, pwszinterface: *mut ::windows::core::PWSTR, pwszmethod: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn GetStackLocation(&self) -> *mut ::core::ffi::c_void;
    fn SetStackLocation(&self, pvstack: *const ::core::ffi::c_void);
    fn SetReturnValue(&self, hr: ::windows::core::HRESULT);
    fn GetReturnValue(&self) -> ::windows::core::Result<()>;
    fn GetParamInfo(&self, iparam: u32) -> ::windows::core::Result<CALLFRAMEPARAMINFO>;
    fn SetParam(&self, iparam: u32, pvar: *const super::VARIANT) -> ::windows::core::Result<()>;
    fn GetParam(&self, iparam: u32) -> ::windows::core::Result<super::VARIANT>;
    fn Copy(&self, copycontrol: CALLFRAME_COPY, pwalker: ::core::option::Option<&ICallFrameWalker>) -> ::windows::core::Result<ICallFrame>;
    fn Free(&self, pframeargsdest: ::core::option::Option<&ICallFrame>, pwalkerdestfree: ::core::option::Option<&ICallFrameWalker>, pwalkercopy: ::core::option::Option<&ICallFrameWalker>, freeflags: u32, pwalkerfree: ::core::option::Option<&ICallFrameWalker>, nullflags: u32) -> ::windows::core::Result<()>;
    fn FreeParam(&self, iparam: u32, freeflags: u32, pwalkerfree: ::core::option::Option<&ICallFrameWalker>, nullflags: u32) -> ::windows::core::Result<()>;
    fn WalkFrame(&self, walkwhat: u32, pwalker: ::core::option::Option<&ICallFrameWalker>) -> ::windows::core::Result<()>;
    fn GetMarshalSizeMax(&self, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS) -> ::windows::core::Result<u32>;
    fn Marshal(&self, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, pcbbufferused: *mut u32, pdatarep: *mut u32, prpcflags: *mut u32) -> ::windows::core::Result<()>;
    fn Unmarshal(&self, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows::core::Result<u32>;
    fn ReleaseMarshalData(&self, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows::core::Result<()>;
    fn Invoke(&self, pvreceiver: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ICallFrame {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ICallFrame_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: isize>() -> ICallFrame_Vtbl {
        unsafe extern "system" fn GetInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut CALLFRAMEINFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInfo(::core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn GetIIDAndMethod<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piid: *mut ::windows::core::GUID, pimethod: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetIIDAndMethod(::core::mem::transmute_copy(&piid), ::core::mem::transmute_copy(&pimethod)).into()
        }
        unsafe extern "system" fn GetNames<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszinterface: *mut ::windows::core::PWSTR, pwszmethod: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNames(::core::mem::transmute_copy(&pwszinterface), ::core::mem::transmute_copy(&pwszmethod)).into()
        }
        unsafe extern "system" fn GetStackLocation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStackLocation()
        }
        unsafe extern "system" fn SetStackLocation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvstack: *const ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStackLocation(::core::mem::transmute_copy(&pvstack))
        }
        unsafe extern "system" fn SetReturnValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetReturnValue(::core::mem::transmute_copy(&hr))
        }
        unsafe extern "system" fn GetReturnValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetReturnValue().into()
        }
        unsafe extern "system" fn GetParamInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iparam: u32, pinfo: *mut CALLFRAMEPARAMINFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetParamInfo(::core::mem::transmute_copy(&iparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParam<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iparam: u32, pvar: *const super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetParam(::core::mem::transmute_copy(&iparam), ::core::mem::transmute_copy(&pvar)).into()
        }
        unsafe extern "system" fn GetParam<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iparam: u32, pvar: *mut super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetParam(::core::mem::transmute_copy(&iparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvar, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Copy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copycontrol: CALLFRAME_COPY, pwalker: *mut ::core::ffi::c_void, ppframe: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Copy(::core::mem::transmute_copy(&copycontrol), ::windows::core::from_raw_borrowed(&pwalker)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppframe, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Free<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pframeargsdest: *mut ::core::ffi::c_void, pwalkerdestfree: *mut ::core::ffi::c_void, pwalkercopy: *mut ::core::ffi::c_void, freeflags: u32, pwalkerfree: *mut ::core::ffi::c_void, nullflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Free(::windows::core::from_raw_borrowed(&pframeargsdest), ::windows::core::from_raw_borrowed(&pwalkerdestfree), ::windows::core::from_raw_borrowed(&pwalkercopy), ::core::mem::transmute_copy(&freeflags), ::windows::core::from_raw_borrowed(&pwalkerfree), ::core::mem::transmute_copy(&nullflags)).into()
        }
        unsafe extern "system" fn FreeParam<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iparam: u32, freeflags: u32, pwalkerfree: *mut ::core::ffi::c_void, nullflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FreeParam(::core::mem::transmute_copy(&iparam), ::core::mem::transmute_copy(&freeflags), ::windows::core::from_raw_borrowed(&pwalkerfree), ::core::mem::transmute_copy(&nullflags)).into()
        }
        unsafe extern "system" fn WalkFrame<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, walkwhat: u32, pwalker: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WalkFrame(::core::mem::transmute_copy(&walkwhat), ::windows::core::from_raw_borrowed(&pwalker)).into()
        }
        unsafe extern "system" fn GetMarshalSizeMax<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS, pcbbufferneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMarshalSizeMax(::core::mem::transmute_copy(&pmshlcontext), ::core::mem::transmute_copy(&mshlflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbbufferneeded, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Marshal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, pcbbufferused: *mut u32, pdatarep: *mut u32, prpcflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Marshal(::core::mem::transmute_copy(&pmshlcontext), ::core::mem::transmute_copy(&mshlflags), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&pcbbufferused), ::core::mem::transmute_copy(&pdatarep), ::core::mem::transmute_copy(&prpcflags)).into()
        }
        unsafe extern "system" fn Unmarshal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT, pcbunmarshalled: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Unmarshal(::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&datarep), ::core::mem::transmute_copy(&pcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbunmarshalled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseMarshalData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseMarshalData(::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&ibfirstrelease), ::core::mem::transmute_copy(&datarep), ::core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn Invoke<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvreceiver: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Invoke(::core::mem::transmute_copy(&pvreceiver)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInfo: GetInfo::<Identity, Impl, OFFSET>,
            GetIIDAndMethod: GetIIDAndMethod::<Identity, Impl, OFFSET>,
            GetNames: GetNames::<Identity, Impl, OFFSET>,
            GetStackLocation: GetStackLocation::<Identity, Impl, OFFSET>,
            SetStackLocation: SetStackLocation::<Identity, Impl, OFFSET>,
            SetReturnValue: SetReturnValue::<Identity, Impl, OFFSET>,
            GetReturnValue: GetReturnValue::<Identity, Impl, OFFSET>,
            GetParamInfo: GetParamInfo::<Identity, Impl, OFFSET>,
            SetParam: SetParam::<Identity, Impl, OFFSET>,
            GetParam: GetParam::<Identity, Impl, OFFSET>,
            Copy: Copy::<Identity, Impl, OFFSET>,
            Free: Free::<Identity, Impl, OFFSET>,
            FreeParam: FreeParam::<Identity, Impl, OFFSET>,
            WalkFrame: WalkFrame::<Identity, Impl, OFFSET>,
            GetMarshalSizeMax: GetMarshalSizeMax::<Identity, Impl, OFFSET>,
            Marshal: Marshal::<Identity, Impl, OFFSET>,
            Unmarshal: Unmarshal::<Identity, Impl, OFFSET>,
            ReleaseMarshalData: ReleaseMarshalData::<Identity, Impl, OFFSET>,
            Invoke: Invoke::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICallFrame as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`, `\"implement\"`*"]
pub trait ICallFrameEvents_Impl: Sized {
    fn OnCall(&self, pframe: ::core::option::Option<&ICallFrame>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICallFrameEvents {}
impl ICallFrameEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallFrameEvents_Impl, const OFFSET: isize>() -> ICallFrameEvents_Vtbl {
        unsafe extern "system" fn OnCall<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallFrameEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pframe: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCall(::windows::core::from_raw_borrowed(&pframe)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnCall: OnCall::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICallFrameEvents as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICallFrameWalker_Impl: Sized {
    fn OnWalkInterface(&self, iid: *const ::windows::core::GUID, ppvinterface: *const *const ::core::ffi::c_void, fin: super::super::super::Foundation::BOOL, fout: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ICallFrameWalker {}
#[cfg(feature = "Win32_Foundation")]
impl ICallFrameWalker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallFrameWalker_Impl, const OFFSET: isize>() -> ICallFrameWalker_Vtbl {
        unsafe extern "system" fn OnWalkInterface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallFrameWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, ppvinterface: *const *const ::core::ffi::c_void, fin: super::super::super::Foundation::BOOL, fout: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnWalkInterface(::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&ppvinterface), ::core::mem::transmute_copy(&fin), ::core::mem::transmute_copy(&fout)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnWalkInterface: OnWalkInterface::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICallFrameWalker as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICallIndirect_Impl: Sized {
    fn CallIndirect(&self, phrreturn: *mut ::windows::core::HRESULT, imethod: u32, pvargs: *const ::core::ffi::c_void, cbargs: *mut u32) -> ::windows::core::Result<()>;
    fn GetMethodInfo(&self, imethod: u32, pinfo: *mut CALLFRAMEINFO, pwszmethod: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn GetStackSize(&self, imethod: u32) -> ::windows::core::Result<u32>;
    fn GetIID(&self, piid: *mut ::windows::core::GUID, pfderivesfromidispatch: *mut super::super::super::Foundation::BOOL, pcmethod: *mut u32, pwszinterface: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ICallIndirect {}
#[cfg(feature = "Win32_Foundation")]
impl ICallIndirect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallIndirect_Impl, const OFFSET: isize>() -> ICallIndirect_Vtbl {
        unsafe extern "system" fn CallIndirect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallIndirect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrreturn: *mut ::windows::core::HRESULT, imethod: u32, pvargs: *const ::core::ffi::c_void, cbargs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CallIndirect(::core::mem::transmute_copy(&phrreturn), ::core::mem::transmute_copy(&imethod), ::core::mem::transmute_copy(&pvargs), ::core::mem::transmute_copy(&cbargs)).into()
        }
        unsafe extern "system" fn GetMethodInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallIndirect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imethod: u32, pinfo: *mut CALLFRAMEINFO, pwszmethod: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMethodInfo(::core::mem::transmute_copy(&imethod), ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&pwszmethod)).into()
        }
        unsafe extern "system" fn GetStackSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallIndirect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imethod: u32, cbargs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStackSize(::core::mem::transmute_copy(&imethod)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cbargs, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallIndirect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piid: *mut ::windows::core::GUID, pfderivesfromidispatch: *mut super::super::super::Foundation::BOOL, pcmethod: *mut u32, pwszinterface: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetIID(::core::mem::transmute_copy(&piid), ::core::mem::transmute_copy(&pfderivesfromidispatch), ::core::mem::transmute_copy(&pcmethod), ::core::mem::transmute_copy(&pwszinterface)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CallIndirect: CallIndirect::<Identity, Impl, OFFSET>,
            GetMethodInfo: GetMethodInfo::<Identity, Impl, OFFSET>,
            GetStackSize: GetStackSize::<Identity, Impl, OFFSET>,
            GetIID: GetIID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICallIndirect as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICallInterceptor_Impl: Sized + ICallIndirect_Impl {
    fn RegisterSink(&self, psink: ::core::option::Option<&ICallFrameEvents>) -> ::windows::core::Result<()>;
    fn GetRegisteredSink(&self) -> ::windows::core::Result<ICallFrameEvents>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ICallInterceptor {}
#[cfg(feature = "Win32_Foundation")]
impl ICallInterceptor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallInterceptor_Impl, const OFFSET: isize>() -> ICallInterceptor_Vtbl {
        unsafe extern "system" fn RegisterSink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallInterceptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterSink(::windows::core::from_raw_borrowed(&psink)).into()
        }
        unsafe extern "system" fn GetRegisteredSink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallInterceptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsink: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRegisteredSink() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsink, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ICallIndirect_Vtbl::new::<Identity, Impl, OFFSET>(),
            RegisterSink: RegisterSink::<Identity, Impl, OFFSET>,
            GetRegisteredSink: GetRegisteredSink::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICallInterceptor as ::windows::core::ComInterface>::IID || iid == &<ICallIndirect as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICallUnmarshal_Impl: Sized {
    fn Unmarshal(&self, imethod: u32, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, fforcebuffercopy: super::super::super::Foundation::BOOL, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT, pcbunmarshalled: *mut u32, ppframe: *mut ::core::option::Option<ICallFrame>) -> ::windows::core::Result<()>;
    fn ReleaseMarshalData(&self, imethod: u32, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ICallUnmarshal {}
#[cfg(feature = "Win32_Foundation")]
impl ICallUnmarshal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallUnmarshal_Impl, const OFFSET: isize>() -> ICallUnmarshal_Vtbl {
        unsafe extern "system" fn Unmarshal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallUnmarshal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imethod: u32, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, fforcebuffercopy: super::super::super::Foundation::BOOL, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT, pcbunmarshalled: *mut u32, ppframe: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unmarshal(::core::mem::transmute_copy(&imethod), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&fforcebuffercopy), ::core::mem::transmute_copy(&datarep), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&pcbunmarshalled), ::core::mem::transmute_copy(&ppframe)).into()
        }
        unsafe extern "system" fn ReleaseMarshalData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallUnmarshal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imethod: u32, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseMarshalData(::core::mem::transmute_copy(&imethod), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&ibfirstrelease), ::core::mem::transmute_copy(&datarep), ::core::mem::transmute_copy(&pcontext)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Unmarshal: Unmarshal::<Identity, Impl, OFFSET>,
            ReleaseMarshalData: ReleaseMarshalData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICallUnmarshal as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`, `\"implement\"`*"]
pub trait IInterfaceRelated_Impl: Sized {
    fn SetIID(&self, iid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetIID(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
impl ::windows::core::RuntimeName for IInterfaceRelated {}
impl IInterfaceRelated_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInterfaceRelated_Impl, const OFFSET: isize>() -> IInterfaceRelated_Vtbl {
        unsafe extern "system" fn SetIID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInterfaceRelated_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIID(::core::mem::transmute_copy(&iid)).into()
        }
        unsafe extern "system" fn GetIID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInterfaceRelated_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(piid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetIID: SetIID::<Identity, Impl, OFFSET>,
            GetIID: GetIID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInterfaceRelated as ::windows::core::ComInterface>::IID
    }
}
