#[cfg(feature = "implement_exclusive")]
pub trait IDirect3D11CaptureFrameImpl: Sized {
    fn Surface(&self) -> ::windows::core::Result<super::DirectX::Direct3D11::IDirect3DSurface>;
    fn SystemRelativeTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn ContentSize(&self) -> ::windows::core::Result<super::SizeInt32>;
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
pub trait IDirect3D11CaptureFramePoolStaticsImpl: Sized {
    fn Create(&self, device: &::core::option::Option<super::DirectX::Direct3D11::IDirect3DDevice>, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: &super::SizeInt32) -> ::windows::core::Result<Direct3D11CaptureFramePool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDirect3D11CaptureFramePoolStatics2Impl: Sized {
    fn CreateFreeThreaded(&self, device: &::core::option::Option<super::DirectX::Direct3D11::IDirect3DDevice>, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: &super::SizeInt32) -> ::windows::core::Result<Direct3D11CaptureFramePool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGraphicsCaptureAccessStaticsImpl: Sized {
    fn RequestAccessAsync(&self, request: GraphicsCaptureAccessKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessStatus>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGraphicsCaptureItemImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Size(&self) -> ::windows::core::Result<super::SizeInt32>;
    fn Closed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GraphicsCaptureItem, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGraphicsCaptureItemStaticsImpl: Sized {
    fn CreateFromVisual(&self, visual: &::core::option::Option<super::super::UI::Composition::Visual>) -> ::windows::core::Result<GraphicsCaptureItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGraphicsCaptureItemStatics2Impl: Sized {
    fn TryCreateFromWindowId(&self, windowid: &super::super::UI::WindowId) -> ::windows::core::Result<GraphicsCaptureItem>;
    fn TryCreateFromDisplayId(&self, displayid: &super::DisplayId) -> ::windows::core::Result<GraphicsCaptureItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGraphicsCapturePickerImpl: Sized {
    fn PickSingleItemAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GraphicsCaptureItem>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGraphicsCaptureSessionImpl: Sized {
    fn StartCapture(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGraphicsCaptureSession2Impl: Sized {
    fn IsCursorCaptureEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsCursorCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGraphicsCaptureSession3Impl: Sized {
    fn IsBorderRequired(&self) -> ::windows::core::Result<bool>;
    fn SetIsBorderRequired(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGraphicsCaptureSessionStaticsImpl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
}
