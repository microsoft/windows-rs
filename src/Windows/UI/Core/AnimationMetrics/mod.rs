#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AnimationDescription(pub ::windows::core::IInspectable);
impl AnimationDescription {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Animations(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<IPropertyAnimation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<IPropertyAnimation>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StaggerDelay(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    pub fn StaggerDelayFactor(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DelayLimit(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    pub fn ZOrder(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn CreateInstance(effect: AnimationEffect, target: AnimationEffectTarget) -> ::windows::core::Result<AnimationDescription> {
        Self::IAnimationDescriptionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), effect, target, &mut result__).from_abi::<AnimationDescription>(result__)
        })
    }
    pub fn IAnimationDescriptionFactory<R, F: FnOnce(&IAnimationDescriptionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AnimationDescription, IAnimationDescriptionFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AnimationDescription {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.AnimationMetrics.AnimationDescription;{7d11a549-be3d-41de-b081-05c149962f9b})");
}
unsafe impl ::windows::core::Interface for AnimationDescription {
    type Vtable = IAnimationDescription_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d11a549_be3d_41de_b081_05c149962f9b);
}
impl ::windows::core::RuntimeName for AnimationDescription {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.AnimationDescription";
}
impl ::core::convert::From<AnimationDescription> for ::windows::core::IUnknown {
    fn from(value: AnimationDescription) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AnimationDescription> for ::windows::core::IUnknown {
    fn from(value: &AnimationDescription) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AnimationDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AnimationDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AnimationDescription> for ::windows::core::IInspectable {
    fn from(value: AnimationDescription) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AnimationDescription> for ::windows::core::IInspectable {
    fn from(value: &AnimationDescription) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AnimationDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AnimationDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AnimationDescription {}
unsafe impl ::core::marker::Sync for AnimationDescription {}
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
unsafe impl ::windows::core::Abi for AnimationEffect {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AnimationEffect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Core.AnimationMetrics.AnimationEffect;i4)");
}
impl ::windows::core::DefaultType for AnimationEffect {
    type DefaultType = Self;
}
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
unsafe impl ::windows::core::Abi for AnimationEffectTarget {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AnimationEffectTarget {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Core.AnimationMetrics.AnimationEffectTarget;i4)");
}
impl ::windows::core::DefaultType for AnimationEffectTarget {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAnimationDescription(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAnimationDescription {
    type Vtable = IAnimationDescription_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d11a549_be3d_41de_b081_05c149962f9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnimationDescription_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAnimationDescriptionFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAnimationDescriptionFactory {
    type Vtable = IAnimationDescriptionFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6e27abe_c1fb_48b5_9271_ecc70ac86ef0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnimationDescriptionFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, effect: AnimationEffect, target: AnimationEffectTarget, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IOpacityAnimation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IOpacityAnimation {
    type Vtable = IOpacityAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x803aabe5_ee7e_455f_84e9_2506afb8d2b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpacityAnimation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPropertyAnimation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPropertyAnimation {
    type Vtable = IPropertyAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a01b4da_4d8c_411e_b615_1ade683a9903);
}
impl IPropertyAnimation {
    pub fn Type(&self) -> ::windows::core::Result<PropertyAnimationType> {
        let this = self;
        unsafe {
            let mut result__: PropertyAnimationType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PropertyAnimationType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Delay(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Control1(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Control2(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IPropertyAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{3a01b4da-4d8c-411e-b615-1ade683a9903}");
}
impl ::core::convert::From<IPropertyAnimation> for ::windows::core::IUnknown {
    fn from(value: IPropertyAnimation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPropertyAnimation> for ::windows::core::IUnknown {
    fn from(value: &IPropertyAnimation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPropertyAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPropertyAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPropertyAnimation> for ::windows::core::IInspectable {
    fn from(value: IPropertyAnimation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPropertyAnimation> for ::windows::core::IInspectable {
    fn from(value: &IPropertyAnimation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPropertyAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IPropertyAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyAnimation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PropertyAnimationType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IScaleAnimation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IScaleAnimation {
    type Vtable = IScaleAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x023552c7_71ab_428c_9c9f_d31780964995);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScaleAnimation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct OpacityAnimation(pub ::windows::core::IInspectable);
impl OpacityAnimation {
    #[cfg(feature = "Foundation")]
    pub fn InitialOpacity(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<f32>>(result__)
        }
    }
    pub fn FinalOpacity(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<PropertyAnimationType> {
        let this = &::windows::core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__: PropertyAnimationType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PropertyAnimationType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Delay(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Control1(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = &::windows::core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Control2(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = &::windows::core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for OpacityAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.AnimationMetrics.OpacityAnimation;{803aabe5-ee7e-455f-84e9-2506afb8d2b4})");
}
unsafe impl ::windows::core::Interface for OpacityAnimation {
    type Vtable = IOpacityAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x803aabe5_ee7e_455f_84e9_2506afb8d2b4);
}
impl ::windows::core::RuntimeName for OpacityAnimation {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.OpacityAnimation";
}
impl ::core::convert::From<OpacityAnimation> for ::windows::core::IUnknown {
    fn from(value: OpacityAnimation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&OpacityAnimation> for ::windows::core::IUnknown {
    fn from(value: &OpacityAnimation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for OpacityAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a OpacityAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<OpacityAnimation> for ::windows::core::IInspectable {
    fn from(value: OpacityAnimation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&OpacityAnimation> for ::windows::core::IInspectable {
    fn from(value: &OpacityAnimation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for OpacityAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a OpacityAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<OpacityAnimation> for IPropertyAnimation {
    type Error = ::windows::core::Error;
    fn try_from(value: OpacityAnimation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&OpacityAnimation> for IPropertyAnimation {
    type Error = ::windows::core::Error;
    fn try_from(value: &OpacityAnimation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPropertyAnimation> for OpacityAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, IPropertyAnimation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPropertyAnimation> for &OpacityAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, IPropertyAnimation> {
        ::core::convert::TryInto::<IPropertyAnimation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for OpacityAnimation {}
unsafe impl ::core::marker::Sync for OpacityAnimation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PropertyAnimation(pub ::windows::core::IInspectable);
impl PropertyAnimation {
    pub fn Type(&self) -> ::windows::core::Result<PropertyAnimationType> {
        let this = self;
        unsafe {
            let mut result__: PropertyAnimationType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PropertyAnimationType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Delay(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Control1(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Control2(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PropertyAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.AnimationMetrics.PropertyAnimation;{3a01b4da-4d8c-411e-b615-1ade683a9903})");
}
unsafe impl ::windows::core::Interface for PropertyAnimation {
    type Vtable = IPropertyAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a01b4da_4d8c_411e_b615_1ade683a9903);
}
impl ::windows::core::RuntimeName for PropertyAnimation {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.PropertyAnimation";
}
impl ::core::convert::From<PropertyAnimation> for ::windows::core::IUnknown {
    fn from(value: PropertyAnimation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PropertyAnimation> for ::windows::core::IUnknown {
    fn from(value: &PropertyAnimation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PropertyAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PropertyAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PropertyAnimation> for ::windows::core::IInspectable {
    fn from(value: PropertyAnimation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PropertyAnimation> for ::windows::core::IInspectable {
    fn from(value: &PropertyAnimation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PropertyAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PropertyAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IPropertyAnimation> for PropertyAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, IPropertyAnimation> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPropertyAnimation> for &PropertyAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, IPropertyAnimation> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PropertyAnimation {}
unsafe impl ::core::marker::Sync for PropertyAnimation {}
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
unsafe impl ::windows::core::Abi for PropertyAnimationType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PropertyAnimationType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Core.AnimationMetrics.PropertyAnimationType;i4)");
}
impl ::windows::core::DefaultType for PropertyAnimationType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ScaleAnimation(pub ::windows::core::IInspectable);
impl ScaleAnimation {
    #[cfg(feature = "Foundation")]
    pub fn InitialScaleX(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<f32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn InitialScaleY(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<f32>>(result__)
        }
    }
    pub fn FinalScaleX(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn FinalScaleY(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn NormalizedOrigin(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<PropertyAnimationType> {
        let this = &::windows::core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__: PropertyAnimationType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PropertyAnimationType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Delay(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Control1(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = &::windows::core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Control2(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = &::windows::core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ScaleAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.AnimationMetrics.ScaleAnimation;{023552c7-71ab-428c-9c9f-d31780964995})");
}
unsafe impl ::windows::core::Interface for ScaleAnimation {
    type Vtable = IScaleAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x023552c7_71ab_428c_9c9f_d31780964995);
}
impl ::windows::core::RuntimeName for ScaleAnimation {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.ScaleAnimation";
}
impl ::core::convert::From<ScaleAnimation> for ::windows::core::IUnknown {
    fn from(value: ScaleAnimation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ScaleAnimation> for ::windows::core::IUnknown {
    fn from(value: &ScaleAnimation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ScaleAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ScaleAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ScaleAnimation> for ::windows::core::IInspectable {
    fn from(value: ScaleAnimation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ScaleAnimation> for ::windows::core::IInspectable {
    fn from(value: &ScaleAnimation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ScaleAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ScaleAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<ScaleAnimation> for IPropertyAnimation {
    type Error = ::windows::core::Error;
    fn try_from(value: ScaleAnimation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ScaleAnimation> for IPropertyAnimation {
    type Error = ::windows::core::Error;
    fn try_from(value: &ScaleAnimation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPropertyAnimation> for ScaleAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, IPropertyAnimation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPropertyAnimation> for &ScaleAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, IPropertyAnimation> {
        ::core::convert::TryInto::<IPropertyAnimation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ScaleAnimation {}
unsafe impl ::core::marker::Sync for ScaleAnimation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TranslationAnimation(pub ::windows::core::IInspectable);
impl TranslationAnimation {
    pub fn Type(&self) -> ::windows::core::Result<PropertyAnimationType> {
        let this = self;
        unsafe {
            let mut result__: PropertyAnimationType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PropertyAnimationType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Delay(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Control1(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Control2(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for TranslationAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.AnimationMetrics.TranslationAnimation;{3a01b4da-4d8c-411e-b615-1ade683a9903})");
}
unsafe impl ::windows::core::Interface for TranslationAnimation {
    type Vtable = IPropertyAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a01b4da_4d8c_411e_b615_1ade683a9903);
}
impl ::windows::core::RuntimeName for TranslationAnimation {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.TranslationAnimation";
}
impl ::core::convert::From<TranslationAnimation> for ::windows::core::IUnknown {
    fn from(value: TranslationAnimation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TranslationAnimation> for ::windows::core::IUnknown {
    fn from(value: &TranslationAnimation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TranslationAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TranslationAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TranslationAnimation> for ::windows::core::IInspectable {
    fn from(value: TranslationAnimation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TranslationAnimation> for ::windows::core::IInspectable {
    fn from(value: &TranslationAnimation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TranslationAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TranslationAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IPropertyAnimation> for TranslationAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, IPropertyAnimation> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPropertyAnimation> for &TranslationAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, IPropertyAnimation> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TranslationAnimation {}
unsafe impl ::core::marker::Sync for TranslationAnimation {}
