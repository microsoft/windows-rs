pub trait IMILBitmapEffectImpl: Sized {
    fn GetOutput();
    fn GetParentEffect();
    fn SetInputSource();
}
impl ::windows::core::RuntimeName for IMILBitmapEffect {
    const NAME: &'static str = "Windows.Win32.UI.Wpf.IMILBitmapEffect";
}
impl IMILBitmapEffectVtbl {
    pub const fn new<Impl: IMILBitmapEffectImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMILBitmapEffectVtbl {
        unsafe extern "system" fn GetOutput<Impl: IMILBitmapEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, pcontext: ::windows::core::RawPtr, ppbitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutput(uiindex, &*(&pcontext as *const <IMILBitmapEffectRenderContext as ::windows::core::Abi>::Abi as *const <IMILBitmapEffectRenderContext as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppbitmapsource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParentEffect<Impl: IMILBitmapEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppparenteffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetParentEffect(::core::mem::transmute_copy(&ppparenteffect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInputSource<Impl: IMILBitmapEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, pbitmapsource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetInputSource(uiindex, &*(&pbitmapsource as *const <super::super::Graphics::Imaging::IWICBitmapSource as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::IWICBitmapSource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMILBitmapEffect>, base.5, GetOutput::<Impl, OFFSET>, GetParentEffect::<Impl, OFFSET>, SetInputSource::<Impl, OFFSET>)
    }
}
pub trait IMILBitmapEffectConnectionsImpl: Sized {
    fn GetInputConnector();
    fn GetOutputConnector();
}
impl ::windows::core::RuntimeName for IMILBitmapEffectConnections {
    const NAME: &'static str = "Windows.Win32.UI.Wpf.IMILBitmapEffectConnections";
}
impl IMILBitmapEffectConnectionsVtbl {
    pub const fn new<Impl: IMILBitmapEffectConnectionsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMILBitmapEffectConnectionsVtbl {
        unsafe extern "system" fn GetInputConnector<Impl: IMILBitmapEffectConnectionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInputConnector(uiindex, ::core::mem::transmute_copy(&ppconnector)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputConnector<Impl: IMILBitmapEffectConnectionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutputConnector(uiindex, ::core::mem::transmute_copy(&ppconnector)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMILBitmapEffectConnections>, base.5, GetInputConnector::<Impl, OFFSET>, GetOutputConnector::<Impl, OFFSET>)
    }
}
pub trait IMILBitmapEffectConnectionsInfoImpl: Sized {
    fn GetNumberInputs();
    fn GetNumberOutputs();
    fn GetInputConnectorInfo();
    fn GetOutputConnectorInfo();
}
impl ::windows::core::RuntimeName for IMILBitmapEffectConnectionsInfo {
    const NAME: &'static str = "Windows.Win32.UI.Wpf.IMILBitmapEffectConnectionsInfo";
}
impl IMILBitmapEffectConnectionsInfoVtbl {
    pub const fn new<Impl: IMILBitmapEffectConnectionsInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMILBitmapEffectConnectionsInfoVtbl {
        unsafe extern "system" fn GetNumberInputs<Impl: IMILBitmapEffectConnectionsInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puinuminputs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNumberInputs(::core::mem::transmute_copy(&puinuminputs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumberOutputs<Impl: IMILBitmapEffectConnectionsInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puinumoutputs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNumberOutputs(::core::mem::transmute_copy(&puinumoutputs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputConnectorInfo<Impl: IMILBitmapEffectConnectionsInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnectorinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInputConnectorInfo(uiindex, ::core::mem::transmute_copy(&ppconnectorinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputConnectorInfo<Impl: IMILBitmapEffectConnectionsInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnectorinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutputConnectorInfo(uiindex, ::core::mem::transmute_copy(&ppconnectorinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMILBitmapEffectConnectionsInfo>, base.5, GetNumberInputs::<Impl, OFFSET>, GetNumberOutputs::<Impl, OFFSET>, GetInputConnectorInfo::<Impl, OFFSET>, GetOutputConnectorInfo::<Impl, OFFSET>)
    }
}
pub trait IMILBitmapEffectConnectorImpl: Sized + IMILBitmapEffectConnectorInfoImpl {
    fn IsConnected();
    fn GetBitmapEffect();
}
impl ::windows::core::RuntimeName for IMILBitmapEffectConnector {
    const NAME: &'static str = "Windows.Win32.UI.Wpf.IMILBitmapEffectConnector";
}
impl IMILBitmapEffectConnectorVtbl {
    pub const fn new<Impl: IMILBitmapEffectConnectorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMILBitmapEffectConnectorVtbl {
        unsafe extern "system" fn IsConnected<Impl: IMILBitmapEffectConnectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsConnected(::core::mem::transmute_copy(&pfconnected)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBitmapEffect<Impl: IMILBitmapEffectConnectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBitmapEffect(::core::mem::transmute_copy(&ppeffect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMILBitmapEffectConnector>, base.5, IsConnected::<Impl, OFFSET>, GetBitmapEffect::<Impl, OFFSET>)
    }
}
pub trait IMILBitmapEffectConnectorInfoImpl: Sized {
    fn GetIndex();
    fn GetOptimalFormat();
    fn GetNumberFormats();
    fn GetFormat();
}
impl ::windows::core::RuntimeName for IMILBitmapEffectConnectorInfo {
    const NAME: &'static str = "Windows.Win32.UI.Wpf.IMILBitmapEffectConnectorInfo";
}
impl IMILBitmapEffectConnectorInfoVtbl {
    pub const fn new<Impl: IMILBitmapEffectConnectorInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMILBitmapEffectConnectorInfoVtbl {
        unsafe extern "system" fn GetIndex<Impl: IMILBitmapEffectConnectorInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puiindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIndex(::core::mem::transmute_copy(&puiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptimalFormat<Impl: IMILBitmapEffectConnectorInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOptimalFormat(::core::mem::transmute_copy(&pformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumberFormats<Impl: IMILBitmapEffectConnectorInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulnumberformats: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNumberFormats(::core::mem::transmute_copy(&pulnumberformats)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormat<Impl: IMILBitmapEffectConnectorInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulindex: u32, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFormat(ulindex, ::core::mem::transmute_copy(&pformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMILBitmapEffectConnectorInfo>, base.5, GetIndex::<Impl, OFFSET>, GetOptimalFormat::<Impl, OFFSET>, GetNumberFormats::<Impl, OFFSET>, GetFormat::<Impl, OFFSET>)
    }
}
pub trait IMILBitmapEffectEventsImpl: Sized {
    fn PropertyChange();
    fn DirtyRegion();
}
impl ::windows::core::RuntimeName for IMILBitmapEffectEvents {
    const NAME: &'static str = "Windows.Win32.UI.Wpf.IMILBitmapEffectEvents";
}
impl IMILBitmapEffectEventsVtbl {
    pub const fn new<Impl: IMILBitmapEffectEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMILBitmapEffectEventsVtbl {
        unsafe extern "system" fn PropertyChange<Impl: IMILBitmapEffectEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peffect: ::windows::core::RawPtr, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PropertyChange(&*(&peffect as *const <IMILBitmapEffect as ::windows::core::Abi>::Abi as *const <IMILBitmapEffect as ::windows::core::DefaultType>::DefaultType), &*(&bstrpropertyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DirtyRegion<Impl: IMILBitmapEffectEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peffect: ::windows::core::RawPtr, prect: *const MilRectD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DirtyRegion(&*(&peffect as *const <IMILBitmapEffect as ::windows::core::Abi>::Abi as *const <IMILBitmapEffect as ::windows::core::DefaultType>::DefaultType), &*(&prect as *const <MilRectD as ::windows::core::Abi>::Abi as *const <MilRectD as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMILBitmapEffectEvents>, base.5, PropertyChange::<Impl, OFFSET>, DirtyRegion::<Impl, OFFSET>)
    }
}
pub trait IMILBitmapEffectFactoryImpl: Sized {
    fn CreateEffect();
    fn CreateContext();
    fn CreateEffectOuter();
}
impl ::windows::core::RuntimeName for IMILBitmapEffectFactory {
    const NAME: &'static str = "Windows.Win32.UI.Wpf.IMILBitmapEffectFactory";
}
impl IMILBitmapEffectFactoryVtbl {
    pub const fn new<Impl: IMILBitmapEffectFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMILBitmapEffectFactoryVtbl {
        unsafe extern "system" fn CreateEffect<Impl: IMILBitmapEffectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguideffect: *const ::windows::core::GUID, ppeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateEffect(&*(&pguideffect as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppeffect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateContext<Impl: IMILBitmapEffectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateContext(::core::mem::transmute_copy(&ppcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEffectOuter<Impl: IMILBitmapEffectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateEffectOuter(::core::mem::transmute_copy(&ppeffect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMILBitmapEffectFactory>, base.5, CreateEffect::<Impl, OFFSET>, CreateContext::<Impl, OFFSET>, CreateEffectOuter::<Impl, OFFSET>)
    }
}
pub trait IMILBitmapEffectGroupImpl: Sized {
    fn GetInteriorInputConnector();
    fn GetInteriorOutputConnector();
    fn Add();
}
impl ::windows::core::RuntimeName for IMILBitmapEffectGroup {
    const NAME: &'static str = "Windows.Win32.UI.Wpf.IMILBitmapEffectGroup";
}
impl IMILBitmapEffectGroupVtbl {
    pub const fn new<Impl: IMILBitmapEffectGroupImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMILBitmapEffectGroupVtbl {
        unsafe extern "system" fn GetInteriorInputConnector<Impl: IMILBitmapEffectGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInteriorInputConnector(uiindex, ::core::mem::transmute_copy(&ppconnector)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInteriorOutputConnector<Impl: IMILBitmapEffectGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInteriorOutputConnector(uiindex, ::core::mem::transmute_copy(&ppconnector)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IMILBitmapEffectGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peffect: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Add(&*(&peffect as *const <IMILBitmapEffect as ::windows::core::Abi>::Abi as *const <IMILBitmapEffect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMILBitmapEffectGroup>, base.5, GetInteriorInputConnector::<Impl, OFFSET>, GetInteriorOutputConnector::<Impl, OFFSET>, Add::<Impl, OFFSET>)
    }
}
pub trait IMILBitmapEffectGroupImplImpl: Sized {
    fn Preprocess();
    fn GetNumberChildren();
    fn GetChildren();
}
impl ::windows::core::RuntimeName for IMILBitmapEffectGroupImpl {
    const NAME: &'static str = "Windows.Win32.UI.Wpf.IMILBitmapEffectGroupImpl";
}
impl IMILBitmapEffectGroupImplVtbl {
    pub const fn new<Impl: IMILBitmapEffectGroupImplImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMILBitmapEffectGroupImplVtbl {
        unsafe extern "system" fn Preprocess<Impl: IMILBitmapEffectGroupImplImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Preprocess(&*(&pcontext as *const <IMILBitmapEffectRenderContext as ::windows::core::Abi>::Abi as *const <IMILBitmapEffectRenderContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumberChildren<Impl: IMILBitmapEffectGroupImplImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puinumberchildren: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNumberChildren(::core::mem::transmute_copy(&puinumberchildren)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChildren<Impl: IMILBitmapEffectGroupImplImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchildren: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetChildren(::core::mem::transmute_copy(&pchildren)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMILBitmapEffectGroupImpl>, base.5, Preprocess::<Impl, OFFSET>, GetNumberChildren::<Impl, OFFSET>, GetChildren::<Impl, OFFSET>)
    }
}
pub trait IMILBitmapEffectImplImpl: Sized {
    fn IsInPlaceModificationAllowed();
    fn SetParentEffect();
    fn GetInputSource();
    fn GetInputSourceBounds();
    fn GetInputBitmapSource();
    fn GetOutputBitmapSource();
    fn Initialize();
}
impl ::windows::core::RuntimeName for IMILBitmapEffectImpl {
    const NAME: &'static str = "Windows.Win32.UI.Wpf.IMILBitmapEffectImpl";
}
impl IMILBitmapEffectImplVtbl {
    pub const fn new<Impl: IMILBitmapEffectImplImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMILBitmapEffectImplVtbl {
        unsafe extern "system" fn IsInPlaceModificationAllowed<Impl: IMILBitmapEffectImplImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poutputconnector: ::windows::core::RawPtr, pfmodifyinplace: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsInPlaceModificationAllowed(&*(&poutputconnector as *const <IMILBitmapEffectOutputConnector as ::windows::core::Abi>::Abi as *const <IMILBitmapEffectOutputConnector as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfmodifyinplace)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParentEffect<Impl: IMILBitmapEffectImplImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pparenteffect: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetParentEffect(&*(&pparenteffect as *const <IMILBitmapEffectGroup as ::windows::core::Abi>::Abi as *const <IMILBitmapEffectGroup as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputSource<Impl: IMILBitmapEffectImplImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppbitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInputSource(uiindex, ::core::mem::transmute_copy(&ppbitmapsource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputSourceBounds<Impl: IMILBitmapEffectImplImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, prect: *mut MilRectD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInputSourceBounds(uiindex, ::core::mem::transmute_copy(&prect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputBitmapSource<Impl: IMILBitmapEffectImplImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, prendercontext: ::windows::core::RawPtr, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInputBitmapSource(uiindex, &*(&prendercontext as *const <IMILBitmapEffectRenderContext as ::windows::core::Abi>::Abi as *const <IMILBitmapEffectRenderContext as ::windows::core::DefaultType>::DefaultType), pfmodifyinplace, ::core::mem::transmute_copy(&ppbitmapsource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputBitmapSource<Impl: IMILBitmapEffectImplImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, prendercontext: ::windows::core::RawPtr, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutputBitmapSource(uiindex, &*(&prendercontext as *const <IMILBitmapEffectRenderContext as ::windows::core::Abi>::Abi as *const <IMILBitmapEffectRenderContext as ::windows::core::DefaultType>::DefaultType), pfmodifyinplace, ::core::mem::transmute_copy(&ppbitmapsource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IMILBitmapEffectImplImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinner: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&pinner as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMILBitmapEffectImpl>, base.5, IsInPlaceModificationAllowed::<Impl, OFFSET>, SetParentEffect::<Impl, OFFSET>, GetInputSource::<Impl, OFFSET>, GetInputSourceBounds::<Impl, OFFSET>, GetInputBitmapSource::<Impl, OFFSET>, GetOutputBitmapSource::<Impl, OFFSET>, Initialize::<Impl, OFFSET>)
    }
}
pub trait IMILBitmapEffectInputConnectorImpl: Sized + IMILBitmapEffectConnectorImpl + IMILBitmapEffectConnectorInfoImpl {
    fn ConnectTo();
    fn GetConnection();
}
impl ::windows::core::RuntimeName for IMILBitmapEffectInputConnector {
    const NAME: &'static str = "Windows.Win32.UI.Wpf.IMILBitmapEffectInputConnector";
}
impl IMILBitmapEffectInputConnectorVtbl {
    pub const fn new<Impl: IMILBitmapEffectInputConnectorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMILBitmapEffectInputConnectorVtbl {
        unsafe extern "system" fn ConnectTo<Impl: IMILBitmapEffectInputConnectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconnector: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectTo(&*(&pconnector as *const <IMILBitmapEffectOutputConnector as ::windows::core::Abi>::Abi as *const <IMILBitmapEffectOutputConnector as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnection<Impl: IMILBitmapEffectInputConnectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConnection(::core::mem::transmute_copy(&ppconnector)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMILBitmapEffectInputConnector>, base.5, ConnectTo::<Impl, OFFSET>, GetConnection::<Impl, OFFSET>)
    }
}
pub trait IMILBitmapEffectInteriorInputConnectorImpl: Sized {
    fn GetInputConnector();
}
impl ::windows::core::RuntimeName for IMILBitmapEffectInteriorInputConnector {
    const NAME: &'static str = "Windows.Win32.UI.Wpf.IMILBitmapEffectInteriorInputConnector";
}
impl IMILBitmapEffectInteriorInputConnectorVtbl {
    pub const fn new<Impl: IMILBitmapEffectInteriorInputConnectorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMILBitmapEffectInteriorInputConnectorVtbl {
        unsafe extern "system" fn GetInputConnector<Impl: IMILBitmapEffectInteriorInputConnectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinputconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInputConnector(::core::mem::transmute_copy(&pinputconnector)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMILBitmapEffectInteriorInputConnector>, base.5, GetInputConnector::<Impl, OFFSET>)
    }
}
pub trait IMILBitmapEffectInteriorOutputConnectorImpl: Sized {
    fn GetOutputConnector();
}
impl ::windows::core::RuntimeName for IMILBitmapEffectInteriorOutputConnector {
    const NAME: &'static str = "Windows.Win32.UI.Wpf.IMILBitmapEffectInteriorOutputConnector";
}
impl IMILBitmapEffectInteriorOutputConnectorVtbl {
    pub const fn new<Impl: IMILBitmapEffectInteriorOutputConnectorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMILBitmapEffectInteriorOutputConnectorVtbl {
        unsafe extern "system" fn GetOutputConnector<Impl: IMILBitmapEffectInteriorOutputConnectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poutputconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutputConnector(::core::mem::transmute_copy(&poutputconnector)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMILBitmapEffectInteriorOutputConnector>, base.5, GetOutputConnector::<Impl, OFFSET>)
    }
}
pub trait IMILBitmapEffectOutputConnectorImpl: Sized + IMILBitmapEffectConnectorImpl + IMILBitmapEffectConnectorInfoImpl {
    fn GetNumberConnections();
    fn GetConnection();
}
impl ::windows::core::RuntimeName for IMILBitmapEffectOutputConnector {
    const NAME: &'static str = "Windows.Win32.UI.Wpf.IMILBitmapEffectOutputConnector";
}
impl IMILBitmapEffectOutputConnectorVtbl {
    pub const fn new<Impl: IMILBitmapEffectOutputConnectorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMILBitmapEffectOutputConnectorVtbl {
        unsafe extern "system" fn GetNumberConnections<Impl: IMILBitmapEffectOutputConnectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puinumberconnections: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNumberConnections(::core::mem::transmute_copy(&puinumberconnections)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnection<Impl: IMILBitmapEffectOutputConnectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConnection(uiindex, ::core::mem::transmute_copy(&ppconnection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMILBitmapEffectOutputConnector>, base.5, GetNumberConnections::<Impl, OFFSET>, GetConnection::<Impl, OFFSET>)
    }
}
pub trait IMILBitmapEffectOutputConnectorImplImpl: Sized {
    fn AddBackLink();
    fn RemoveBackLink();
}
impl ::windows::core::RuntimeName for IMILBitmapEffectOutputConnectorImpl {
    const NAME: &'static str = "Windows.Win32.UI.Wpf.IMILBitmapEffectOutputConnectorImpl";
}
impl IMILBitmapEffectOutputConnectorImplVtbl {
    pub const fn new<Impl: IMILBitmapEffectOutputConnectorImplImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMILBitmapEffectOutputConnectorImplVtbl {
        unsafe extern "system" fn AddBackLink<Impl: IMILBitmapEffectOutputConnectorImplImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddBackLink(&*(&pconnection as *const <IMILBitmapEffectInputConnector as ::windows::core::Abi>::Abi as *const <IMILBitmapEffectInputConnector as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBackLink<Impl: IMILBitmapEffectOutputConnectorImplImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveBackLink(&*(&pconnection as *const <IMILBitmapEffectInputConnector as ::windows::core::Abi>::Abi as *const <IMILBitmapEffectInputConnector as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMILBitmapEffectOutputConnectorImpl>, base.5, AddBackLink::<Impl, OFFSET>, RemoveBackLink::<Impl, OFFSET>)
    }
}
pub trait IMILBitmapEffectPrimitiveImpl: Sized {
    fn GetOutput();
    fn TransformPoint();
    fn TransformRect();
    fn HasAffineTransform();
    fn HasInverseTransform();
    fn GetAffineMatrix();
}
impl ::windows::core::RuntimeName for IMILBitmapEffectPrimitive {
    const NAME: &'static str = "Windows.Win32.UI.Wpf.IMILBitmapEffectPrimitive";
}
impl IMILBitmapEffectPrimitiveVtbl {
    pub const fn new<Impl: IMILBitmapEffectPrimitiveImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMILBitmapEffectPrimitiveVtbl {
        unsafe extern "system" fn GetOutput<Impl: IMILBitmapEffectPrimitiveImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, pcontext: ::windows::core::RawPtr, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutput(uiindex, &*(&pcontext as *const <IMILBitmapEffectRenderContext as ::windows::core::Abi>::Abi as *const <IMILBitmapEffectRenderContext as ::windows::core::DefaultType>::DefaultType), pfmodifyinplace, ::core::mem::transmute_copy(&ppbitmapsource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransformPoint<Impl: IMILBitmapEffectPrimitiveImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, p: *mut MilPoint2D, fforwardtransform: i16, pcontext: ::windows::core::RawPtr, pfpointtransformed: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransformPoint(uiindex, &*(&p as *const <MilPoint2D as ::windows::core::Abi>::Abi as *const <MilPoint2D as ::windows::core::DefaultType>::DefaultType), fforwardtransform, &*(&pcontext as *const <IMILBitmapEffectRenderContext as ::windows::core::Abi>::Abi as *const <IMILBitmapEffectRenderContext as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfpointtransformed)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransformRect<Impl: IMILBitmapEffectPrimitiveImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, p: *mut MilRectD, fforwardtransform: i16, pcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransformRect(uiindex, &*(&p as *const <MilRectD as ::windows::core::Abi>::Abi as *const <MilRectD as ::windows::core::DefaultType>::DefaultType), fforwardtransform, &*(&pcontext as *const <IMILBitmapEffectRenderContext as ::windows::core::Abi>::Abi as *const <IMILBitmapEffectRenderContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasAffineTransform<Impl: IMILBitmapEffectPrimitiveImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, pfaffine: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HasAffineTransform(uiindex, ::core::mem::transmute_copy(&pfaffine)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasInverseTransform<Impl: IMILBitmapEffectPrimitiveImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, pfhasinverse: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HasInverseTransform(uiindex, ::core::mem::transmute_copy(&pfhasinverse)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAffineMatrix<Impl: IMILBitmapEffectPrimitiveImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, pmatrix: *mut super::super::Graphics::Dwm::MilMatrix3x2D) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAffineMatrix(uiindex, &*(&pmatrix as *const <super::super::Graphics::Dwm::MilMatrix3x2D as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Dwm::MilMatrix3x2D as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMILBitmapEffectPrimitive>, base.5, GetOutput::<Impl, OFFSET>, TransformPoint::<Impl, OFFSET>, TransformRect::<Impl, OFFSET>, HasAffineTransform::<Impl, OFFSET>, HasInverseTransform::<Impl, OFFSET>, GetAffineMatrix::<Impl, OFFSET>)
    }
}
pub trait IMILBitmapEffectPrimitiveImplImpl: Sized {
    fn IsDirty();
    fn IsVolatile();
}
impl ::windows::core::RuntimeName for IMILBitmapEffectPrimitiveImpl {
    const NAME: &'static str = "Windows.Win32.UI.Wpf.IMILBitmapEffectPrimitiveImpl";
}
impl IMILBitmapEffectPrimitiveImplVtbl {
    pub const fn new<Impl: IMILBitmapEffectPrimitiveImplImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMILBitmapEffectPrimitiveImplVtbl {
        unsafe extern "system" fn IsDirty<Impl: IMILBitmapEffectPrimitiveImplImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uioutputindex: u32, pfdirty: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDirty(uioutputindex, ::core::mem::transmute_copy(&pfdirty)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVolatile<Impl: IMILBitmapEffectPrimitiveImplImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uioutputindex: u32, pfvolatile: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsVolatile(uioutputindex, ::core::mem::transmute_copy(&pfvolatile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMILBitmapEffectPrimitiveImpl>, base.5, IsDirty::<Impl, OFFSET>, IsVolatile::<Impl, OFFSET>)
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
impl ::windows::core::RuntimeName for IMILBitmapEffectRenderContext {
    const NAME: &'static str = "Windows.Win32.UI.Wpf.IMILBitmapEffectRenderContext";
}
impl IMILBitmapEffectRenderContextVtbl {
    pub const fn new<Impl: IMILBitmapEffectRenderContextImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMILBitmapEffectRenderContextVtbl {
        unsafe extern "system" fn SetOutputPixelFormat<Impl: IMILBitmapEffectRenderContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetOutputPixelFormat(&*(&format as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputPixelFormat<Impl: IMILBitmapEffectRenderContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutputPixelFormat(::core::mem::transmute_copy(&pformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseSoftwareRenderer<Impl: IMILBitmapEffectRenderContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fsoftware: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetUseSoftwareRenderer(fsoftware) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialTransform<Impl: IMILBitmapEffectRenderContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmatrix: *const MILMatrixF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetInitialTransform(&*(&pmatrix as *const <MILMatrixF as ::windows::core::Abi>::Abi as *const <MILMatrixF as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalTransform<Impl: IMILBitmapEffectRenderContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmatrix: *mut MILMatrixF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFinalTransform(::core::mem::transmute_copy(&pmatrix)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputDPI<Impl: IMILBitmapEffectRenderContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dbldpix: f64, dbldpiy: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetOutputDPI(dbldpix, dbldpiy) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputDPI<Impl: IMILBitmapEffectRenderContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdbldpix: *mut f64, pdbldpiy: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutputDPI(::core::mem::transmute_copy(&pdbldpix), ::core::mem::transmute_copy(&pdbldpiy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRegionOfInterest<Impl: IMILBitmapEffectRenderContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prect: *const MilRectD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRegionOfInterest(&*(&prect as *const <MilRectD as ::windows::core::Abi>::Abi as *const <MilRectD as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMILBitmapEffectRenderContext>, base.5, SetOutputPixelFormat::<Impl, OFFSET>, GetOutputPixelFormat::<Impl, OFFSET>, SetUseSoftwareRenderer::<Impl, OFFSET>, SetInitialTransform::<Impl, OFFSET>, GetFinalTransform::<Impl, OFFSET>, SetOutputDPI::<Impl, OFFSET>, GetOutputDPI::<Impl, OFFSET>, SetRegionOfInterest::<Impl, OFFSET>)
    }
}
pub trait IMILBitmapEffectRenderContextImplImpl: Sized {
    fn GetUseSoftwareRenderer();
    fn GetTransform();
    fn UpdateTransform();
    fn GetOutputBounds();
    fn UpdateOutputBounds();
}
impl ::windows::core::RuntimeName for IMILBitmapEffectRenderContextImpl {
    const NAME: &'static str = "Windows.Win32.UI.Wpf.IMILBitmapEffectRenderContextImpl";
}
impl IMILBitmapEffectRenderContextImplVtbl {
    pub const fn new<Impl: IMILBitmapEffectRenderContextImplImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMILBitmapEffectRenderContextImplVtbl {
        unsafe extern "system" fn GetUseSoftwareRenderer<Impl: IMILBitmapEffectRenderContextImplImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfsoftware: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetUseSoftwareRenderer(::core::mem::transmute_copy(&pfsoftware)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransform<Impl: IMILBitmapEffectRenderContextImplImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmatrix: *mut MILMatrixF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTransform(&*(&pmatrix as *const <MILMatrixF as ::windows::core::Abi>::Abi as *const <MILMatrixF as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateTransform<Impl: IMILBitmapEffectRenderContextImplImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmatrix: *const MILMatrixF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateTransform(&*(&pmatrix as *const <MILMatrixF as ::windows::core::Abi>::Abi as *const <MILMatrixF as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputBounds<Impl: IMILBitmapEffectRenderContextImplImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prect: *mut MilRectD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutputBounds(&*(&prect as *const <MilRectD as ::windows::core::Abi>::Abi as *const <MilRectD as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateOutputBounds<Impl: IMILBitmapEffectRenderContextImplImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prect: *const MilRectD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateOutputBounds(&*(&prect as *const <MilRectD as ::windows::core::Abi>::Abi as *const <MilRectD as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMILBitmapEffectRenderContextImpl>, base.5, GetUseSoftwareRenderer::<Impl, OFFSET>, GetTransform::<Impl, OFFSET>, UpdateTransform::<Impl, OFFSET>, GetOutputBounds::<Impl, OFFSET>, UpdateOutputBounds::<Impl, OFFSET>)
    }
}
pub trait IMILBitmapEffectsImpl: Sized {
    fn _NewEnum();
    fn Parent();
    fn Item();
    fn Count();
}
impl ::windows::core::RuntimeName for IMILBitmapEffects {
    const NAME: &'static str = "Windows.Win32.UI.Wpf.IMILBitmapEffects";
}
impl IMILBitmapEffectsVtbl {
    pub const fn new<Impl: IMILBitmapEffectsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMILBitmapEffectsVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IMILBitmapEffectsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppiureturn: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppiureturn)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Impl: IMILBitmapEffectsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Parent(::core::mem::transmute_copy(&ppeffect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IMILBitmapEffectsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32, ppeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(uindex, ::core::mem::transmute_copy(&ppeffect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IMILBitmapEffectsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puicount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&puicount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMILBitmapEffects>, base.5, _NewEnum::<Impl, OFFSET>, Parent::<Impl, OFFSET>, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>)
    }
}
