#[doc = "*Required features: `\"Win32_UI_Wpf\"`, `\"Win32_Graphics_Imaging\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Graphics_Imaging")]
pub trait IMILBitmapEffect_Impl: Sized {
    fn GetOutput(&self, uiindex: u32, pcontext: ::core::option::Option<&IMILBitmapEffectRenderContext>) -> ::windows::core::Result<super::super::Graphics::Imaging::IWICBitmapSource>;
    fn GetParentEffect(&self) -> ::windows::core::Result<IMILBitmapEffectGroup>;
    fn SetInputSource(&self, uiindex: u32, pbitmapsource: ::core::option::Option<&super::super::Graphics::Imaging::IWICBitmapSource>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Imaging")]
impl ::windows::core::RuntimeName for IMILBitmapEffect {}
#[cfg(feature = "Win32_Graphics_Imaging")]
impl IMILBitmapEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffect_Impl, const OFFSET: isize>() -> IMILBitmapEffect_Vtbl {
        unsafe extern "system" fn GetOutput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, pcontext: *mut ::core::ffi::c_void, ppbitmapsource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOutput(::core::mem::transmute_copy(&uiindex), ::windows::core::from_raw_borrowed(&pcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbitmapsource, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParentEffect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparenteffect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetParentEffect() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppparenteffect, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInputSource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, pbitmapsource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInputSource(::core::mem::transmute_copy(&uiindex), ::windows::core::from_raw_borrowed(&pbitmapsource)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOutput: GetOutput::<Identity, Impl, OFFSET>,
            GetParentEffect: GetParentEffect::<Identity, Impl, OFFSET>,
            SetInputSource: SetInputSource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffect as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`, `\"implement\"`*"]
pub trait IMILBitmapEffectConnections_Impl: Sized {
    fn GetInputConnector(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectInputConnector>;
    fn GetOutputConnector(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectOutputConnector>;
}
impl ::windows::core::RuntimeName for IMILBitmapEffectConnections {}
impl IMILBitmapEffectConnections_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectConnections_Impl, const OFFSET: isize>() -> IMILBitmapEffectConnections_Vtbl {
        unsafe extern "system" fn GetInputConnector<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectConnections_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInputConnector(::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconnector, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputConnector<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectConnections_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOutputConnector(::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconnector, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInputConnector: GetInputConnector::<Identity, Impl, OFFSET>,
            GetOutputConnector: GetOutputConnector::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectConnections as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`, `\"implement\"`*"]
pub trait IMILBitmapEffectConnectionsInfo_Impl: Sized {
    fn GetNumberInputs(&self) -> ::windows::core::Result<u32>;
    fn GetNumberOutputs(&self) -> ::windows::core::Result<u32>;
    fn GetInputConnectorInfo(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectConnectorInfo>;
    fn GetOutputConnectorInfo(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectConnectorInfo>;
}
impl ::windows::core::RuntimeName for IMILBitmapEffectConnectionsInfo {}
impl IMILBitmapEffectConnectionsInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectConnectionsInfo_Impl, const OFFSET: isize>() -> IMILBitmapEffectConnectionsInfo_Vtbl {
        unsafe extern "system" fn GetNumberInputs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectConnectionsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puinuminputs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNumberInputs() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puinuminputs, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumberOutputs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectConnectionsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puinumoutputs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNumberOutputs() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puinumoutputs, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputConnectorInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectConnectionsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnectorinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInputConnectorInfo(::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconnectorinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputConnectorInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectConnectionsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnectorinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOutputConnectorInfo(::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconnectorinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNumberInputs: GetNumberInputs::<Identity, Impl, OFFSET>,
            GetNumberOutputs: GetNumberOutputs::<Identity, Impl, OFFSET>,
            GetInputConnectorInfo: GetInputConnectorInfo::<Identity, Impl, OFFSET>,
            GetOutputConnectorInfo: GetOutputConnectorInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectConnectionsInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IMILBitmapEffectConnector_Impl: Sized + IMILBitmapEffectConnectorInfo_Impl {
    fn IsConnected(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetBitmapEffect(&self) -> ::windows::core::Result<IMILBitmapEffect>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IMILBitmapEffectConnector {}
#[cfg(feature = "Win32_Foundation")]
impl IMILBitmapEffectConnector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectConnector_Impl, const OFFSET: isize>() -> IMILBitmapEffectConnector_Vtbl {
        unsafe extern "system" fn IsConnected<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsConnected() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfconnected, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBitmapEffect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBitmapEffect() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppeffect, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IMILBitmapEffectConnectorInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsConnected: IsConnected::<Identity, Impl, OFFSET>,
            GetBitmapEffect: GetBitmapEffect::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectConnector as ::windows::core::ComInterface>::IID || iid == &<IMILBitmapEffectConnectorInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`, `\"implement\"`*"]
pub trait IMILBitmapEffectConnectorInfo_Impl: Sized {
    fn GetIndex(&self) -> ::windows::core::Result<u32>;
    fn GetOptimalFormat(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetNumberFormats(&self) -> ::windows::core::Result<u32>;
    fn GetFormat(&self, ulindex: u32) -> ::windows::core::Result<::windows::core::GUID>;
}
impl ::windows::core::RuntimeName for IMILBitmapEffectConnectorInfo {}
impl IMILBitmapEffectConnectorInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectConnectorInfo_Impl, const OFFSET: isize>() -> IMILBitmapEffectConnectorInfo_Vtbl {
        unsafe extern "system" fn GetIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectConnectorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIndex() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puiindex, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptimalFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectConnectorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOptimalFormat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pformat, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumberFormats<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectConnectorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulnumberformats: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNumberFormats() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulnumberformats, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectConnectorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFormat(::core::mem::transmute_copy(&ulindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pformat, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIndex: GetIndex::<Identity, Impl, OFFSET>,
            GetOptimalFormat: GetOptimalFormat::<Identity, Impl, OFFSET>,
            GetNumberFormats: GetNumberFormats::<Identity, Impl, OFFSET>,
            GetFormat: GetFormat::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectConnectorInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`, `\"implement\"`*"]
pub trait IMILBitmapEffectEvents_Impl: Sized {
    fn PropertyChange(&self, peffect: ::core::option::Option<&IMILBitmapEffect>, bstrpropertyname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn DirtyRegion(&self, peffect: ::core::option::Option<&IMILBitmapEffect>, prect: *const MilRectD) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMILBitmapEffectEvents {}
impl IMILBitmapEffectEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectEvents_Impl, const OFFSET: isize>() -> IMILBitmapEffectEvents_Vtbl {
        unsafe extern "system" fn PropertyChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peffect: *mut ::core::ffi::c_void, bstrpropertyname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PropertyChange(::windows::core::from_raw_borrowed(&peffect), ::core::mem::transmute(&bstrpropertyname)).into()
        }
        unsafe extern "system" fn DirtyRegion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peffect: *mut ::core::ffi::c_void, prect: *const MilRectD) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DirtyRegion(::windows::core::from_raw_borrowed(&peffect), ::core::mem::transmute_copy(&prect)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PropertyChange: PropertyChange::<Identity, Impl, OFFSET>,
            DirtyRegion: DirtyRegion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectEvents as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`, `\"implement\"`*"]
pub trait IMILBitmapEffectFactory_Impl: Sized {
    fn CreateEffect(&self, pguideffect: *const ::windows::core::GUID) -> ::windows::core::Result<IMILBitmapEffect>;
    fn CreateContext(&self) -> ::windows::core::Result<IMILBitmapEffectRenderContext>;
    fn CreateEffectOuter(&self) -> ::windows::core::Result<IMILBitmapEffect>;
}
impl ::windows::core::RuntimeName for IMILBitmapEffectFactory {}
impl IMILBitmapEffectFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectFactory_Impl, const OFFSET: isize>() -> IMILBitmapEffectFactory_Vtbl {
        unsafe extern "system" fn CreateEffect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguideffect: *const ::windows::core::GUID, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateEffect(::core::mem::transmute_copy(&pguideffect)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppeffect, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEffectOuter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateEffectOuter() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppeffect, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateEffect: CreateEffect::<Identity, Impl, OFFSET>,
            CreateContext: CreateContext::<Identity, Impl, OFFSET>,
            CreateEffectOuter: CreateEffectOuter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectFactory as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`, `\"implement\"`*"]
pub trait IMILBitmapEffectGroup_Impl: Sized {
    fn GetInteriorInputConnector(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectOutputConnector>;
    fn GetInteriorOutputConnector(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectInputConnector>;
    fn Add(&self, peffect: ::core::option::Option<&IMILBitmapEffect>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMILBitmapEffectGroup {}
impl IMILBitmapEffectGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectGroup_Impl, const OFFSET: isize>() -> IMILBitmapEffectGroup_Vtbl {
        unsafe extern "system" fn GetInteriorInputConnector<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInteriorInputConnector(::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconnector, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInteriorOutputConnector<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInteriorOutputConnector(::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconnector, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peffect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add(::windows::core::from_raw_borrowed(&peffect)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInteriorInputConnector: GetInteriorInputConnector::<Identity, Impl, OFFSET>,
            GetInteriorOutputConnector: GetInteriorOutputConnector::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectGroup as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`, `\"implement\"`*"]
pub trait IMILBitmapEffectGroupImpl_Impl: Sized {
    fn Preprocess(&self, pcontext: ::core::option::Option<&IMILBitmapEffectRenderContext>) -> ::windows::core::Result<()>;
    fn GetNumberChildren(&self) -> ::windows::core::Result<u32>;
    fn GetChildren(&self) -> ::windows::core::Result<IMILBitmapEffects>;
}
impl ::windows::core::RuntimeName for IMILBitmapEffectGroupImpl {}
impl IMILBitmapEffectGroupImpl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectGroupImpl_Impl, const OFFSET: isize>() -> IMILBitmapEffectGroupImpl_Vtbl {
        unsafe extern "system" fn Preprocess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectGroupImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Preprocess(::windows::core::from_raw_borrowed(&pcontext)).into()
        }
        unsafe extern "system" fn GetNumberChildren<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectGroupImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puinumberchildren: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNumberChildren() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puinumberchildren, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChildren<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectGroupImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchildren: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetChildren() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pchildren, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Preprocess: Preprocess::<Identity, Impl, OFFSET>,
            GetNumberChildren: GetNumberChildren::<Identity, Impl, OFFSET>,
            GetChildren: GetChildren::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectGroupImpl as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Imaging\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging"))]
pub trait IMILBitmapEffectImpl_Impl: Sized {
    fn IsInPlaceModificationAllowed(&self, poutputconnector: ::core::option::Option<&IMILBitmapEffectOutputConnector>) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetParentEffect(&self, pparenteffect: ::core::option::Option<&IMILBitmapEffectGroup>) -> ::windows::core::Result<()>;
    fn GetInputSource(&self, uiindex: u32) -> ::windows::core::Result<super::super::Graphics::Imaging::IWICBitmapSource>;
    fn GetInputSourceBounds(&self, uiindex: u32, prect: *mut MilRectD) -> ::windows::core::Result<()>;
    fn GetInputBitmapSource(&self, uiindex: u32, prendercontext: ::core::option::Option<&IMILBitmapEffectRenderContext>, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL, ppbitmapsource: *mut ::core::option::Option<super::super::Graphics::Imaging::IWICBitmapSource>) -> ::windows::core::Result<()>;
    fn GetOutputBitmapSource(&self, uiindex: u32, prendercontext: ::core::option::Option<&IMILBitmapEffectRenderContext>, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL, ppbitmapsource: *mut ::core::option::Option<super::super::Graphics::Imaging::IWICBitmapSource>) -> ::windows::core::Result<()>;
    fn Initialize(&self, pinner: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging"))]
impl ::windows::core::RuntimeName for IMILBitmapEffectImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging"))]
impl IMILBitmapEffectImpl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectImpl_Impl, const OFFSET: isize>() -> IMILBitmapEffectImpl_Vtbl {
        unsafe extern "system" fn IsInPlaceModificationAllowed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutputconnector: *mut ::core::ffi::c_void, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsInPlaceModificationAllowed(::windows::core::from_raw_borrowed(&poutputconnector)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfmodifyinplace, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParentEffect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparenteffect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetParentEffect(::windows::core::from_raw_borrowed(&pparenteffect)).into()
        }
        unsafe extern "system" fn GetInputSource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppbitmapsource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInputSource(::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbitmapsource, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputSourceBounds<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, prect: *mut MilRectD) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInputSourceBounds(::core::mem::transmute_copy(&uiindex), ::core::mem::transmute_copy(&prect)).into()
        }
        unsafe extern "system" fn GetInputBitmapSource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, prendercontext: *mut ::core::ffi::c_void, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL, ppbitmapsource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInputBitmapSource(::core::mem::transmute_copy(&uiindex), ::windows::core::from_raw_borrowed(&prendercontext), ::core::mem::transmute_copy(&pfmodifyinplace), ::core::mem::transmute_copy(&ppbitmapsource)).into()
        }
        unsafe extern "system" fn GetOutputBitmapSource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, prendercontext: *mut ::core::ffi::c_void, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL, ppbitmapsource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOutputBitmapSource(::core::mem::transmute_copy(&uiindex), ::windows::core::from_raw_borrowed(&prendercontext), ::core::mem::transmute_copy(&pfmodifyinplace), ::core::mem::transmute_copy(&ppbitmapsource)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinner: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows::core::from_raw_borrowed(&pinner)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IMILBitmapEffectImpl as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IMILBitmapEffectInputConnector_Impl: Sized + IMILBitmapEffectConnector_Impl {
    fn ConnectTo(&self, pconnector: ::core::option::Option<&IMILBitmapEffectOutputConnector>) -> ::windows::core::Result<()>;
    fn GetConnection(&self) -> ::windows::core::Result<IMILBitmapEffectOutputConnector>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IMILBitmapEffectInputConnector {}
#[cfg(feature = "Win32_Foundation")]
impl IMILBitmapEffectInputConnector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectInputConnector_Impl, const OFFSET: isize>() -> IMILBitmapEffectInputConnector_Vtbl {
        unsafe extern "system" fn ConnectTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectInputConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnector: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConnectTo(::windows::core::from_raw_borrowed(&pconnector)).into()
        }
        unsafe extern "system" fn GetConnection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectInputConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetConnection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconnector, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IMILBitmapEffectConnector_Vtbl::new::<Identity, Impl, OFFSET>(),
            ConnectTo: ConnectTo::<Identity, Impl, OFFSET>,
            GetConnection: GetConnection::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectInputConnector as ::windows::core::ComInterface>::IID || iid == &<IMILBitmapEffectConnectorInfo as ::windows::core::ComInterface>::IID || iid == &<IMILBitmapEffectConnector as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`, `\"implement\"`*"]
pub trait IMILBitmapEffectInteriorInputConnector_Impl: Sized {
    fn GetInputConnector(&self) -> ::windows::core::Result<IMILBitmapEffectInputConnector>;
}
impl ::windows::core::RuntimeName for IMILBitmapEffectInteriorInputConnector {}
impl IMILBitmapEffectInteriorInputConnector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectInteriorInputConnector_Impl, const OFFSET: isize>() -> IMILBitmapEffectInteriorInputConnector_Vtbl {
        unsafe extern "system" fn GetInputConnector<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectInteriorInputConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputconnector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInputConnector() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinputconnector, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetInputConnector: GetInputConnector::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectInteriorInputConnector as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`, `\"implement\"`*"]
pub trait IMILBitmapEffectInteriorOutputConnector_Impl: Sized {
    fn GetOutputConnector(&self) -> ::windows::core::Result<IMILBitmapEffectOutputConnector>;
}
impl ::windows::core::RuntimeName for IMILBitmapEffectInteriorOutputConnector {}
impl IMILBitmapEffectInteriorOutputConnector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectInteriorOutputConnector_Impl, const OFFSET: isize>() -> IMILBitmapEffectInteriorOutputConnector_Vtbl {
        unsafe extern "system" fn GetOutputConnector<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectInteriorOutputConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutputconnector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOutputConnector() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(poutputconnector, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetOutputConnector: GetOutputConnector::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectInteriorOutputConnector as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IMILBitmapEffectOutputConnector_Impl: Sized + IMILBitmapEffectConnector_Impl {
    fn GetNumberConnections(&self) -> ::windows::core::Result<u32>;
    fn GetConnection(&self, uiindex: u32) -> ::windows::core::Result<IMILBitmapEffectInputConnector>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IMILBitmapEffectOutputConnector {}
#[cfg(feature = "Win32_Foundation")]
impl IMILBitmapEffectOutputConnector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectOutputConnector_Impl, const OFFSET: isize>() -> IMILBitmapEffectOutputConnector_Vtbl {
        unsafe extern "system" fn GetNumberConnections<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectOutputConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puinumberconnections: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNumberConnections() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puinumberconnections, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectOutputConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetConnection(::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconnection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IMILBitmapEffectConnector_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetNumberConnections: GetNumberConnections::<Identity, Impl, OFFSET>,
            GetConnection: GetConnection::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectOutputConnector as ::windows::core::ComInterface>::IID || iid == &<IMILBitmapEffectConnectorInfo as ::windows::core::ComInterface>::IID || iid == &<IMILBitmapEffectConnector as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`, `\"implement\"`*"]
pub trait IMILBitmapEffectOutputConnectorImpl_Impl: Sized {
    fn AddBackLink(&self, pconnection: ::core::option::Option<&IMILBitmapEffectInputConnector>) -> ::windows::core::Result<()>;
    fn RemoveBackLink(&self, pconnection: ::core::option::Option<&IMILBitmapEffectInputConnector>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMILBitmapEffectOutputConnectorImpl {}
impl IMILBitmapEffectOutputConnectorImpl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectOutputConnectorImpl_Impl, const OFFSET: isize>() -> IMILBitmapEffectOutputConnectorImpl_Vtbl {
        unsafe extern "system" fn AddBackLink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectOutputConnectorImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnection: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddBackLink(::windows::core::from_raw_borrowed(&pconnection)).into()
        }
        unsafe extern "system" fn RemoveBackLink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectOutputConnectorImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnection: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveBackLink(::windows::core::from_raw_borrowed(&pconnection)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddBackLink: AddBackLink::<Identity, Impl, OFFSET>,
            RemoveBackLink: RemoveBackLink::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectOutputConnectorImpl as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dwm\"`, `\"Win32_Graphics_Imaging\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dwm", feature = "Win32_Graphics_Imaging"))]
pub trait IMILBitmapEffectPrimitive_Impl: Sized {
    fn GetOutput(&self, uiindex: u32, pcontext: ::core::option::Option<&IMILBitmapEffectRenderContext>, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL, ppbitmapsource: *mut ::core::option::Option<super::super::Graphics::Imaging::IWICBitmapSource>) -> ::windows::core::Result<()>;
    fn TransformPoint(&self, uiindex: u32, p: *mut MilPoint2D, fforwardtransform: super::super::Foundation::VARIANT_BOOL, pcontext: ::core::option::Option<&IMILBitmapEffectRenderContext>, pfpointtransformed: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn TransformRect(&self, uiindex: u32, p: *mut MilRectD, fforwardtransform: super::super::Foundation::VARIANT_BOOL, pcontext: ::core::option::Option<&IMILBitmapEffectRenderContext>) -> ::windows::core::Result<()>;
    fn HasAffineTransform(&self, uiindex: u32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn HasInverseTransform(&self, uiindex: u32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetAffineMatrix(&self, uiindex: u32, pmatrix: *mut super::super::Graphics::Dwm::MilMatrix3x2D) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dwm", feature = "Win32_Graphics_Imaging"))]
impl ::windows::core::RuntimeName for IMILBitmapEffectPrimitive {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dwm", feature = "Win32_Graphics_Imaging"))]
impl IMILBitmapEffectPrimitive_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectPrimitive_Impl, const OFFSET: isize>() -> IMILBitmapEffectPrimitive_Vtbl {
        unsafe extern "system" fn GetOutput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectPrimitive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, pcontext: *mut ::core::ffi::c_void, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL, ppbitmapsource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOutput(::core::mem::transmute_copy(&uiindex), ::windows::core::from_raw_borrowed(&pcontext), ::core::mem::transmute_copy(&pfmodifyinplace), ::core::mem::transmute_copy(&ppbitmapsource)).into()
        }
        unsafe extern "system" fn TransformPoint<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectPrimitive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, p: *mut MilPoint2D, fforwardtransform: super::super::Foundation::VARIANT_BOOL, pcontext: *mut ::core::ffi::c_void, pfpointtransformed: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TransformPoint(::core::mem::transmute_copy(&uiindex), ::core::mem::transmute_copy(&p), ::core::mem::transmute_copy(&fforwardtransform), ::windows::core::from_raw_borrowed(&pcontext), ::core::mem::transmute_copy(&pfpointtransformed)).into()
        }
        unsafe extern "system" fn TransformRect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectPrimitive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, p: *mut MilRectD, fforwardtransform: super::super::Foundation::VARIANT_BOOL, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TransformRect(::core::mem::transmute_copy(&uiindex), ::core::mem::transmute_copy(&p), ::core::mem::transmute_copy(&fforwardtransform), ::windows::core::from_raw_borrowed(&pcontext)).into()
        }
        unsafe extern "system" fn HasAffineTransform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectPrimitive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, pfaffine: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HasAffineTransform(::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaffine, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasInverseTransform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectPrimitive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, pfhasinverse: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HasInverseTransform(::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfhasinverse, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAffineMatrix<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectPrimitive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, pmatrix: *mut super::super::Graphics::Dwm::MilMatrix3x2D) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAffineMatrix(::core::mem::transmute_copy(&uiindex), ::core::mem::transmute_copy(&pmatrix)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOutput: GetOutput::<Identity, Impl, OFFSET>,
            TransformPoint: TransformPoint::<Identity, Impl, OFFSET>,
            TransformRect: TransformRect::<Identity, Impl, OFFSET>,
            HasAffineTransform: HasAffineTransform::<Identity, Impl, OFFSET>,
            HasInverseTransform: HasInverseTransform::<Identity, Impl, OFFSET>,
            GetAffineMatrix: GetAffineMatrix::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectPrimitive as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IMILBitmapEffectPrimitiveImpl_Impl: Sized {
    fn IsDirty(&self, uioutputindex: u32, pfdirty: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT;
    fn IsVolatile(&self, uioutputindex: u32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IMILBitmapEffectPrimitiveImpl {}
#[cfg(feature = "Win32_Foundation")]
impl IMILBitmapEffectPrimitiveImpl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectPrimitiveImpl_Impl, const OFFSET: isize>() -> IMILBitmapEffectPrimitiveImpl_Vtbl {
        unsafe extern "system" fn IsDirty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectPrimitiveImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uioutputindex: u32, pfdirty: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsDirty(::core::mem::transmute_copy(&uioutputindex), ::core::mem::transmute_copy(&pfdirty))
        }
        unsafe extern "system" fn IsVolatile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectPrimitiveImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uioutputindex: u32, pfvolatile: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsVolatile(::core::mem::transmute_copy(&uioutputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfvolatile, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsDirty: IsDirty::<Identity, Impl, OFFSET>,
            IsVolatile: IsVolatile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectPrimitiveImpl as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IMILBitmapEffectRenderContext_Impl: Sized {
    fn SetOutputPixelFormat(&self, format: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetOutputPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetUseSoftwareRenderer(&self, fsoftware: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn SetInitialTransform(&self, pmatrix: *const MILMatrixF) -> ::windows::core::Result<()>;
    fn GetFinalTransform(&self, pmatrix: *mut MILMatrixF) -> ::windows::core::Result<()>;
    fn SetOutputDPI(&self, dbldpix: f64, dbldpiy: f64) -> ::windows::core::Result<()>;
    fn GetOutputDPI(&self, pdbldpix: *mut f64, pdbldpiy: *mut f64) -> ::windows::core::Result<()>;
    fn SetRegionOfInterest(&self, prect: *const MilRectD) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IMILBitmapEffectRenderContext {}
#[cfg(feature = "Win32_Foundation")]
impl IMILBitmapEffectRenderContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>() -> IMILBitmapEffectRenderContext_Vtbl {
        unsafe extern "system" fn SetOutputPixelFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOutputPixelFormat(::core::mem::transmute_copy(&format)).into()
        }
        unsafe extern "system" fn GetOutputPixelFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOutputPixelFormat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pformat, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseSoftwareRenderer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fsoftware: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUseSoftwareRenderer(::core::mem::transmute_copy(&fsoftware)).into()
        }
        unsafe extern "system" fn SetInitialTransform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmatrix: *const MILMatrixF) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInitialTransform(::core::mem::transmute_copy(&pmatrix)).into()
        }
        unsafe extern "system" fn GetFinalTransform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmatrix: *mut MILMatrixF) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFinalTransform(::core::mem::transmute_copy(&pmatrix)).into()
        }
        unsafe extern "system" fn SetOutputDPI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dbldpix: f64, dbldpiy: f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOutputDPI(::core::mem::transmute_copy(&dbldpix), ::core::mem::transmute_copy(&dbldpiy)).into()
        }
        unsafe extern "system" fn GetOutputDPI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdbldpix: *mut f64, pdbldpiy: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOutputDPI(::core::mem::transmute_copy(&pdbldpix), ::core::mem::transmute_copy(&pdbldpiy)).into()
        }
        unsafe extern "system" fn SetRegionOfInterest<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const MilRectD) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRegionOfInterest(::core::mem::transmute_copy(&prect)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IMILBitmapEffectRenderContext as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IMILBitmapEffectRenderContextImpl_Impl: Sized {
    fn GetUseSoftwareRenderer(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetTransform(&self, pmatrix: *mut MILMatrixF) -> ::windows::core::Result<()>;
    fn UpdateTransform(&self, pmatrix: *const MILMatrixF) -> ::windows::core::Result<()>;
    fn GetOutputBounds(&self, prect: *mut MilRectD) -> ::windows::core::Result<()>;
    fn UpdateOutputBounds(&self, prect: *const MilRectD) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IMILBitmapEffectRenderContextImpl {}
#[cfg(feature = "Win32_Foundation")]
impl IMILBitmapEffectRenderContextImpl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectRenderContextImpl_Impl, const OFFSET: isize>() -> IMILBitmapEffectRenderContextImpl_Vtbl {
        unsafe extern "system" fn GetUseSoftwareRenderer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectRenderContextImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfsoftware: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUseSoftwareRenderer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfsoftware, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectRenderContextImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmatrix: *mut MILMatrixF) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTransform(::core::mem::transmute_copy(&pmatrix)).into()
        }
        unsafe extern "system" fn UpdateTransform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectRenderContextImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmatrix: *const MILMatrixF) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateTransform(::core::mem::transmute_copy(&pmatrix)).into()
        }
        unsafe extern "system" fn GetOutputBounds<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectRenderContextImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *mut MilRectD) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOutputBounds(::core::mem::transmute_copy(&prect)).into()
        }
        unsafe extern "system" fn UpdateOutputBounds<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffectRenderContextImpl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const MilRectD) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateOutputBounds(::core::mem::transmute_copy(&prect)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetUseSoftwareRenderer: GetUseSoftwareRenderer::<Identity, Impl, OFFSET>,
            GetTransform: GetTransform::<Identity, Impl, OFFSET>,
            UpdateTransform: UpdateTransform::<Identity, Impl, OFFSET>,
            GetOutputBounds: GetOutputBounds::<Identity, Impl, OFFSET>,
            UpdateOutputBounds: UpdateOutputBounds::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectRenderContextImpl as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`, `\"implement\"`*"]
pub trait IMILBitmapEffects_Impl: Sized {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Parent(&self) -> ::windows::core::Result<IMILBitmapEffectGroup>;
    fn Item(&self, uindex: u32) -> ::windows::core::Result<IMILBitmapEffect>;
    fn Count(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IMILBitmapEffects {}
impl IMILBitmapEffects_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffects_Impl, const OFFSET: isize>() -> IMILBitmapEffects_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiureturn: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiureturn, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Parent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppeffect, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&uindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppeffect, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMILBitmapEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puicount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puicount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffects as ::windows::core::ComInterface>::IID
    }
}
