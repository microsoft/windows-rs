#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `UI_Core_AnimationMetrics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AnimationDescription(pub ::windows::runtime::IInspectable);
impl AnimationDescription {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Core_AnimationMetrics`, `Foundation_Collections`*"]
    pub fn Animations(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<IPropertyAnimation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<IPropertyAnimation>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Core_AnimationMetrics`, `Foundation`*"]
    pub fn StaggerDelay(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `UI_Core_AnimationMetrics`*"]
    pub fn StaggerDelayFactor(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Core_AnimationMetrics`, `Foundation`*"]
    pub fn DelayLimit(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `UI_Core_AnimationMetrics`*"]
    pub fn ZOrder(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Core_AnimationMetrics`*"]
    pub fn CreateInstance(effect: AnimationEffect, target: AnimationEffectTarget) -> ::windows::runtime::Result<AnimationDescription> {
        Self::IAnimationDescriptionFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), effect, target, &mut result__).from_abi::<AnimationDescription>(result__)
        })
    }
    pub fn IAnimationDescriptionFactory<R, F: FnOnce(&IAnimationDescriptionFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AnimationDescription, IAnimationDescriptionFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AnimationDescription {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Core.AnimationMetrics.AnimationDescription;{7d11a549-be3d-41de-b081-05c149962f9b})");
}
unsafe impl ::windows::runtime::Interface for AnimationDescription {
    type Vtable = IAnimationDescription_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2098308425, 48701, 16862, [176, 129, 5, 193, 73, 150, 47, 155]);
}
impl ::windows::runtime::RuntimeName for AnimationDescription {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.AnimationDescription";
}
impl ::core::convert::From<AnimationDescription> for ::windows::runtime::IUnknown {
    fn from(value: AnimationDescription) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AnimationDescription> for ::windows::runtime::IUnknown {
    fn from(value: &AnimationDescription) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AnimationDescription {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AnimationDescription {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AnimationDescription> for ::windows::runtime::IInspectable {
    fn from(value: AnimationDescription) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AnimationDescription> for ::windows::runtime::IInspectable {
    fn from(value: &AnimationDescription) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AnimationDescription {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AnimationDescription {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AnimationDescription {}
unsafe impl ::core::marker::Sync for AnimationDescription {}
#[doc = "*Required features: `UI_Core_AnimationMetrics`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AnimationEffect(pub i32);
impl AnimationEffect {
    pub const Expand: AnimationEffect = AnimationEffect(0i32);
    pub const Collapse: AnimationEffect = AnimationEffect(1i32);
    pub const Reposition: AnimationEffect = AnimationEffect(2i32);
    pub const FadeIn: AnimationEffect = AnimationEffect(3i32);
    pub const FadeOut: AnimationEffect = AnimationEffect(4i32);
    pub const AddToList: AnimationEffect = AnimationEffect(5i32);
    pub const DeleteFromList: AnimationEffect = AnimationEffect(6i32);
    pub const AddToGrid: AnimationEffect = AnimationEffect(7i32);
    pub const DeleteFromGrid: AnimationEffect = AnimationEffect(8i32);
    pub const AddToSearchGrid: AnimationEffect = AnimationEffect(9i32);
    pub const DeleteFromSearchGrid: AnimationEffect = AnimationEffect(10i32);
    pub const AddToSearchList: AnimationEffect = AnimationEffect(11i32);
    pub const DeleteFromSearchList: AnimationEffect = AnimationEffect(12i32);
    pub const ShowEdgeUI: AnimationEffect = AnimationEffect(13i32);
    pub const ShowPanel: AnimationEffect = AnimationEffect(14i32);
    pub const HideEdgeUI: AnimationEffect = AnimationEffect(15i32);
    pub const HidePanel: AnimationEffect = AnimationEffect(16i32);
    pub const ShowPopup: AnimationEffect = AnimationEffect(17i32);
    pub const HidePopup: AnimationEffect = AnimationEffect(18i32);
    pub const PointerDown: AnimationEffect = AnimationEffect(19i32);
    pub const PointerUp: AnimationEffect = AnimationEffect(20i32);
    pub const DragSourceStart: AnimationEffect = AnimationEffect(21i32);
    pub const DragSourceEnd: AnimationEffect = AnimationEffect(22i32);
    pub const TransitionContent: AnimationEffect = AnimationEffect(23i32);
    pub const Reveal: AnimationEffect = AnimationEffect(24i32);
    pub const Hide: AnimationEffect = AnimationEffect(25i32);
    pub const DragBetweenEnter: AnimationEffect = AnimationEffect(26i32);
    pub const DragBetweenLeave: AnimationEffect = AnimationEffect(27i32);
    pub const SwipeSelect: AnimationEffect = AnimationEffect(28i32);
    pub const SwipeDeselect: AnimationEffect = AnimationEffect(29i32);
    pub const SwipeReveal: AnimationEffect = AnimationEffect(30i32);
    pub const EnterPage: AnimationEffect = AnimationEffect(31i32);
    pub const TransitionPage: AnimationEffect = AnimationEffect(32i32);
    pub const CrossFade: AnimationEffect = AnimationEffect(33i32);
    pub const Peek: AnimationEffect = AnimationEffect(34i32);
    pub const UpdateBadge: AnimationEffect = AnimationEffect(35i32);
}
impl ::core::convert::From<i32> for AnimationEffect {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AnimationEffect {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AnimationEffect {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Core.AnimationMetrics.AnimationEffect;i4)");
}
impl ::windows::runtime::DefaultType for AnimationEffect {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Core_AnimationMetrics`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AnimationEffectTarget(pub i32);
impl AnimationEffectTarget {
    pub const Primary: AnimationEffectTarget = AnimationEffectTarget(0i32);
    pub const Added: AnimationEffectTarget = AnimationEffectTarget(1i32);
    pub const Affected: AnimationEffectTarget = AnimationEffectTarget(2i32);
    pub const Background: AnimationEffectTarget = AnimationEffectTarget(3i32);
    pub const Content: AnimationEffectTarget = AnimationEffectTarget(4i32);
    pub const Deleted: AnimationEffectTarget = AnimationEffectTarget(5i32);
    pub const Deselected: AnimationEffectTarget = AnimationEffectTarget(6i32);
    pub const DragSource: AnimationEffectTarget = AnimationEffectTarget(7i32);
    pub const Hidden: AnimationEffectTarget = AnimationEffectTarget(8i32);
    pub const Incoming: AnimationEffectTarget = AnimationEffectTarget(9i32);
    pub const Outgoing: AnimationEffectTarget = AnimationEffectTarget(10i32);
    pub const Outline: AnimationEffectTarget = AnimationEffectTarget(11i32);
    pub const Remaining: AnimationEffectTarget = AnimationEffectTarget(12i32);
    pub const Revealed: AnimationEffectTarget = AnimationEffectTarget(13i32);
    pub const RowIn: AnimationEffectTarget = AnimationEffectTarget(14i32);
    pub const RowOut: AnimationEffectTarget = AnimationEffectTarget(15i32);
    pub const Selected: AnimationEffectTarget = AnimationEffectTarget(16i32);
    pub const Selection: AnimationEffectTarget = AnimationEffectTarget(17i32);
    pub const Shown: AnimationEffectTarget = AnimationEffectTarget(18i32);
    pub const Tapped: AnimationEffectTarget = AnimationEffectTarget(19i32);
}
impl ::core::convert::From<i32> for AnimationEffectTarget {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AnimationEffectTarget {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AnimationEffectTarget {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Core.AnimationMetrics.AnimationEffectTarget;i4)");
}
impl ::windows::runtime::DefaultType for AnimationEffectTarget {
    type DefaultType = Self;
}
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct AnimationMetricsContract(pub u8);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAnimationDescription(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAnimationDescription {
    type Vtable = IAnimationDescription_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2098308425, 48701, 16862, [176, 129, 5, 193, 73, 150, 47, 155]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnimationDescription_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAnimationDescriptionFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAnimationDescriptionFactory {
    type Vtable = IAnimationDescriptionFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3336731326, 49659, 18613, [146, 113, 236, 199, 10, 200, 110, 240]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnimationDescriptionFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, effect: AnimationEffect, target: AnimationEffectTarget, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IOpacityAnimation(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IOpacityAnimation {
    type Vtable = IOpacityAnimation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2151328741, 61054, 17759, [132, 233, 37, 6, 175, 184, 210, 180]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpacityAnimation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `UI_Core_AnimationMetrics`*"]
pub struct IPropertyAnimation(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPropertyAnimation {
    type Vtable = IPropertyAnimation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(973190362, 19852, 16670, [182, 21, 26, 222, 104, 58, 153, 3]);
}
impl IPropertyAnimation {
    #[doc = "*Required features: `UI_Core_AnimationMetrics`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<PropertyAnimationType> {
        let this = self;
        unsafe {
            let mut result__: PropertyAnimationType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PropertyAnimationType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Core_AnimationMetrics`, `Foundation`*"]
    pub fn Delay(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Core_AnimationMetrics`, `Foundation`*"]
    pub fn Duration(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Core_AnimationMetrics`, `Foundation`*"]
    pub fn Control1(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Core_AnimationMetrics`, `Foundation`*"]
    pub fn Control2(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IPropertyAnimation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{3a01b4da-4d8c-411e-b615-1ade683a9903}");
}
impl ::core::convert::From<IPropertyAnimation> for ::windows::runtime::IUnknown {
    fn from(value: IPropertyAnimation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPropertyAnimation> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertyAnimation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPropertyAnimation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPropertyAnimation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPropertyAnimation> for ::windows::runtime::IInspectable {
    fn from(value: IPropertyAnimation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPropertyAnimation> for ::windows::runtime::IInspectable {
    fn from(value: &IPropertyAnimation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IPropertyAnimation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IPropertyAnimation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyAnimation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PropertyAnimationType) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IScaleAnimation(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IScaleAnimation {
    type Vtable = IScaleAnimation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(37049031, 29099, 17036, [156, 159, 211, 23, 128, 150, 73, 149]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScaleAnimation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc = "*Required features: `UI_Core_AnimationMetrics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct OpacityAnimation(pub ::windows::runtime::IInspectable);
impl OpacityAnimation {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Core_AnimationMetrics`, `Foundation`*"]
    pub fn InitialOpacity(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<f32>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Core_AnimationMetrics`*"]
    pub fn FinalOpacity(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Core_AnimationMetrics`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<PropertyAnimationType> {
        let this = &::windows::runtime::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__: PropertyAnimationType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PropertyAnimationType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Core_AnimationMetrics`, `Foundation`*"]
    pub fn Delay(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = &::windows::runtime::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Core_AnimationMetrics`, `Foundation`*"]
    pub fn Duration(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = &::windows::runtime::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Core_AnimationMetrics`, `Foundation`*"]
    pub fn Control1(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Point> {
        let this = &::windows::runtime::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Core_AnimationMetrics`, `Foundation`*"]
    pub fn Control2(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Point> {
        let this = &::windows::runtime::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for OpacityAnimation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Core.AnimationMetrics.OpacityAnimation;{803aabe5-ee7e-455f-84e9-2506afb8d2b4})");
}
unsafe impl ::windows::runtime::Interface for OpacityAnimation {
    type Vtable = IOpacityAnimation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2151328741, 61054, 17759, [132, 233, 37, 6, 175, 184, 210, 180]);
}
impl ::windows::runtime::RuntimeName for OpacityAnimation {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.OpacityAnimation";
}
impl ::core::convert::From<OpacityAnimation> for ::windows::runtime::IUnknown {
    fn from(value: OpacityAnimation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&OpacityAnimation> for ::windows::runtime::IUnknown {
    fn from(value: &OpacityAnimation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for OpacityAnimation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a OpacityAnimation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<OpacityAnimation> for ::windows::runtime::IInspectable {
    fn from(value: OpacityAnimation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&OpacityAnimation> for ::windows::runtime::IInspectable {
    fn from(value: &OpacityAnimation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for OpacityAnimation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a OpacityAnimation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<OpacityAnimation> for IPropertyAnimation {
    type Error = ::windows::runtime::Error;
    fn try_from(value: OpacityAnimation) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&OpacityAnimation> for IPropertyAnimation {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &OpacityAnimation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPropertyAnimation> for OpacityAnimation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPropertyAnimation> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPropertyAnimation> for &OpacityAnimation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPropertyAnimation> {
        ::core::convert::TryInto::<IPropertyAnimation>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for OpacityAnimation {}
unsafe impl ::core::marker::Sync for OpacityAnimation {}
#[doc = "*Required features: `UI_Core_AnimationMetrics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PropertyAnimation(pub ::windows::runtime::IInspectable);
impl PropertyAnimation {
    #[doc = "*Required features: `UI_Core_AnimationMetrics`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<PropertyAnimationType> {
        let this = self;
        unsafe {
            let mut result__: PropertyAnimationType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PropertyAnimationType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Core_AnimationMetrics`, `Foundation`*"]
    pub fn Delay(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Core_AnimationMetrics`, `Foundation`*"]
    pub fn Duration(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Core_AnimationMetrics`, `Foundation`*"]
    pub fn Control1(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Core_AnimationMetrics`, `Foundation`*"]
    pub fn Control2(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PropertyAnimation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Core.AnimationMetrics.PropertyAnimation;{3a01b4da-4d8c-411e-b615-1ade683a9903})");
}
unsafe impl ::windows::runtime::Interface for PropertyAnimation {
    type Vtable = IPropertyAnimation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(973190362, 19852, 16670, [182, 21, 26, 222, 104, 58, 153, 3]);
}
impl ::windows::runtime::RuntimeName for PropertyAnimation {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.PropertyAnimation";
}
impl ::core::convert::From<PropertyAnimation> for ::windows::runtime::IUnknown {
    fn from(value: PropertyAnimation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PropertyAnimation> for ::windows::runtime::IUnknown {
    fn from(value: &PropertyAnimation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PropertyAnimation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PropertyAnimation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PropertyAnimation> for ::windows::runtime::IInspectable {
    fn from(value: PropertyAnimation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PropertyAnimation> for ::windows::runtime::IInspectable {
    fn from(value: &PropertyAnimation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PropertyAnimation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PropertyAnimation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PropertyAnimation> for IPropertyAnimation {
    fn from(value: PropertyAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PropertyAnimation> for IPropertyAnimation {
    fn from(value: &PropertyAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPropertyAnimation> for PropertyAnimation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPropertyAnimation> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPropertyAnimation> for &PropertyAnimation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPropertyAnimation> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PropertyAnimation {}
unsafe impl ::core::marker::Sync for PropertyAnimation {}
#[doc = "*Required features: `UI_Core_AnimationMetrics`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PropertyAnimationType(pub i32);
impl PropertyAnimationType {
    pub const Scale: PropertyAnimationType = PropertyAnimationType(0i32);
    pub const Translation: PropertyAnimationType = PropertyAnimationType(1i32);
    pub const Opacity: PropertyAnimationType = PropertyAnimationType(2i32);
}
impl ::core::convert::From<i32> for PropertyAnimationType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PropertyAnimationType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PropertyAnimationType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Core.AnimationMetrics.PropertyAnimationType;i4)");
}
impl ::windows::runtime::DefaultType for PropertyAnimationType {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Core_AnimationMetrics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ScaleAnimation(pub ::windows::runtime::IInspectable);
impl ScaleAnimation {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Core_AnimationMetrics`, `Foundation`*"]
    pub fn InitialScaleX(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<f32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Core_AnimationMetrics`, `Foundation`*"]
    pub fn InitialScaleY(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<f32>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Core_AnimationMetrics`*"]
    pub fn FinalScaleX(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Core_AnimationMetrics`*"]
    pub fn FinalScaleY(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Core_AnimationMetrics`, `Foundation`*"]
    pub fn NormalizedOrigin(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `UI_Core_AnimationMetrics`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<PropertyAnimationType> {
        let this = &::windows::runtime::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__: PropertyAnimationType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PropertyAnimationType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Core_AnimationMetrics`, `Foundation`*"]
    pub fn Delay(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = &::windows::runtime::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Core_AnimationMetrics`, `Foundation`*"]
    pub fn Duration(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = &::windows::runtime::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Core_AnimationMetrics`, `Foundation`*"]
    pub fn Control1(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Point> {
        let this = &::windows::runtime::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Core_AnimationMetrics`, `Foundation`*"]
    pub fn Control2(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Point> {
        let this = &::windows::runtime::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ScaleAnimation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Core.AnimationMetrics.ScaleAnimation;{023552c7-71ab-428c-9c9f-d31780964995})");
}
unsafe impl ::windows::runtime::Interface for ScaleAnimation {
    type Vtable = IScaleAnimation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(37049031, 29099, 17036, [156, 159, 211, 23, 128, 150, 73, 149]);
}
impl ::windows::runtime::RuntimeName for ScaleAnimation {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.ScaleAnimation";
}
impl ::core::convert::From<ScaleAnimation> for ::windows::runtime::IUnknown {
    fn from(value: ScaleAnimation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ScaleAnimation> for ::windows::runtime::IUnknown {
    fn from(value: &ScaleAnimation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ScaleAnimation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ScaleAnimation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ScaleAnimation> for ::windows::runtime::IInspectable {
    fn from(value: ScaleAnimation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ScaleAnimation> for ::windows::runtime::IInspectable {
    fn from(value: &ScaleAnimation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ScaleAnimation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ScaleAnimation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<ScaleAnimation> for IPropertyAnimation {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ScaleAnimation) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ScaleAnimation> for IPropertyAnimation {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ScaleAnimation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPropertyAnimation> for ScaleAnimation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPropertyAnimation> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPropertyAnimation> for &ScaleAnimation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPropertyAnimation> {
        ::core::convert::TryInto::<IPropertyAnimation>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for ScaleAnimation {}
unsafe impl ::core::marker::Sync for ScaleAnimation {}
#[doc = "*Required features: `UI_Core_AnimationMetrics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TranslationAnimation(pub ::windows::runtime::IInspectable);
impl TranslationAnimation {
    #[doc = "*Required features: `UI_Core_AnimationMetrics`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<PropertyAnimationType> {
        let this = self;
        unsafe {
            let mut result__: PropertyAnimationType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PropertyAnimationType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Core_AnimationMetrics`, `Foundation`*"]
    pub fn Delay(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Core_AnimationMetrics`, `Foundation`*"]
    pub fn Duration(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Core_AnimationMetrics`, `Foundation`*"]
    pub fn Control1(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Core_AnimationMetrics`, `Foundation`*"]
    pub fn Control2(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TranslationAnimation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Core.AnimationMetrics.TranslationAnimation;{3a01b4da-4d8c-411e-b615-1ade683a9903})");
}
unsafe impl ::windows::runtime::Interface for TranslationAnimation {
    type Vtable = IPropertyAnimation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(973190362, 19852, 16670, [182, 21, 26, 222, 104, 58, 153, 3]);
}
impl ::windows::runtime::RuntimeName for TranslationAnimation {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.TranslationAnimation";
}
impl ::core::convert::From<TranslationAnimation> for ::windows::runtime::IUnknown {
    fn from(value: TranslationAnimation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TranslationAnimation> for ::windows::runtime::IUnknown {
    fn from(value: &TranslationAnimation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TranslationAnimation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a TranslationAnimation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TranslationAnimation> for ::windows::runtime::IInspectable {
    fn from(value: TranslationAnimation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TranslationAnimation> for ::windows::runtime::IInspectable {
    fn from(value: &TranslationAnimation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TranslationAnimation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TranslationAnimation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<TranslationAnimation> for IPropertyAnimation {
    fn from(value: TranslationAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TranslationAnimation> for IPropertyAnimation {
    fn from(value: &TranslationAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPropertyAnimation> for TranslationAnimation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPropertyAnimation> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPropertyAnimation> for &TranslationAnimation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPropertyAnimation> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TranslationAnimation {}
unsafe impl ::core::marker::Sync for TranslationAnimation {}
