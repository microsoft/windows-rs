#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'UI_Input_Inking_Core'*"]
#[repr(transparent)]
pub struct CoreIncrementalInkStroke(::windows::core::IUnknown);
impl CoreIncrementalInkStroke {
    #[doc = "*Required features: 'UI_Input_Inking_Core', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn AppendInkPoints<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<super::InkPoint>>>(&self, inkpoints: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), inkpoints.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core'*"]
    pub fn CreateInkStroke(&self) -> ::windows::core::Result<super::InkStroke> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::InkStroke>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core'*"]
    pub fn DrawingAttributes(&self) -> ::windows::core::Result<super::InkDrawingAttributes> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::InkDrawingAttributes>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core', 'Foundation_Numerics'*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PointTransform(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Numerics::Matrix3x2> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Numerics::Matrix3x2 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Numerics::Matrix3x2>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core', 'Foundation_Numerics'*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::InkDrawingAttributes>, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Numerics::Matrix3x2>>(drawingattributes: Param0, pointtransform: Param1) -> ::windows::core::Result<CoreIncrementalInkStroke> {
        Self::ICoreIncrementalInkStrokeFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), drawingattributes.into_param().abi(), pointtransform.into_param().abi(), &mut result__).from_abi::<CoreIncrementalInkStroke>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICoreIncrementalInkStrokeFactory<R, F: FnOnce(&ICoreIncrementalInkStrokeFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreIncrementalInkStroke, ICoreIncrementalInkStrokeFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CoreIncrementalInkStroke {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreIncrementalInkStroke {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreIncrementalInkStroke {}
impl ::core::fmt::Debug for CoreIncrementalInkStroke {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreIncrementalInkStroke").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreIncrementalInkStroke {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Core.CoreIncrementalInkStroke;{fda015d3-9d66-4f7d-a57f-cc70b9cfaa76})");
}
unsafe impl ::windows::core::Interface for CoreIncrementalInkStroke {
    type Vtable = ICoreIncrementalInkStrokeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfda015d3_9d66_4f7d_a57f_cc70b9cfaa76);
}
impl ::windows::core::RuntimeName for CoreIncrementalInkStroke {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.CoreIncrementalInkStroke";
}
impl ::core::convert::From<CoreIncrementalInkStroke> for ::windows::core::IUnknown {
    fn from(value: CoreIncrementalInkStroke) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreIncrementalInkStroke> for ::windows::core::IUnknown {
    fn from(value: &CoreIncrementalInkStroke) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreIncrementalInkStroke {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CoreIncrementalInkStroke {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreIncrementalInkStroke> for ::windows::core::IInspectable {
    fn from(value: CoreIncrementalInkStroke) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreIncrementalInkStroke> for ::windows::core::IInspectable {
    fn from(value: &CoreIncrementalInkStroke) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreIncrementalInkStroke {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CoreIncrementalInkStroke {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreIncrementalInkStroke {}
unsafe impl ::core::marker::Sync for CoreIncrementalInkStroke {}
#[doc = "*Required features: 'UI_Input_Inking_Core'*"]
#[repr(transparent)]
pub struct CoreInkIndependentInputSource(::windows::core::IUnknown);
impl CoreInkIndependentInputSource {
    #[doc = "*Required features: 'UI_Input_Inking_Core', 'Foundation', 'UI_Core'*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerEntering<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerEntering<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core', 'Foundation', 'UI_Core'*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerHovering<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerHovering<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core', 'Foundation', 'UI_Core'*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerExiting<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerExiting<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core', 'Foundation', 'UI_Core'*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerPressing<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerPressing<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core', 'Foundation', 'UI_Core'*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerMoving<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerMoving<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core', 'Foundation', 'UI_Core'*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerReleasing<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerReleasing<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core', 'Foundation', 'UI_Core'*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerLost<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerLost<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core'*"]
    pub fn InkPresenter(&self) -> ::windows::core::Result<super::InkPresenter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::InkPresenter>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core', 'UI_Core'*"]
    #[cfg(feature = "UI_Core")]
    pub fn PointerCursor(&self) -> ::windows::core::Result<super::super::super::Core::CoreCursor> {
        let this = &::windows::core::Interface::cast::<ICoreInkIndependentInputSource2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Core::CoreCursor>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core', 'UI_Core'*"]
    #[cfg(feature = "UI_Core")]
    pub fn SetPointerCursor<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Core::CoreCursor>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreInkIndependentInputSource2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core'*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::InkPresenter>>(inkpresenter: Param0) -> ::windows::core::Result<CoreInkIndependentInputSource> {
        Self::ICoreInkIndependentInputSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), inkpresenter.into_param().abi(), &mut result__).from_abi::<CoreInkIndependentInputSource>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICoreInkIndependentInputSourceStatics<R, F: FnOnce(&ICoreInkIndependentInputSourceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreInkIndependentInputSource, ICoreInkIndependentInputSourceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CoreInkIndependentInputSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreInkIndependentInputSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreInkIndependentInputSource {}
impl ::core::fmt::Debug for CoreInkIndependentInputSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreInkIndependentInputSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreInkIndependentInputSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Core.CoreInkIndependentInputSource;{39b38da9-7639-4499-a5b5-191d00e35b16})");
}
unsafe impl ::windows::core::Interface for CoreInkIndependentInputSource {
    type Vtable = ICoreInkIndependentInputSourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39b38da9_7639_4499_a5b5_191d00e35b16);
}
impl ::windows::core::RuntimeName for CoreInkIndependentInputSource {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.CoreInkIndependentInputSource";
}
impl ::core::convert::From<CoreInkIndependentInputSource> for ::windows::core::IUnknown {
    fn from(value: CoreInkIndependentInputSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInkIndependentInputSource> for ::windows::core::IUnknown {
    fn from(value: &CoreInkIndependentInputSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreInkIndependentInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CoreInkIndependentInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreInkIndependentInputSource> for ::windows::core::IInspectable {
    fn from(value: CoreInkIndependentInputSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInkIndependentInputSource> for ::windows::core::IInspectable {
    fn from(value: &CoreInkIndependentInputSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreInkIndependentInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CoreInkIndependentInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreInkIndependentInputSource {}
unsafe impl ::core::marker::Sync for CoreInkIndependentInputSource {}
#[doc = "*Required features: 'UI_Input_Inking_Core'*"]
#[repr(transparent)]
pub struct CoreInkPresenterHost(::windows::core::IUnknown);
impl CoreInkPresenterHost {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreInkPresenterHost, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core'*"]
    pub fn InkPresenter(&self) -> ::windows::core::Result<super::InkPresenter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::InkPresenter>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core', 'UI_Composition'*"]
    #[cfg(feature = "UI_Composition")]
    pub fn RootVisual(&self) -> ::windows::core::Result<super::super::super::Composition::ContainerVisual> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Composition::ContainerVisual>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core', 'UI_Composition'*"]
    #[cfg(feature = "UI_Composition")]
    pub fn SetRootVisual<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Composition::ContainerVisual>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for CoreInkPresenterHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreInkPresenterHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreInkPresenterHost {}
impl ::core::fmt::Debug for CoreInkPresenterHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreInkPresenterHost").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreInkPresenterHost {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Core.CoreInkPresenterHost;{396e89e6-7d55-4617-9e58-68c70c9169b9})");
}
unsafe impl ::windows::core::Interface for CoreInkPresenterHost {
    type Vtable = ICoreInkPresenterHostVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x396e89e6_7d55_4617_9e58_68c70c9169b9);
}
impl ::windows::core::RuntimeName for CoreInkPresenterHost {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.CoreInkPresenterHost";
}
impl ::core::convert::From<CoreInkPresenterHost> for ::windows::core::IUnknown {
    fn from(value: CoreInkPresenterHost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInkPresenterHost> for ::windows::core::IUnknown {
    fn from(value: &CoreInkPresenterHost) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreInkPresenterHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CoreInkPresenterHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreInkPresenterHost> for ::windows::core::IInspectable {
    fn from(value: CoreInkPresenterHost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInkPresenterHost> for ::windows::core::IInspectable {
    fn from(value: &CoreInkPresenterHost) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreInkPresenterHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CoreInkPresenterHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreInkPresenterHost {}
unsafe impl ::core::marker::Sync for CoreInkPresenterHost {}
#[doc = "*Required features: 'UI_Input_Inking_Core'*"]
#[repr(transparent)]
pub struct CoreWetStrokeDisposition(pub i32);
impl CoreWetStrokeDisposition {
    pub const Inking: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const Canceled: Self = Self(2i32);
}
impl ::core::marker::Copy for CoreWetStrokeDisposition {}
impl ::core::clone::Clone for CoreWetStrokeDisposition {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CoreWetStrokeDisposition {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CoreWetStrokeDisposition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWetStrokeDisposition {}
impl ::core::fmt::Debug for CoreWetStrokeDisposition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWetStrokeDisposition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWetStrokeDisposition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.Core.CoreWetStrokeDisposition;i4)");
}
impl ::windows::core::DefaultType for CoreWetStrokeDisposition {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Input_Inking_Core'*"]
#[repr(transparent)]
pub struct CoreWetStrokeUpdateEventArgs(::windows::core::IUnknown);
impl CoreWetStrokeUpdateEventArgs {
    #[doc = "*Required features: 'UI_Input_Inking_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn NewInkPoints(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<super::InkPoint>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVector<super::InkPoint>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core'*"]
    pub fn PointerId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core'*"]
    pub fn Disposition(&self) -> ::windows::core::Result<CoreWetStrokeDisposition> {
        let this = self;
        unsafe {
            let mut result__: CoreWetStrokeDisposition = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreWetStrokeDisposition>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core'*"]
    pub fn SetDisposition(&self, value: CoreWetStrokeDisposition) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for CoreWetStrokeUpdateEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWetStrokeUpdateEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWetStrokeUpdateEventArgs {}
impl ::core::fmt::Debug for CoreWetStrokeUpdateEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWetStrokeUpdateEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWetStrokeUpdateEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Core.CoreWetStrokeUpdateEventArgs;{fb07d14c-3380-457a-a987-991357896c1b})");
}
unsafe impl ::windows::core::Interface for CoreWetStrokeUpdateEventArgs {
    type Vtable = ICoreWetStrokeUpdateEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb07d14c_3380_457a_a987_991357896c1b);
}
impl ::windows::core::RuntimeName for CoreWetStrokeUpdateEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.CoreWetStrokeUpdateEventArgs";
}
impl ::core::convert::From<CoreWetStrokeUpdateEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreWetStrokeUpdateEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWetStrokeUpdateEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreWetStrokeUpdateEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreWetStrokeUpdateEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CoreWetStrokeUpdateEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWetStrokeUpdateEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreWetStrokeUpdateEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWetStrokeUpdateEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreWetStrokeUpdateEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreWetStrokeUpdateEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CoreWetStrokeUpdateEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWetStrokeUpdateEventArgs {}
unsafe impl ::core::marker::Sync for CoreWetStrokeUpdateEventArgs {}
#[doc = "*Required features: 'UI_Input_Inking_Core'*"]
#[repr(transparent)]
pub struct CoreWetStrokeUpdateSource(::windows::core::IUnknown);
impl CoreWetStrokeUpdateSource {
    #[doc = "*Required features: 'UI_Input_Inking_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn WetStrokeStarting<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveWetStrokeStarting<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn WetStrokeContinuing<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveWetStrokeContinuing<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn WetStrokeStopping<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveWetStrokeStopping<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn WetStrokeCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveWetStrokeCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn WetStrokeCanceled<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveWetStrokeCanceled<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core'*"]
    pub fn InkPresenter(&self) -> ::windows::core::Result<super::InkPresenter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::InkPresenter>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking_Core'*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::InkPresenter>>(inkpresenter: Param0) -> ::windows::core::Result<CoreWetStrokeUpdateSource> {
        Self::ICoreWetStrokeUpdateSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), inkpresenter.into_param().abi(), &mut result__).from_abi::<CoreWetStrokeUpdateSource>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICoreWetStrokeUpdateSourceStatics<R, F: FnOnce(&ICoreWetStrokeUpdateSourceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreWetStrokeUpdateSource, ICoreWetStrokeUpdateSourceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CoreWetStrokeUpdateSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWetStrokeUpdateSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWetStrokeUpdateSource {}
impl ::core::fmt::Debug for CoreWetStrokeUpdateSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWetStrokeUpdateSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWetStrokeUpdateSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Core.CoreWetStrokeUpdateSource;{1f718e22-ee52-4e00-8209-4c3e5b21a3cc})");
}
unsafe impl ::windows::core::Interface for CoreWetStrokeUpdateSource {
    type Vtable = ICoreWetStrokeUpdateSourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f718e22_ee52_4e00_8209_4c3e5b21a3cc);
}
impl ::windows::core::RuntimeName for CoreWetStrokeUpdateSource {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.CoreWetStrokeUpdateSource";
}
impl ::core::convert::From<CoreWetStrokeUpdateSource> for ::windows::core::IUnknown {
    fn from(value: CoreWetStrokeUpdateSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWetStrokeUpdateSource> for ::windows::core::IUnknown {
    fn from(value: &CoreWetStrokeUpdateSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreWetStrokeUpdateSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CoreWetStrokeUpdateSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWetStrokeUpdateSource> for ::windows::core::IInspectable {
    fn from(value: CoreWetStrokeUpdateSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWetStrokeUpdateSource> for ::windows::core::IInspectable {
    fn from(value: &CoreWetStrokeUpdateSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreWetStrokeUpdateSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CoreWetStrokeUpdateSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreWetStrokeUpdateSource {}
unsafe impl ::core::marker::Sync for CoreWetStrokeUpdateSource {}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreIncrementalInkStroke(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreIncrementalInkStroke {
    type Vtable = ICoreIncrementalInkStrokeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfda015d3_9d66_4f7d_a57f_cc70b9cfaa76);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreIncrementalInkStrokeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inkpoints: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreIncrementalInkStrokeFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreIncrementalInkStrokeFactory {
    type Vtable = ICoreIncrementalInkStrokeFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7c59f46_8da8_4f70_9751_e53bb6df4596);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreIncrementalInkStrokeFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, drawingattributes: ::windows::core::RawPtr, pointtransform: super::super::super::super::Foundation::Numerics::Matrix3x2, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInkIndependentInputSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreInkIndependentInputSource {
    type Vtable = ICoreInkIndependentInputSourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39b38da9_7639_4499_a5b5_191d00e35b16);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInkIndependentInputSourceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInkIndependentInputSource2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreInkIndependentInputSource2 {
    type Vtable = ICoreInkIndependentInputSource2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2846b012_0b59_5bb9_a3c5_becb7cf03a33);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInkIndependentInputSource2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Core")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))] usize,
    #[cfg(feature = "UI_Core")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInkIndependentInputSourceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreInkIndependentInputSourceStatics {
    type Vtable = ICoreInkIndependentInputSourceStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73e6011b_80c0_4dfb_9b66_10ba7f3f9c84);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInkIndependentInputSourceStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inkpresenter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInkPresenterHost(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreInkPresenterHost {
    type Vtable = ICoreInkPresenterHostVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x396e89e6_7d55_4617_9e58_68c70c9169b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInkPresenterHostVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWetStrokeUpdateEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWetStrokeUpdateEventArgs {
    type Vtable = ICoreWetStrokeUpdateEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb07d14c_3380_457a_a987_991357896c1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWetStrokeUpdateEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreWetStrokeDisposition) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CoreWetStrokeDisposition) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWetStrokeUpdateSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWetStrokeUpdateSource {
    type Vtable = ICoreWetStrokeUpdateSourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f718e22_ee52_4e00_8209_4c3e5b21a3cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWetStrokeUpdateSourceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWetStrokeUpdateSourceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWetStrokeUpdateSourceStatics {
    type Vtable = ICoreWetStrokeUpdateSourceStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3dad9cba_1d3d_46ae_ab9d_8647486c6f90);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWetStrokeUpdateSourceStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inkpresenter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
