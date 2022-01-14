#[cfg(feature = "Win32_Graphics_Imaging")]
pub trait IMILBitmapEffect_Impl: Sized {
    fn GetOutput(&mut self, uiindex: u32, pcontext: &::core::option::Option<IMILBitmapEffectRenderContext>) -> ::windows::core::Result<super::super::Graphics::Imaging::IWICBitmapSource>;
    fn GetParentEffect(&mut self) -> ::windows::core::Result<IMILBitmapEffectGroup>;
    fn SetInputSource(&mut self, uiindex: u32, pbitmapsource: &::core::option::Option<super::super::Graphics::Imaging::IWICBitmapSource>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Imaging")]
impl IMILBitmapEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffect_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffect_Vtbl {
        unsafe extern "system" fn GetOutput<Impl: IMILBitmapEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, pcontext: ::windows::core::RawPtr, ppbitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutput(::core::mem::transmute_copy(&uiindex), ::core::mem::transmute(&pcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbitmapsource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParentEffect<Impl: IMILBitmapEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparenteffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParentEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *ppparenteffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInputSource<Impl: IMILBitmapEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, pbitmapsource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInputSource(::core::mem::transmute_copy(&uiindex), ::core::mem::transmute(&pbitmapsource)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetOutput: GetOutput::<Impl, IMPL_OFFSET>,
            GetParentEffect: GetParentEffect::<Impl, IMPL_OFFSET>,
            SetInputSource: SetInputSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffect as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectConnections_Impl: Sized {
    fn GetInputConnector(&mut self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectInputConnector>;
    fn GetOutputConnector(&mut self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectOutputConnector>;
}
impl IMILBitmapEffectConnections_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectConnections_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectConnections_Vtbl {
        unsafe extern "system" fn GetInputConnector<Impl: IMILBitmapEffectConnections_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputConnector(::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppconnector = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputConnector<Impl: IMILBitmapEffectConnections_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputConnector(::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppconnector = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetInputConnector: GetInputConnector::<Impl, IMPL_OFFSET>,
            GetOutputConnector: GetOutputConnector::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectConnections as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectConnectionsInfo_Impl: Sized {
    fn GetNumberInputs(&mut self) -> ::windows::core::Result<u32>;
    fn GetNumberOutputs(&mut self) -> ::windows::core::Result<u32>;
    fn GetInputConnectorInfo(&mut self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectConnectorInfo>;
    fn GetOutputConnectorInfo(&mut self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectConnectorInfo>;
}
impl IMILBitmapEffectConnectionsInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectConnectionsInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectConnectionsInfo_Vtbl {
        unsafe extern "system" fn GetNumberInputs<Impl: IMILBitmapEffectConnectionsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puinuminputs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumberInputs() {
                ::core::result::Result::Ok(ok__) => {
                    *puinuminputs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumberOutputs<Impl: IMILBitmapEffectConnectionsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puinumoutputs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumberOutputs() {
                ::core::result::Result::Ok(ok__) => {
                    *puinumoutputs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputConnectorInfo<Impl: IMILBitmapEffectConnectionsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnectorinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputConnectorInfo(::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppconnectorinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputConnectorInfo<Impl: IMILBitmapEffectConnectionsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnectorinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputConnectorInfo(::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppconnectorinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetNumberInputs: GetNumberInputs::<Impl, IMPL_OFFSET>,
            GetNumberOutputs: GetNumberOutputs::<Impl, IMPL_OFFSET>,
            GetInputConnectorInfo: GetInputConnectorInfo::<Impl, IMPL_OFFSET>,
            GetOutputConnectorInfo: GetOutputConnectorInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectConnectionsInfo as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectConnector_Impl: Sized + IMILBitmapEffectConnectorInfo_Impl {
    fn IsConnected(&mut self) -> ::windows::core::Result<i16>;
    fn GetBitmapEffect(&mut self) -> ::windows::core::Result<IMILBitmapEffect>;
}
impl IMILBitmapEffectConnector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectConnector_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectConnector_Vtbl {
        unsafe extern "system" fn IsConnected<Impl: IMILBitmapEffectConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsConnected() {
                ::core::result::Result::Ok(ok__) => {
                    *pfconnected = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBitmapEffect<Impl: IMILBitmapEffectConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBitmapEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *ppeffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IMILBitmapEffectConnectorInfo_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            IsConnected: IsConnected::<Impl, IMPL_OFFSET>,
            GetBitmapEffect: GetBitmapEffect::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectConnector as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectConnectorInfo_Impl: Sized {
    fn GetIndex(&mut self) -> ::windows::core::Result<u32>;
    fn GetOptimalFormat(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetNumberFormats(&mut self) -> ::windows::core::Result<u32>;
    fn GetFormat(&mut self, ulindex: u32) -> ::windows::core::Result<::windows::core::GUID>;
}
impl IMILBitmapEffectConnectorInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectConnectorInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectConnectorInfo_Vtbl {
        unsafe extern "system" fn GetIndex<Impl: IMILBitmapEffectConnectorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *puiindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptimalFormat<Impl: IMILBitmapEffectConnectorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOptimalFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *pformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumberFormats<Impl: IMILBitmapEffectConnectorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulnumberformats: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumberFormats() {
                ::core::result::Result::Ok(ok__) => {
                    *pulnumberformats = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormat<Impl: IMILBitmapEffectConnectorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormat(::core::mem::transmute_copy(&ulindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetIndex: GetIndex::<Impl, IMPL_OFFSET>,
            GetOptimalFormat: GetOptimalFormat::<Impl, IMPL_OFFSET>,
            GetNumberFormats: GetNumberFormats::<Impl, IMPL_OFFSET>,
            GetFormat: GetFormat::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectConnectorInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMILBitmapEffectEvents_Impl: Sized {
    fn PropertyChange(&mut self, peffect: &::core::option::Option<IMILBitmapEffect>, bstrpropertyname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DirtyRegion(&mut self, peffect: &::core::option::Option<IMILBitmapEffect>, prect: *const MilRectD) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMILBitmapEffectEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectEvents_Vtbl {
        unsafe extern "system" fn PropertyChange<Impl: IMILBitmapEffectEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peffect: ::windows::core::RawPtr, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PropertyChange(::core::mem::transmute(&peffect), ::core::mem::transmute_copy(&bstrpropertyname)).into()
        }
        unsafe extern "system" fn DirtyRegion<Impl: IMILBitmapEffectEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peffect: ::windows::core::RawPtr, prect: *const MilRectD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DirtyRegion(::core::mem::transmute(&peffect), ::core::mem::transmute_copy(&prect)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            PropertyChange: PropertyChange::<Impl, IMPL_OFFSET>,
            DirtyRegion: DirtyRegion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectEvents as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectFactory_Impl: Sized {
    fn CreateEffect(&mut self, pguideffect: *const ::windows::core::GUID) -> ::windows::core::Result<IMILBitmapEffect>;
    fn CreateContext(&mut self) -> ::windows::core::Result<IMILBitmapEffectRenderContext>;
    fn CreateEffectOuter(&mut self) -> ::windows::core::Result<IMILBitmapEffect>;
}
impl IMILBitmapEffectFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectFactory_Vtbl {
        unsafe extern "system" fn CreateEffect<Impl: IMILBitmapEffectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguideffect: *const ::windows::core::GUID, ppeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEffect(::core::mem::transmute_copy(&pguideffect)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppeffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateContext<Impl: IMILBitmapEffectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateContext() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEffectOuter<Impl: IMILBitmapEffectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEffectOuter() {
                ::core::result::Result::Ok(ok__) => {
                    *ppeffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateEffect: CreateEffect::<Impl, IMPL_OFFSET>,
            CreateContext: CreateContext::<Impl, IMPL_OFFSET>,
            CreateEffectOuter: CreateEffectOuter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectFactory as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectGroup_Impl: Sized {
    fn GetInteriorInputConnector(&mut self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectOutputConnector>;
    fn GetInteriorOutputConnector(&mut self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectInputConnector>;
    fn Add(&mut self, peffect: &::core::option::Option<IMILBitmapEffect>) -> ::windows::core::Result<()>;
}
impl IMILBitmapEffectGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectGroup_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectGroup_Vtbl {
        unsafe extern "system" fn GetInteriorInputConnector<Impl: IMILBitmapEffectGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInteriorInputConnector(::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppconnector = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInteriorOutputConnector<Impl: IMILBitmapEffectGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInteriorOutputConnector(::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppconnector = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IMILBitmapEffectGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peffect: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&peffect)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetInteriorInputConnector: GetInteriorInputConnector::<Impl, IMPL_OFFSET>,
            GetInteriorOutputConnector: GetInteriorOutputConnector::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectGroup as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectGroupImpl_Impl: Sized {
    fn Preprocess(&mut self, pcontext: &::core::option::Option<IMILBitmapEffectRenderContext>) -> ::windows::core::Result<()>;
    fn GetNumberChildren(&mut self) -> ::windows::core::Result<u32>;
    fn GetChildren(&mut self) -> ::windows::core::Result<IMILBitmapEffects>;
}
impl IMILBitmapEffectGroupImpl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectGroupImpl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectGroupImpl_Vtbl {
        unsafe extern "system" fn Preprocess<Impl: IMILBitmapEffectGroupImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Preprocess(::core::mem::transmute(&pcontext)).into()
        }
        unsafe extern "system" fn GetNumberChildren<Impl: IMILBitmapEffectGroupImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puinumberchildren: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumberChildren() {
                ::core::result::Result::Ok(ok__) => {
                    *puinumberchildren = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChildren<Impl: IMILBitmapEffectGroupImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchildren: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChildren() {
                ::core::result::Result::Ok(ok__) => {
                    *pchildren = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Preprocess: Preprocess::<Impl, IMPL_OFFSET>,
            GetNumberChildren: GetNumberChildren::<Impl, IMPL_OFFSET>,
            GetChildren: GetChildren::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectGroupImpl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Imaging")]
pub trait IMILBitmapEffectImpl_Impl: Sized {
    fn IsInPlaceModificationAllowed(&mut self, poutputconnector: &::core::option::Option<IMILBitmapEffectOutputConnector>) -> ::windows::core::Result<i16>;
    fn SetParentEffect(&mut self, pparenteffect: &::core::option::Option<IMILBitmapEffectGroup>) -> ::windows::core::Result<()>;
    fn GetInputSource(&mut self, uiindex: u32) -> ::windows::core::Result<super::super::Graphics::Imaging::IWICBitmapSource>;
    fn GetInputSourceBounds(&mut self, uiindex: u32) -> ::windows::core::Result<MilRectD>;
    fn GetInputBitmapSource(&mut self, uiindex: u32, prendercontext: &::core::option::Option<IMILBitmapEffectRenderContext>, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::core::option::Option<super::super::Graphics::Imaging::IWICBitmapSource>) -> ::windows::core::Result<()>;
    fn GetOutputBitmapSource(&mut self, uiindex: u32, prendercontext: &::core::option::Option<IMILBitmapEffectRenderContext>, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::core::option::Option<super::super::Graphics::Imaging::IWICBitmapSource>) -> ::windows::core::Result<()>;
    fn Initialize(&mut self, pinner: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Imaging")]
impl IMILBitmapEffectImpl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectImpl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectImpl_Vtbl {
        unsafe extern "system" fn IsInPlaceModificationAllowed<Impl: IMILBitmapEffectImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutputconnector: ::windows::core::RawPtr, pfmodifyinplace: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInPlaceModificationAllowed(::core::mem::transmute(&poutputconnector)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfmodifyinplace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParentEffect<Impl: IMILBitmapEffectImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparenteffect: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParentEffect(::core::mem::transmute(&pparenteffect)).into()
        }
        unsafe extern "system" fn GetInputSource<Impl: IMILBitmapEffectImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppbitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputSource(::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbitmapsource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputSourceBounds<Impl: IMILBitmapEffectImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, prect: *mut MilRectD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputSourceBounds(::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *prect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputBitmapSource<Impl: IMILBitmapEffectImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, prendercontext: ::windows::core::RawPtr, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInputBitmapSource(::core::mem::transmute_copy(&uiindex), ::core::mem::transmute(&prendercontext), ::core::mem::transmute_copy(&pfmodifyinplace), ::core::mem::transmute_copy(&ppbitmapsource)).into()
        }
        unsafe extern "system" fn GetOutputBitmapSource<Impl: IMILBitmapEffectImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, prendercontext: ::windows::core::RawPtr, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOutputBitmapSource(::core::mem::transmute_copy(&uiindex), ::core::mem::transmute(&prendercontext), ::core::mem::transmute_copy(&pfmodifyinplace), ::core::mem::transmute_copy(&ppbitmapsource)).into()
        }
        unsafe extern "system" fn Initialize<Impl: IMILBitmapEffectImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinner: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pinner)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            IsInPlaceModificationAllowed: IsInPlaceModificationAllowed::<Impl, IMPL_OFFSET>,
            SetParentEffect: SetParentEffect::<Impl, IMPL_OFFSET>,
            GetInputSource: GetInputSource::<Impl, IMPL_OFFSET>,
            GetInputSourceBounds: GetInputSourceBounds::<Impl, IMPL_OFFSET>,
            GetInputBitmapSource: GetInputBitmapSource::<Impl, IMPL_OFFSET>,
            GetOutputBitmapSource: GetOutputBitmapSource::<Impl, IMPL_OFFSET>,
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectImpl as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectInputConnector_Impl: Sized + IMILBitmapEffectConnectorInfo_Impl + IMILBitmapEffectConnector_Impl {
    fn ConnectTo(&mut self, pconnector: &::core::option::Option<IMILBitmapEffectOutputConnector>) -> ::windows::core::Result<()>;
    fn GetConnection(&mut self) -> ::windows::core::Result<IMILBitmapEffectOutputConnector>;
}
impl IMILBitmapEffectInputConnector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectInputConnector_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectInputConnector_Vtbl {
        unsafe extern "system" fn ConnectTo<Impl: IMILBitmapEffectInputConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnector: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConnectTo(::core::mem::transmute(&pconnector)).into()
        }
        unsafe extern "system" fn GetConnection<Impl: IMILBitmapEffectInputConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnection() {
                ::core::result::Result::Ok(ok__) => {
                    *ppconnector = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IMILBitmapEffectConnector_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ConnectTo: ConnectTo::<Impl, IMPL_OFFSET>,
            GetConnection: GetConnection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectInputConnector as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectInteriorInputConnector_Impl: Sized {
    fn GetInputConnector(&mut self) -> ::windows::core::Result<IMILBitmapEffectInputConnector>;
}
impl IMILBitmapEffectInteriorInputConnector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectInteriorInputConnector_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectInteriorInputConnector_Vtbl {
        unsafe extern "system" fn GetInputConnector<Impl: IMILBitmapEffectInteriorInputConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputConnector() {
                ::core::result::Result::Ok(ok__) => {
                    *pinputconnector = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetInputConnector: GetInputConnector::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectInteriorInputConnector as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectInteriorOutputConnector_Impl: Sized {
    fn GetOutputConnector(&mut self) -> ::windows::core::Result<IMILBitmapEffectOutputConnector>;
}
impl IMILBitmapEffectInteriorOutputConnector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectInteriorOutputConnector_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectInteriorOutputConnector_Vtbl {
        unsafe extern "system" fn GetOutputConnector<Impl: IMILBitmapEffectInteriorOutputConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutputconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputConnector() {
                ::core::result::Result::Ok(ok__) => {
                    *poutputconnector = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetOutputConnector: GetOutputConnector::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectInteriorOutputConnector as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectOutputConnector_Impl: Sized + IMILBitmapEffectConnectorInfo_Impl + IMILBitmapEffectConnector_Impl {
    fn GetNumberConnections(&mut self) -> ::windows::core::Result<u32>;
    fn GetConnection(&mut self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectInputConnector>;
}
impl IMILBitmapEffectOutputConnector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectOutputConnector_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectOutputConnector_Vtbl {
        unsafe extern "system" fn GetNumberConnections<Impl: IMILBitmapEffectOutputConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puinumberconnections: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumberConnections() {
                ::core::result::Result::Ok(ok__) => {
                    *puinumberconnections = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnection<Impl: IMILBitmapEffectOutputConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnection(::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppconnection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IMILBitmapEffectConnector_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetNumberConnections: GetNumberConnections::<Impl, IMPL_OFFSET>,
            GetConnection: GetConnection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectOutputConnector as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectOutputConnectorImpl_Impl: Sized {
    fn AddBackLink(&mut self, pconnection: &::core::option::Option<IMILBitmapEffectInputConnector>) -> ::windows::core::Result<()>;
    fn RemoveBackLink(&mut self, pconnection: &::core::option::Option<IMILBitmapEffectInputConnector>) -> ::windows::core::Result<()>;
}
impl IMILBitmapEffectOutputConnectorImpl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectOutputConnectorImpl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectOutputConnectorImpl_Vtbl {
        unsafe extern "system" fn AddBackLink<Impl: IMILBitmapEffectOutputConnectorImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddBackLink(::core::mem::transmute(&pconnection)).into()
        }
        unsafe extern "system" fn RemoveBackLink<Impl: IMILBitmapEffectOutputConnectorImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBackLink(::core::mem::transmute(&pconnection)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddBackLink: AddBackLink::<Impl, IMPL_OFFSET>,
            RemoveBackLink: RemoveBackLink::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectOutputConnectorImpl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dwm", feature = "Win32_Graphics_Imaging"))]
pub trait IMILBitmapEffectPrimitive_Impl: Sized {
    fn GetOutput(&mut self, uiindex: u32, pcontext: &::core::option::Option<IMILBitmapEffectRenderContext>, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::core::option::Option<super::super::Graphics::Imaging::IWICBitmapSource>) -> ::windows::core::Result<()>;
    fn TransformPoint(&mut self, uiindex: u32, p: *mut MilPoint2D, fforwardtransform: i16, pcontext: &::core::option::Option<IMILBitmapEffectRenderContext>, pfpointtransformed: *mut i16) -> ::windows::core::Result<()>;
    fn TransformRect(&mut self, uiindex: u32, p: *mut MilRectD, fforwardtransform: i16, pcontext: &::core::option::Option<IMILBitmapEffectRenderContext>) -> ::windows::core::Result<()>;
    fn HasAffineTransform(&mut self, uiindex: u32) -> ::windows::core::Result<i16>;
    fn HasInverseTransform(&mut self, uiindex: u32) -> ::windows::core::Result<i16>;
    fn GetAffineMatrix(&mut self, uiindex: u32, pmatrix: *mut super::super::Graphics::Dwm::MilMatrix3x2D) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Dwm", feature = "Win32_Graphics_Imaging"))]
impl IMILBitmapEffectPrimitive_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectPrimitive_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectPrimitive_Vtbl {
        unsafe extern "system" fn GetOutput<Impl: IMILBitmapEffectPrimitive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, pcontext: ::windows::core::RawPtr, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOutput(::core::mem::transmute_copy(&uiindex), ::core::mem::transmute(&pcontext), ::core::mem::transmute_copy(&pfmodifyinplace), ::core::mem::transmute_copy(&ppbitmapsource)).into()
        }
        unsafe extern "system" fn TransformPoint<Impl: IMILBitmapEffectPrimitive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, p: *mut MilPoint2D, fforwardtransform: i16, pcontext: ::windows::core::RawPtr, pfpointtransformed: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TransformPoint(::core::mem::transmute_copy(&uiindex), ::core::mem::transmute_copy(&p), ::core::mem::transmute_copy(&fforwardtransform), ::core::mem::transmute(&pcontext), ::core::mem::transmute_copy(&pfpointtransformed)).into()
        }
        unsafe extern "system" fn TransformRect<Impl: IMILBitmapEffectPrimitive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, p: *mut MilRectD, fforwardtransform: i16, pcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TransformRect(::core::mem::transmute_copy(&uiindex), ::core::mem::transmute_copy(&p), ::core::mem::transmute_copy(&fforwardtransform), ::core::mem::transmute(&pcontext)).into()
        }
        unsafe extern "system" fn HasAffineTransform<Impl: IMILBitmapEffectPrimitive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, pfaffine: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasAffineTransform(::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfaffine = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasInverseTransform<Impl: IMILBitmapEffectPrimitive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, pfhasinverse: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasInverseTransform(::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfhasinverse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAffineMatrix<Impl: IMILBitmapEffectPrimitive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, pmatrix: *mut super::super::Graphics::Dwm::MilMatrix3x2D) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAffineMatrix(::core::mem::transmute_copy(&uiindex), ::core::mem::transmute_copy(&pmatrix)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetOutput: GetOutput::<Impl, IMPL_OFFSET>,
            TransformPoint: TransformPoint::<Impl, IMPL_OFFSET>,
            TransformRect: TransformRect::<Impl, IMPL_OFFSET>,
            HasAffineTransform: HasAffineTransform::<Impl, IMPL_OFFSET>,
            HasInverseTransform: HasInverseTransform::<Impl, IMPL_OFFSET>,
            GetAffineMatrix: GetAffineMatrix::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectPrimitive as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectPrimitiveImpl_Impl: Sized {
    fn IsDirty(&mut self, uioutputindex: u32) -> ::windows::core::Result<i16>;
    fn IsVolatile(&mut self, uioutputindex: u32) -> ::windows::core::Result<i16>;
}
impl IMILBitmapEffectPrimitiveImpl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectPrimitiveImpl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectPrimitiveImpl_Vtbl {
        unsafe extern "system" fn IsDirty<Impl: IMILBitmapEffectPrimitiveImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uioutputindex: u32, pfdirty: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDirty(::core::mem::transmute_copy(&uioutputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfdirty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVolatile<Impl: IMILBitmapEffectPrimitiveImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uioutputindex: u32, pfvolatile: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsVolatile(::core::mem::transmute_copy(&uioutputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfvolatile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            IsDirty: IsDirty::<Impl, IMPL_OFFSET>,
            IsVolatile: IsVolatile::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectPrimitiveImpl as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectRenderContext_Impl: Sized {
    fn SetOutputPixelFormat(&mut self, format: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetOutputPixelFormat(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetUseSoftwareRenderer(&mut self, fsoftware: i16) -> ::windows::core::Result<()>;
    fn SetInitialTransform(&mut self, pmatrix: *const MILMatrixF) -> ::windows::core::Result<()>;
    fn GetFinalTransform(&mut self) -> ::windows::core::Result<MILMatrixF>;
    fn SetOutputDPI(&mut self, dbldpix: f64, dbldpiy: f64) -> ::windows::core::Result<()>;
    fn GetOutputDPI(&mut self, pdbldpix: *mut f64, pdbldpiy: *mut f64) -> ::windows::core::Result<()>;
    fn SetRegionOfInterest(&mut self, prect: *const MilRectD) -> ::windows::core::Result<()>;
}
impl IMILBitmapEffectRenderContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectRenderContext_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectRenderContext_Vtbl {
        unsafe extern "system" fn SetOutputPixelFormat<Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutputPixelFormat(::core::mem::transmute_copy(&format)).into()
        }
        unsafe extern "system" fn GetOutputPixelFormat<Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputPixelFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *pformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseSoftwareRenderer<Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fsoftware: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUseSoftwareRenderer(::core::mem::transmute_copy(&fsoftware)).into()
        }
        unsafe extern "system" fn SetInitialTransform<Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmatrix: *const MILMatrixF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialTransform(::core::mem::transmute_copy(&pmatrix)).into()
        }
        unsafe extern "system" fn GetFinalTransform<Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmatrix: *mut MILMatrixF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFinalTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *pmatrix = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputDPI<Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dbldpix: f64, dbldpiy: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutputDPI(::core::mem::transmute_copy(&dbldpix), ::core::mem::transmute_copy(&dbldpiy)).into()
        }
        unsafe extern "system" fn GetOutputDPI<Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdbldpix: *mut f64, pdbldpiy: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOutputDPI(::core::mem::transmute_copy(&pdbldpix), ::core::mem::transmute_copy(&pdbldpiy)).into()
        }
        unsafe extern "system" fn SetRegionOfInterest<Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const MilRectD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRegionOfInterest(::core::mem::transmute_copy(&prect)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetOutputPixelFormat: SetOutputPixelFormat::<Impl, IMPL_OFFSET>,
            GetOutputPixelFormat: GetOutputPixelFormat::<Impl, IMPL_OFFSET>,
            SetUseSoftwareRenderer: SetUseSoftwareRenderer::<Impl, IMPL_OFFSET>,
            SetInitialTransform: SetInitialTransform::<Impl, IMPL_OFFSET>,
            GetFinalTransform: GetFinalTransform::<Impl, IMPL_OFFSET>,
            SetOutputDPI: SetOutputDPI::<Impl, IMPL_OFFSET>,
            GetOutputDPI: GetOutputDPI::<Impl, IMPL_OFFSET>,
            SetRegionOfInterest: SetRegionOfInterest::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectRenderContext as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectRenderContextImpl_Impl: Sized {
    fn GetUseSoftwareRenderer(&mut self) -> ::windows::core::Result<i16>;
    fn GetTransform(&mut self, pmatrix: *mut MILMatrixF) -> ::windows::core::Result<()>;
    fn UpdateTransform(&mut self, pmatrix: *const MILMatrixF) -> ::windows::core::Result<()>;
    fn GetOutputBounds(&mut self, prect: *mut MilRectD) -> ::windows::core::Result<()>;
    fn UpdateOutputBounds(&mut self, prect: *const MilRectD) -> ::windows::core::Result<()>;
}
impl IMILBitmapEffectRenderContextImpl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectRenderContextImpl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectRenderContextImpl_Vtbl {
        unsafe extern "system" fn GetUseSoftwareRenderer<Impl: IMILBitmapEffectRenderContextImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfsoftware: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUseSoftwareRenderer() {
                ::core::result::Result::Ok(ok__) => {
                    *pfsoftware = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransform<Impl: IMILBitmapEffectRenderContextImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmatrix: *mut MILMatrixF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTransform(::core::mem::transmute_copy(&pmatrix)).into()
        }
        unsafe extern "system" fn UpdateTransform<Impl: IMILBitmapEffectRenderContextImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmatrix: *const MILMatrixF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateTransform(::core::mem::transmute_copy(&pmatrix)).into()
        }
        unsafe extern "system" fn GetOutputBounds<Impl: IMILBitmapEffectRenderContextImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *mut MilRectD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOutputBounds(::core::mem::transmute_copy(&prect)).into()
        }
        unsafe extern "system" fn UpdateOutputBounds<Impl: IMILBitmapEffectRenderContextImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const MilRectD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateOutputBounds(::core::mem::transmute_copy(&prect)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetUseSoftwareRenderer: GetUseSoftwareRenderer::<Impl, IMPL_OFFSET>,
            GetTransform: GetTransform::<Impl, IMPL_OFFSET>,
            UpdateTransform: UpdateTransform::<Impl, IMPL_OFFSET>,
            GetOutputBounds: GetOutputBounds::<Impl, IMPL_OFFSET>,
            UpdateOutputBounds: UpdateOutputBounds::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectRenderContextImpl as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffects_Impl: Sized {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Parent(&mut self) -> ::windows::core::Result<IMILBitmapEffectGroup>;
    fn Item(&mut self, uindex: u32) -> ::windows::core::Result<IMILBitmapEffect>;
    fn Count(&mut self) -> ::windows::core::Result<u32>;
}
impl IMILBitmapEffects_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffects_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffects_Vtbl {
        unsafe extern "system" fn _NewEnum<Impl: IMILBitmapEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiureturn: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiureturn = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Impl: IMILBitmapEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parent() {
                ::core::result::Result::Ok(ok__) => {
                    *ppeffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IMILBitmapEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, ppeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&uindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppeffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IMILBitmapEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puicount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *puicount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Parent: Parent::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffects as ::windows::core::Interface>::IID
    }
}
