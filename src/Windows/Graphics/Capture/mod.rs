#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Graphics_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Direct3D11CaptureFrame(::windows::runtime::IInspectable);
impl Direct3D11CaptureFrame {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Capture`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    #[doc = "*Required features: `Graphics_Capture`, `Graphics_DirectX_Direct3D11`*"]
    pub fn Surface(&self) -> ::windows::runtime::Result<super::DirectX::Direct3D11::IDirect3DSurface> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::DirectX::Direct3D11::IDirect3DSurface>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Capture`, `Foundation`*"]
    pub fn SystemRelativeTime(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Capture`*"]
    pub fn ContentSize(&self) -> ::windows::runtime::Result<super::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__: super::SizeInt32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::SizeInt32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Direct3D11CaptureFrame {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Capture.Direct3D11CaptureFrame;{fa50c623-38da-4b32-acf3-fa9734ad800e})");
}
unsafe impl ::windows::runtime::Interface for Direct3D11CaptureFrame {
    type Vtable = IDirect3D11CaptureFrame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4199597603, 14554, 19250, [172, 243, 250, 151, 52, 173, 128, 14]);
}
impl ::windows::runtime::RuntimeName for Direct3D11CaptureFrame {
    const NAME: &'static str = "Windows.Graphics.Capture.Direct3D11CaptureFrame";
}
impl ::std::convert::From<Direct3D11CaptureFrame> for ::windows::runtime::IUnknown {
    fn from(value: Direct3D11CaptureFrame) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Direct3D11CaptureFrame> for ::windows::runtime::IUnknown {
    fn from(value: &Direct3D11CaptureFrame) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Direct3D11CaptureFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Direct3D11CaptureFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Direct3D11CaptureFrame> for ::windows::runtime::IInspectable {
    fn from(value: Direct3D11CaptureFrame) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Direct3D11CaptureFrame> for ::windows::runtime::IInspectable {
    fn from(value: &Direct3D11CaptureFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Direct3D11CaptureFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Direct3D11CaptureFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<Direct3D11CaptureFrame> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: Direct3D11CaptureFrame) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&Direct3D11CaptureFrame> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &Direct3D11CaptureFrame) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for Direct3D11CaptureFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &Direct3D11CaptureFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for Direct3D11CaptureFrame {}
unsafe impl ::std::marker::Sync for Direct3D11CaptureFrame {}
#[doc = "*Required features: `Graphics_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Direct3D11CaptureFramePool(::windows::runtime::IInspectable);
impl Direct3D11CaptureFramePool {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Capture`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11"))]
    #[doc = "*Required features: `Graphics_Capture`, `Graphics_DirectX`, `Graphics_DirectX_Direct3D11`*"]
    pub fn Recreate<'a, Param0: ::windows::runtime::IntoParam<'a, super::DirectX::Direct3D11::IDirect3DDevice>, Param3: ::windows::runtime::IntoParam<'a, super::SizeInt32>>(&self, device: Param0, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: Param3) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), device.into_param().abi(), pixelformat, numberofbuffers, size.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Capture`*"]
    pub fn TryGetNextFrame(&self) -> ::windows::runtime::Result<Direct3D11CaptureFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Direct3D11CaptureFrame>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Capture`, `Foundation`*"]
    pub fn FrameArrived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<Direct3D11CaptureFramePool, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Capture`, `Foundation`*"]
    pub fn RemoveFrameArrived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Capture`*"]
    pub fn CreateCaptureSession<'a, Param0: ::windows::runtime::IntoParam<'a, GraphicsCaptureItem>>(&self, item: Param0) -> ::windows::runtime::Result<GraphicsCaptureSession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), item.into_param().abi(), &mut result__).from_abi::<GraphicsCaptureSession>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Graphics_Capture`, `System`*"]
    pub fn DispatcherQueue(&self) -> ::windows::runtime::Result<super::super::System::DispatcherQueue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::DispatcherQueue>(result__)
        }
    }
    #[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11"))]
    #[doc = "*Required features: `Graphics_Capture`, `Graphics_DirectX`, `Graphics_DirectX_Direct3D11`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::DirectX::Direct3D11::IDirect3DDevice>, Param3: ::windows::runtime::IntoParam<'a, super::SizeInt32>>(device: Param0, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: Param3) -> ::windows::runtime::Result<Direct3D11CaptureFramePool> {
        Self::IDirect3D11CaptureFramePoolStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), device.into_param().abi(), pixelformat, numberofbuffers, size.into_param().abi(), &mut result__).from_abi::<Direct3D11CaptureFramePool>(result__)
        })
    }
    #[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11"))]
    #[doc = "*Required features: `Graphics_Capture`, `Graphics_DirectX`, `Graphics_DirectX_Direct3D11`*"]
    pub fn CreateFreeThreaded<'a, Param0: ::windows::runtime::IntoParam<'a, super::DirectX::Direct3D11::IDirect3DDevice>, Param3: ::windows::runtime::IntoParam<'a, super::SizeInt32>>(device: Param0, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: Param3) -> ::windows::runtime::Result<Direct3D11CaptureFramePool> {
        Self::IDirect3D11CaptureFramePoolStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), device.into_param().abi(), pixelformat, numberofbuffers, size.into_param().abi(), &mut result__).from_abi::<Direct3D11CaptureFramePool>(result__)
        })
    }
    pub fn IDirect3D11CaptureFramePoolStatics<R, F: FnOnce(&IDirect3D11CaptureFramePoolStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Direct3D11CaptureFramePool, IDirect3D11CaptureFramePoolStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IDirect3D11CaptureFramePoolStatics2<R, F: FnOnce(&IDirect3D11CaptureFramePoolStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Direct3D11CaptureFramePool, IDirect3D11CaptureFramePoolStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Direct3D11CaptureFramePool {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Capture.Direct3D11CaptureFramePool;{24eb6d22-1975-422e-82e7-780dbd8ddf24})");
}
unsafe impl ::windows::runtime::Interface for Direct3D11CaptureFramePool {
    type Vtable = IDirect3D11CaptureFramePool_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(619408674, 6517, 16942, [130, 231, 120, 13, 189, 141, 223, 36]);
}
impl ::windows::runtime::RuntimeName for Direct3D11CaptureFramePool {
    const NAME: &'static str = "Windows.Graphics.Capture.Direct3D11CaptureFramePool";
}
impl ::std::convert::From<Direct3D11CaptureFramePool> for ::windows::runtime::IUnknown {
    fn from(value: Direct3D11CaptureFramePool) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Direct3D11CaptureFramePool> for ::windows::runtime::IUnknown {
    fn from(value: &Direct3D11CaptureFramePool) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Direct3D11CaptureFramePool {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Direct3D11CaptureFramePool {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Direct3D11CaptureFramePool> for ::windows::runtime::IInspectable {
    fn from(value: Direct3D11CaptureFramePool) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Direct3D11CaptureFramePool> for ::windows::runtime::IInspectable {
    fn from(value: &Direct3D11CaptureFramePool) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Direct3D11CaptureFramePool {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Direct3D11CaptureFramePool {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<Direct3D11CaptureFramePool> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: Direct3D11CaptureFramePool) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&Direct3D11CaptureFramePool> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &Direct3D11CaptureFramePool) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for Direct3D11CaptureFramePool {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &Direct3D11CaptureFramePool {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for Direct3D11CaptureFramePool {}
unsafe impl ::std::marker::Sync for Direct3D11CaptureFramePool {}
#[doc = "*Required features: `Graphics_Capture`*"]
pub struct GraphicsCaptureAccess {}
impl GraphicsCaptureAccess {
    #[cfg(all(feature = "Foundation", feature = "Security_Authorization_AppCapabilityAccess"))]
    #[doc = "*Required features: `Graphics_Capture`, `Foundation`, `Security_Authorization_AppCapabilityAccess`*"]
    pub fn RequestAccessAsync(request: GraphicsCaptureAccessKind) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessStatus>> {
        Self::IGraphicsCaptureAccessStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), request, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessStatus>>(result__)
        })
    }
    pub fn IGraphicsCaptureAccessStatics<R, F: FnOnce(&IGraphicsCaptureAccessStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<GraphicsCaptureAccess, IGraphicsCaptureAccessStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for GraphicsCaptureAccess {
    const NAME: &'static str = "Windows.Graphics.Capture.GraphicsCaptureAccess";
}
#[doc = "*Required features: `Graphics_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GraphicsCaptureAccessKind(pub i32);
impl GraphicsCaptureAccessKind {
    pub const Borderless: GraphicsCaptureAccessKind = GraphicsCaptureAccessKind(0i32);
    pub const Programmatic: GraphicsCaptureAccessKind = GraphicsCaptureAccessKind(1i32);
}
impl ::std::convert::From<i32> for GraphicsCaptureAccessKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GraphicsCaptureAccessKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GraphicsCaptureAccessKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Capture.GraphicsCaptureAccessKind;i4)");
}
#[doc = "*Required features: `Graphics_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct GraphicsCaptureItem(::windows::runtime::IInspectable);
impl GraphicsCaptureItem {
    #[doc = "*Required features: `Graphics_Capture`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Capture`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<super::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__: super::SizeInt32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::SizeInt32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Capture`, `Foundation`*"]
    pub fn Closed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<GraphicsCaptureItem, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Capture`, `Foundation`*"]
    pub fn RemoveClosed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Composition")]
    #[doc = "*Required features: `Graphics_Capture`, `UI_Composition`*"]
    pub fn CreateFromVisual<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::Composition::Visual>>(visual: Param0) -> ::windows::runtime::Result<GraphicsCaptureItem> {
        Self::IGraphicsCaptureItemStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), visual.into_param().abi(), &mut result__).from_abi::<GraphicsCaptureItem>(result__)
        })
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Graphics_Capture`, `UI`*"]
    pub fn TryCreateFromWindowId<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::WindowId>>(windowid: Param0) -> ::windows::runtime::Result<GraphicsCaptureItem> {
        Self::IGraphicsCaptureItemStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), windowid.into_param().abi(), &mut result__).from_abi::<GraphicsCaptureItem>(result__)
        })
    }
    #[doc = "*Required features: `Graphics_Capture`*"]
    pub fn TryCreateFromDisplayId<'a, Param0: ::windows::runtime::IntoParam<'a, super::DisplayId>>(displayid: Param0) -> ::windows::runtime::Result<GraphicsCaptureItem> {
        Self::IGraphicsCaptureItemStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), displayid.into_param().abi(), &mut result__).from_abi::<GraphicsCaptureItem>(result__)
        })
    }
    pub fn IGraphicsCaptureItemStatics<R, F: FnOnce(&IGraphicsCaptureItemStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<GraphicsCaptureItem, IGraphicsCaptureItemStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IGraphicsCaptureItemStatics2<R, F: FnOnce(&IGraphicsCaptureItemStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<GraphicsCaptureItem, IGraphicsCaptureItemStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GraphicsCaptureItem {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Capture.GraphicsCaptureItem;{79c3f95b-31f7-4ec2-a464-632ef5d30760})");
}
unsafe impl ::windows::runtime::Interface for GraphicsCaptureItem {
    type Vtable = IGraphicsCaptureItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2042886491, 12791, 20162, [164, 100, 99, 46, 245, 211, 7, 96]);
}
impl ::windows::runtime::RuntimeName for GraphicsCaptureItem {
    const NAME: &'static str = "Windows.Graphics.Capture.GraphicsCaptureItem";
}
impl ::std::convert::From<GraphicsCaptureItem> for ::windows::runtime::IUnknown {
    fn from(value: GraphicsCaptureItem) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GraphicsCaptureItem> for ::windows::runtime::IUnknown {
    fn from(value: &GraphicsCaptureItem) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GraphicsCaptureItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GraphicsCaptureItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<GraphicsCaptureItem> for ::windows::runtime::IInspectable {
    fn from(value: GraphicsCaptureItem) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GraphicsCaptureItem> for ::windows::runtime::IInspectable {
    fn from(value: &GraphicsCaptureItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GraphicsCaptureItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GraphicsCaptureItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for GraphicsCaptureItem {}
unsafe impl ::std::marker::Sync for GraphicsCaptureItem {}
#[doc = "*Required features: `Graphics_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct GraphicsCapturePicker(::windows::runtime::IInspectable);
impl GraphicsCapturePicker {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<GraphicsCapturePicker, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Capture`, `Foundation`*"]
    pub fn PickSingleItemAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<GraphicsCaptureItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<GraphicsCaptureItem>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GraphicsCapturePicker {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Capture.GraphicsCapturePicker;{5a1711b3-ad79-4b4a-9336-1318fdde3539})");
}
unsafe impl ::windows::runtime::Interface for GraphicsCapturePicker {
    type Vtable = IGraphicsCapturePicker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1511461299, 44409, 19274, [147, 54, 19, 24, 253, 222, 53, 57]);
}
impl ::windows::runtime::RuntimeName for GraphicsCapturePicker {
    const NAME: &'static str = "Windows.Graphics.Capture.GraphicsCapturePicker";
}
impl ::std::convert::From<GraphicsCapturePicker> for ::windows::runtime::IUnknown {
    fn from(value: GraphicsCapturePicker) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GraphicsCapturePicker> for ::windows::runtime::IUnknown {
    fn from(value: &GraphicsCapturePicker) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GraphicsCapturePicker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GraphicsCapturePicker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<GraphicsCapturePicker> for ::windows::runtime::IInspectable {
    fn from(value: GraphicsCapturePicker) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GraphicsCapturePicker> for ::windows::runtime::IInspectable {
    fn from(value: &GraphicsCapturePicker) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GraphicsCapturePicker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GraphicsCapturePicker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for GraphicsCapturePicker {}
unsafe impl ::std::marker::Sync for GraphicsCapturePicker {}
#[doc = "*Required features: `Graphics_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct GraphicsCaptureSession(::windows::runtime::IInspectable);
impl GraphicsCaptureSession {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Capture`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Graphics_Capture`*"]
    pub fn StartCapture(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Graphics_Capture`*"]
    pub fn IsSupported() -> ::windows::runtime::Result<bool> {
        Self::IGraphicsCaptureSessionStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Graphics_Capture`*"]
    pub fn IsCursorCaptureEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IGraphicsCaptureSession2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Capture`*"]
    pub fn SetIsCursorCaptureEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGraphicsCaptureSession2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Capture`*"]
    pub fn IsBorderRequired(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IGraphicsCaptureSession3>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Capture`*"]
    pub fn SetIsBorderRequired(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGraphicsCaptureSession3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn IGraphicsCaptureSessionStatics<R, F: FnOnce(&IGraphicsCaptureSessionStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<GraphicsCaptureSession, IGraphicsCaptureSessionStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GraphicsCaptureSession {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Capture.GraphicsCaptureSession;{814e42a9-f70f-4ad7-939b-fddcc6eb880d})");
}
unsafe impl ::windows::runtime::Interface for GraphicsCaptureSession {
    type Vtable = IGraphicsCaptureSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2169389737, 63247, 19159, [147, 155, 253, 220, 198, 235, 136, 13]);
}
impl ::windows::runtime::RuntimeName for GraphicsCaptureSession {
    const NAME: &'static str = "Windows.Graphics.Capture.GraphicsCaptureSession";
}
impl ::std::convert::From<GraphicsCaptureSession> for ::windows::runtime::IUnknown {
    fn from(value: GraphicsCaptureSession) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GraphicsCaptureSession> for ::windows::runtime::IUnknown {
    fn from(value: &GraphicsCaptureSession) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GraphicsCaptureSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GraphicsCaptureSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<GraphicsCaptureSession> for ::windows::runtime::IInspectable {
    fn from(value: GraphicsCaptureSession) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GraphicsCaptureSession> for ::windows::runtime::IInspectable {
    fn from(value: &GraphicsCaptureSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GraphicsCaptureSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GraphicsCaptureSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<GraphicsCaptureSession> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: GraphicsCaptureSession) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&GraphicsCaptureSession> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &GraphicsCaptureSession) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for GraphicsCaptureSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &GraphicsCaptureSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for GraphicsCaptureSession {}
unsafe impl ::std::marker::Sync for GraphicsCaptureSession {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IDirect3D11CaptureFrame(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDirect3D11CaptureFrame {
    type Vtable = IDirect3D11CaptureFrame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4199597603, 14554, 19250, [172, 243, 250, 151, 52, 173, 128, 14]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3D11CaptureFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::SizeInt32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IDirect3D11CaptureFramePool(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDirect3D11CaptureFramePool {
    type Vtable = IDirect3D11CaptureFramePool_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(619408674, 6517, 16942, [130, 231, 120, 13, 189, 141, 223, 36]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3D11CaptureFramePool_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, device: ::windows::runtime::RawPtr, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, item: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IDirect3D11CaptureFramePoolStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDirect3D11CaptureFramePoolStatics {
    type Vtable = IDirect3D11CaptureFramePoolStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2005140842, 26538, 19795, [174, 84, 16, 136, 213, 168, 202, 33]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3D11CaptureFramePoolStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, device: ::windows::runtime::RawPtr, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IDirect3D11CaptureFramePoolStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDirect3D11CaptureFramePoolStatics2 {
    type Vtable = IDirect3D11CaptureFramePoolStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1486557247, 27580, 24053, [169, 145, 2, 226, 139, 59, 102, 213]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3D11CaptureFramePoolStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, device: ::windows::runtime::RawPtr, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IGraphicsCaptureAccessStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGraphicsCaptureAccessStatics {
    type Vtable = IGraphicsCaptureAccessStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1950274416, 1772, 20544, [165, 138, 144, 31, 15, 117, 112, 149]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureAccessStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Security_Authorization_AppCapabilityAccess"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, request: GraphicsCaptureAccessKind, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Authorization_AppCapabilityAccess")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IGraphicsCaptureItem(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGraphicsCaptureItem {
    type Vtable = IGraphicsCaptureItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2042886491, 12791, 20162, [164, 100, 99, 46, 245, 211, 7, 96]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureItem_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::SizeInt32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IGraphicsCaptureItemStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGraphicsCaptureItemStatics {
    type Vtable = IGraphicsCaptureItemStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2826878629, 17788, 22408, [171, 71, 12, 241, 211, 99, 126, 116]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureItemStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, visual: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IGraphicsCaptureItemStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGraphicsCaptureItemStatics2 {
    type Vtable = IGraphicsCaptureItemStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(999468233, 58756, 22626, [191, 92, 156, 49, 108, 109, 45, 187]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureItemStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, windowid: super::super::UI::WindowId, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, displayid: super::DisplayId, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IGraphicsCapturePicker(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGraphicsCapturePicker {
    type Vtable = IGraphicsCapturePicker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1511461299, 44409, 19274, [147, 54, 19, 24, 253, 222, 53, 57]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCapturePicker_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IGraphicsCaptureSession(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGraphicsCaptureSession {
    type Vtable = IGraphicsCaptureSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2169389737, 63247, 19159, [147, 155, 253, 220, 198, 235, 136, 13]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureSession_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IGraphicsCaptureSession2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGraphicsCaptureSession2 {
    type Vtable = IGraphicsCaptureSession2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(741977664, 32046, 20548, [128, 78, 139, 103, 153, 212, 207, 158]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureSession2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IGraphicsCaptureSession3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGraphicsCaptureSession3 {
    type Vtable = IGraphicsCaptureSession3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4073576806, 8878, 24225, [149, 150, 58, 40, 147, 68, 195, 190]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureSession3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IGraphicsCaptureSessionStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGraphicsCaptureSessionStatics {
    type Vtable = IGraphicsCaptureSessionStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(572826944, 22900, 18858, [178, 50, 8, 130, 83, 111, 76, 181]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureSessionStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
