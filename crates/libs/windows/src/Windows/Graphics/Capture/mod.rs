#[doc(hidden)]
#[repr(transparent)]
pub struct IDirect3D11CaptureFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDirect3D11CaptureFrame {
    type Vtable = IDirect3D11CaptureFrame_Vtbl;
}
unsafe impl ::windows::core::Interface for IDirect3D11CaptureFrame {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa50c623_38da_4b32_acf3_fa9734ad800e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3D11CaptureFrame_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Surface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Surface: usize,
    #[cfg(feature = "Foundation")]
    pub SystemRelativeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemRelativeTime: usize,
    pub ContentSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::SizeInt32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDirect3D11CaptureFramePool(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDirect3D11CaptureFramePool {
    type Vtable = IDirect3D11CaptureFramePool_Vtbl;
}
unsafe impl ::windows::core::Interface for IDirect3D11CaptureFramePool {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24eb6d22_1975_422e_82e7_780dbd8ddf24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3D11CaptureFramePool_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Recreate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Recreate: usize,
    pub TryGetNextFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FrameArrived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFrameArrived: usize,
    pub CreateCaptureSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub DispatcherQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    DispatcherQueue: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDirect3D11CaptureFramePoolStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDirect3D11CaptureFramePoolStatics {
    type Vtable = IDirect3D11CaptureFramePoolStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IDirect3D11CaptureFramePoolStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7784056a_67aa_4d53_ae54_1088d5a8ca21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3D11CaptureFramePoolStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDirect3D11CaptureFramePoolStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDirect3D11CaptureFramePoolStatics2 {
    type Vtable = IDirect3D11CaptureFramePoolStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IDirect3D11CaptureFramePoolStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x589b103f_6bbc_5df5_a991_02e28b3b66d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3D11CaptureFramePoolStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateFreeThreaded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateFreeThreaded: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGraphicsCaptureAccessStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGraphicsCaptureAccessStatics {
    type Vtable = IGraphicsCaptureAccessStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IGraphicsCaptureAccessStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x743ed370_06ec_5040_a58a_901f0f757095);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureAccessStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Security_Authorization_AppCapabilityAccess"))]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: GraphicsCaptureAccessKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Authorization_AppCapabilityAccess")))]
    RequestAccessAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGraphicsCaptureItem(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGraphicsCaptureItem {
    type Vtable = IGraphicsCaptureItem_Vtbl;
}
unsafe impl ::windows::core::Interface for IGraphicsCaptureItem {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79c3f95b_31f7_4ec2_a464_632ef5d30760);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureItem_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::SizeInt32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGraphicsCaptureItemStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGraphicsCaptureItemStatics {
    type Vtable = IGraphicsCaptureItemStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IGraphicsCaptureItemStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa87ebea5_457c_5788_ab47_0cf1d3637e74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureItemStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Composition")]
    pub CreateFromVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateFromVisual: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGraphicsCaptureItemStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGraphicsCaptureItemStatics2 {
    type Vtable = IGraphicsCaptureItemStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IGraphicsCaptureItemStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b92acc9_e584_5862_bf5c_9c316c6d2dbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureItemStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "UI")]
    pub TryCreateFromWindowId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windowid: super::super::UI::WindowId, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    TryCreateFromWindowId: usize,
    pub TryCreateFromDisplayId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayid: super::DisplayId, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGraphicsCapturePicker(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGraphicsCapturePicker {
    type Vtable = IGraphicsCapturePicker_Vtbl;
}
unsafe impl ::windows::core::Interface for IGraphicsCapturePicker {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a1711b3_ad79_4b4a_9336_1318fdde3539);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCapturePicker_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PickSingleItemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickSingleItemAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGraphicsCaptureSession(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGraphicsCaptureSession {
    type Vtable = IGraphicsCaptureSession_Vtbl;
}
unsafe impl ::windows::core::Interface for IGraphicsCaptureSession {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x814e42a9_f70f_4ad7_939b_fddcc6eb880d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureSession_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub StartCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGraphicsCaptureSession2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGraphicsCaptureSession2 {
    type Vtable = IGraphicsCaptureSession2_Vtbl;
}
unsafe impl ::windows::core::Interface for IGraphicsCaptureSession2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c39ae40_7d2e_5044_804e_8b6799d4cf9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureSession2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsCursorCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsCursorCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGraphicsCaptureSession3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGraphicsCaptureSession3 {
    type Vtable = IGraphicsCaptureSession3_Vtbl;
}
unsafe impl ::windows::core::Interface for IGraphicsCaptureSession3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2cdd966_22ae_5ea1_9596_3a289344c3be);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureSession3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsBorderRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsBorderRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGraphicsCaptureSessionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGraphicsCaptureSessionStatics {
    type Vtable = IGraphicsCaptureSessionStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IGraphicsCaptureSessionStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2224a540_5974_49aa_b232_0882536f4cb5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureSessionStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Graphics_Capture\"`*"]
#[repr(transparent)]
pub struct Direct3D11CaptureFrame(::windows::core::IUnknown);
impl Direct3D11CaptureFrame {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Surface(&self) -> ::windows::core::Result<super::DirectX::Direct3D11::IDirect3DSurface> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Surface)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SystemRelativeTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SystemRelativeTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ContentSize(&self) -> ::windows::core::Result<super::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentSize)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for Direct3D11CaptureFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for Direct3D11CaptureFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Capture.Direct3D11CaptureFrame;{fa50c623-38da-4b32-acf3-fa9734ad800e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for Direct3D11CaptureFrame {
    type Vtable = IDirect3D11CaptureFrame_Vtbl;
}
unsafe impl ::windows::core::Interface for Direct3D11CaptureFrame {
    const IID: ::windows::core::GUID = <IDirect3D11CaptureFrame as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Direct3D11CaptureFrame {
    const NAME: &'static str = "Windows.Graphics.Capture.Direct3D11CaptureFrame";
}
::windows::core::interface_hierarchy!(Direct3D11CaptureFrame, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<Direct3D11CaptureFrame> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: Direct3D11CaptureFrame) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&Direct3D11CaptureFrame> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &Direct3D11CaptureFrame) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&Direct3D11CaptureFrame> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &Direct3D11CaptureFrame) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for Direct3D11CaptureFrame {}
unsafe impl ::core::marker::Sync for Direct3D11CaptureFrame {}
#[doc = "*Required features: `\"Graphics_Capture\"`*"]
#[repr(transparent)]
pub struct Direct3D11CaptureFramePool(::windows::core::IUnknown);
impl Direct3D11CaptureFramePool {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Recreate<P0, E0>(&self, device: P0, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::DirectX::Direct3D11::IDirect3DDevice>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Recreate)(::windows::core::Vtable::as_raw(this), device.try_into().map_err(|e| e.into())?.abi(), pixelformat, numberofbuffers, size).ok() }
    }
    pub fn TryGetNextFrame(&self) -> ::windows::core::Result<Direct3D11CaptureFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetNextFrame)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FrameArrived(&self, handler: &super::super::Foundation::TypedEventHandler<Direct3D11CaptureFramePool, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FrameArrived)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFrameArrived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveFrameArrived)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn CreateCaptureSession(&self, item: &GraphicsCaptureItem) -> ::windows::core::Result<GraphicsCaptureSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateCaptureSession)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(item), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::System::DispatcherQueue> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Create<P0, E0>(device: P0, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32) -> ::windows::core::Result<Direct3D11CaptureFramePool>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::DirectX::Direct3D11::IDirect3DDevice>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IDirect3D11CaptureFramePoolStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), device.try_into().map_err(|e| e.into())?.abi(), pixelformat, numberofbuffers, size, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CreateFreeThreaded<P0, E0>(device: P0, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32) -> ::windows::core::Result<Direct3D11CaptureFramePool>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::DirectX::Direct3D11::IDirect3DDevice>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IDirect3D11CaptureFramePoolStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFreeThreaded)(::windows::core::Vtable::as_raw(this), device.try_into().map_err(|e| e.into())?.abi(), pixelformat, numberofbuffers, size, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDirect3D11CaptureFramePoolStatics<R, F: FnOnce(&IDirect3D11CaptureFramePoolStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Direct3D11CaptureFramePool, IDirect3D11CaptureFramePoolStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IDirect3D11CaptureFramePoolStatics2<R, F: FnOnce(&IDirect3D11CaptureFramePoolStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Direct3D11CaptureFramePool, IDirect3D11CaptureFramePoolStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for Direct3D11CaptureFramePool {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for Direct3D11CaptureFramePool {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Capture.Direct3D11CaptureFramePool;{24eb6d22-1975-422e-82e7-780dbd8ddf24})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for Direct3D11CaptureFramePool {
    type Vtable = IDirect3D11CaptureFramePool_Vtbl;
}
unsafe impl ::windows::core::Interface for Direct3D11CaptureFramePool {
    const IID: ::windows::core::GUID = <IDirect3D11CaptureFramePool as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Direct3D11CaptureFramePool {
    const NAME: &'static str = "Windows.Graphics.Capture.Direct3D11CaptureFramePool";
}
::windows::core::interface_hierarchy!(Direct3D11CaptureFramePool, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<Direct3D11CaptureFramePool> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: Direct3D11CaptureFramePool) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&Direct3D11CaptureFramePool> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &Direct3D11CaptureFramePool) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&Direct3D11CaptureFramePool> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &Direct3D11CaptureFramePool) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for Direct3D11CaptureFramePool {}
unsafe impl ::core::marker::Sync for Direct3D11CaptureFramePool {}
#[doc = "*Required features: `\"Graphics_Capture\"`*"]
pub struct GraphicsCaptureAccess;
impl GraphicsCaptureAccess {
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Authorization_AppCapabilityAccess\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Authorization_AppCapabilityAccess"))]
    pub fn RequestAccessAsync(request: GraphicsCaptureAccessKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessStatus>> {
        Self::IGraphicsCaptureAccessStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestAccessAsync)(::windows::core::Vtable::as_raw(this), request, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGraphicsCaptureAccessStatics<R, F: FnOnce(&IGraphicsCaptureAccessStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<GraphicsCaptureAccess, IGraphicsCaptureAccessStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for GraphicsCaptureAccess {
    const NAME: &'static str = "Windows.Graphics.Capture.GraphicsCaptureAccess";
}
#[doc = "*Required features: `\"Graphics_Capture\"`*"]
#[repr(transparent)]
pub struct GraphicsCaptureItem(::windows::core::IUnknown);
impl GraphicsCaptureItem {
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Size(&self) -> ::windows::core::Result<super::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed(&self, handler: &super::super::Foundation::TypedEventHandler<GraphicsCaptureItem, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Closed)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveClosed)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn CreateFromVisual<P0>(visual: P0) -> ::windows::core::Result<GraphicsCaptureItem>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::UI::Composition::Visual>>,
    {
        Self::IGraphicsCaptureItemStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromVisual)(::windows::core::Vtable::as_raw(this), visual.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn TryCreateFromWindowId(windowid: super::super::UI::WindowId) -> ::windows::core::Result<GraphicsCaptureItem> {
        Self::IGraphicsCaptureItemStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryCreateFromWindowId)(::windows::core::Vtable::as_raw(this), windowid, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TryCreateFromDisplayId(displayid: super::DisplayId) -> ::windows::core::Result<GraphicsCaptureItem> {
        Self::IGraphicsCaptureItemStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryCreateFromDisplayId)(::windows::core::Vtable::as_raw(this), displayid, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGraphicsCaptureItemStatics<R, F: FnOnce(&IGraphicsCaptureItemStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<GraphicsCaptureItem, IGraphicsCaptureItemStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGraphicsCaptureItemStatics2<R, F: FnOnce(&IGraphicsCaptureItemStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<GraphicsCaptureItem, IGraphicsCaptureItemStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for GraphicsCaptureItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for GraphicsCaptureItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Capture.GraphicsCaptureItem;{79c3f95b-31f7-4ec2-a464-632ef5d30760})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GraphicsCaptureItem {
    type Vtable = IGraphicsCaptureItem_Vtbl;
}
unsafe impl ::windows::core::Interface for GraphicsCaptureItem {
    const IID: ::windows::core::GUID = <IGraphicsCaptureItem as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GraphicsCaptureItem {
    const NAME: &'static str = "Windows.Graphics.Capture.GraphicsCaptureItem";
}
::windows::core::interface_hierarchy!(GraphicsCaptureItem, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GraphicsCaptureItem {}
unsafe impl ::core::marker::Sync for GraphicsCaptureItem {}
#[doc = "*Required features: `\"Graphics_Capture\"`*"]
#[repr(transparent)]
pub struct GraphicsCapturePicker(::windows::core::IUnknown);
impl GraphicsCapturePicker {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<GraphicsCapturePicker, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PickSingleItemAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GraphicsCaptureItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PickSingleItemAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for GraphicsCapturePicker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for GraphicsCapturePicker {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Capture.GraphicsCapturePicker;{5a1711b3-ad79-4b4a-9336-1318fdde3539})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GraphicsCapturePicker {
    type Vtable = IGraphicsCapturePicker_Vtbl;
}
unsafe impl ::windows::core::Interface for GraphicsCapturePicker {
    const IID: ::windows::core::GUID = <IGraphicsCapturePicker as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GraphicsCapturePicker {
    const NAME: &'static str = "Windows.Graphics.Capture.GraphicsCapturePicker";
}
::windows::core::interface_hierarchy!(GraphicsCapturePicker, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GraphicsCapturePicker {}
unsafe impl ::core::marker::Sync for GraphicsCapturePicker {}
#[doc = "*Required features: `\"Graphics_Capture\"`*"]
#[repr(transparent)]
pub struct GraphicsCaptureSession(::windows::core::IUnknown);
impl GraphicsCaptureSession {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn StartCapture(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).StartCapture)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn IsCursorCaptureEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IGraphicsCaptureSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCursorCaptureEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsCursorCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGraphicsCaptureSession2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsCursorCaptureEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsBorderRequired(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IGraphicsCaptureSession3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsBorderRequired)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsBorderRequired(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGraphicsCaptureSession3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsBorderRequired)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IGraphicsCaptureSessionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGraphicsCaptureSessionStatics<R, F: FnOnce(&IGraphicsCaptureSessionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<GraphicsCaptureSession, IGraphicsCaptureSessionStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for GraphicsCaptureSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for GraphicsCaptureSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Capture.GraphicsCaptureSession;{814e42a9-f70f-4ad7-939b-fddcc6eb880d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GraphicsCaptureSession {
    type Vtable = IGraphicsCaptureSession_Vtbl;
}
unsafe impl ::windows::core::Interface for GraphicsCaptureSession {
    const IID: ::windows::core::GUID = <IGraphicsCaptureSession as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GraphicsCaptureSession {
    const NAME: &'static str = "Windows.Graphics.Capture.GraphicsCaptureSession";
}
::windows::core::interface_hierarchy!(GraphicsCaptureSession, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<GraphicsCaptureSession> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: GraphicsCaptureSession) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&GraphicsCaptureSession> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &GraphicsCaptureSession) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&GraphicsCaptureSession> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &GraphicsCaptureSession) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
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
unsafe impl ::windows::core::Abi for GraphicsCaptureAccessKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for GraphicsCaptureAccessKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GraphicsCaptureAccessKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GraphicsCaptureAccessKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Capture.GraphicsCaptureAccessKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
