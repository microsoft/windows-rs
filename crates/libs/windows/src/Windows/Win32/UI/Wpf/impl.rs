#[cfg(feature = "Win32_Graphics_Imaging")]
pub trait IMILBitmapEffect_Impl: Sized {
    fn GetOutput(&mut self, uiindex: u32, pcontext: &::core::option::Option<IMILBitmapEffectRenderContext>) -> ::windows::core::Result<super::super::Graphics::Imaging::IWICBitmapSource>;
    fn GetParentEffect(&mut self) -> ::windows::core::Result<IMILBitmapEffectGroup>;
    fn SetInputSource(&mut self, uiindex: u32, pbitmapsource: &::core::option::Option<super::super::Graphics::Imaging::IWICBitmapSource>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Imaging")]
impl IMILBitmapEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffect_Impl, const OFFSET: isize>() -> IMILBitmapEffect_Vtbl {
        unsafe extern "system" fn GetOutput<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, pcontext: ::windows::core::RawPtr, ppbitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOutput(::core::mem::transmute_copy(&uiindex), ::core::mem::transmute(&pcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbitmapsource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParentEffect<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparenteffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetParentEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *ppparenteffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInputSource<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, pbitmapsource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInputSource(::core::mem::transmute_copy(&uiindex), ::core::mem::transmute(&pbitmapsource)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetOutput: GetOutput::<Identity, Impl, OFFSET>,
            GetParentEffect: GetParentEffect::<Identity, Impl, OFFSET>,
            SetInputSource: SetInputSource::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectConnections_Impl, const OFFSET: isize>() -> IMILBitmapEffectConnections_Vtbl {
        unsafe extern "system" fn GetInputConnector<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectConnections_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInputConnector(::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppconnector = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputConnector<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectConnections_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOutputConnector(::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppconnector = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetInputConnector: GetInputConnector::<Identity, Impl, OFFSET>,
            GetOutputConnector: GetOutputConnector::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectConnectionsInfo_Impl, const OFFSET: isize>() -> IMILBitmapEffectConnectionsInfo_Vtbl {
        unsafe extern "system" fn GetNumberInputs<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectConnectionsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puinuminputs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNumberInputs() {
                ::core::result::Result::Ok(ok__) => {
                    *puinuminputs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumberOutputs<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectConnectionsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puinumoutputs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNumberOutputs() {
                ::core::result::Result::Ok(ok__) => {
                    *puinumoutputs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputConnectorInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectConnectionsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnectorinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInputConnectorInfo(::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppconnectorinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputConnectorInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectConnectionsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnectorinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOutputConnectorInfo(::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppconnectorinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetNumberInputs: GetNumberInputs::<Identity, Impl, OFFSET>,
            GetNumberOutputs: GetNumberOutputs::<Identity, Impl, OFFSET>,
            GetInputConnectorInfo: GetInputConnectorInfo::<Identity, Impl, OFFSET>,
            GetOutputConnectorInfo: GetOutputConnectorInfo::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectConnector_Impl, const OFFSET: isize>() -> IMILBitmapEffectConnector_Vtbl {
        unsafe extern "system" fn IsConnected<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsConnected() {
                ::core::result::Result::Ok(ok__) => {
                    *pfconnected = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBitmapEffect<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBitmapEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *ppeffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IMILBitmapEffectConnectorInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsConnected: IsConnected::<Identity, Impl, OFFSET>,
            GetBitmapEffect: GetBitmapEffect::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectConnector as ::windows::core::Interface>::IID || iid == &<IMILBitmapEffectConnectorInfo as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectConnectorInfo_Impl: Sized {
    fn GetIndex(&mut self) -> ::windows::core::Result<u32>;
    fn GetOptimalFormat(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetNumberFormats(&mut self) -> ::windows::core::Result<u32>;
    fn GetFormat(&mut self, ulindex: u32) -> ::windows::core::Result<::windows::core::GUID>;
}
impl IMILBitmapEffectConnectorInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectConnectorInfo_Impl, const OFFSET: isize>() -> IMILBitmapEffectConnectorInfo_Vtbl {
        unsafe extern "system" fn GetIndex<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectConnectorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *puiindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptimalFormat<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectConnectorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOptimalFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *pformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumberFormats<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectConnectorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulnumberformats: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNumberFormats() {
                ::core::result::Result::Ok(ok__) => {
                    *pulnumberformats = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormat<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectConnectorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFormat(::core::mem::transmute_copy(&ulindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetIndex: GetIndex::<Identity, Impl, OFFSET>,
            GetOptimalFormat: GetOptimalFormat::<Identity, Impl, OFFSET>,
            GetNumberFormats: GetNumberFormats::<Identity, Impl, OFFSET>,
            GetFormat: GetFormat::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectEvents_Impl, const OFFSET: isize>() -> IMILBitmapEffectEvents_Vtbl {
        unsafe extern "system" fn PropertyChange<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peffect: ::windows::core::RawPtr, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PropertyChange(::core::mem::transmute(&peffect), ::core::mem::transmute_copy(&bstrpropertyname)).into()
        }
        unsafe extern "system" fn DirtyRegion<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peffect: ::windows::core::RawPtr, prect: *const MilRectD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DirtyRegion(::core::mem::transmute(&peffect), ::core::mem::transmute_copy(&prect)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            PropertyChange: PropertyChange::<Identity, Impl, OFFSET>,
            DirtyRegion: DirtyRegion::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectFactory_Impl, const OFFSET: isize>() -> IMILBitmapEffectFactory_Vtbl {
        unsafe extern "system" fn CreateEffect<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguideffect: *const ::windows::core::GUID, ppeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateEffect(::core::mem::transmute_copy(&pguideffect)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppeffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateContext<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateContext() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEffectOuter<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateEffectOuter() {
                ::core::result::Result::Ok(ok__) => {
                    *ppeffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateEffect: CreateEffect::<Identity, Impl, OFFSET>,
            CreateContext: CreateContext::<Identity, Impl, OFFSET>,
            CreateEffectOuter: CreateEffectOuter::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectGroup_Impl, const OFFSET: isize>() -> IMILBitmapEffectGroup_Vtbl {
        unsafe extern "system" fn GetInteriorInputConnector<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInteriorInputConnector(::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppconnector = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInteriorOutputConnector<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInteriorOutputConnector(::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppconnector = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peffect: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Add(::core::mem::transmute(&peffect)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetInteriorInputConnector: GetInteriorInputConnector::<Identity, Impl, OFFSET>,
            GetInteriorOutputConnector: GetInteriorOutputConnector::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectGroupImpl_Impl, const OFFSET: isize>() -> IMILBitmapEffectGroupImpl_Vtbl {
        unsafe extern "system" fn Preprocess<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectGroupImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Preprocess(::core::mem::transmute(&pcontext)).into()
        }
        unsafe extern "system" fn GetNumberChildren<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectGroupImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puinumberchildren: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNumberChildren() {
                ::core::result::Result::Ok(ok__) => {
                    *puinumberchildren = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChildren<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectGroupImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchildren: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetChildren() {
                ::core::result::Result::Ok(ok__) => {
                    *pchildren = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Preprocess: Preprocess::<Identity, Impl, OFFSET>,
            GetNumberChildren: GetNumberChildren::<Identity, Impl, OFFSET>,
            GetChildren: GetChildren::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectImpl_Impl, const OFFSET: isize>() -> IMILBitmapEffectImpl_Vtbl {
        unsafe extern "system" fn IsInPlaceModificationAllowed<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutputconnector: ::windows::core::RawPtr, pfmodifyinplace: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsInPlaceModificationAllowed(::core::mem::transmute(&poutputconnector)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfmodifyinplace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParentEffect<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparenteffect: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetParentEffect(::core::mem::transmute(&pparenteffect)).into()
        }
        unsafe extern "system" fn GetInputSource<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppbitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInputSource(::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbitmapsource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputSourceBounds<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, prect: *mut MilRectD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInputSourceBounds(::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *prect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputBitmapSource<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, prendercontext: ::windows::core::RawPtr, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInputBitmapSource(::core::mem::transmute_copy(&uiindex), ::core::mem::transmute(&prendercontext), ::core::mem::transmute_copy(&pfmodifyinplace), ::core::mem::transmute_copy(&ppbitmapsource)).into()
        }
        unsafe extern "system" fn GetOutputBitmapSource<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, prendercontext: ::windows::core::RawPtr, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOutputBitmapSource(::core::mem::transmute_copy(&uiindex), ::core::mem::transmute(&prendercontext), ::core::mem::transmute_copy(&pfmodifyinplace), ::core::mem::transmute_copy(&ppbitmapsource)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinner: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pinner)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            IsInPlaceModificationAllowed: IsInPlaceModificationAllowed::<Identity, Impl, OFFSET>,
            SetParentEffect: SetParentEffect::<Identity, Impl, OFFSET>,
            GetInputSource: GetInputSource::<Identity, Impl, OFFSET>,
            GetInputSourceBounds: GetInputSourceBounds::<Identity, Impl, OFFSET>,
            GetInputBitmapSource: GetInputBitmapSource::<Identity, Impl, OFFSET>,
            GetOutputBitmapSource: GetOutputBitmapSource::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectInputConnector_Impl, const OFFSET: isize>() -> IMILBitmapEffectInputConnector_Vtbl {
        unsafe extern "system" fn ConnectTo<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectInputConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnector: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConnectTo(::core::mem::transmute(&pconnector)).into()
        }
        unsafe extern "system" fn GetConnection<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectInputConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetConnection() {
                ::core::result::Result::Ok(ok__) => {
                    *ppconnector = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IMILBitmapEffectConnector_Vtbl::new::<Identity, Impl, OFFSET>(),
            ConnectTo: ConnectTo::<Identity, Impl, OFFSET>,
            GetConnection: GetConnection::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectInputConnector as ::windows::core::Interface>::IID || iid == &<IMILBitmapEffectConnectorInfo as ::windows::core::Interface>::IID || iid == &<IMILBitmapEffectConnector as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectInteriorInputConnector_Impl: Sized {
    fn GetInputConnector(&mut self) -> ::windows::core::Result<IMILBitmapEffectInputConnector>;
}
impl IMILBitmapEffectInteriorInputConnector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectInteriorInputConnector_Impl, const OFFSET: isize>() -> IMILBitmapEffectInteriorInputConnector_Vtbl {
        unsafe extern "system" fn GetInputConnector<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectInteriorInputConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInputConnector() {
                ::core::result::Result::Ok(ok__) => {
                    *pinputconnector = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetInputConnector: GetInputConnector::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectInteriorInputConnector as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectInteriorOutputConnector_Impl: Sized {
    fn GetOutputConnector(&mut self) -> ::windows::core::Result<IMILBitmapEffectOutputConnector>;
}
impl IMILBitmapEffectInteriorOutputConnector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectInteriorOutputConnector_Impl, const OFFSET: isize>() -> IMILBitmapEffectInteriorOutputConnector_Vtbl {
        unsafe extern "system" fn GetOutputConnector<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectInteriorOutputConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutputconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOutputConnector() {
                ::core::result::Result::Ok(ok__) => {
                    *poutputconnector = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetOutputConnector: GetOutputConnector::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectOutputConnector_Impl, const OFFSET: isize>() -> IMILBitmapEffectOutputConnector_Vtbl {
        unsafe extern "system" fn GetNumberConnections<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectOutputConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puinumberconnections: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNumberConnections() {
                ::core::result::Result::Ok(ok__) => {
                    *puinumberconnections = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnection<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectOutputConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetConnection(::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppconnection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IMILBitmapEffectConnector_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetNumberConnections: GetNumberConnections::<Identity, Impl, OFFSET>,
            GetConnection: GetConnection::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectOutputConnector as ::windows::core::Interface>::IID || iid == &<IMILBitmapEffectConnectorInfo as ::windows::core::Interface>::IID || iid == &<IMILBitmapEffectConnector as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectOutputConnectorImpl_Impl: Sized {
    fn AddBackLink(&mut self, pconnection: &::core::option::Option<IMILBitmapEffectInputConnector>) -> ::windows::core::Result<()>;
    fn RemoveBackLink(&mut self, pconnection: &::core::option::Option<IMILBitmapEffectInputConnector>) -> ::windows::core::Result<()>;
}
impl IMILBitmapEffectOutputConnectorImpl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectOutputConnectorImpl_Impl, const OFFSET: isize>() -> IMILBitmapEffectOutputConnectorImpl_Vtbl {
        unsafe extern "system" fn AddBackLink<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectOutputConnectorImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddBackLink(::core::mem::transmute(&pconnection)).into()
        }
        unsafe extern "system" fn RemoveBackLink<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectOutputConnectorImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveBackLink(::core::mem::transmute(&pconnection)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddBackLink: AddBackLink::<Identity, Impl, OFFSET>,
            RemoveBackLink: RemoveBackLink::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectPrimitive_Impl, const OFFSET: isize>() -> IMILBitmapEffectPrimitive_Vtbl {
        unsafe extern "system" fn GetOutput<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectPrimitive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, pcontext: ::windows::core::RawPtr, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOutput(::core::mem::transmute_copy(&uiindex), ::core::mem::transmute(&pcontext), ::core::mem::transmute_copy(&pfmodifyinplace), ::core::mem::transmute_copy(&ppbitmapsource)).into()
        }
        unsafe extern "system" fn TransformPoint<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectPrimitive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, p: *mut MilPoint2D, fforwardtransform: i16, pcontext: ::windows::core::RawPtr, pfpointtransformed: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TransformPoint(::core::mem::transmute_copy(&uiindex), ::core::mem::transmute_copy(&p), ::core::mem::transmute_copy(&fforwardtransform), ::core::mem::transmute(&pcontext), ::core::mem::transmute_copy(&pfpointtransformed)).into()
        }
        unsafe extern "system" fn TransformRect<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectPrimitive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, p: *mut MilRectD, fforwardtransform: i16, pcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TransformRect(::core::mem::transmute_copy(&uiindex), ::core::mem::transmute_copy(&p), ::core::mem::transmute_copy(&fforwardtransform), ::core::mem::transmute(&pcontext)).into()
        }
        unsafe extern "system" fn HasAffineTransform<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectPrimitive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, pfaffine: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HasAffineTransform(::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfaffine = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasInverseTransform<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectPrimitive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, pfhasinverse: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HasInverseTransform(::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfhasinverse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAffineMatrix<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectPrimitive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, pmatrix: *mut super::super::Graphics::Dwm::MilMatrix3x2D) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAffineMatrix(::core::mem::transmute_copy(&uiindex), ::core::mem::transmute_copy(&pmatrix)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetOutput: GetOutput::<Identity, Impl, OFFSET>,
            TransformPoint: TransformPoint::<Identity, Impl, OFFSET>,
            TransformRect: TransformRect::<Identity, Impl, OFFSET>,
            HasAffineTransform: HasAffineTransform::<Identity, Impl, OFFSET>,
            HasInverseTransform: HasInverseTransform::<Identity, Impl, OFFSET>,
            GetAffineMatrix: GetAffineMatrix::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectPrimitiveImpl_Impl, const OFFSET: isize>() -> IMILBitmapEffectPrimitiveImpl_Vtbl {
        unsafe extern "system" fn IsDirty<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectPrimitiveImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uioutputindex: u32, pfdirty: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsDirty(::core::mem::transmute_copy(&uioutputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfdirty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVolatile<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectPrimitiveImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uioutputindex: u32, pfvolatile: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsVolatile(::core::mem::transmute_copy(&uioutputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfvolatile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            IsDirty: IsDirty::<Identity, Impl, OFFSET>,
            IsVolatile: IsVolatile::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>() -> IMILBitmapEffectRenderContext_Vtbl {
        unsafe extern "system" fn SetOutputPixelFormat<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOutputPixelFormat(::core::mem::transmute_copy(&format)).into()
        }
        unsafe extern "system" fn GetOutputPixelFormat<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOutputPixelFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *pformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseSoftwareRenderer<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fsoftware: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUseSoftwareRenderer(::core::mem::transmute_copy(&fsoftware)).into()
        }
        unsafe extern "system" fn SetInitialTransform<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmatrix: *const MILMatrixF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInitialTransform(::core::mem::transmute_copy(&pmatrix)).into()
        }
        unsafe extern "system" fn GetFinalTransform<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmatrix: *mut MILMatrixF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFinalTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *pmatrix = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputDPI<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dbldpix: f64, dbldpiy: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOutputDPI(::core::mem::transmute_copy(&dbldpix), ::core::mem::transmute_copy(&dbldpiy)).into()
        }
        unsafe extern "system" fn GetOutputDPI<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdbldpix: *mut f64, pdbldpiy: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOutputDPI(::core::mem::transmute_copy(&pdbldpix), ::core::mem::transmute_copy(&pdbldpiy)).into()
        }
        unsafe extern "system" fn SetRegionOfInterest<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const MilRectD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRegionOfInterest(::core::mem::transmute_copy(&prect)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetOutputPixelFormat: SetOutputPixelFormat::<Identity, Impl, OFFSET>,
            GetOutputPixelFormat: GetOutputPixelFormat::<Identity, Impl, OFFSET>,
            SetUseSoftwareRenderer: SetUseSoftwareRenderer::<Identity, Impl, OFFSET>,
            SetInitialTransform: SetInitialTransform::<Identity, Impl, OFFSET>,
            GetFinalTransform: GetFinalTransform::<Identity, Impl, OFFSET>,
            SetOutputDPI: SetOutputDPI::<Identity, Impl, OFFSET>,
            GetOutputDPI: GetOutputDPI::<Identity, Impl, OFFSET>,
            SetRegionOfInterest: SetRegionOfInterest::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectRenderContextImpl_Impl, const OFFSET: isize>() -> IMILBitmapEffectRenderContextImpl_Vtbl {
        unsafe extern "system" fn GetUseSoftwareRenderer<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectRenderContextImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfsoftware: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetUseSoftwareRenderer() {
                ::core::result::Result::Ok(ok__) => {
                    *pfsoftware = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransform<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectRenderContextImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmatrix: *mut MILMatrixF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetTransform(::core::mem::transmute_copy(&pmatrix)).into()
        }
        unsafe extern "system" fn UpdateTransform<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectRenderContextImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmatrix: *const MILMatrixF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateTransform(::core::mem::transmute_copy(&pmatrix)).into()
        }
        unsafe extern "system" fn GetOutputBounds<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectRenderContextImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *mut MilRectD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOutputBounds(::core::mem::transmute_copy(&prect)).into()
        }
        unsafe extern "system" fn UpdateOutputBounds<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectRenderContextImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const MilRectD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateOutputBounds(::core::mem::transmute_copy(&prect)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetUseSoftwareRenderer: GetUseSoftwareRenderer::<Identity, Impl, OFFSET>,
            GetTransform: GetTransform::<Identity, Impl, OFFSET>,
            UpdateTransform: UpdateTransform::<Identity, Impl, OFFSET>,
            GetOutputBounds: GetOutputBounds::<Identity, Impl, OFFSET>,
            UpdateOutputBounds: UpdateOutputBounds::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffects_Impl, const OFFSET: isize>() -> IMILBitmapEffects_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiureturn: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiureturn = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Parent() {
                ::core::result::Result::Ok(ok__) => {
                    *ppeffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, ppeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&uindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppeffect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puicount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *puicount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffects as ::windows::core::Interface>::IID
    }
}
