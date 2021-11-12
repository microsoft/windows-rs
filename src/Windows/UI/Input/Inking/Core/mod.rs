#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreIncrementalInkStroke(pub ::windows::core::IInspectable);
impl CoreIncrementalInkStroke {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn AppendInkPoints<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<super::InkPoint>>>(&self, inkpoints: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), inkpoints.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
    pub fn CreateInkStroke(&self) -> ::windows::core::Result<super::InkStroke> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::InkStroke>(result__)
        }
    }
    pub fn DrawingAttributes(&self) -> ::windows::core::Result<super::InkDrawingAttributes> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::InkDrawingAttributes>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PointTransform(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Numerics::Matrix3x2> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Numerics::Matrix3x2 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Numerics::Matrix3x2>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::InkDrawingAttributes>, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Numerics::Matrix3x2>>(drawingattributes: Param0, pointtransform: Param1) -> ::windows::core::Result<CoreIncrementalInkStroke> {
        Self::ICoreIncrementalInkStrokeFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), drawingattributes.into_param().abi(), pointtransform.into_param().abi(), &mut result__).from_abi::<CoreIncrementalInkStroke>(result__)
        })
    }
    pub fn ICoreIncrementalInkStrokeFactory<R, F: FnOnce(&ICoreIncrementalInkStrokeFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreIncrementalInkStroke, ICoreIncrementalInkStrokeFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreIncrementalInkStroke {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Core.CoreIncrementalInkStroke;{fda015d3-9d66-4f7d-a57f-cc70b9cfaa76})");
}
unsafe impl ::windows::core::Interface for CoreIncrementalInkStroke {
    type Vtable = ICoreIncrementalInkStroke_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfda015d3_9d66_4f7d_a57f_cc70b9cfaa76);
}
impl ::windows::core::RuntimeName for CoreIncrementalInkStroke {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.CoreIncrementalInkStroke";
}
impl ::core::convert::From<CoreIncrementalInkStroke> for ::windows::core::IUnknown {
    fn from(value: CoreIncrementalInkStroke) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreIncrementalInkStroke> for ::windows::core::IUnknown {
    fn from(value: &CoreIncrementalInkStroke) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreIncrementalInkStroke {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreIncrementalInkStroke {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreIncrementalInkStroke> for ::windows::core::IInspectable {
    fn from(value: CoreIncrementalInkStroke) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreIncrementalInkStroke> for ::windows::core::IInspectable {
    fn from(value: &CoreIncrementalInkStroke) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreIncrementalInkStroke {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreIncrementalInkStroke {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreIncrementalInkStroke {}
unsafe impl ::core::marker::Sync for CoreIncrementalInkStroke {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreInkIndependentInputSource(pub ::windows::core::IInspectable);
impl CoreInkIndependentInputSource {
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerEntering<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerEntering<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerHovering<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerHovering<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerExiting<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerExiting<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerPressing<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerPressing<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerMoving<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerMoving<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerReleasing<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerReleasing<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerLost<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerLost<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    pub fn InkPresenter(&self) -> ::windows::core::Result<super::InkPresenter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::InkPresenter>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::InkPresenter>>(inkpresenter: Param0) -> ::windows::core::Result<CoreInkIndependentInputSource> {
        Self::ICoreInkIndependentInputSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), inkpresenter.into_param().abi(), &mut result__).from_abi::<CoreInkIndependentInputSource>(result__)
        })
    }
    #[cfg(feature = "UI_Core")]
    pub fn PointerCursor(&self) -> ::windows::core::Result<super::super::super::Core::CoreCursor> {
        let this = &::windows::core::Interface::cast::<ICoreInkIndependentInputSource2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Core::CoreCursor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn SetPointerCursor<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Core::CoreCursor>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreInkIndependentInputSource2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ICoreInkIndependentInputSourceStatics<R, F: FnOnce(&ICoreInkIndependentInputSourceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreInkIndependentInputSource, ICoreInkIndependentInputSourceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreInkIndependentInputSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Core.CoreInkIndependentInputSource;{39b38da9-7639-4499-a5b5-191d00e35b16})");
}
unsafe impl ::windows::core::Interface for CoreInkIndependentInputSource {
    type Vtable = ICoreInkIndependentInputSource_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39b38da9_7639_4499_a5b5_191d00e35b16);
}
impl ::windows::core::RuntimeName for CoreInkIndependentInputSource {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.CoreInkIndependentInputSource";
}
impl ::core::convert::From<CoreInkIndependentInputSource> for ::windows::core::IUnknown {
    fn from(value: CoreInkIndependentInputSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreInkIndependentInputSource> for ::windows::core::IUnknown {
    fn from(value: &CoreInkIndependentInputSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreInkIndependentInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreInkIndependentInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreInkIndependentInputSource> for ::windows::core::IInspectable {
    fn from(value: CoreInkIndependentInputSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreInkIndependentInputSource> for ::windows::core::IInspectable {
    fn from(value: &CoreInkIndependentInputSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreInkIndependentInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreInkIndependentInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreInkIndependentInputSource {}
unsafe impl ::core::marker::Sync for CoreInkIndependentInputSource {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreInkPresenterHost(pub ::windows::core::IInspectable);
impl CoreInkPresenterHost {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreInkPresenterHost, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn InkPresenter(&self) -> ::windows::core::Result<super::InkPresenter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::InkPresenter>(result__)
        }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn RootVisual(&self) -> ::windows::core::Result<super::super::super::Composition::ContainerVisual> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Composition::ContainerVisual>(result__)
        }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn SetRootVisual<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Composition::ContainerVisual>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreInkPresenterHost {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Core.CoreInkPresenterHost;{396e89e6-7d55-4617-9e58-68c70c9169b9})");
}
unsafe impl ::windows::core::Interface for CoreInkPresenterHost {
    type Vtable = ICoreInkPresenterHost_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x396e89e6_7d55_4617_9e58_68c70c9169b9);
}
impl ::windows::core::RuntimeName for CoreInkPresenterHost {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.CoreInkPresenterHost";
}
impl ::core::convert::From<CoreInkPresenterHost> for ::windows::core::IUnknown {
    fn from(value: CoreInkPresenterHost) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreInkPresenterHost> for ::windows::core::IUnknown {
    fn from(value: &CoreInkPresenterHost) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreInkPresenterHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreInkPresenterHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreInkPresenterHost> for ::windows::core::IInspectable {
    fn from(value: CoreInkPresenterHost) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreInkPresenterHost> for ::windows::core::IInspectable {
    fn from(value: &CoreInkPresenterHost) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreInkPresenterHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreInkPresenterHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreInkPresenterHost {}
unsafe impl ::core::marker::Sync for CoreInkPresenterHost {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CoreWetStrokeDisposition(pub i32);
impl CoreWetStrokeDisposition {
    pub const Inking: CoreWetStrokeDisposition = CoreWetStrokeDisposition(0i32);
    pub const Completed: CoreWetStrokeDisposition = CoreWetStrokeDisposition(1i32);
    pub const Canceled: CoreWetStrokeDisposition = CoreWetStrokeDisposition(2i32);
}
impl ::core::convert::From<i32> for CoreWetStrokeDisposition {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreWetStrokeDisposition {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreWetStrokeDisposition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.Core.CoreWetStrokeDisposition;i4)");
}
impl ::windows::core::DefaultType for CoreWetStrokeDisposition {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreWetStrokeUpdateEventArgs(pub ::windows::core::IInspectable);
impl CoreWetStrokeUpdateEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    pub fn NewInkPoints(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<super::InkPoint>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVector<super::InkPoint>>(result__)
        }
    }
    pub fn PointerId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Disposition(&self) -> ::windows::core::Result<CoreWetStrokeDisposition> {
        let this = self;
        unsafe {
            let mut result__: CoreWetStrokeDisposition = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreWetStrokeDisposition>(result__)
        }
    }
    pub fn SetDisposition(&self, value: CoreWetStrokeDisposition) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWetStrokeUpdateEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Core.CoreWetStrokeUpdateEventArgs;{fb07d14c-3380-457a-a987-991357896c1b})");
}
unsafe impl ::windows::core::Interface for CoreWetStrokeUpdateEventArgs {
    type Vtable = ICoreWetStrokeUpdateEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb07d14c_3380_457a_a987_991357896c1b);
}
impl ::windows::core::RuntimeName for CoreWetStrokeUpdateEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.CoreWetStrokeUpdateEventArgs";
}
impl ::core::convert::From<CoreWetStrokeUpdateEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreWetStrokeUpdateEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWetStrokeUpdateEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreWetStrokeUpdateEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreWetStrokeUpdateEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreWetStrokeUpdateEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWetStrokeUpdateEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreWetStrokeUpdateEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWetStrokeUpdateEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreWetStrokeUpdateEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreWetStrokeUpdateEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreWetStrokeUpdateEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWetStrokeUpdateEventArgs {}
unsafe impl ::core::marker::Sync for CoreWetStrokeUpdateEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreWetStrokeUpdateSource(pub ::windows::core::IInspectable);
impl CoreWetStrokeUpdateSource {
    #[cfg(feature = "Foundation")]
    pub fn WetStrokeStarting<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveWetStrokeStarting<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn WetStrokeContinuing<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveWetStrokeContinuing<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn WetStrokeStopping<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveWetStrokeStopping<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn WetStrokeCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveWetStrokeCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn WetStrokeCanceled<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveWetStrokeCanceled<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    pub fn InkPresenter(&self) -> ::windows::core::Result<super::InkPresenter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::InkPresenter>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::InkPresenter>>(inkpresenter: Param0) -> ::windows::core::Result<CoreWetStrokeUpdateSource> {
        Self::ICoreWetStrokeUpdateSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), inkpresenter.into_param().abi(), &mut result__).from_abi::<CoreWetStrokeUpdateSource>(result__)
        })
    }
    pub fn ICoreWetStrokeUpdateSourceStatics<R, F: FnOnce(&ICoreWetStrokeUpdateSourceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreWetStrokeUpdateSource, ICoreWetStrokeUpdateSourceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWetStrokeUpdateSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Core.CoreWetStrokeUpdateSource;{1f718e22-ee52-4e00-8209-4c3e5b21a3cc})");
}
unsafe impl ::windows::core::Interface for CoreWetStrokeUpdateSource {
    type Vtable = ICoreWetStrokeUpdateSource_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f718e22_ee52_4e00_8209_4c3e5b21a3cc);
}
impl ::windows::core::RuntimeName for CoreWetStrokeUpdateSource {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.CoreWetStrokeUpdateSource";
}
impl ::core::convert::From<CoreWetStrokeUpdateSource> for ::windows::core::IUnknown {
    fn from(value: CoreWetStrokeUpdateSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWetStrokeUpdateSource> for ::windows::core::IUnknown {
    fn from(value: &CoreWetStrokeUpdateSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreWetStrokeUpdateSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreWetStrokeUpdateSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWetStrokeUpdateSource> for ::windows::core::IInspectable {
    fn from(value: CoreWetStrokeUpdateSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWetStrokeUpdateSource> for ::windows::core::IInspectable {
    fn from(value: &CoreWetStrokeUpdateSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreWetStrokeUpdateSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreWetStrokeUpdateSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWetStrokeUpdateSource {}
unsafe impl ::core::marker::Sync for CoreWetStrokeUpdateSource {}
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreIncrementalInkStroke(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreIncrementalInkStroke {
    type Vtable = ICoreIncrementalInkStroke_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfda015d3_9d66_4f7d_a57f_cc70b9cfaa76);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreIncrementalInkStroke_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inkpoints: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreIncrementalInkStrokeFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreIncrementalInkStrokeFactory {
    type Vtable = ICoreIncrementalInkStrokeFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7c59f46_8da8_4f70_9751_e53bb6df4596);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreIncrementalInkStrokeFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, drawingattributes: ::windows::core::RawPtr, pointtransform: super::super::super::super::Foundation::Numerics::Matrix3x2, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreInkIndependentInputSource(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreInkIndependentInputSource {
    type Vtable = ICoreInkIndependentInputSource_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39b38da9_7639_4499_a5b5_191d00e35b16);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInkIndependentInputSource_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreInkIndependentInputSource2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreInkIndependentInputSource2 {
    type Vtable = ICoreInkIndependentInputSource2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2846b012_0b59_5bb9_a3c5_becb7cf03a33);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInkIndependentInputSource2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Core")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))] usize,
    #[cfg(feature = "UI_Core")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreInkIndependentInputSourceStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreInkIndependentInputSourceStatics {
    type Vtable = ICoreInkIndependentInputSourceStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73e6011b_80c0_4dfb_9b66_10ba7f3f9c84);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInkIndependentInputSourceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inkpresenter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreInkPresenterHost(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreInkPresenterHost {
    type Vtable = ICoreInkPresenterHost_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x396e89e6_7d55_4617_9e58_68c70c9169b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInkPresenterHost_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWetStrokeUpdateEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWetStrokeUpdateEventArgs {
    type Vtable = ICoreWetStrokeUpdateEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb07d14c_3380_457a_a987_991357896c1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWetStrokeUpdateEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CoreWetStrokeDisposition) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: CoreWetStrokeDisposition) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWetStrokeUpdateSource(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWetStrokeUpdateSource {
    type Vtable = ICoreWetStrokeUpdateSource_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f718e22_ee52_4e00_8209_4c3e5b21a3cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWetStrokeUpdateSource_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWetStrokeUpdateSourceStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWetStrokeUpdateSourceStatics {
    type Vtable = ICoreWetStrokeUpdateSourceStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3dad9cba_1d3d_46ae_ab9d_8647486c6f90);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWetStrokeUpdateSourceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inkpresenter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
