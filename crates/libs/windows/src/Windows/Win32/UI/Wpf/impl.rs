#[cfg(feature = "Win32_Graphics_Imaging")]
pub trait IMILBitmapEffectImpl: Sized {
    fn GetOutput();
    fn GetParentEffect();
    fn SetInputSource();
}
#[cfg(feature = "Win32_Graphics_Imaging")]
impl IMILBitmapEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectVtbl {
        unsafe extern "system" fn GetOutput<Impl: IMILBitmapEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, pcontext: ::windows::core::RawPtr, ppbitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetParentEffect<Impl: IMILBitmapEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparenteffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInputSource<Impl: IMILBitmapEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, pbitmapsource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetOutput::<Impl, IMPL_OFFSET>, GetParentEffect::<Impl, IMPL_OFFSET>, SetInputSource::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffect as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectConnectionsImpl: Sized {
    fn GetInputConnector();
    fn GetOutputConnector();
}
impl IMILBitmapEffectConnectionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectConnectionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectConnectionsVtbl {
        unsafe extern "system" fn GetInputConnector<Impl: IMILBitmapEffectConnectionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputConnector<Impl: IMILBitmapEffectConnectionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetInputConnector::<Impl, IMPL_OFFSET>, GetOutputConnector::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectConnections as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectConnectionsInfoImpl: Sized {
    fn GetNumberInputs();
    fn GetNumberOutputs();
    fn GetInputConnectorInfo();
    fn GetOutputConnectorInfo();
}
impl IMILBitmapEffectConnectionsInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectConnectionsInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectConnectionsInfoVtbl {
        unsafe extern "system" fn GetNumberInputs<Impl: IMILBitmapEffectConnectionsInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puinuminputs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNumberOutputs<Impl: IMILBitmapEffectConnectionsInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puinumoutputs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputConnectorInfo<Impl: IMILBitmapEffectConnectionsInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnectorinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputConnectorInfo<Impl: IMILBitmapEffectConnectionsInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnectorinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetNumberInputs::<Impl, IMPL_OFFSET>, GetNumberOutputs::<Impl, IMPL_OFFSET>, GetInputConnectorInfo::<Impl, IMPL_OFFSET>, GetOutputConnectorInfo::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectConnectionsInfo as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectConnectorImpl: Sized + IMILBitmapEffectConnectorInfoImpl {
    fn IsConnected();
    fn GetBitmapEffect();
}
impl IMILBitmapEffectConnectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectConnectorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectConnectorVtbl {
        unsafe extern "system" fn IsConnected<Impl: IMILBitmapEffectConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBitmapEffect<Impl: IMILBitmapEffectConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetIndex::<Impl, IMPL_OFFSET>, GetOptimalFormat::<Impl, IMPL_OFFSET>, GetNumberFormats::<Impl, IMPL_OFFSET>, GetFormat::<Impl, IMPL_OFFSET>, IsConnected::<Impl, IMPL_OFFSET>, GetBitmapEffect::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectConnector as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectConnectorInfoImpl: Sized {
    fn GetIndex();
    fn GetOptimalFormat();
    fn GetNumberFormats();
    fn GetFormat();
}
impl IMILBitmapEffectConnectorInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectConnectorInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectConnectorInfoVtbl {
        unsafe extern "system" fn GetIndex<Impl: IMILBitmapEffectConnectorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOptimalFormat<Impl: IMILBitmapEffectConnectorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNumberFormats<Impl: IMILBitmapEffectConnectorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulnumberformats: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFormat<Impl: IMILBitmapEffectConnectorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetIndex::<Impl, IMPL_OFFSET>, GetOptimalFormat::<Impl, IMPL_OFFSET>, GetNumberFormats::<Impl, IMPL_OFFSET>, GetFormat::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectConnectorInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMILBitmapEffectEventsImpl: Sized {
    fn PropertyChange();
    fn DirtyRegion();
}
#[cfg(feature = "Win32_Foundation")]
impl IMILBitmapEffectEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectEventsVtbl {
        unsafe extern "system" fn PropertyChange<Impl: IMILBitmapEffectEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peffect: ::windows::core::RawPtr, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DirtyRegion<Impl: IMILBitmapEffectEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peffect: ::windows::core::RawPtr, prect: *const MilRectD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, PropertyChange::<Impl, IMPL_OFFSET>, DirtyRegion::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectEvents as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectFactoryImpl: Sized {
    fn CreateEffect();
    fn CreateContext();
    fn CreateEffectOuter();
}
impl IMILBitmapEffectFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectFactoryVtbl {
        unsafe extern "system" fn CreateEffect<Impl: IMILBitmapEffectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguideffect: *const ::windows::core::GUID, ppeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateContext<Impl: IMILBitmapEffectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateEffectOuter<Impl: IMILBitmapEffectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateEffect::<Impl, IMPL_OFFSET>, CreateContext::<Impl, IMPL_OFFSET>, CreateEffectOuter::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectFactory as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectGroupImpl: Sized {
    fn GetInteriorInputConnector();
    fn GetInteriorOutputConnector();
    fn Add();
}
impl IMILBitmapEffectGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectGroupVtbl {
        unsafe extern "system" fn GetInteriorInputConnector<Impl: IMILBitmapEffectGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInteriorOutputConnector<Impl: IMILBitmapEffectGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IMILBitmapEffectGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peffect: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetInteriorInputConnector::<Impl, IMPL_OFFSET>, GetInteriorOutputConnector::<Impl, IMPL_OFFSET>, Add::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectGroup as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectGroupImplImpl: Sized {
    fn Preprocess();
    fn GetNumberChildren();
    fn GetChildren();
}
impl IMILBitmapEffectGroupImplVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectGroupImplImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectGroupImplVtbl {
        unsafe extern "system" fn Preprocess<Impl: IMILBitmapEffectGroupImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNumberChildren<Impl: IMILBitmapEffectGroupImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puinumberchildren: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChildren<Impl: IMILBitmapEffectGroupImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchildren: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Preprocess::<Impl, IMPL_OFFSET>, GetNumberChildren::<Impl, IMPL_OFFSET>, GetChildren::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectGroupImpl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Imaging")]
pub trait IMILBitmapEffectImplImpl: Sized {
    fn IsInPlaceModificationAllowed();
    fn SetParentEffect();
    fn GetInputSource();
    fn GetInputSourceBounds();
    fn GetInputBitmapSource();
    fn GetOutputBitmapSource();
    fn Initialize();
}
#[cfg(feature = "Win32_Graphics_Imaging")]
impl IMILBitmapEffectImplVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectImplImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectImplVtbl {
        unsafe extern "system" fn IsInPlaceModificationAllowed<Impl: IMILBitmapEffectImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutputconnector: ::windows::core::RawPtr, pfmodifyinplace: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetParentEffect<Impl: IMILBitmapEffectImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparenteffect: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputSource<Impl: IMILBitmapEffectImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppbitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputSourceBounds<Impl: IMILBitmapEffectImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, prect: *mut MilRectD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputBitmapSource<Impl: IMILBitmapEffectImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, prendercontext: ::windows::core::RawPtr, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputBitmapSource<Impl: IMILBitmapEffectImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, prendercontext: ::windows::core::RawPtr, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Initialize<Impl: IMILBitmapEffectImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinner: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, IsInPlaceModificationAllowed::<Impl, IMPL_OFFSET>, SetParentEffect::<Impl, IMPL_OFFSET>, GetInputSource::<Impl, IMPL_OFFSET>, GetInputSourceBounds::<Impl, IMPL_OFFSET>, GetInputBitmapSource::<Impl, IMPL_OFFSET>, GetOutputBitmapSource::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectImpl as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectInputConnectorImpl: Sized + IMILBitmapEffectConnectorImpl + IMILBitmapEffectConnectorInfoImpl {
    fn ConnectTo();
    fn GetConnection();
}
impl IMILBitmapEffectInputConnectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectInputConnectorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectInputConnectorVtbl {
        unsafe extern "system" fn ConnectTo<Impl: IMILBitmapEffectInputConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnector: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConnection<Impl: IMILBitmapEffectInputConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetIndex::<Impl, IMPL_OFFSET>, GetOptimalFormat::<Impl, IMPL_OFFSET>, GetNumberFormats::<Impl, IMPL_OFFSET>, GetFormat::<Impl, IMPL_OFFSET>, IsConnected::<Impl, IMPL_OFFSET>, GetBitmapEffect::<Impl, IMPL_OFFSET>, ConnectTo::<Impl, IMPL_OFFSET>, GetConnection::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectInputConnector as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectInteriorInputConnectorImpl: Sized {
    fn GetInputConnector();
}
impl IMILBitmapEffectInteriorInputConnectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectInteriorInputConnectorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectInteriorInputConnectorVtbl {
        unsafe extern "system" fn GetInputConnector<Impl: IMILBitmapEffectInteriorInputConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetInputConnector::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectInteriorInputConnector as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectInteriorOutputConnectorImpl: Sized {
    fn GetOutputConnector();
}
impl IMILBitmapEffectInteriorOutputConnectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectInteriorOutputConnectorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectInteriorOutputConnectorVtbl {
        unsafe extern "system" fn GetOutputConnector<Impl: IMILBitmapEffectInteriorOutputConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutputconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetOutputConnector::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectInteriorOutputConnector as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectOutputConnectorImpl: Sized + IMILBitmapEffectConnectorImpl + IMILBitmapEffectConnectorInfoImpl {
    fn GetNumberConnections();
    fn GetConnection();
}
impl IMILBitmapEffectOutputConnectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectOutputConnectorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectOutputConnectorVtbl {
        unsafe extern "system" fn GetNumberConnections<Impl: IMILBitmapEffectOutputConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puinumberconnections: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConnection<Impl: IMILBitmapEffectOutputConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetIndex::<Impl, IMPL_OFFSET>, GetOptimalFormat::<Impl, IMPL_OFFSET>, GetNumberFormats::<Impl, IMPL_OFFSET>, GetFormat::<Impl, IMPL_OFFSET>, IsConnected::<Impl, IMPL_OFFSET>, GetBitmapEffect::<Impl, IMPL_OFFSET>, GetNumberConnections::<Impl, IMPL_OFFSET>, GetConnection::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectOutputConnector as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectOutputConnectorImplImpl: Sized {
    fn AddBackLink();
    fn RemoveBackLink();
}
impl IMILBitmapEffectOutputConnectorImplVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectOutputConnectorImplImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectOutputConnectorImplVtbl {
        unsafe extern "system" fn AddBackLink<Impl: IMILBitmapEffectOutputConnectorImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveBackLink<Impl: IMILBitmapEffectOutputConnectorImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddBackLink::<Impl, IMPL_OFFSET>, RemoveBackLink::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectOutputConnectorImpl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dwm", feature = "Win32_Graphics_Imaging"))]
pub trait IMILBitmapEffectPrimitiveImpl: Sized {
    fn GetOutput();
    fn TransformPoint();
    fn TransformRect();
    fn HasAffineTransform();
    fn HasInverseTransform();
    fn GetAffineMatrix();
}
#[cfg(all(feature = "Win32_Graphics_Dwm", feature = "Win32_Graphics_Imaging"))]
impl IMILBitmapEffectPrimitiveVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectPrimitiveImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectPrimitiveVtbl {
        unsafe extern "system" fn GetOutput<Impl: IMILBitmapEffectPrimitiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, pcontext: ::windows::core::RawPtr, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TransformPoint<Impl: IMILBitmapEffectPrimitiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, p: *mut MilPoint2D, fforwardtransform: i16, pcontext: ::windows::core::RawPtr, pfpointtransformed: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TransformRect<Impl: IMILBitmapEffectPrimitiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, p: *mut MilRectD, fforwardtransform: i16, pcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HasAffineTransform<Impl: IMILBitmapEffectPrimitiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, pfaffine: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HasInverseTransform<Impl: IMILBitmapEffectPrimitiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, pfhasinverse: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAffineMatrix<Impl: IMILBitmapEffectPrimitiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, pmatrix: *mut super::super::Graphics::Dwm::MilMatrix3x2D) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetOutput::<Impl, IMPL_OFFSET>, TransformPoint::<Impl, IMPL_OFFSET>, TransformRect::<Impl, IMPL_OFFSET>, HasAffineTransform::<Impl, IMPL_OFFSET>, HasInverseTransform::<Impl, IMPL_OFFSET>, GetAffineMatrix::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectPrimitive as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectPrimitiveImplImpl: Sized {
    fn IsDirty();
    fn IsVolatile();
}
impl IMILBitmapEffectPrimitiveImplVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectPrimitiveImplImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectPrimitiveImplVtbl {
        unsafe extern "system" fn IsDirty<Impl: IMILBitmapEffectPrimitiveImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uioutputindex: u32, pfdirty: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsVolatile<Impl: IMILBitmapEffectPrimitiveImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uioutputindex: u32, pfvolatile: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, IsDirty::<Impl, IMPL_OFFSET>, IsVolatile::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectPrimitiveImpl as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectRenderContextImpl: Sized {
    fn SetOutputPixelFormat();
    fn GetOutputPixelFormat();
    fn SetUseSoftwareRenderer();
    fn SetInitialTransform();
    fn GetFinalTransform();
    fn SetOutputDPI();
    fn GetOutputDPI();
    fn SetRegionOfInterest();
}
impl IMILBitmapEffectRenderContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectRenderContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectRenderContextVtbl {
        unsafe extern "system" fn SetOutputPixelFormat<Impl: IMILBitmapEffectRenderContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputPixelFormat<Impl: IMILBitmapEffectRenderContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUseSoftwareRenderer<Impl: IMILBitmapEffectRenderContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fsoftware: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInitialTransform<Impl: IMILBitmapEffectRenderContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmatrix: *const MILMatrixF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFinalTransform<Impl: IMILBitmapEffectRenderContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmatrix: *mut MILMatrixF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOutputDPI<Impl: IMILBitmapEffectRenderContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dbldpix: f64, dbldpiy: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputDPI<Impl: IMILBitmapEffectRenderContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdbldpix: *mut f64, pdbldpiy: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRegionOfInterest<Impl: IMILBitmapEffectRenderContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const MilRectD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetOutputPixelFormat::<Impl, IMPL_OFFSET>,
            GetOutputPixelFormat::<Impl, IMPL_OFFSET>,
            SetUseSoftwareRenderer::<Impl, IMPL_OFFSET>,
            SetInitialTransform::<Impl, IMPL_OFFSET>,
            GetFinalTransform::<Impl, IMPL_OFFSET>,
            SetOutputDPI::<Impl, IMPL_OFFSET>,
            GetOutputDPI::<Impl, IMPL_OFFSET>,
            SetRegionOfInterest::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectRenderContext as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectRenderContextImplImpl: Sized {
    fn GetUseSoftwareRenderer();
    fn GetTransform();
    fn UpdateTransform();
    fn GetOutputBounds();
    fn UpdateOutputBounds();
}
impl IMILBitmapEffectRenderContextImplVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectRenderContextImplImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectRenderContextImplVtbl {
        unsafe extern "system" fn GetUseSoftwareRenderer<Impl: IMILBitmapEffectRenderContextImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfsoftware: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTransform<Impl: IMILBitmapEffectRenderContextImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmatrix: *mut MILMatrixF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateTransform<Impl: IMILBitmapEffectRenderContextImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmatrix: *const MILMatrixF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputBounds<Impl: IMILBitmapEffectRenderContextImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *mut MilRectD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateOutputBounds<Impl: IMILBitmapEffectRenderContextImplImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const MilRectD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetUseSoftwareRenderer::<Impl, IMPL_OFFSET>, GetTransform::<Impl, IMPL_OFFSET>, UpdateTransform::<Impl, IMPL_OFFSET>, GetOutputBounds::<Impl, IMPL_OFFSET>, UpdateOutputBounds::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffectRenderContextImpl as ::windows::core::Interface>::IID
    }
}
pub trait IMILBitmapEffectsImpl: Sized {
    fn _NewEnum();
    fn Parent();
    fn Item();
    fn Count();
}
impl IMILBitmapEffectsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMILBitmapEffectsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMILBitmapEffectsVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IMILBitmapEffectsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiureturn: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Parent<Impl: IMILBitmapEffectsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IMILBitmapEffectsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, ppeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: IMILBitmapEffectsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puicount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>, Parent::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMILBitmapEffects as ::windows::core::Interface>::IID
    }
}
