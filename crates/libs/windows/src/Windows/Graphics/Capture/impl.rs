#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
pub trait IDirect3D11CaptureFrameImpl: Sized {
    fn Surface(&self) -> ::windows::core::Result<super::DirectX::Direct3D11::IDirect3DSurface>;
    fn SystemRelativeTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn ContentSize(&self) -> ::windows::core::Result<super::SizeInt32>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDirect3D11CaptureFrame {
    const NAME: &'static str = "Windows.Graphics.Capture.IDirect3D11CaptureFrame";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl IDirect3D11CaptureFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3D11CaptureFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3D11CaptureFrameVtbl {
        unsafe extern "system" fn Surface<Impl: IDirect3D11CaptureFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SystemRelativeTime<Impl: IDirect3D11CaptureFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ContentSize<Impl: IDirect3D11CaptureFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::SizeInt32) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDirect3D11CaptureFrame>, ::windows::core::GetTrustLevel, Surface::<Impl, IMPL_OFFSET>, SystemRelativeTime::<Impl, IMPL_OFFSET>, ContentSize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3D11CaptureFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "System", feature = "implement_exclusive"))]
pub trait IDirect3D11CaptureFramePoolImpl: Sized {
    fn Recreate(&self, device: &::core::option::Option<super::DirectX::Direct3D11::IDirect3DDevice>, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: &super::SizeInt32) -> ::windows::core::Result<()>;
    fn TryGetNextFrame(&self) -> ::windows::core::Result<Direct3D11CaptureFrame>;
    fn FrameArrived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Direct3D11CaptureFramePool, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameArrived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CreateCaptureSession(&self, item: &::core::option::Option<GraphicsCaptureItem>) -> ::windows::core::Result<GraphicsCaptureSession>;
    fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::System::DispatcherQueue>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDirect3D11CaptureFramePool {
    const NAME: &'static str = "Windows.Graphics.Capture.IDirect3D11CaptureFramePool";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "System", feature = "implement_exclusive"))]
impl IDirect3D11CaptureFramePoolVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3D11CaptureFramePoolImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3D11CaptureFramePoolVtbl {
        unsafe extern "system" fn Recreate<Impl: IDirect3D11CaptureFramePoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, device: ::windows::core::RawPtr, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Recreate(&*(&device as *const <super::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::Abi>::Abi as *const <super::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::DefaultType>::DefaultType), pixelformat, numberofbuffers, &*(&size as *const <super::SizeInt32 as ::windows::core::Abi>::Abi as *const <super::SizeInt32 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryGetNextFrame<Impl: IDirect3D11CaptureFramePoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FrameArrived<Impl: IDirect3D11CaptureFramePoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveFrameArrived<Impl: IDirect3D11CaptureFramePoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFrameArrived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CreateCaptureSession<Impl: IDirect3D11CaptureFramePoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DispatcherQueue<Impl: IDirect3D11CaptureFramePoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDirect3D11CaptureFramePool>,
            ::windows::core::GetTrustLevel,
            Recreate::<Impl, IMPL_OFFSET>,
            TryGetNextFrame::<Impl, IMPL_OFFSET>,
            FrameArrived::<Impl, IMPL_OFFSET>,
            RemoveFrameArrived::<Impl, IMPL_OFFSET>,
            CreateCaptureSession::<Impl, IMPL_OFFSET>,
            DispatcherQueue::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3D11CaptureFramePool as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
pub trait IDirect3D11CaptureFramePoolStaticsImpl: Sized {
    fn Create(&self, device: &::core::option::Option<super::DirectX::Direct3D11::IDirect3DDevice>, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: &super::SizeInt32) -> ::windows::core::Result<Direct3D11CaptureFramePool>;
}
#[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDirect3D11CaptureFramePoolStatics {
    const NAME: &'static str = "Windows.Graphics.Capture.IDirect3D11CaptureFramePoolStatics";
}
#[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl IDirect3D11CaptureFramePoolStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3D11CaptureFramePoolStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3D11CaptureFramePoolStaticsVtbl {
        unsafe extern "system" fn Create<Impl: IDirect3D11CaptureFramePoolStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, device: ::windows::core::RawPtr, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDirect3D11CaptureFramePoolStatics>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3D11CaptureFramePoolStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
pub trait IDirect3D11CaptureFramePoolStatics2Impl: Sized {
    fn CreateFreeThreaded(&self, device: &::core::option::Option<super::DirectX::Direct3D11::IDirect3DDevice>, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: &super::SizeInt32) -> ::windows::core::Result<Direct3D11CaptureFramePool>;
}
#[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDirect3D11CaptureFramePoolStatics2 {
    const NAME: &'static str = "Windows.Graphics.Capture.IDirect3D11CaptureFramePoolStatics2";
}
#[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl IDirect3D11CaptureFramePoolStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3D11CaptureFramePoolStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3D11CaptureFramePoolStatics2Vtbl {
        unsafe extern "system" fn CreateFreeThreaded<Impl: IDirect3D11CaptureFramePoolStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, device: ::windows::core::RawPtr, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDirect3D11CaptureFramePoolStatics2>, ::windows::core::GetTrustLevel, CreateFreeThreaded::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3D11CaptureFramePoolStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Security_Authorization_AppCapabilityAccess", feature = "implement_exclusive"))]
pub trait IGraphicsCaptureAccessStaticsImpl: Sized {
    fn RequestAccessAsync(&self, request: GraphicsCaptureAccessKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "Security_Authorization_AppCapabilityAccess", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGraphicsCaptureAccessStatics {
    const NAME: &'static str = "Windows.Graphics.Capture.IGraphicsCaptureAccessStatics";
}
#[cfg(all(feature = "Foundation", feature = "Security_Authorization_AppCapabilityAccess", feature = "implement_exclusive"))]
impl IGraphicsCaptureAccessStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGraphicsCaptureAccessStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGraphicsCaptureAccessStaticsVtbl {
        unsafe extern "system" fn RequestAccessAsync<Impl: IGraphicsCaptureAccessStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, request: GraphicsCaptureAccessKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGraphicsCaptureAccessStatics>, ::windows::core::GetTrustLevel, RequestAccessAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGraphicsCaptureAccessStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGraphicsCaptureItemImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Size(&self) -> ::windows::core::Result<super::SizeInt32>;
    fn Closed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GraphicsCaptureItem, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGraphicsCaptureItem {
    const NAME: &'static str = "Windows.Graphics.Capture.IGraphicsCaptureItem";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGraphicsCaptureItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGraphicsCaptureItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGraphicsCaptureItemVtbl {
        unsafe extern "system" fn DisplayName<Impl: IGraphicsCaptureItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Size<Impl: IGraphicsCaptureItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::SizeInt32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Closed<Impl: IGraphicsCaptureItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveClosed<Impl: IGraphicsCaptureItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGraphicsCaptureItem>, ::windows::core::GetTrustLevel, DisplayName::<Impl, IMPL_OFFSET>, Size::<Impl, IMPL_OFFSET>, Closed::<Impl, IMPL_OFFSET>, RemoveClosed::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGraphicsCaptureItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
pub trait IGraphicsCaptureItemStaticsImpl: Sized {
    fn CreateFromVisual(&self, visual: &::core::option::Option<super::super::UI::Composition::Visual>) -> ::windows::core::Result<GraphicsCaptureItem>;
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGraphicsCaptureItemStatics {
    const NAME: &'static str = "Windows.Graphics.Capture.IGraphicsCaptureItemStatics";
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
impl IGraphicsCaptureItemStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGraphicsCaptureItemStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGraphicsCaptureItemStaticsVtbl {
        unsafe extern "system" fn CreateFromVisual<Impl: IGraphicsCaptureItemStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGraphicsCaptureItemStatics>, ::windows::core::GetTrustLevel, CreateFromVisual::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGraphicsCaptureItemStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI", feature = "implement_exclusive"))]
pub trait IGraphicsCaptureItemStatics2Impl: Sized {
    fn TryCreateFromWindowId(&self, windowid: &super::super::UI::WindowId) -> ::windows::core::Result<GraphicsCaptureItem>;
    fn TryCreateFromDisplayId(&self, displayid: &super::DisplayId) -> ::windows::core::Result<GraphicsCaptureItem>;
}
#[cfg(all(feature = "UI", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGraphicsCaptureItemStatics2 {
    const NAME: &'static str = "Windows.Graphics.Capture.IGraphicsCaptureItemStatics2";
}
#[cfg(all(feature = "UI", feature = "implement_exclusive"))]
impl IGraphicsCaptureItemStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGraphicsCaptureItemStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGraphicsCaptureItemStatics2Vtbl {
        unsafe extern "system" fn TryCreateFromWindowId<Impl: IGraphicsCaptureItemStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowid: super::super::UI::WindowId, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryCreateFromDisplayId<Impl: IGraphicsCaptureItemStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayid: super::DisplayId, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGraphicsCaptureItemStatics2>, ::windows::core::GetTrustLevel, TryCreateFromWindowId::<Impl, IMPL_OFFSET>, TryCreateFromDisplayId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGraphicsCaptureItemStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGraphicsCapturePickerImpl: Sized {
    fn PickSingleItemAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GraphicsCaptureItem>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGraphicsCapturePicker {
    const NAME: &'static str = "Windows.Graphics.Capture.IGraphicsCapturePicker";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGraphicsCapturePickerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGraphicsCapturePickerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGraphicsCapturePickerVtbl {
        unsafe extern "system" fn PickSingleItemAsync<Impl: IGraphicsCapturePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGraphicsCapturePicker>, ::windows::core::GetTrustLevel, PickSingleItemAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGraphicsCapturePicker as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGraphicsCaptureSessionImpl: Sized {
    fn StartCapture(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGraphicsCaptureSession {
    const NAME: &'static str = "Windows.Graphics.Capture.IGraphicsCaptureSession";
}
#[cfg(feature = "implement_exclusive")]
impl IGraphicsCaptureSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGraphicsCaptureSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGraphicsCaptureSessionVtbl {
        unsafe extern "system" fn StartCapture<Impl: IGraphicsCaptureSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartCapture().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGraphicsCaptureSession>, ::windows::core::GetTrustLevel, StartCapture::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGraphicsCaptureSession as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGraphicsCaptureSession2Impl: Sized {
    fn IsCursorCaptureEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsCursorCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGraphicsCaptureSession2 {
    const NAME: &'static str = "Windows.Graphics.Capture.IGraphicsCaptureSession2";
}
#[cfg(feature = "implement_exclusive")]
impl IGraphicsCaptureSession2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGraphicsCaptureSession2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGraphicsCaptureSession2Vtbl {
        unsafe extern "system" fn IsCursorCaptureEnabled<Impl: IGraphicsCaptureSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsCursorCaptureEnabled<Impl: IGraphicsCaptureSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsCursorCaptureEnabled(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGraphicsCaptureSession2>, ::windows::core::GetTrustLevel, IsCursorCaptureEnabled::<Impl, IMPL_OFFSET>, SetIsCursorCaptureEnabled::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGraphicsCaptureSession2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGraphicsCaptureSession3Impl: Sized {
    fn IsBorderRequired(&self) -> ::windows::core::Result<bool>;
    fn SetIsBorderRequired(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGraphicsCaptureSession3 {
    const NAME: &'static str = "Windows.Graphics.Capture.IGraphicsCaptureSession3";
}
#[cfg(feature = "implement_exclusive")]
impl IGraphicsCaptureSession3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGraphicsCaptureSession3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGraphicsCaptureSession3Vtbl {
        unsafe extern "system" fn IsBorderRequired<Impl: IGraphicsCaptureSession3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsBorderRequired<Impl: IGraphicsCaptureSession3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsBorderRequired(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGraphicsCaptureSession3>, ::windows::core::GetTrustLevel, IsBorderRequired::<Impl, IMPL_OFFSET>, SetIsBorderRequired::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGraphicsCaptureSession3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGraphicsCaptureSessionStaticsImpl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGraphicsCaptureSessionStatics {
    const NAME: &'static str = "Windows.Graphics.Capture.IGraphicsCaptureSessionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGraphicsCaptureSessionStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGraphicsCaptureSessionStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGraphicsCaptureSessionStaticsVtbl {
        unsafe extern "system" fn IsSupported<Impl: IGraphicsCaptureSessionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGraphicsCaptureSessionStatics>, ::windows::core::GetTrustLevel, IsSupported::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGraphicsCaptureSessionStatics as ::windows::core::Interface>::IID
    }
}
