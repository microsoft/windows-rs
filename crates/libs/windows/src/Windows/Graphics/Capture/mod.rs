#[doc(hidden)]
#[repr(transparent)]
pub struct IDirect3D11CaptureFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDirect3D11CaptureFrame {
    type Vtable = IDirect3D11CaptureFrame_Vtbl;
}
impl ::core::clone::Clone for IDirect3D11CaptureFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDirect3D11CaptureFrame {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfa50c623_38da_4b32_acf3_fa9734ad800e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3D11CaptureFrame_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Surface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Surface: usize,
    #[cfg(feature = "Foundation")]
    pub SystemRelativeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemRelativeTime: usize,
    pub ContentSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::SizeInt32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDirect3D11CaptureFramePool(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDirect3D11CaptureFramePool {
    type Vtable = IDirect3D11CaptureFramePool_Vtbl;
}
impl ::core::clone::Clone for IDirect3D11CaptureFramePool {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDirect3D11CaptureFramePool {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x24eb6d22_1975_422e_82e7_780dbd8ddf24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3D11CaptureFramePool_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Recreate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Recreate: usize,
    pub TryGetNextFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FrameArrived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFrameArrived: usize,
    pub CreateCaptureSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub DispatcherQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    DispatcherQueue: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDirect3D11CaptureFramePoolStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDirect3D11CaptureFramePoolStatics {
    type Vtable = IDirect3D11CaptureFramePoolStatics_Vtbl;
}
impl ::core::clone::Clone for IDirect3D11CaptureFramePoolStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDirect3D11CaptureFramePoolStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7784056a_67aa_4d53_ae54_1088d5a8ca21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3D11CaptureFramePoolStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDirect3D11CaptureFramePoolStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDirect3D11CaptureFramePoolStatics2 {
    type Vtable = IDirect3D11CaptureFramePoolStatics2_Vtbl;
}
impl ::core::clone::Clone for IDirect3D11CaptureFramePoolStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDirect3D11CaptureFramePoolStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x589b103f_6bbc_5df5_a991_02e28b3b66d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3D11CaptureFramePoolStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateFreeThreaded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateFreeThreaded: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGraphicsCaptureAccessStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGraphicsCaptureAccessStatics {
    type Vtable = IGraphicsCaptureAccessStatics_Vtbl;
}
impl ::core::clone::Clone for IGraphicsCaptureAccessStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGraphicsCaptureAccessStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x743ed370_06ec_5040_a58a_901f0f757095);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureAccessStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Security_Authorization_AppCapabilityAccess"))]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: GraphicsCaptureAccessKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Authorization_AppCapabilityAccess")))]
    RequestAccessAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGraphicsCaptureItem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGraphicsCaptureItem {
    type Vtable = IGraphicsCaptureItem_Vtbl;
}
impl ::core::clone::Clone for IGraphicsCaptureItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGraphicsCaptureItem {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79c3f95b_31f7_4ec2_a464_632ef5d30760);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureItem_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::SizeInt32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGraphicsCaptureItemStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGraphicsCaptureItemStatics {
    type Vtable = IGraphicsCaptureItemStatics_Vtbl;
}
impl ::core::clone::Clone for IGraphicsCaptureItemStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGraphicsCaptureItemStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa87ebea5_457c_5788_ab47_0cf1d3637e74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureItemStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Composition")]
    pub CreateFromVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateFromVisual: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGraphicsCaptureItemStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGraphicsCaptureItemStatics2 {
    type Vtable = IGraphicsCaptureItemStatics2_Vtbl;
}
impl ::core::clone::Clone for IGraphicsCaptureItemStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGraphicsCaptureItemStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3b92acc9_e584_5862_bf5c_9c316c6d2dbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureItemStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI")]
    pub TryCreateFromWindowId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windowid: super::super::UI::WindowId, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    TryCreateFromWindowId: usize,
    pub TryCreateFromDisplayId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayid: super::DisplayId, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGraphicsCapturePicker(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGraphicsCapturePicker {
    type Vtable = IGraphicsCapturePicker_Vtbl;
}
impl ::core::clone::Clone for IGraphicsCapturePicker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGraphicsCapturePicker {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5a1711b3_ad79_4b4a_9336_1318fdde3539);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCapturePicker_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PickSingleItemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickSingleItemAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGraphicsCaptureSession(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGraphicsCaptureSession {
    type Vtable = IGraphicsCaptureSession_Vtbl;
}
impl ::core::clone::Clone for IGraphicsCaptureSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGraphicsCaptureSession {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x814e42a9_f70f_4ad7_939b_fddcc6eb880d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureSession_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub StartCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGraphicsCaptureSession2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGraphicsCaptureSession2 {
    type Vtable = IGraphicsCaptureSession2_Vtbl;
}
impl ::core::clone::Clone for IGraphicsCaptureSession2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGraphicsCaptureSession2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c39ae40_7d2e_5044_804e_8b6799d4cf9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureSession2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsCursorCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsCursorCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGraphicsCaptureSession3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGraphicsCaptureSession3 {
    type Vtable = IGraphicsCaptureSession3_Vtbl;
}
impl ::core::clone::Clone for IGraphicsCaptureSession3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGraphicsCaptureSession3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf2cdd966_22ae_5ea1_9596_3a289344c3be);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureSession3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsBorderRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsBorderRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGraphicsCaptureSessionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGraphicsCaptureSessionStatics {
    type Vtable = IGraphicsCaptureSessionStatics_Vtbl;
}
impl ::core::clone::Clone for IGraphicsCaptureSessionStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGraphicsCaptureSessionStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2224a540_5974_49aa_b232_0882536f4cb5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureSessionStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Graphics_Capture\"`*"]
#[repr(transparent)]
pub struct Direct3D11CaptureFrame(::windows_core::IUnknown);
impl Direct3D11CaptureFrame {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Surface(&self) -> ::windows_core::Result<super::DirectX::Direct3D11::IDirect3DSurface> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Surface)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SystemRelativeTime(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemRelativeTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ContentSize(&self) -> ::windows_core::Result<super::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentSize)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for Direct3D11CaptureFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Direct3D11CaptureFrame {}
impl ::core::fmt::Debug for Direct3D11CaptureFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Direct3D11CaptureFrame").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for Direct3D11CaptureFrame {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Capture.Direct3D11CaptureFrame;{fa50c623-38da-4b32-acf3-fa9734ad800e})");
}
impl ::core::clone::Clone for Direct3D11CaptureFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for Direct3D11CaptureFrame {
    type Vtable = IDirect3D11CaptureFrame_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Direct3D11CaptureFrame {
    const IID: ::windows_core::GUID = <IDirect3D11CaptureFrame as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Direct3D11CaptureFrame {
    const NAME: &'static str = "Windows.Graphics.Capture.Direct3D11CaptureFrame";
}
::windows_core::imp::interface_hierarchy!(Direct3D11CaptureFrame, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for Direct3D11CaptureFrame {}
unsafe impl ::core::marker::Send for Direct3D11CaptureFrame {}
unsafe impl ::core::marker::Sync for Direct3D11CaptureFrame {}
#[doc = "*Required features: `\"Graphics_Capture\"`*"]
#[repr(transparent)]
pub struct Direct3D11CaptureFramePool(::windows_core::IUnknown);
impl Direct3D11CaptureFramePool {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Recreate<P0>(&self, device: P0, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DirectX::Direct3D11::IDirect3DDevice>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Recreate)(::windows_core::Interface::as_raw(this), device.try_into_param()?.abi(), pixelformat, numberofbuffers, size).ok() }
    }
    pub fn TryGetNextFrame(&self) -> ::windows_core::Result<Direct3D11CaptureFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetNextFrame)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FrameArrived<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<Direct3D11CaptureFramePool, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameArrived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFrameArrived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFrameArrived)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CreateCaptureSession<P0>(&self, item: P0) -> ::windows_core::Result<GraphicsCaptureSession>
    where
        P0: ::windows_core::IntoParam<GraphicsCaptureItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateCaptureSession)(::windows_core::Interface::as_raw(this), item.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::super::System::DispatcherQueue> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Create<P0>(device: P0, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32) -> ::windows_core::Result<Direct3D11CaptureFramePool>
    where
        P0: ::windows_core::TryIntoParam<super::DirectX::Direct3D11::IDirect3DDevice>,
    {
        Self::IDirect3D11CaptureFramePoolStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), device.try_into_param()?.abi(), pixelformat, numberofbuffers, size, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CreateFreeThreaded<P0>(device: P0, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32) -> ::windows_core::Result<Direct3D11CaptureFramePool>
    where
        P0: ::windows_core::TryIntoParam<super::DirectX::Direct3D11::IDirect3DDevice>,
    {
        Self::IDirect3D11CaptureFramePoolStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFreeThreaded)(::windows_core::Interface::as_raw(this), device.try_into_param()?.abi(), pixelformat, numberofbuffers, size, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDirect3D11CaptureFramePoolStatics<R, F: FnOnce(&IDirect3D11CaptureFramePoolStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Direct3D11CaptureFramePool, IDirect3D11CaptureFramePoolStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IDirect3D11CaptureFramePoolStatics2<R, F: FnOnce(&IDirect3D11CaptureFramePoolStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Direct3D11CaptureFramePool, IDirect3D11CaptureFramePoolStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for Direct3D11CaptureFramePool {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Direct3D11CaptureFramePool {}
impl ::core::fmt::Debug for Direct3D11CaptureFramePool {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Direct3D11CaptureFramePool").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for Direct3D11CaptureFramePool {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Capture.Direct3D11CaptureFramePool;{24eb6d22-1975-422e-82e7-780dbd8ddf24})");
}
impl ::core::clone::Clone for Direct3D11CaptureFramePool {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for Direct3D11CaptureFramePool {
    type Vtable = IDirect3D11CaptureFramePool_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Direct3D11CaptureFramePool {
    const IID: ::windows_core::GUID = <IDirect3D11CaptureFramePool as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Direct3D11CaptureFramePool {
    const NAME: &'static str = "Windows.Graphics.Capture.Direct3D11CaptureFramePool";
}
::windows_core::imp::interface_hierarchy!(Direct3D11CaptureFramePool, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for Direct3D11CaptureFramePool {}
unsafe impl ::core::marker::Send for Direct3D11CaptureFramePool {}
unsafe impl ::core::marker::Sync for Direct3D11CaptureFramePool {}
#[doc = "*Required features: `\"Graphics_Capture\"`*"]
pub struct GraphicsCaptureAccess;
impl GraphicsCaptureAccess {
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Authorization_AppCapabilityAccess\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Authorization_AppCapabilityAccess"))]
    pub fn RequestAccessAsync(request: GraphicsCaptureAccessKind) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessStatus>> {
        Self::IGraphicsCaptureAccessStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessAsync)(::windows_core::Interface::as_raw(this), request, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGraphicsCaptureAccessStatics<R, F: FnOnce(&IGraphicsCaptureAccessStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GraphicsCaptureAccess, IGraphicsCaptureAccessStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for GraphicsCaptureAccess {
    const NAME: &'static str = "Windows.Graphics.Capture.GraphicsCaptureAccess";
}
#[doc = "*Required features: `\"Graphics_Capture\"`*"]
#[repr(transparent)]
pub struct GraphicsCaptureItem(::windows_core::IUnknown);
impl GraphicsCaptureItem {
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Size(&self) -> ::windows_core::Result<super::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<GraphicsCaptureItem, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Closed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveClosed)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn CreateFromVisual<P0>(visual: P0) -> ::windows_core::Result<GraphicsCaptureItem>
    where
        P0: ::windows_core::TryIntoParam<super::super::UI::Composition::Visual>,
    {
        Self::IGraphicsCaptureItemStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromVisual)(::windows_core::Interface::as_raw(this), visual.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn TryCreateFromWindowId(windowid: super::super::UI::WindowId) -> ::windows_core::Result<GraphicsCaptureItem> {
        Self::IGraphicsCaptureItemStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateFromWindowId)(::windows_core::Interface::as_raw(this), windowid, &mut result__).from_abi(result__)
        })
    }
    pub fn TryCreateFromDisplayId(displayid: super::DisplayId) -> ::windows_core::Result<GraphicsCaptureItem> {
        Self::IGraphicsCaptureItemStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateFromDisplayId)(::windows_core::Interface::as_raw(this), displayid, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGraphicsCaptureItemStatics<R, F: FnOnce(&IGraphicsCaptureItemStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GraphicsCaptureItem, IGraphicsCaptureItemStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGraphicsCaptureItemStatics2<R, F: FnOnce(&IGraphicsCaptureItemStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GraphicsCaptureItem, IGraphicsCaptureItemStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for GraphicsCaptureItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GraphicsCaptureItem {}
impl ::core::fmt::Debug for GraphicsCaptureItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GraphicsCaptureItem").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GraphicsCaptureItem {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Capture.GraphicsCaptureItem;{79c3f95b-31f7-4ec2-a464-632ef5d30760})");
}
impl ::core::clone::Clone for GraphicsCaptureItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for GraphicsCaptureItem {
    type Vtable = IGraphicsCaptureItem_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GraphicsCaptureItem {
    const IID: ::windows_core::GUID = <IGraphicsCaptureItem as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GraphicsCaptureItem {
    const NAME: &'static str = "Windows.Graphics.Capture.GraphicsCaptureItem";
}
::windows_core::imp::interface_hierarchy!(GraphicsCaptureItem, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GraphicsCaptureItem {}
unsafe impl ::core::marker::Sync for GraphicsCaptureItem {}
#[doc = "*Required features: `\"Graphics_Capture\"`*"]
#[repr(transparent)]
pub struct GraphicsCapturePicker(::windows_core::IUnknown);
impl GraphicsCapturePicker {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GraphicsCapturePicker, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PickSingleItemAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<GraphicsCaptureItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PickSingleItemAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GraphicsCapturePicker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GraphicsCapturePicker {}
impl ::core::fmt::Debug for GraphicsCapturePicker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GraphicsCapturePicker").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GraphicsCapturePicker {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Capture.GraphicsCapturePicker;{5a1711b3-ad79-4b4a-9336-1318fdde3539})");
}
impl ::core::clone::Clone for GraphicsCapturePicker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for GraphicsCapturePicker {
    type Vtable = IGraphicsCapturePicker_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GraphicsCapturePicker {
    const IID: ::windows_core::GUID = <IGraphicsCapturePicker as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GraphicsCapturePicker {
    const NAME: &'static str = "Windows.Graphics.Capture.GraphicsCapturePicker";
}
::windows_core::imp::interface_hierarchy!(GraphicsCapturePicker, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GraphicsCapturePicker {}
unsafe impl ::core::marker::Sync for GraphicsCapturePicker {}
#[doc = "*Required features: `\"Graphics_Capture\"`*"]
#[repr(transparent)]
pub struct GraphicsCaptureSession(::windows_core::IUnknown);
impl GraphicsCaptureSession {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn StartCapture(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StartCapture)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn IsCursorCaptureEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IGraphicsCaptureSession2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsCursorCaptureEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsCursorCaptureEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IGraphicsCaptureSession2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsCursorCaptureEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsBorderRequired(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IGraphicsCaptureSession3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsBorderRequired)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsBorderRequired(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IGraphicsCaptureSession3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsBorderRequired)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsSupported() -> ::windows_core::Result<bool> {
        Self::IGraphicsCaptureSessionStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGraphicsCaptureSessionStatics<R, F: FnOnce(&IGraphicsCaptureSessionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GraphicsCaptureSession, IGraphicsCaptureSessionStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for GraphicsCaptureSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GraphicsCaptureSession {}
impl ::core::fmt::Debug for GraphicsCaptureSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GraphicsCaptureSession").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GraphicsCaptureSession {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Capture.GraphicsCaptureSession;{814e42a9-f70f-4ad7-939b-fddcc6eb880d})");
}
impl ::core::clone::Clone for GraphicsCaptureSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for GraphicsCaptureSession {
    type Vtable = IGraphicsCaptureSession_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GraphicsCaptureSession {
    const IID: ::windows_core::GUID = <IGraphicsCaptureSession as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GraphicsCaptureSession {
    const NAME: &'static str = "Windows.Graphics.Capture.GraphicsCaptureSession";
}
::windows_core::imp::interface_hierarchy!(GraphicsCaptureSession, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for GraphicsCaptureSession {}
unsafe impl ::core::marker::Send for GraphicsCaptureSession {}
unsafe impl ::core::marker::Sync for GraphicsCaptureSession {}
#[doc = "*Required features: `\"Graphics_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GraphicsCaptureAccessKind(pub i32);
impl GraphicsCaptureAccessKind {
    pub const Borderless: Self = Self(0i32);
    pub const Programmatic: Self = Self(1i32);
}
impl ::core::marker::Copy for GraphicsCaptureAccessKind {}
impl ::core::clone::Clone for GraphicsCaptureAccessKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GraphicsCaptureAccessKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GraphicsCaptureAccessKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GraphicsCaptureAccessKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GraphicsCaptureAccessKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GraphicsCaptureAccessKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Capture.GraphicsCaptureAccessKind;i4)");
}
