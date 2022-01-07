#[cfg(feature = "implement_exclusive")]
pub trait IDirect3D11CaptureFrameImpl: Sized {
    fn Surface(&self) -> ::windows::core::Result<super::DirectX::Direct3D11::IDirect3DSurface>;
    fn SystemRelativeTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn ContentSize(&self) -> ::windows::core::Result<super::SizeInt32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDirect3D11CaptureFrame {
    const NAME: &'static str = "Windows.Graphics.Capture.IDirect3D11CaptureFrame";
}
#[cfg(feature = "implement_exclusive")]
impl IDirect3D11CaptureFrameVtbl {
    pub const fn new<Impl: IDirect3D11CaptureFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirect3D11CaptureFrameVtbl {
        unsafe extern "system" fn Surface<Impl: IDirect3D11CaptureFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Surface() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemRelativeTime<Impl: IDirect3D11CaptureFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SystemRelativeTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentSize<Impl: IDirect3D11CaptureFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::SizeInt32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContentSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirect3D11CaptureFrame>, base.5, Surface::<Impl, OFFSET>, SystemRelativeTime::<Impl, OFFSET>, ContentSize::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDirect3D11CaptureFramePoolImpl: Sized {
    fn Recreate(&self, device: &::core::option::Option<super::DirectX::Direct3D11::IDirect3DDevice>, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: &super::SizeInt32) -> ::windows::core::Result<()>;
    fn TryGetNextFrame(&self) -> ::windows::core::Result<Direct3D11CaptureFrame>;
    fn FrameArrived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Direct3D11CaptureFramePool, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameArrived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CreateCaptureSession(&self, item: &::core::option::Option<GraphicsCaptureItem>) -> ::windows::core::Result<GraphicsCaptureSession>;
    fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::System::DispatcherQueue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDirect3D11CaptureFramePool {
    const NAME: &'static str = "Windows.Graphics.Capture.IDirect3D11CaptureFramePool";
}
#[cfg(feature = "implement_exclusive")]
impl IDirect3D11CaptureFramePoolVtbl {
    pub const fn new<Impl: IDirect3D11CaptureFramePoolImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirect3D11CaptureFramePoolVtbl {
        unsafe extern "system" fn Recreate<Impl: IDirect3D11CaptureFramePoolImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, device: ::windows::core::RawPtr, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Recreate(&*(&device as *const <super::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::Abi>::Abi as *const <super::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::DefaultType>::DefaultType), pixelformat, numberofbuffers, &*(&size as *const <super::SizeInt32 as ::windows::core::Abi>::Abi as *const <super::SizeInt32 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryGetNextFrame<Impl: IDirect3D11CaptureFramePoolImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetNextFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FrameArrived<Impl: IDirect3D11CaptureFramePoolImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameArrived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<Direct3D11CaptureFramePool, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<Direct3D11CaptureFramePool, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFrameArrived<Impl: IDirect3D11CaptureFramePoolImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveFrameArrived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CreateCaptureSession<Impl: IDirect3D11CaptureFramePoolImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCaptureSession(&*(&item as *const <GraphicsCaptureItem as ::windows::core::Abi>::Abi as *const <GraphicsCaptureItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DispatcherQueue<Impl: IDirect3D11CaptureFramePoolImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DispatcherQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirect3D11CaptureFramePool>, base.5, Recreate::<Impl, OFFSET>, TryGetNextFrame::<Impl, OFFSET>, FrameArrived::<Impl, OFFSET>, RemoveFrameArrived::<Impl, OFFSET>, CreateCaptureSession::<Impl, OFFSET>, DispatcherQueue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDirect3D11CaptureFramePoolStaticsImpl: Sized {
    fn Create(&self, device: &::core::option::Option<super::DirectX::Direct3D11::IDirect3DDevice>, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: &super::SizeInt32) -> ::windows::core::Result<Direct3D11CaptureFramePool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDirect3D11CaptureFramePoolStatics {
    const NAME: &'static str = "Windows.Graphics.Capture.IDirect3D11CaptureFramePoolStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDirect3D11CaptureFramePoolStaticsVtbl {
    pub const fn new<Impl: IDirect3D11CaptureFramePoolStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirect3D11CaptureFramePoolStaticsVtbl {
        unsafe extern "system" fn Create<Impl: IDirect3D11CaptureFramePoolStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, device: ::windows::core::RawPtr, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&device as *const <super::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::Abi>::Abi as *const <super::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::DefaultType>::DefaultType), pixelformat, numberofbuffers, &*(&size as *const <super::SizeInt32 as ::windows::core::Abi>::Abi as *const <super::SizeInt32 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirect3D11CaptureFramePoolStatics>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDirect3D11CaptureFramePoolStatics2Impl: Sized {
    fn CreateFreeThreaded(&self, device: &::core::option::Option<super::DirectX::Direct3D11::IDirect3DDevice>, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: &super::SizeInt32) -> ::windows::core::Result<Direct3D11CaptureFramePool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDirect3D11CaptureFramePoolStatics2 {
    const NAME: &'static str = "Windows.Graphics.Capture.IDirect3D11CaptureFramePoolStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IDirect3D11CaptureFramePoolStatics2Vtbl {
    pub const fn new<Impl: IDirect3D11CaptureFramePoolStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirect3D11CaptureFramePoolStatics2Vtbl {
        unsafe extern "system" fn CreateFreeThreaded<Impl: IDirect3D11CaptureFramePoolStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, device: ::windows::core::RawPtr, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFreeThreaded(&*(&device as *const <super::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::Abi>::Abi as *const <super::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::DefaultType>::DefaultType), pixelformat, numberofbuffers, &*(&size as *const <super::SizeInt32 as ::windows::core::Abi>::Abi as *const <super::SizeInt32 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirect3D11CaptureFramePoolStatics2>, base.5, CreateFreeThreaded::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGraphicsCaptureAccessStaticsImpl: Sized {
    fn RequestAccessAsync(&self, request: GraphicsCaptureAccessKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessStatus>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGraphicsCaptureAccessStatics {
    const NAME: &'static str = "Windows.Graphics.Capture.IGraphicsCaptureAccessStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGraphicsCaptureAccessStaticsVtbl {
    pub const fn new<Impl: IGraphicsCaptureAccessStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGraphicsCaptureAccessStaticsVtbl {
        unsafe extern "system" fn RequestAccessAsync<Impl: IGraphicsCaptureAccessStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, request: GraphicsCaptureAccessKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestAccessAsync(request) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGraphicsCaptureAccessStatics>, base.5, RequestAccessAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGraphicsCaptureItemImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Size(&self) -> ::windows::core::Result<super::SizeInt32>;
    fn Closed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GraphicsCaptureItem, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGraphicsCaptureItem {
    const NAME: &'static str = "Windows.Graphics.Capture.IGraphicsCaptureItem";
}
#[cfg(feature = "implement_exclusive")]
impl IGraphicsCaptureItemVtbl {
    pub const fn new<Impl: IGraphicsCaptureItemImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGraphicsCaptureItemVtbl {
        unsafe extern "system" fn DisplayName<Impl: IGraphicsCaptureItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Impl: IGraphicsCaptureItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::SizeInt32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Closed<Impl: IGraphicsCaptureItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Closed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<GraphicsCaptureItem, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<GraphicsCaptureItem, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Impl: IGraphicsCaptureItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGraphicsCaptureItem>, base.5, DisplayName::<Impl, OFFSET>, Size::<Impl, OFFSET>, Closed::<Impl, OFFSET>, RemoveClosed::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGraphicsCaptureItemStaticsImpl: Sized {
    fn CreateFromVisual(&self, visual: &::core::option::Option<super::super::UI::Composition::Visual>) -> ::windows::core::Result<GraphicsCaptureItem>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGraphicsCaptureItemStatics {
    const NAME: &'static str = "Windows.Graphics.Capture.IGraphicsCaptureItemStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGraphicsCaptureItemStaticsVtbl {
    pub const fn new<Impl: IGraphicsCaptureItemStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGraphicsCaptureItemStaticsVtbl {
        unsafe extern "system" fn CreateFromVisual<Impl: IGraphicsCaptureItemStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromVisual(&*(&visual as *const <super::super::UI::Composition::Visual as ::windows::core::Abi>::Abi as *const <super::super::UI::Composition::Visual as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGraphicsCaptureItemStatics>, base.5, CreateFromVisual::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGraphicsCaptureItemStatics2Impl: Sized {
    fn TryCreateFromWindowId(&self, windowid: &super::super::UI::WindowId) -> ::windows::core::Result<GraphicsCaptureItem>;
    fn TryCreateFromDisplayId(&self, displayid: &super::DisplayId) -> ::windows::core::Result<GraphicsCaptureItem>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGraphicsCaptureItemStatics2 {
    const NAME: &'static str = "Windows.Graphics.Capture.IGraphicsCaptureItemStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IGraphicsCaptureItemStatics2Vtbl {
    pub const fn new<Impl: IGraphicsCaptureItemStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGraphicsCaptureItemStatics2Vtbl {
        unsafe extern "system" fn TryCreateFromWindowId<Impl: IGraphicsCaptureItemStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, windowid: super::super::UI::WindowId, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryCreateFromWindowId(&*(&windowid as *const <super::super::UI::WindowId as ::windows::core::Abi>::Abi as *const <super::super::UI::WindowId as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryCreateFromDisplayId<Impl: IGraphicsCaptureItemStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, displayid: super::DisplayId, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryCreateFromDisplayId(&*(&displayid as *const <super::DisplayId as ::windows::core::Abi>::Abi as *const <super::DisplayId as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGraphicsCaptureItemStatics2>, base.5, TryCreateFromWindowId::<Impl, OFFSET>, TryCreateFromDisplayId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGraphicsCapturePickerImpl: Sized {
    fn PickSingleItemAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GraphicsCaptureItem>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGraphicsCapturePicker {
    const NAME: &'static str = "Windows.Graphics.Capture.IGraphicsCapturePicker";
}
#[cfg(feature = "implement_exclusive")]
impl IGraphicsCapturePickerVtbl {
    pub const fn new<Impl: IGraphicsCapturePickerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGraphicsCapturePickerVtbl {
        unsafe extern "system" fn PickSingleItemAsync<Impl: IGraphicsCapturePickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PickSingleItemAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGraphicsCapturePicker>, base.5, PickSingleItemAsync::<Impl, OFFSET>)
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
    pub const fn new<Impl: IGraphicsCaptureSessionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGraphicsCaptureSessionVtbl {
        unsafe extern "system" fn StartCapture<Impl: IGraphicsCaptureSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StartCapture().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGraphicsCaptureSession>, base.5, StartCapture::<Impl, OFFSET>)
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
    pub const fn new<Impl: IGraphicsCaptureSession2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGraphicsCaptureSession2Vtbl {
        unsafe extern "system" fn IsCursorCaptureEnabled<Impl: IGraphicsCaptureSession2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCursorCaptureEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsCursorCaptureEnabled<Impl: IGraphicsCaptureSession2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsCursorCaptureEnabled(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGraphicsCaptureSession2>, base.5, IsCursorCaptureEnabled::<Impl, OFFSET>, SetIsCursorCaptureEnabled::<Impl, OFFSET>)
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
    pub const fn new<Impl: IGraphicsCaptureSession3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGraphicsCaptureSession3Vtbl {
        unsafe extern "system" fn IsBorderRequired<Impl: IGraphicsCaptureSession3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsBorderRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsBorderRequired<Impl: IGraphicsCaptureSession3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsBorderRequired(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGraphicsCaptureSession3>, base.5, IsBorderRequired::<Impl, OFFSET>, SetIsBorderRequired::<Impl, OFFSET>)
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
    pub const fn new<Impl: IGraphicsCaptureSessionStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGraphicsCaptureSessionStaticsVtbl {
        unsafe extern "system" fn IsSupported<Impl: IGraphicsCaptureSessionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGraphicsCaptureSessionStatics>, base.5, IsSupported::<Impl, OFFSET>)
    }
}
