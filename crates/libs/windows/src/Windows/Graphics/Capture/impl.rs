#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
pub trait IDirect3D11CaptureFrame_Impl: Sized {
    fn Surface(&mut self) -> ::windows::core::Result<super::DirectX::Direct3D11::IDirect3DSurface>;
    fn SystemRelativeTime(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn ContentSize(&mut self) -> ::windows::core::Result<super::SizeInt32>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDirect3D11CaptureFrame {
    const NAME: &'static str = "Windows.Graphics.Capture.IDirect3D11CaptureFrame";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl IDirect3D11CaptureFrame_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3D11CaptureFrame_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3D11CaptureFrame_Vtbl {
        unsafe extern "system" fn Surface<Impl: IDirect3D11CaptureFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Surface() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemRelativeTime<Impl: IDirect3D11CaptureFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemRelativeTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentSize<Impl: IDirect3D11CaptureFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::SizeInt32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDirect3D11CaptureFrame, BASE_OFFSET>(),
            Surface: Surface::<Impl, IMPL_OFFSET>,
            SystemRelativeTime: SystemRelativeTime::<Impl, IMPL_OFFSET>,
            ContentSize: ContentSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3D11CaptureFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "System", feature = "implement_exclusive"))]
pub trait IDirect3D11CaptureFramePool_Impl: Sized {
    fn Recreate(&mut self, device: &::core::option::Option<super::DirectX::Direct3D11::IDirect3DDevice>, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: &super::SizeInt32) -> ::windows::core::Result<()>;
    fn TryGetNextFrame(&mut self) -> ::windows::core::Result<Direct3D11CaptureFrame>;
    fn FrameArrived(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Direct3D11CaptureFramePool, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameArrived(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CreateCaptureSession(&mut self, item: &::core::option::Option<GraphicsCaptureItem>) -> ::windows::core::Result<GraphicsCaptureSession>;
    fn DispatcherQueue(&mut self) -> ::windows::core::Result<super::super::System::DispatcherQueue>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDirect3D11CaptureFramePool {
    const NAME: &'static str = "Windows.Graphics.Capture.IDirect3D11CaptureFramePool";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "System", feature = "implement_exclusive"))]
impl IDirect3D11CaptureFramePool_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3D11CaptureFramePool_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3D11CaptureFramePool_Vtbl {
        unsafe extern "system" fn Recreate<Impl: IDirect3D11CaptureFramePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, device: ::windows::core::RawPtr, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Recreate(&*(&device as *const <super::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::Abi>::Abi as *const <super::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::DefaultType>::DefaultType), pixelformat, numberofbuffers, &*(&size as *const <super::SizeInt32 as ::windows::core::Abi>::Abi as *const <super::SizeInt32 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryGetNextFrame<Impl: IDirect3D11CaptureFramePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetNextFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FrameArrived<Impl: IDirect3D11CaptureFramePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameArrived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<Direct3D11CaptureFramePool, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<Direct3D11CaptureFramePool, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFrameArrived<Impl: IDirect3D11CaptureFramePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFrameArrived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CreateCaptureSession<Impl: IDirect3D11CaptureFramePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCaptureSession(&*(&item as *const <GraphicsCaptureItem as ::windows::core::Abi>::Abi as *const <GraphicsCaptureItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DispatcherQueue<Impl: IDirect3D11CaptureFramePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DispatcherQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDirect3D11CaptureFramePool, BASE_OFFSET>(),
            Recreate: Recreate::<Impl, IMPL_OFFSET>,
            TryGetNextFrame: TryGetNextFrame::<Impl, IMPL_OFFSET>,
            FrameArrived: FrameArrived::<Impl, IMPL_OFFSET>,
            RemoveFrameArrived: RemoveFrameArrived::<Impl, IMPL_OFFSET>,
            CreateCaptureSession: CreateCaptureSession::<Impl, IMPL_OFFSET>,
            DispatcherQueue: DispatcherQueue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3D11CaptureFramePool as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
pub trait IDirect3D11CaptureFramePoolStatics_Impl: Sized {
    fn Create(&mut self, device: &::core::option::Option<super::DirectX::Direct3D11::IDirect3DDevice>, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: &super::SizeInt32) -> ::windows::core::Result<Direct3D11CaptureFramePool>;
}
#[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDirect3D11CaptureFramePoolStatics {
    const NAME: &'static str = "Windows.Graphics.Capture.IDirect3D11CaptureFramePoolStatics";
}
#[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl IDirect3D11CaptureFramePoolStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3D11CaptureFramePoolStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3D11CaptureFramePoolStatics_Vtbl {
        unsafe extern "system" fn Create<Impl: IDirect3D11CaptureFramePoolStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, device: ::windows::core::RawPtr, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&device as *const <super::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::Abi>::Abi as *const <super::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::DefaultType>::DefaultType), pixelformat, numberofbuffers, &*(&size as *const <super::SizeInt32 as ::windows::core::Abi>::Abi as *const <super::SizeInt32 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDirect3D11CaptureFramePoolStatics, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3D11CaptureFramePoolStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
pub trait IDirect3D11CaptureFramePoolStatics2_Impl: Sized {
    fn CreateFreeThreaded(&mut self, device: &::core::option::Option<super::DirectX::Direct3D11::IDirect3DDevice>, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: &super::SizeInt32) -> ::windows::core::Result<Direct3D11CaptureFramePool>;
}
#[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDirect3D11CaptureFramePoolStatics2 {
    const NAME: &'static str = "Windows.Graphics.Capture.IDirect3D11CaptureFramePoolStatics2";
}
#[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl IDirect3D11CaptureFramePoolStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3D11CaptureFramePoolStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3D11CaptureFramePoolStatics2_Vtbl {
        unsafe extern "system" fn CreateFreeThreaded<Impl: IDirect3D11CaptureFramePoolStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, device: ::windows::core::RawPtr, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFreeThreaded(&*(&device as *const <super::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::Abi>::Abi as *const <super::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::DefaultType>::DefaultType), pixelformat, numberofbuffers, &*(&size as *const <super::SizeInt32 as ::windows::core::Abi>::Abi as *const <super::SizeInt32 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDirect3D11CaptureFramePoolStatics2, BASE_OFFSET>(),
            CreateFreeThreaded: CreateFreeThreaded::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3D11CaptureFramePoolStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Security_Authorization_AppCapabilityAccess", feature = "implement_exclusive"))]
pub trait IGraphicsCaptureAccessStatics_Impl: Sized {
    fn RequestAccessAsync(&mut self, request: GraphicsCaptureAccessKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "Security_Authorization_AppCapabilityAccess", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGraphicsCaptureAccessStatics {
    const NAME: &'static str = "Windows.Graphics.Capture.IGraphicsCaptureAccessStatics";
}
#[cfg(all(feature = "Foundation", feature = "Security_Authorization_AppCapabilityAccess", feature = "implement_exclusive"))]
impl IGraphicsCaptureAccessStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGraphicsCaptureAccessStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGraphicsCaptureAccessStatics_Vtbl {
        unsafe extern "system" fn RequestAccessAsync<Impl: IGraphicsCaptureAccessStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, request: GraphicsCaptureAccessKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAccessAsync(request) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGraphicsCaptureAccessStatics, BASE_OFFSET>(),
            RequestAccessAsync: RequestAccessAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGraphicsCaptureAccessStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGraphicsCaptureItem_Impl: Sized {
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Size(&mut self) -> ::windows::core::Result<super::SizeInt32>;
    fn Closed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GraphicsCaptureItem, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGraphicsCaptureItem {
    const NAME: &'static str = "Windows.Graphics.Capture.IGraphicsCaptureItem";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGraphicsCaptureItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGraphicsCaptureItem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGraphicsCaptureItem_Vtbl {
        unsafe extern "system" fn DisplayName<Impl: IGraphicsCaptureItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Impl: IGraphicsCaptureItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::SizeInt32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Closed<Impl: IGraphicsCaptureItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Closed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<GraphicsCaptureItem, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<GraphicsCaptureItem, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Impl: IGraphicsCaptureItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGraphicsCaptureItem, BASE_OFFSET>(),
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            Size: Size::<Impl, IMPL_OFFSET>,
            Closed: Closed::<Impl, IMPL_OFFSET>,
            RemoveClosed: RemoveClosed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGraphicsCaptureItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
pub trait IGraphicsCaptureItemStatics_Impl: Sized {
    fn CreateFromVisual(&mut self, visual: &::core::option::Option<super::super::UI::Composition::Visual>) -> ::windows::core::Result<GraphicsCaptureItem>;
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGraphicsCaptureItemStatics {
    const NAME: &'static str = "Windows.Graphics.Capture.IGraphicsCaptureItemStatics";
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
impl IGraphicsCaptureItemStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGraphicsCaptureItemStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGraphicsCaptureItemStatics_Vtbl {
        unsafe extern "system" fn CreateFromVisual<Impl: IGraphicsCaptureItemStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromVisual(&*(&visual as *const <super::super::UI::Composition::Visual as ::windows::core::Abi>::Abi as *const <super::super::UI::Composition::Visual as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGraphicsCaptureItemStatics, BASE_OFFSET>(),
            CreateFromVisual: CreateFromVisual::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGraphicsCaptureItemStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI", feature = "implement_exclusive"))]
pub trait IGraphicsCaptureItemStatics2_Impl: Sized {
    fn TryCreateFromWindowId(&mut self, windowid: &super::super::UI::WindowId) -> ::windows::core::Result<GraphicsCaptureItem>;
    fn TryCreateFromDisplayId(&mut self, displayid: &super::DisplayId) -> ::windows::core::Result<GraphicsCaptureItem>;
}
#[cfg(all(feature = "UI", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGraphicsCaptureItemStatics2 {
    const NAME: &'static str = "Windows.Graphics.Capture.IGraphicsCaptureItemStatics2";
}
#[cfg(all(feature = "UI", feature = "implement_exclusive"))]
impl IGraphicsCaptureItemStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGraphicsCaptureItemStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGraphicsCaptureItemStatics2_Vtbl {
        unsafe extern "system" fn TryCreateFromWindowId<Impl: IGraphicsCaptureItemStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowid: super::super::UI::WindowId, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryCreateFromWindowId(&*(&windowid as *const <super::super::UI::WindowId as ::windows::core::Abi>::Abi as *const <super::super::UI::WindowId as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryCreateFromDisplayId<Impl: IGraphicsCaptureItemStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayid: super::DisplayId, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryCreateFromDisplayId(&*(&displayid as *const <super::DisplayId as ::windows::core::Abi>::Abi as *const <super::DisplayId as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGraphicsCaptureItemStatics2, BASE_OFFSET>(),
            TryCreateFromWindowId: TryCreateFromWindowId::<Impl, IMPL_OFFSET>,
            TryCreateFromDisplayId: TryCreateFromDisplayId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGraphicsCaptureItemStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGraphicsCapturePicker_Impl: Sized {
    fn PickSingleItemAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GraphicsCaptureItem>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGraphicsCapturePicker {
    const NAME: &'static str = "Windows.Graphics.Capture.IGraphicsCapturePicker";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGraphicsCapturePicker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGraphicsCapturePicker_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGraphicsCapturePicker_Vtbl {
        unsafe extern "system" fn PickSingleItemAsync<Impl: IGraphicsCapturePicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PickSingleItemAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGraphicsCapturePicker, BASE_OFFSET>(),
            PickSingleItemAsync: PickSingleItemAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGraphicsCapturePicker as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGraphicsCaptureSession_Impl: Sized {
    fn StartCapture(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGraphicsCaptureSession {
    const NAME: &'static str = "Windows.Graphics.Capture.IGraphicsCaptureSession";
}
#[cfg(feature = "implement_exclusive")]
impl IGraphicsCaptureSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGraphicsCaptureSession_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGraphicsCaptureSession_Vtbl {
        unsafe extern "system" fn StartCapture<Impl: IGraphicsCaptureSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartCapture().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGraphicsCaptureSession, BASE_OFFSET>(),
            StartCapture: StartCapture::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGraphicsCaptureSession as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGraphicsCaptureSession2_Impl: Sized {
    fn IsCursorCaptureEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsCursorCaptureEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGraphicsCaptureSession2 {
    const NAME: &'static str = "Windows.Graphics.Capture.IGraphicsCaptureSession2";
}
#[cfg(feature = "implement_exclusive")]
impl IGraphicsCaptureSession2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGraphicsCaptureSession2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGraphicsCaptureSession2_Vtbl {
        unsafe extern "system" fn IsCursorCaptureEnabled<Impl: IGraphicsCaptureSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCursorCaptureEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsCursorCaptureEnabled<Impl: IGraphicsCaptureSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsCursorCaptureEnabled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGraphicsCaptureSession2, BASE_OFFSET>(),
            IsCursorCaptureEnabled: IsCursorCaptureEnabled::<Impl, IMPL_OFFSET>,
            SetIsCursorCaptureEnabled: SetIsCursorCaptureEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGraphicsCaptureSession2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGraphicsCaptureSession3_Impl: Sized {
    fn IsBorderRequired(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsBorderRequired(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGraphicsCaptureSession3 {
    const NAME: &'static str = "Windows.Graphics.Capture.IGraphicsCaptureSession3";
}
#[cfg(feature = "implement_exclusive")]
impl IGraphicsCaptureSession3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGraphicsCaptureSession3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGraphicsCaptureSession3_Vtbl {
        unsafe extern "system" fn IsBorderRequired<Impl: IGraphicsCaptureSession3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBorderRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsBorderRequired<Impl: IGraphicsCaptureSession3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsBorderRequired(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGraphicsCaptureSession3, BASE_OFFSET>(),
            IsBorderRequired: IsBorderRequired::<Impl, IMPL_OFFSET>,
            SetIsBorderRequired: SetIsBorderRequired::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGraphicsCaptureSession3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGraphicsCaptureSessionStatics_Impl: Sized {
    fn IsSupported(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGraphicsCaptureSessionStatics {
    const NAME: &'static str = "Windows.Graphics.Capture.IGraphicsCaptureSessionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGraphicsCaptureSessionStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGraphicsCaptureSessionStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGraphicsCaptureSessionStatics_Vtbl {
        unsafe extern "system" fn IsSupported<Impl: IGraphicsCaptureSessionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGraphicsCaptureSessionStatics, BASE_OFFSET>(),
            IsSupported: IsSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGraphicsCaptureSessionStatics as ::windows::core::Interface>::IID
    }
}
