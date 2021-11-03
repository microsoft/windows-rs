#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "UI_Input_Core")]
pub mod Core;
#[cfg(feature = "UI_Input_Inking")]
pub mod Inking;
#[cfg(feature = "UI_Input_Preview")]
pub mod Preview;
#[cfg(feature = "UI_Input_Spatial")]
pub mod Spatial;
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AttachableInputObject(::windows::runtime::IInspectable);
impl AttachableInputObject {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AttachableInputObject {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.AttachableInputObject;{9b822734-a3c1-542a-b2f4-0e32b773fb07})");
}
unsafe impl ::windows::runtime::Interface for AttachableInputObject {
    type Vtable = IAttachableInputObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2608998196, 41921, 21546, [178, 244, 14, 50, 183, 115, 251, 7]);
}
impl ::windows::runtime::RuntimeName for AttachableInputObject {
    const NAME: &'static str = "Windows.UI.Input.AttachableInputObject";
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<AttachableInputObject> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AttachableInputObject) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&AttachableInputObject> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AttachableInputObject) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for AttachableInputObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &AttachableInputObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for AttachableInputObject {}
unsafe impl ::std::marker::Sync for AttachableInputObject {}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `UI_Input`*"]
pub struct CrossSlideThresholds {
    pub SelectionStart: f32,
    pub SpeedBumpStart: f32,
    pub SpeedBumpEnd: f32,
    pub RearrangeStart: f32,
}
impl CrossSlideThresholds {}
impl ::std::default::Default for CrossSlideThresholds {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CrossSlideThresholds {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CrossSlideThresholds").field("SelectionStart", &self.SelectionStart).field("SpeedBumpStart", &self.SpeedBumpStart).field("SpeedBumpEnd", &self.SpeedBumpEnd).field("RearrangeStart", &self.RearrangeStart).finish()
    }
}
impl ::std::cmp::PartialEq for CrossSlideThresholds {
    fn eq(&self, other: &Self) -> bool {
        self.SelectionStart == other.SelectionStart && self.SpeedBumpStart == other.SpeedBumpStart && self.SpeedBumpEnd == other.SpeedBumpEnd && self.RearrangeStart == other.RearrangeStart
    }
}
impl ::std::cmp::Eq for CrossSlideThresholds {}
unsafe impl ::windows::runtime::Abi for CrossSlideThresholds {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CrossSlideThresholds {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.UI.Input.CrossSlideThresholds;f4;f4;f4;f4)");
}
impl ::windows::runtime::DefaultType for CrossSlideThresholds {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct CrossSlidingEventArgs(::windows::runtime::IInspectable);
impl CrossSlidingEventArgs {
    #[cfg(feature = "Devices_Input")]
    #[doc = "*Required features: `UI_Input`, `Devices_Input`*"]
    pub fn PointerDeviceType(&self) -> ::windows::runtime::Result<super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::Input::PointerDeviceType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn CrossSlidingState(&self) -> ::windows::runtime::Result<CrossSlidingState> {
        let this = self;
        unsafe {
            let mut result__: CrossSlidingState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CrossSlidingState>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn ContactCount(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<ICrossSlidingEventArgs2>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CrossSlidingEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.CrossSlidingEventArgs;{e9374738-6f88-41d9-8720-78e08e398349})");
}
unsafe impl ::windows::runtime::Interface for CrossSlidingEventArgs {
    type Vtable = ICrossSlidingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3912714040, 28552, 16857, [135, 32, 120, 224, 142, 57, 131, 73]);
}
impl ::windows::runtime::RuntimeName for CrossSlidingEventArgs {
    const NAME: &'static str = "Windows.UI.Input.CrossSlidingEventArgs";
}
#[doc = "*Required features: `UI_Input`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CrossSlidingState(pub i32);
impl CrossSlidingState {
    pub const Started: CrossSlidingState = CrossSlidingState(0i32);
    pub const Dragging: CrossSlidingState = CrossSlidingState(1i32);
    pub const Selecting: CrossSlidingState = CrossSlidingState(2i32);
    pub const SelectSpeedBumping: CrossSlidingState = CrossSlidingState(3i32);
    pub const SpeedBumping: CrossSlidingState = CrossSlidingState(4i32);
    pub const Rearranging: CrossSlidingState = CrossSlidingState(5i32);
    pub const Completed: CrossSlidingState = CrossSlidingState(6i32);
}
impl ::std::convert::From<i32> for CrossSlidingState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CrossSlidingState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CrossSlidingState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.CrossSlidingState;i4)");
}
impl ::windows::runtime::DefaultType for CrossSlidingState {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct DraggingEventArgs(::windows::runtime::IInspectable);
impl DraggingEventArgs {
    #[cfg(feature = "Devices_Input")]
    #[doc = "*Required features: `UI_Input`, `Devices_Input`*"]
    pub fn PointerDeviceType(&self) -> ::windows::runtime::Result<super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::Input::PointerDeviceType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn DraggingState(&self) -> ::windows::runtime::Result<DraggingState> {
        let this = self;
        unsafe {
            let mut result__: DraggingState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DraggingState>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn ContactCount(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IDraggingEventArgs2>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DraggingEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.DraggingEventArgs;{1c905384-083c-4bd3-b559-179cddeb33ec})");
}
unsafe impl ::windows::runtime::Interface for DraggingEventArgs {
    type Vtable = IDraggingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(479220612, 2108, 19411, [181, 89, 23, 156, 221, 235, 51, 236]);
}
impl ::windows::runtime::RuntimeName for DraggingEventArgs {
    const NAME: &'static str = "Windows.UI.Input.DraggingEventArgs";
}
#[doc = "*Required features: `UI_Input`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DraggingState(pub i32);
impl DraggingState {
    pub const Started: DraggingState = DraggingState(0i32);
    pub const Continuing: DraggingState = DraggingState(1i32);
    pub const Completed: DraggingState = DraggingState(2i32);
}
impl ::std::convert::From<i32> for DraggingState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DraggingState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DraggingState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.DraggingState;i4)");
}
impl ::windows::runtime::DefaultType for DraggingState {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct EdgeGesture(::windows::runtime::IInspectable);
impl EdgeGesture {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn Starting<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<EdgeGesture, EdgeGestureEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn RemoveStarting<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn Completed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<EdgeGesture, EdgeGestureEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn RemoveCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn Canceled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<EdgeGesture, EdgeGestureEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn RemoveCanceled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn GetForCurrentView() -> ::windows::runtime::Result<EdgeGesture> {
        Self::IEdgeGestureStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<EdgeGesture>(result__)
        })
    }
    pub fn IEdgeGestureStatics<R, F: FnOnce(&IEdgeGestureStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<EdgeGesture, IEdgeGestureStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EdgeGesture {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.EdgeGesture;{580d5292-2ab1-49aa-a7f0-33bd3f8df9f1})");
}
unsafe impl ::windows::runtime::Interface for EdgeGesture {
    type Vtable = IEdgeGesture_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1477268114, 10929, 18858, [167, 240, 51, 189, 63, 141, 249, 241]);
}
impl ::windows::runtime::RuntimeName for EdgeGesture {
    const NAME: &'static str = "Windows.UI.Input.EdgeGesture";
}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct EdgeGestureEventArgs(::windows::runtime::IInspectable);
impl EdgeGestureEventArgs {
    #[doc = "*Required features: `UI_Input`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<EdgeGestureKind> {
        let this = self;
        unsafe {
            let mut result__: EdgeGestureKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<EdgeGestureKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EdgeGestureEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.EdgeGestureEventArgs;{44fa4a24-2d09-42e1-8b5e-368208796a4c})");
}
unsafe impl ::windows::runtime::Interface for EdgeGestureEventArgs {
    type Vtable = IEdgeGestureEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1157253668, 11529, 17121, [139, 94, 54, 130, 8, 121, 106, 76]);
}
impl ::windows::runtime::RuntimeName for EdgeGestureEventArgs {
    const NAME: &'static str = "Windows.UI.Input.EdgeGestureEventArgs";
}
#[doc = "*Required features: `UI_Input`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct EdgeGestureKind(pub i32);
impl EdgeGestureKind {
    pub const Touch: EdgeGestureKind = EdgeGestureKind(0i32);
    pub const Keyboard: EdgeGestureKind = EdgeGestureKind(1i32);
    pub const Mouse: EdgeGestureKind = EdgeGestureKind(2i32);
}
impl ::std::convert::From<i32> for EdgeGestureKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EdgeGestureKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for EdgeGestureKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.EdgeGestureKind;i4)");
}
impl ::windows::runtime::DefaultType for EdgeGestureKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Input`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GazeInputAccessStatus(pub i32);
impl GazeInputAccessStatus {
    pub const Unspecified: GazeInputAccessStatus = GazeInputAccessStatus(0i32);
    pub const Allowed: GazeInputAccessStatus = GazeInputAccessStatus(1i32);
    pub const DeniedByUser: GazeInputAccessStatus = GazeInputAccessStatus(2i32);
    pub const DeniedBySystem: GazeInputAccessStatus = GazeInputAccessStatus(3i32);
}
impl ::std::convert::From<i32> for GazeInputAccessStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GazeInputAccessStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GazeInputAccessStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.GazeInputAccessStatus;i4)");
}
impl ::windows::runtime::DefaultType for GazeInputAccessStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct GestureRecognizer(::windows::runtime::IInspectable);
impl GestureRecognizer {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<GestureRecognizer, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn GestureSettings(&self) -> ::windows::runtime::Result<GestureSettings> {
        let this = self;
        unsafe {
            let mut result__: GestureSettings = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GestureSettings>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetGestureSettings(&self, value: GestureSettings) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn IsInertial(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn IsActive(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn ShowGestureFeedback(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetShowGestureFeedback(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn PivotCenter(&self) -> ::windows::runtime::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn SetPivotCenter<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn PivotRadius(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetPivotRadius(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn InertiaTranslationDeceleration(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetInertiaTranslationDeceleration(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn InertiaRotationDeceleration(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetInertiaRotationDeceleration(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn InertiaExpansionDeceleration(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetInertiaExpansionDeceleration(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn InertiaTranslationDisplacement(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetInertiaTranslationDisplacement(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn InertiaRotationAngle(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetInertiaRotationAngle(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn InertiaExpansion(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetInertiaExpansion(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn ManipulationExact(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetManipulationExact(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn CrossSlideThresholds(&self) -> ::windows::runtime::Result<CrossSlideThresholds> {
        let this = self;
        unsafe {
            let mut result__: CrossSlideThresholds = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CrossSlideThresholds>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetCrossSlideThresholds<'a, Param0: ::windows::runtime::IntoParam<'a, CrossSlideThresholds>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn CrossSlideHorizontally(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetCrossSlideHorizontally(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn CrossSlideExact(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetCrossSlideExact(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).35)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn AutoProcessInertia(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetAutoProcessInertia(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).37)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn MouseWheelParameters(&self) -> ::windows::runtime::Result<MouseWheelParameters> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).38)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MouseWheelParameters>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn CanBeDoubleTap<'a, Param0: ::windows::runtime::IntoParam<'a, PointerPoint>>(&self, value: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn ProcessDownEvent<'a, Param0: ::windows::runtime::IntoParam<'a, PointerPoint>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).40)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Input`, `Foundation_Collections`*"]
    pub fn ProcessMoveEvents<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IVector<PointerPoint>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).41)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn ProcessUpEvent<'a, Param0: ::windows::runtime::IntoParam<'a, PointerPoint>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).42)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn ProcessMouseWheelEvent<'a, Param0: ::windows::runtime::IntoParam<'a, PointerPoint>>(&self, value: Param0, isshiftkeydown: bool, iscontrolkeydown: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).43)(::std::mem::transmute_copy(this), value.into_param().abi(), isshiftkeydown, iscontrolkeydown).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn ProcessInertia(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).44)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn CompleteGesture(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).45)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn Tapped<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<GestureRecognizer, TappedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).46)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn RemoveTapped<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).47)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn RightTapped<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<GestureRecognizer, RightTappedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).48)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn RemoveRightTapped<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).49)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn Holding<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<GestureRecognizer, HoldingEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).50)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn RemoveHolding<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).51)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn Dragging<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<GestureRecognizer, DraggingEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).52)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn RemoveDragging<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).53)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn ManipulationStarted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationStartedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).54)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn RemoveManipulationStarted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).55)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn ManipulationUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).56)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn RemoveManipulationUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).57)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn ManipulationInertiaStarting<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationInertiaStartingEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).58)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn RemoveManipulationInertiaStarting<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).59)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn ManipulationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).60)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn RemoveManipulationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).61)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn CrossSliding<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<GestureRecognizer, CrossSlidingEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).62)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn RemoveCrossSliding<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).63)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn TapMinContactCount(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetTapMinContactCount(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn TapMaxContactCount(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetTapMaxContactCount(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn HoldMinContactCount(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetHoldMinContactCount(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn HoldMaxContactCount(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetHoldMaxContactCount(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn HoldRadius(&self) -> ::windows::runtime::Result<f32> {
        let this = &::windows::runtime::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetHoldRadius(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn HoldStartDelay(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::runtime::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn SetHoldStartDelay<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn TranslationMinContactCount(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetTranslationMinContactCount(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn TranslationMaxContactCount(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetTranslationMaxContactCount(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GestureRecognizer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.GestureRecognizer;{b47a37bf-3d6b-4f88-83e8-6dcb4012ffb0})");
}
unsafe impl ::windows::runtime::Interface for GestureRecognizer {
    type Vtable = IGestureRecognizer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3027908543, 15723, 20360, [131, 232, 109, 203, 64, 18, 255, 176]);
}
impl ::windows::runtime::RuntimeName for GestureRecognizer {
    const NAME: &'static str = "Windows.UI.Input.GestureRecognizer";
}
#[doc = "*Required features: `UI_Input`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GestureSettings(pub u32);
impl GestureSettings {
    pub const None: GestureSettings = GestureSettings(0u32);
    pub const Tap: GestureSettings = GestureSettings(1u32);
    pub const DoubleTap: GestureSettings = GestureSettings(2u32);
    pub const Hold: GestureSettings = GestureSettings(4u32);
    pub const HoldWithMouse: GestureSettings = GestureSettings(8u32);
    pub const RightTap: GestureSettings = GestureSettings(16u32);
    pub const Drag: GestureSettings = GestureSettings(32u32);
    pub const ManipulationTranslateX: GestureSettings = GestureSettings(64u32);
    pub const ManipulationTranslateY: GestureSettings = GestureSettings(128u32);
    pub const ManipulationTranslateRailsX: GestureSettings = GestureSettings(256u32);
    pub const ManipulationTranslateRailsY: GestureSettings = GestureSettings(512u32);
    pub const ManipulationRotate: GestureSettings = GestureSettings(1024u32);
    pub const ManipulationScale: GestureSettings = GestureSettings(2048u32);
    pub const ManipulationTranslateInertia: GestureSettings = GestureSettings(4096u32);
    pub const ManipulationRotateInertia: GestureSettings = GestureSettings(8192u32);
    pub const ManipulationScaleInertia: GestureSettings = GestureSettings(16384u32);
    pub const CrossSlide: GestureSettings = GestureSettings(32768u32);
    pub const ManipulationMultipleFingerPanning: GestureSettings = GestureSettings(65536u32);
}
impl ::std::convert::From<u32> for GestureSettings {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GestureSettings {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GestureSettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.GestureSettings;u4)");
}
impl ::windows::runtime::DefaultType for GestureSettings {
    type DefaultType = Self;
}
impl ::std::ops::BitOr for GestureSettings {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for GestureSettings {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for GestureSettings {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for GestureSettings {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for GestureSettings {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct HoldingEventArgs(::windows::runtime::IInspectable);
impl HoldingEventArgs {
    #[cfg(feature = "Devices_Input")]
    #[doc = "*Required features: `UI_Input`, `Devices_Input`*"]
    pub fn PointerDeviceType(&self) -> ::windows::runtime::Result<super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::Input::PointerDeviceType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn HoldingState(&self) -> ::windows::runtime::Result<HoldingState> {
        let this = self;
        unsafe {
            let mut result__: HoldingState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HoldingState>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn ContactCount(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IHoldingEventArgs2>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn CurrentContactCount(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IHoldingEventArgs2>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HoldingEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.HoldingEventArgs;{2bf755c5-e799-41b4-bb40-242f40959b71})");
}
unsafe impl ::windows::runtime::Interface for HoldingEventArgs {
    type Vtable = IHoldingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(737629637, 59289, 16820, [187, 64, 36, 47, 64, 149, 155, 113]);
}
impl ::windows::runtime::RuntimeName for HoldingEventArgs {
    const NAME: &'static str = "Windows.UI.Input.HoldingEventArgs";
}
#[doc = "*Required features: `UI_Input`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HoldingState(pub i32);
impl HoldingState {
    pub const Started: HoldingState = HoldingState(0i32);
    pub const Completed: HoldingState = HoldingState(1i32);
    pub const Canceled: HoldingState = HoldingState(2i32);
}
impl ::std::convert::From<i32> for HoldingState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HoldingState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for HoldingState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.HoldingState;i4)");
}
impl ::windows::runtime::DefaultType for HoldingState {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAttachableInputObject(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAttachableInputObject {
    type Vtable = IAttachableInputObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2608998196, 41921, 21546, [178, 244, 14, 50, 183, 115, 251, 7]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAttachableInputObject_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAttachableInputObjectFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAttachableInputObjectFactory {
    type Vtable = IAttachableInputObjectFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2764393550, 17084, 22778, [166, 64, 234, 21, 22, 244, 192, 107]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAttachableInputObjectFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICrossSlidingEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICrossSlidingEventArgs {
    type Vtable = ICrossSlidingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3912714040, 28552, 16857, [135, 32, 120, 224, 142, 57, 131, 73]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrossSlidingEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Input")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Input"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CrossSlidingState) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICrossSlidingEventArgs2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICrossSlidingEventArgs2 {
    type Vtable = ICrossSlidingEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4009459016, 49264, 23027, [141, 171, 188, 175, 98, 29, 134, 135]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrossSlidingEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDraggingEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDraggingEventArgs {
    type Vtable = IDraggingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(479220612, 2108, 19411, [181, 89, 23, 156, 221, 235, 51, 236]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDraggingEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Input")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Input"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DraggingState) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDraggingEventArgs2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDraggingEventArgs2 {
    type Vtable = IDraggingEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1911544825, 14378, 21962, [180, 185, 0, 129, 35, 193, 191, 26]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDraggingEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEdgeGesture(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEdgeGesture {
    type Vtable = IEdgeGesture_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1477268114, 10929, 18858, [167, 240, 51, 189, 63, 141, 249, 241]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEdgeGesture_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEdgeGestureEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEdgeGestureEventArgs {
    type Vtable = IEdgeGestureEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1157253668, 11529, 17121, [139, 94, 54, 130, 8, 121, 106, 76]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEdgeGestureEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut EdgeGestureKind) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEdgeGestureStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEdgeGestureStatics {
    type Vtable = IEdgeGestureStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3161097497, 6382, 16451, [152, 57, 79, 197, 132, 214, 10, 20]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEdgeGestureStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGestureRecognizer(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGestureRecognizer {
    type Vtable = IGestureRecognizer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3027908543, 15723, 20360, [131, 232, 109, 203, 64, 18, 255, 176]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGestureRecognizer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut GestureSettings) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: GestureSettings) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CrossSlideThresholds) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: CrossSlideThresholds) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, isshiftkeydown: bool, iscontrolkeydown: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGestureRecognizer2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGestureRecognizer2 {
    type Vtable = IGestureRecognizer2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3594914175, 28407, 22342, [139, 168, 143, 242, 32, 110, 111, 59]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGestureRecognizer2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHoldingEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHoldingEventArgs {
    type Vtable = IHoldingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(737629637, 59289, 16820, [187, 64, 36, 47, 64, 149, 155, 113]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHoldingEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Input")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Input"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut HoldingState) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHoldingEventArgs2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHoldingEventArgs2 {
    type Vtable = IHoldingEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(337488362, 19577, 22132, [175, 234, 73, 63, 222, 185, 31, 25]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHoldingEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInputActivationListener(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInputActivationListener {
    type Vtable = IInputActivationListener_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1567444690, 10439, 23267, [170, 116, 201, 24, 169, 242, 67, 202]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputActivationListener_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut InputActivationState) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInputActivationListenerActivationChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInputActivationListenerActivationChangedEventArgs {
    type Vtable = IInputActivationListenerActivationChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1989784677, 7631, 22417, [180, 185, 108, 175, 190, 237, 32, 86]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputActivationListenerActivationChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut InputActivationState) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKeyboardDeliveryInterceptor(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IKeyboardDeliveryInterceptor {
    type Vtable = IKeyboardDeliveryInterceptor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3032150120, 36681, 17516, [141, 181, 140, 15, 254, 133, 204, 158]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardDeliveryInterceptor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKeyboardDeliveryInterceptorStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IKeyboardDeliveryInterceptorStatics {
    type Vtable = IKeyboardDeliveryInterceptorStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4193663906, 52922, 18261, [138, 126, 20, 192, 255, 236, 210, 57]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardDeliveryInterceptorStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IManipulationCompletedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IManipulationCompletedEventArgs {
    type Vtable = IManipulationCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3008016939, 53659, 18175, [159, 56, 222, 199, 117, 75, 185, 231]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationCompletedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Input")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Input"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ManipulationDelta) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ManipulationVelocities) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IManipulationCompletedEventArgs2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IManipulationCompletedEventArgs2 {
    type Vtable = IManipulationCompletedEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4039171303, 12457, 23446, [136, 111, 101, 96, 168, 94, 71, 87]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationCompletedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IManipulationInertiaStartingEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IManipulationInertiaStartingEventArgs {
    type Vtable = IManipulationInertiaStartingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3711412376, 9919, 18042, [156, 229, 204, 243, 251, 17, 55, 30]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationInertiaStartingEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Input")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Input"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ManipulationDelta) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ManipulationDelta) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ManipulationVelocities) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IManipulationInertiaStartingEventArgs2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IManipulationInertiaStartingEventArgs2 {
    type Vtable = IManipulationInertiaStartingEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3260287416, 63994, 23109, [189, 151, 220, 187, 178, 32, 24, 96]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationInertiaStartingEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IManipulationStartedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IManipulationStartedEventArgs {
    type Vtable = IManipulationStartedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3723265854, 53198, 18738, [140, 29, 60, 61, 1, 26, 52, 192]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationStartedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Input")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Input"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ManipulationDelta) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IManipulationStartedEventArgs2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IManipulationStartedEventArgs2 {
    type Vtable = IManipulationStartedEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(765713230, 58755, 20565, [175, 170, 22, 253, 152, 101, 49, 166]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationStartedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IManipulationUpdatedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IManipulationUpdatedEventArgs {
    type Vtable = IManipulationUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3409267941, 43960, 20383, [179, 206, 129, 129, 170, 97, 173, 130]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationUpdatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Input")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Input"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ManipulationDelta) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ManipulationDelta) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ManipulationVelocities) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IManipulationUpdatedEventArgs2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IManipulationUpdatedEventArgs2 {
    type Vtable = IManipulationUpdatedEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4091525482, 13062, 22787, [161, 197, 255, 151, 87, 168, 104, 158]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationUpdatedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMouseWheelParameters(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMouseWheelParameters {
    type Vtable = IMouseWheelParameters_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3939551812, 40429, 16439, [129, 73, 94, 76, 194, 86, 68, 104]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMouseWheelParameters_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPointerPoint(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPointerPoint {
    type Vtable = IPointerPoint_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3918868861, 29334, 17113, [130, 51, 197, 190, 115, 183, 74, 74]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPoint_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Input")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Input"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPointerPointProperties(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPointerPointProperties {
    type Vtable = IPointerPointProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3348990539, 49507, 20199, [128, 63, 103, 206, 121, 249, 151, 45]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPointProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PointerUpdateKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usagepage: u32, usageid: u32, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usagepage: u32, usageid: u32, result__: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPointerPointProperties2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPointerPointProperties2 {
    type Vtable = IPointerPointProperties2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(583222074, 51259, 16832, [162, 150, 94, 35, 45, 100, 214, 175]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPointProperties2_abi(
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
#[doc(hidden)]
pub struct IPointerPointStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPointerPointStatics {
    type Vtable = IPointerPointStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2768659341, 10778, 16702, [188, 117, 159, 56, 56, 28, 192, 105]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPointStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pointerid: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pointerid: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pointerid: u32, transform: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pointerid: u32, transform: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `UI_Input`*"]
pub struct IPointerPointTransform(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPointerPointTransform {
    type Vtable = IPointerPointTransform_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1298129231, 47228, 16424, [188, 156, 89, 233, 148, 127, 176, 86]);
}
impl IPointerPointTransform {
    #[doc = "*Required features: `UI_Input`*"]
    pub fn Inverse(&self) -> ::windows::runtime::Result<IPointerPointTransform> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IPointerPointTransform>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn TryTransform<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Point>>(&self, inpoint: Param0, outpoint: &mut super::super::Foundation::Point) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), inpoint.into_param().abi(), outpoint, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn TransformBounds<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Rect>>(&self, rect: Param0) -> ::windows::runtime::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), rect.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IPointerPointTransform {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{4d5fe14f-b87c-4028-bc9c-59e9947fb056}");
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPointTransform_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inpoint: super::super::Foundation::Point, outpoint: *mut super::super::Foundation::Point, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rect: super::super::Foundation::Rect, result__: *mut super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPointerVisualizationSettings(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPointerVisualizationSettings {
    type Vtable = IPointerVisualizationSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1293837409, 34039, 18845, [189, 145, 42, 54, 226, 183, 170, 162]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerVisualizationSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPointerVisualizationSettingsStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPointerVisualizationSettingsStatics {
    type Vtable = IPointerVisualizationSettingsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1753681627, 5723, 16916, [180, 243, 88, 78, 202, 140, 138, 105]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerVisualizationSettingsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRadialController(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRadialController {
    type Vtable = IRadialController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(810930632, 57169, 17364, [178, 59, 14, 16, 55, 70, 122, 9]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialController_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRadialController2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRadialController2 {
    type Vtable = IRadialController2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1029144319, 19694, 4582, [181, 53, 0, 27, 220, 6, 171, 59]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialController2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRadialControllerButtonClickedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRadialControllerButtonClickedEventArgs {
    type Vtable = IRadialControllerButtonClickedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(543859768, 58961, 4581, [191, 98, 44, 39, 215, 64, 78, 133]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerButtonClickedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRadialControllerButtonClickedEventArgs2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRadialControllerButtonClickedEventArgs2 {
    type Vtable = IRadialControllerButtonClickedEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1029144307, 15598, 4582, [181, 53, 0, 27, 220, 6, 171, 59]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerButtonClickedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Haptics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRadialControllerButtonHoldingEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRadialControllerButtonHoldingEventArgs {
    type Vtable = IRadialControllerButtonHoldingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1029144302, 15598, 4582, [181, 53, 0, 27, 220, 6, 171, 59]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerButtonHoldingEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Haptics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRadialControllerButtonPressedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRadialControllerButtonPressedEventArgs {
    type Vtable = IRadialControllerButtonPressedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1029144301, 19694, 4582, [181, 53, 0, 27, 220, 6, 171, 59]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerButtonPressedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Haptics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRadialControllerButtonReleasedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRadialControllerButtonReleasedEventArgs {
    type Vtable = IRadialControllerButtonReleasedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1029144303, 15598, 4582, [181, 53, 0, 27, 220, 6, 171, 59]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerButtonReleasedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Haptics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRadialControllerConfiguration(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRadialControllerConfiguration {
    type Vtable = IRadialControllerConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2797051595, 27218, 17456, [145, 12, 86, 55, 10, 157, 107, 66]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buttons: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: RadialControllerSystemMenuItemKind, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRadialControllerConfiguration2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRadialControllerConfiguration2 {
    type Vtable = IRadialControllerConfiguration2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1029144311, 15598, 4582, [181, 53, 0, 27, 220, 6, 171, 59]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerConfiguration2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRadialControllerConfigurationStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRadialControllerConfigurationStatics {
    type Vtable = IRadialControllerConfigurationStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2042015973, 1690, 17542, [169, 157, 141, 183, 114, 185, 100, 47]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerConfigurationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRadialControllerConfigurationStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRadialControllerConfigurationStatics2 {
    type Vtable = IRadialControllerConfigurationStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1407224599, 57861, 18643, [156, 175, 128, 255, 71, 196, 215, 199]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerConfigurationStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRadialControllerControlAcquiredEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRadialControllerControlAcquiredEventArgs {
    type Vtable = IRadialControllerControlAcquiredEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(543859769, 58961, 4581, [191, 98, 44, 39, 215, 64, 78, 133]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerControlAcquiredEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRadialControllerControlAcquiredEventArgs2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRadialControllerControlAcquiredEventArgs2 {
    type Vtable = IRadialControllerControlAcquiredEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1029144308, 15598, 4582, [181, 53, 0, 27, 220, 6, 171, 59]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerControlAcquiredEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Haptics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRadialControllerMenu(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRadialControllerMenu {
    type Vtable = IRadialControllerMenu_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2231808861, 63040, 17426, [171, 160, 186, 208, 119, 229, 234, 138]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerMenu_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, menuitem: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRadialControllerMenuItem(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRadialControllerMenuItem {
    type Vtable = IRadialControllerMenuItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3356477837, 44299, 19612, [143, 47, 19, 106, 35, 115, 166, 186]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerMenuItem_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRadialControllerMenuItemStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRadialControllerMenuItemStatics {
    type Vtable = IRadialControllerMenuItemStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(614336647, 55362, 17700, [157, 248, 224, 214, 71, 237, 200, 135]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerMenuItemStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, displaytext: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, icon: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, displaytext: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: RadialControllerMenuKnownIcon, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRadialControllerMenuItemStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRadialControllerMenuItemStatics2 {
    type Vtable = IRadialControllerMenuItemStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(213610686, 32318, 18621, [190, 4, 44, 127, 202, 169, 193, 255]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerMenuItemStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, displaytext: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, glyph: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, fontfamily: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, displaytext: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, glyph: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, fontfamily: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, fonturi: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRadialControllerRotationChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRadialControllerRotationChangedEventArgs {
    type Vtable = IRadialControllerRotationChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(543859765, 58961, 4581, [191, 98, 44, 39, 215, 64, 78, 133]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerRotationChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRadialControllerRotationChangedEventArgs2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRadialControllerRotationChangedEventArgs2 {
    type Vtable = IRadialControllerRotationChangedEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1029144300, 19694, 4582, [181, 53, 0, 27, 220, 6, 171, 59]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerRotationChangedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Haptics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRadialControllerScreenContact(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRadialControllerScreenContact {
    type Vtable = IRadialControllerScreenContact_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(543859764, 58961, 4581, [191, 98, 44, 39, 215, 64, 78, 133]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerScreenContact_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRadialControllerScreenContactContinuedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRadialControllerScreenContactContinuedEventArgs {
    type Vtable = IRadialControllerScreenContactContinuedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(543859767, 58961, 4581, [191, 98, 44, 39, 215, 64, 78, 133]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerScreenContactContinuedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRadialControllerScreenContactContinuedEventArgs2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRadialControllerScreenContactContinuedEventArgs2 {
    type Vtable = IRadialControllerScreenContactContinuedEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1029144305, 15598, 4582, [181, 53, 0, 27, 220, 6, 171, 59]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerScreenContactContinuedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Haptics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRadialControllerScreenContactEndedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRadialControllerScreenContactEndedEventArgs {
    type Vtable = IRadialControllerScreenContactEndedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1029144306, 15598, 4582, [181, 53, 0, 27, 220, 6, 171, 59]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerScreenContactEndedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Haptics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRadialControllerScreenContactStartedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRadialControllerScreenContactStartedEventArgs {
    type Vtable = IRadialControllerScreenContactStartedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(543859766, 58961, 4581, [191, 98, 44, 39, 215, 64, 78, 133]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerScreenContactStartedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRadialControllerScreenContactStartedEventArgs2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRadialControllerScreenContactStartedEventArgs2 {
    type Vtable = IRadialControllerScreenContactStartedEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1029144304, 15598, 4582, [181, 53, 0, 27, 220, 6, 171, 59]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerScreenContactStartedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Haptics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRadialControllerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRadialControllerStatics {
    type Vtable = IRadialControllerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4208906423, 47180, 18580, [135, 170, 143, 37, 170, 95, 40, 139]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRightTappedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRightTappedEventArgs {
    type Vtable = IRightTappedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1287602365, 44922, 18998, [148, 118, 177, 220, 225, 65, 112, 154]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRightTappedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Input")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Input"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRightTappedEventArgs2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRightTappedEventArgs2 {
    type Vtable = IRightTappedEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1640478651, 40791, 22615, [163, 60, 197, 140, 61, 250, 149, 158]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRightTappedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemButtonEventController(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemButtonEventController {
    type Vtable = ISystemButtonEventController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1505268649, 29628, 21173, [186, 65, 130, 81, 27, 44, 180, 108]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemButtonEventController_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemButtonEventControllerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemButtonEventControllerStatics {
    type Vtable = ISystemButtonEventControllerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1664069755, 8381, 24085, [175, 74, 0, 219, 242, 6, 79, 250]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemButtonEventControllerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, queue: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemFunctionButtonEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemFunctionButtonEventArgs {
    type Vtable = ISystemFunctionButtonEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1211337071, 32977, 24022, [146, 167, 98, 165, 8, 255, 239, 90]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemFunctionButtonEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemFunctionLockChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemFunctionLockChangedEventArgs {
    type Vtable = ISystemFunctionLockChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3439592968, 64761, 22620, [190, 171, 241, 210, 234, 243, 100, 171]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemFunctionLockChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemFunctionLockIndicatorChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemFunctionLockIndicatorChangedEventArgs {
    type Vtable = ISystemFunctionLockIndicatorChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2987571534, 31343, 22702, [179, 4, 186, 230, 29, 3, 113, 185]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemFunctionLockIndicatorChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITappedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITappedEventArgs {
    type Vtable = ITappedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3483444964, 9530, 19516, [149, 59, 57, 92, 55, 174, 211, 9]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITappedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Input")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Input"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITappedEventArgs2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITappedEventArgs2 {
    type Vtable = ITappedEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(692291826, 6014, 20949, [190, 86, 238, 8, 102, 250, 150, 140]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITappedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct InputActivationListener(::windows::runtime::IInspectable);
impl InputActivationListener {
    #[doc = "*Required features: `UI_Input`*"]
    pub fn State(&self) -> ::windows::runtime::Result<InputActivationState> {
        let this = self;
        unsafe {
            let mut result__: InputActivationState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InputActivationState>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn InputActivationChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<InputActivationListener, InputActivationListenerActivationChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn RemoveInputActivationChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InputActivationListener {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.InputActivationListener;{5d6d4ed2-28c7-5ae3-aa74-c918a9f243ca})");
}
unsafe impl ::windows::runtime::Interface for InputActivationListener {
    type Vtable = IInputActivationListener_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1567444690, 10439, 23267, [170, 116, 201, 24, 169, 242, 67, 202]);
}
impl ::windows::runtime::RuntimeName for InputActivationListener {
    const NAME: &'static str = "Windows.UI.Input.InputActivationListener";
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<InputActivationListener> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InputActivationListener) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&InputActivationListener> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InputActivationListener) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for InputActivationListener {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &InputActivationListener {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<InputActivationListener> for AttachableInputObject {
    fn from(value: InputActivationListener) -> Self {
        ::std::convert::Into::<AttachableInputObject>::into(&value)
    }
}
impl ::std::convert::From<&InputActivationListener> for AttachableInputObject {
    fn from(value: &InputActivationListener) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, AttachableInputObject> for InputActivationListener {
    fn into_param(self) -> ::windows::runtime::Param<'a, AttachableInputObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<AttachableInputObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, AttachableInputObject> for &InputActivationListener {
    fn into_param(self) -> ::windows::runtime::Param<'a, AttachableInputObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<AttachableInputObject>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for InputActivationListener {}
unsafe impl ::std::marker::Sync for InputActivationListener {}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct InputActivationListenerActivationChangedEventArgs(::windows::runtime::IInspectable);
impl InputActivationListenerActivationChangedEventArgs {
    #[doc = "*Required features: `UI_Input`*"]
    pub fn State(&self) -> ::windows::runtime::Result<InputActivationState> {
        let this = self;
        unsafe {
            let mut result__: InputActivationState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InputActivationState>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InputActivationListenerActivationChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.InputActivationListenerActivationChangedEventArgs;{7699b465-1dcf-5791-b4b9-6cafbeed2056})");
}
unsafe impl ::windows::runtime::Interface for InputActivationListenerActivationChangedEventArgs {
    type Vtable = IInputActivationListenerActivationChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1989784677, 7631, 22417, [180, 185, 108, 175, 190, 237, 32, 86]);
}
impl ::windows::runtime::RuntimeName for InputActivationListenerActivationChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.InputActivationListenerActivationChangedEventArgs";
}
unsafe impl ::std::marker::Send for InputActivationListenerActivationChangedEventArgs {}
unsafe impl ::std::marker::Sync for InputActivationListenerActivationChangedEventArgs {}
#[doc = "*Required features: `UI_Input`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InputActivationState(pub i32);
impl InputActivationState {
    pub const None: InputActivationState = InputActivationState(0i32);
    pub const Deactivated: InputActivationState = InputActivationState(1i32);
    pub const ActivatedNotForeground: InputActivationState = InputActivationState(2i32);
    pub const ActivatedInForeground: InputActivationState = InputActivationState(3i32);
}
impl ::std::convert::From<i32> for InputActivationState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InputActivationState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InputActivationState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.InputActivationState;i4)");
}
impl ::windows::runtime::DefaultType for InputActivationState {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct KeyboardDeliveryInterceptor(::windows::runtime::IInspectable);
impl KeyboardDeliveryInterceptor {
    #[doc = "*Required features: `UI_Input`*"]
    pub fn IsInterceptionEnabledWhenInForeground(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetIsInterceptionEnabledWhenInForeground(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    #[doc = "*Required features: `UI_Input`, `Foundation`, `UI_Core`*"]
    pub fn KeyDown<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<KeyboardDeliveryInterceptor, super::Core::KeyEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn RemoveKeyDown<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    #[doc = "*Required features: `UI_Input`, `Foundation`, `UI_Core`*"]
    pub fn KeyUp<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<KeyboardDeliveryInterceptor, super::Core::KeyEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn RemoveKeyUp<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn GetForCurrentView() -> ::windows::runtime::Result<KeyboardDeliveryInterceptor> {
        Self::IKeyboardDeliveryInterceptorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<KeyboardDeliveryInterceptor>(result__)
        })
    }
    pub fn IKeyboardDeliveryInterceptorStatics<R, F: FnOnce(&IKeyboardDeliveryInterceptorStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<KeyboardDeliveryInterceptor, IKeyboardDeliveryInterceptorStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for KeyboardDeliveryInterceptor {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.KeyboardDeliveryInterceptor;{b4baf068-8f49-446c-8db5-8c0ffe85cc9e})");
}
unsafe impl ::windows::runtime::Interface for KeyboardDeliveryInterceptor {
    type Vtable = IKeyboardDeliveryInterceptor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3032150120, 36681, 17516, [141, 181, 140, 15, 254, 133, 204, 158]);
}
impl ::windows::runtime::RuntimeName for KeyboardDeliveryInterceptor {
    const NAME: &'static str = "Windows.UI.Input.KeyboardDeliveryInterceptor";
}
unsafe impl ::std::marker::Send for KeyboardDeliveryInterceptor {}
unsafe impl ::std::marker::Sync for KeyboardDeliveryInterceptor {}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ManipulationCompletedEventArgs(::windows::runtime::IInspectable);
impl ManipulationCompletedEventArgs {
    #[cfg(feature = "Devices_Input")]
    #[doc = "*Required features: `UI_Input`, `Devices_Input`*"]
    pub fn PointerDeviceType(&self) -> ::windows::runtime::Result<super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::Input::PointerDeviceType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn Cumulative(&self) -> ::windows::runtime::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: ManipulationDelta = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ManipulationDelta>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn Velocities(&self) -> ::windows::runtime::Result<ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__: ManipulationVelocities = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ManipulationVelocities>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn ContactCount(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IManipulationCompletedEventArgs2>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn CurrentContactCount(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IManipulationCompletedEventArgs2>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ManipulationCompletedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.ManipulationCompletedEventArgs;{b34ab22b-d19b-46ff-9f38-dec7754bb9e7})");
}
unsafe impl ::windows::runtime::Interface for ManipulationCompletedEventArgs {
    type Vtable = IManipulationCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3008016939, 53659, 18175, [159, 56, 222, 199, 117, 75, 185, 231]);
}
impl ::windows::runtime::RuntimeName for ManipulationCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.ManipulationCompletedEventArgs";
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Foundation")]
#[doc = "*Required features: `UI_Input`, `Foundation`*"]
pub struct ManipulationDelta {
    pub Translation: super::super::Foundation::Point,
    pub Scale: f32,
    pub Rotation: f32,
    pub Expansion: f32,
}
#[cfg(feature = "Foundation")]
impl ManipulationDelta {}
#[cfg(feature = "Foundation")]
impl ::std::default::Default for ManipulationDelta {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation")]
impl ::std::fmt::Debug for ManipulationDelta {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ManipulationDelta").field("Translation", &self.Translation).field("Scale", &self.Scale).field("Rotation", &self.Rotation).field("Expansion", &self.Expansion).finish()
    }
}
#[cfg(feature = "Foundation")]
impl ::std::cmp::PartialEq for ManipulationDelta {
    fn eq(&self, other: &Self) -> bool {
        self.Translation == other.Translation && self.Scale == other.Scale && self.Rotation == other.Rotation && self.Expansion == other.Expansion
    }
}
#[cfg(feature = "Foundation")]
impl ::std::cmp::Eq for ManipulationDelta {}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::runtime::Abi for ManipulationDelta {
    type Abi = Self;
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::runtime::RuntimeType for ManipulationDelta {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.UI.Input.ManipulationDelta;struct(Windows.Foundation.Point;f4;f4);f4;f4;f4)");
}
#[cfg(feature = "Foundation")]
impl ::windows::runtime::DefaultType for ManipulationDelta {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ManipulationInertiaStartingEventArgs(::windows::runtime::IInspectable);
impl ManipulationInertiaStartingEventArgs {
    #[cfg(feature = "Devices_Input")]
    #[doc = "*Required features: `UI_Input`, `Devices_Input`*"]
    pub fn PointerDeviceType(&self) -> ::windows::runtime::Result<super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::Input::PointerDeviceType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn Delta(&self) -> ::windows::runtime::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: ManipulationDelta = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ManipulationDelta>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn Cumulative(&self) -> ::windows::runtime::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: ManipulationDelta = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ManipulationDelta>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn Velocities(&self) -> ::windows::runtime::Result<ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__: ManipulationVelocities = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ManipulationVelocities>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn ContactCount(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IManipulationInertiaStartingEventArgs2>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ManipulationInertiaStartingEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.ManipulationInertiaStartingEventArgs;{dd37a898-26bf-467a-9ce5-ccf3fb11371e})");
}
unsafe impl ::windows::runtime::Interface for ManipulationInertiaStartingEventArgs {
    type Vtable = IManipulationInertiaStartingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3711412376, 9919, 18042, [156, 229, 204, 243, 251, 17, 55, 30]);
}
impl ::windows::runtime::RuntimeName for ManipulationInertiaStartingEventArgs {
    const NAME: &'static str = "Windows.UI.Input.ManipulationInertiaStartingEventArgs";
}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ManipulationStartedEventArgs(::windows::runtime::IInspectable);
impl ManipulationStartedEventArgs {
    #[cfg(feature = "Devices_Input")]
    #[doc = "*Required features: `UI_Input`, `Devices_Input`*"]
    pub fn PointerDeviceType(&self) -> ::windows::runtime::Result<super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::Input::PointerDeviceType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn Cumulative(&self) -> ::windows::runtime::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: ManipulationDelta = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ManipulationDelta>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn ContactCount(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IManipulationStartedEventArgs2>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ManipulationStartedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.ManipulationStartedEventArgs;{ddec873e-cfce-4932-8c1d-3c3d011a34c0})");
}
unsafe impl ::windows::runtime::Interface for ManipulationStartedEventArgs {
    type Vtable = IManipulationStartedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3723265854, 53198, 18738, [140, 29, 60, 61, 1, 26, 52, 192]);
}
impl ::windows::runtime::RuntimeName for ManipulationStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.ManipulationStartedEventArgs";
}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ManipulationUpdatedEventArgs(::windows::runtime::IInspectable);
impl ManipulationUpdatedEventArgs {
    #[cfg(feature = "Devices_Input")]
    #[doc = "*Required features: `UI_Input`, `Devices_Input`*"]
    pub fn PointerDeviceType(&self) -> ::windows::runtime::Result<super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::Input::PointerDeviceType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn Delta(&self) -> ::windows::runtime::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: ManipulationDelta = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ManipulationDelta>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn Cumulative(&self) -> ::windows::runtime::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: ManipulationDelta = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ManipulationDelta>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn Velocities(&self) -> ::windows::runtime::Result<ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__: ManipulationVelocities = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ManipulationVelocities>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn ContactCount(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IManipulationUpdatedEventArgs2>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn CurrentContactCount(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IManipulationUpdatedEventArgs2>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ManipulationUpdatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.ManipulationUpdatedEventArgs;{cb354ce5-abb8-4f9f-b3ce-8181aa61ad82})");
}
unsafe impl ::windows::runtime::Interface for ManipulationUpdatedEventArgs {
    type Vtable = IManipulationUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3409267941, 43960, 20383, [179, 206, 129, 129, 170, 97, 173, 130]);
}
impl ::windows::runtime::RuntimeName for ManipulationUpdatedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.ManipulationUpdatedEventArgs";
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Foundation")]
#[doc = "*Required features: `UI_Input`, `Foundation`*"]
pub struct ManipulationVelocities {
    pub Linear: super::super::Foundation::Point,
    pub Angular: f32,
    pub Expansion: f32,
}
#[cfg(feature = "Foundation")]
impl ManipulationVelocities {}
#[cfg(feature = "Foundation")]
impl ::std::default::Default for ManipulationVelocities {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation")]
impl ::std::fmt::Debug for ManipulationVelocities {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ManipulationVelocities").field("Linear", &self.Linear).field("Angular", &self.Angular).field("Expansion", &self.Expansion).finish()
    }
}
#[cfg(feature = "Foundation")]
impl ::std::cmp::PartialEq for ManipulationVelocities {
    fn eq(&self, other: &Self) -> bool {
        self.Linear == other.Linear && self.Angular == other.Angular && self.Expansion == other.Expansion
    }
}
#[cfg(feature = "Foundation")]
impl ::std::cmp::Eq for ManipulationVelocities {}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::runtime::Abi for ManipulationVelocities {
    type Abi = Self;
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::runtime::RuntimeType for ManipulationVelocities {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.UI.Input.ManipulationVelocities;struct(Windows.Foundation.Point;f4;f4);f4;f4)");
}
#[cfg(feature = "Foundation")]
impl ::windows::runtime::DefaultType for ManipulationVelocities {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct MouseWheelParameters(::windows::runtime::IInspectable);
impl MouseWheelParameters {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn CharTranslation(&self) -> ::windows::runtime::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn SetCharTranslation<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn DeltaScale(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetDeltaScale(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn DeltaRotationAngle(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetDeltaRotationAngle(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn PageTranslation(&self) -> ::windows::runtime::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn SetPageTranslation<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MouseWheelParameters {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.MouseWheelParameters;{ead0ca44-9ded-4037-8149-5e4cc2564468})");
}
unsafe impl ::windows::runtime::Interface for MouseWheelParameters {
    type Vtable = IMouseWheelParameters_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3939551812, 40429, 16439, [129, 73, 94, 76, 194, 86, 68, 104]);
}
impl ::windows::runtime::RuntimeName for MouseWheelParameters {
    const NAME: &'static str = "Windows.UI.Input.MouseWheelParameters";
}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct PointerPoint(::windows::runtime::IInspectable);
impl PointerPoint {
    #[cfg(feature = "Devices_Input")]
    #[doc = "*Required features: `UI_Input`, `Devices_Input`*"]
    pub fn PointerDevice(&self) -> ::windows::runtime::Result<super::super::Devices::Input::PointerDevice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Input::PointerDevice>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn RawPosition(&self) -> ::windows::runtime::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn PointerId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn FrameId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn IsInContact(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<PointerPointProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PointerPointProperties>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn GetCurrentPoint(pointerid: u32) -> ::windows::runtime::Result<PointerPoint> {
        Self::IPointerPointStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), pointerid, &mut result__).from_abi::<PointerPoint>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Input`, `Foundation_Collections`*"]
    pub fn GetIntermediatePoints(pointerid: u32) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<PointerPoint>> {
        Self::IPointerPointStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), pointerid, &mut result__).from_abi::<super::super::Foundation::Collections::IVector<PointerPoint>>(result__)
        })
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn GetCurrentPointTransformed<'a, Param1: ::windows::runtime::IntoParam<'a, IPointerPointTransform>>(pointerid: u32, transform: Param1) -> ::windows::runtime::Result<PointerPoint> {
        Self::IPointerPointStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), pointerid, transform.into_param().abi(), &mut result__).from_abi::<PointerPoint>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Input`, `Foundation_Collections`*"]
    pub fn GetIntermediatePointsTransformed<'a, Param1: ::windows::runtime::IntoParam<'a, IPointerPointTransform>>(pointerid: u32, transform: Param1) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<PointerPoint>> {
        Self::IPointerPointStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), pointerid, transform.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<PointerPoint>>(result__)
        })
    }
    pub fn IPointerPointStatics<R, F: FnOnce(&IPointerPointStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PointerPoint, IPointerPointStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PointerPoint {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.PointerPoint;{e995317d-7296-42d9-8233-c5be73b74a4a})");
}
unsafe impl ::windows::runtime::Interface for PointerPoint {
    type Vtable = IPointerPoint_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3918868861, 29334, 17113, [130, 51, 197, 190, 115, 183, 74, 74]);
}
impl ::windows::runtime::RuntimeName for PointerPoint {
    const NAME: &'static str = "Windows.UI.Input.PointerPoint";
}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct PointerPointProperties(::windows::runtime::IInspectable);
impl PointerPointProperties {
    #[doc = "*Required features: `UI_Input`*"]
    pub fn Pressure(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn IsInverted(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn IsEraser(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn Orientation(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn XTilt(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn YTilt(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn Twist(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn ContactRect(&self) -> ::windows::runtime::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn ContactRectRaw(&self) -> ::windows::runtime::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn TouchConfidence(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn IsLeftButtonPressed(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn IsRightButtonPressed(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn IsMiddleButtonPressed(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn MouseWheelDelta(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn IsHorizontalMouseWheel(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn IsPrimary(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn IsInRange(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn IsCanceled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn IsBarrelButtonPressed(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn IsXButton1Pressed(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn IsXButton2Pressed(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn PointerUpdateKind(&self) -> ::windows::runtime::Result<PointerUpdateKind> {
        let this = self;
        unsafe {
            let mut result__: PointerUpdateKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PointerUpdateKind>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn HasUsage(&self, usagepage: u32, usageid: u32) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), usagepage, usageid, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn GetUsageValue(&self, usagepage: u32, usageid: u32) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), usagepage, usageid, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn ZDistance(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f32>> {
        let this = &::windows::runtime::Interface::cast::<IPointerPointProperties2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f32>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PointerPointProperties {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.PointerPointProperties;{c79d8a4b-c163-4ee7-803f-67ce79f9972d})");
}
unsafe impl ::windows::runtime::Interface for PointerPointProperties {
    type Vtable = IPointerPointProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3348990539, 49507, 20199, [128, 63, 103, 206, 121, 249, 151, 45]);
}
impl ::windows::runtime::RuntimeName for PointerPointProperties {
    const NAME: &'static str = "Windows.UI.Input.PointerPointProperties";
}
#[doc = "*Required features: `UI_Input`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PointerUpdateKind(pub i32);
impl PointerUpdateKind {
    pub const Other: PointerUpdateKind = PointerUpdateKind(0i32);
    pub const LeftButtonPressed: PointerUpdateKind = PointerUpdateKind(1i32);
    pub const LeftButtonReleased: PointerUpdateKind = PointerUpdateKind(2i32);
    pub const RightButtonPressed: PointerUpdateKind = PointerUpdateKind(3i32);
    pub const RightButtonReleased: PointerUpdateKind = PointerUpdateKind(4i32);
    pub const MiddleButtonPressed: PointerUpdateKind = PointerUpdateKind(5i32);
    pub const MiddleButtonReleased: PointerUpdateKind = PointerUpdateKind(6i32);
    pub const XButton1Pressed: PointerUpdateKind = PointerUpdateKind(7i32);
    pub const XButton1Released: PointerUpdateKind = PointerUpdateKind(8i32);
    pub const XButton2Pressed: PointerUpdateKind = PointerUpdateKind(9i32);
    pub const XButton2Released: PointerUpdateKind = PointerUpdateKind(10i32);
}
impl ::std::convert::From<i32> for PointerUpdateKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PointerUpdateKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PointerUpdateKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.PointerUpdateKind;i4)");
}
impl ::windows::runtime::DefaultType for PointerUpdateKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct PointerVisualizationSettings(::windows::runtime::IInspectable);
impl PointerVisualizationSettings {
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetIsContactFeedbackEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn IsContactFeedbackEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetIsBarrelButtonFeedbackEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn IsBarrelButtonFeedbackEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn GetForCurrentView() -> ::windows::runtime::Result<PointerVisualizationSettings> {
        Self::IPointerVisualizationSettingsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PointerVisualizationSettings>(result__)
        })
    }
    pub fn IPointerVisualizationSettingsStatics<R, F: FnOnce(&IPointerVisualizationSettingsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PointerVisualizationSettings, IPointerVisualizationSettingsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PointerVisualizationSettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.PointerVisualizationSettings;{4d1e6461-84f7-499d-bd91-2a36e2b7aaa2})");
}
unsafe impl ::windows::runtime::Interface for PointerVisualizationSettings {
    type Vtable = IPointerVisualizationSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1293837409, 34039, 18845, [189, 145, 42, 54, 226, 183, 170, 162]);
}
impl ::windows::runtime::RuntimeName for PointerVisualizationSettings {
    const NAME: &'static str = "Windows.UI.Input.PointerVisualizationSettings";
}
unsafe impl ::std::marker::Send for PointerVisualizationSettings {}
unsafe impl ::std::marker::Sync for PointerVisualizationSettings {}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct RadialController(::windows::runtime::IInspectable);
impl RadialController {
    #[doc = "*Required features: `UI_Input`*"]
    pub fn Menu(&self) -> ::windows::runtime::Result<RadialControllerMenu> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<RadialControllerMenu>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn RotationResolutionInDegrees(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetRotationResolutionInDegrees(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn UseAutomaticHapticFeedback(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetUseAutomaticHapticFeedback(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn ScreenContactStarted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<RadialController, RadialControllerScreenContactStartedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn RemoveScreenContactStarted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn ScreenContactEnded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<RadialController, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn RemoveScreenContactEnded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn ScreenContactContinued<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<RadialController, RadialControllerScreenContactContinuedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn RemoveScreenContactContinued<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn ControlLost<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<RadialController, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn RemoveControlLost<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn RotationChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<RadialController, RadialControllerRotationChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn RemoveRotationChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn ButtonClicked<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonClickedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn RemoveButtonClicked<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn ControlAcquired<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<RadialController, RadialControllerControlAcquiredEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn RemoveControlAcquired<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn IsSupported() -> ::windows::runtime::Result<bool> {
        Self::IRadialControllerStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn CreateForCurrentView() -> ::windows::runtime::Result<RadialController> {
        Self::IRadialControllerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<RadialController>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn ButtonPressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonPressedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IRadialController2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn RemoveButtonPressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IRadialController2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn ButtonHolding<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonHoldingEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IRadialController2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn RemoveButtonHolding<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IRadialController2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn ButtonReleased<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonReleasedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IRadialController2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn RemoveButtonReleased<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IRadialController2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn IRadialControllerStatics<R, F: FnOnce(&IRadialControllerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RadialController, IRadialControllerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RadialController {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialController;{3055d1c8-df51-43d4-b23b-0e1037467a09})");
}
unsafe impl ::windows::runtime::Interface for RadialController {
    type Vtable = IRadialController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(810930632, 57169, 17364, [178, 59, 14, 16, 55, 70, 122, 9]);
}
impl ::windows::runtime::RuntimeName for RadialController {
    const NAME: &'static str = "Windows.UI.Input.RadialController";
}
unsafe impl ::std::marker::Send for RadialController {}
unsafe impl ::std::marker::Sync for RadialController {}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct RadialControllerButtonClickedEventArgs(::windows::runtime::IInspectable);
impl RadialControllerButtonClickedEventArgs {
    #[doc = "*Required features: `UI_Input`*"]
    pub fn Contact(&self) -> ::windows::runtime::Result<RadialControllerScreenContact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<RadialControllerScreenContact>(result__)
        }
    }
    #[cfg(feature = "Devices_Haptics")]
    #[doc = "*Required features: `UI_Input`, `Devices_Haptics`*"]
    pub fn SimpleHapticsController(&self) -> ::windows::runtime::Result<super::super::Devices::Haptics::SimpleHapticsController> {
        let this = &::windows::runtime::Interface::cast::<IRadialControllerButtonClickedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Haptics::SimpleHapticsController>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RadialControllerButtonClickedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerButtonClickedEventArgs;{206aa438-e651-11e5-bf62-2c27d7404e85})");
}
unsafe impl ::windows::runtime::Interface for RadialControllerButtonClickedEventArgs {
    type Vtable = IRadialControllerButtonClickedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(543859768, 58961, 4581, [191, 98, 44, 39, 215, 64, 78, 133]);
}
impl ::windows::runtime::RuntimeName for RadialControllerButtonClickedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerButtonClickedEventArgs";
}
unsafe impl ::std::marker::Send for RadialControllerButtonClickedEventArgs {}
unsafe impl ::std::marker::Sync for RadialControllerButtonClickedEventArgs {}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct RadialControllerButtonHoldingEventArgs(::windows::runtime::IInspectable);
impl RadialControllerButtonHoldingEventArgs {
    #[doc = "*Required features: `UI_Input`*"]
    pub fn Contact(&self) -> ::windows::runtime::Result<RadialControllerScreenContact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<RadialControllerScreenContact>(result__)
        }
    }
    #[cfg(feature = "Devices_Haptics")]
    #[doc = "*Required features: `UI_Input`, `Devices_Haptics`*"]
    pub fn SimpleHapticsController(&self) -> ::windows::runtime::Result<super::super::Devices::Haptics::SimpleHapticsController> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Haptics::SimpleHapticsController>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RadialControllerButtonHoldingEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerButtonHoldingEventArgs;{3d577eee-3cee-11e6-b535-001bdc06ab3b})");
}
unsafe impl ::windows::runtime::Interface for RadialControllerButtonHoldingEventArgs {
    type Vtable = IRadialControllerButtonHoldingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1029144302, 15598, 4582, [181, 53, 0, 27, 220, 6, 171, 59]);
}
impl ::windows::runtime::RuntimeName for RadialControllerButtonHoldingEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerButtonHoldingEventArgs";
}
unsafe impl ::std::marker::Send for RadialControllerButtonHoldingEventArgs {}
unsafe impl ::std::marker::Sync for RadialControllerButtonHoldingEventArgs {}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct RadialControllerButtonPressedEventArgs(::windows::runtime::IInspectable);
impl RadialControllerButtonPressedEventArgs {
    #[doc = "*Required features: `UI_Input`*"]
    pub fn Contact(&self) -> ::windows::runtime::Result<RadialControllerScreenContact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<RadialControllerScreenContact>(result__)
        }
    }
    #[cfg(feature = "Devices_Haptics")]
    #[doc = "*Required features: `UI_Input`, `Devices_Haptics`*"]
    pub fn SimpleHapticsController(&self) -> ::windows::runtime::Result<super::super::Devices::Haptics::SimpleHapticsController> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Haptics::SimpleHapticsController>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RadialControllerButtonPressedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerButtonPressedEventArgs;{3d577eed-4cee-11e6-b535-001bdc06ab3b})");
}
unsafe impl ::windows::runtime::Interface for RadialControllerButtonPressedEventArgs {
    type Vtable = IRadialControllerButtonPressedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1029144301, 19694, 4582, [181, 53, 0, 27, 220, 6, 171, 59]);
}
impl ::windows::runtime::RuntimeName for RadialControllerButtonPressedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerButtonPressedEventArgs";
}
unsafe impl ::std::marker::Send for RadialControllerButtonPressedEventArgs {}
unsafe impl ::std::marker::Sync for RadialControllerButtonPressedEventArgs {}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct RadialControllerButtonReleasedEventArgs(::windows::runtime::IInspectable);
impl RadialControllerButtonReleasedEventArgs {
    #[doc = "*Required features: `UI_Input`*"]
    pub fn Contact(&self) -> ::windows::runtime::Result<RadialControllerScreenContact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<RadialControllerScreenContact>(result__)
        }
    }
    #[cfg(feature = "Devices_Haptics")]
    #[doc = "*Required features: `UI_Input`, `Devices_Haptics`*"]
    pub fn SimpleHapticsController(&self) -> ::windows::runtime::Result<super::super::Devices::Haptics::SimpleHapticsController> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Haptics::SimpleHapticsController>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RadialControllerButtonReleasedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerButtonReleasedEventArgs;{3d577eef-3cee-11e6-b535-001bdc06ab3b})");
}
unsafe impl ::windows::runtime::Interface for RadialControllerButtonReleasedEventArgs {
    type Vtable = IRadialControllerButtonReleasedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1029144303, 15598, 4582, [181, 53, 0, 27, 220, 6, 171, 59]);
}
impl ::windows::runtime::RuntimeName for RadialControllerButtonReleasedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerButtonReleasedEventArgs";
}
unsafe impl ::std::marker::Send for RadialControllerButtonReleasedEventArgs {}
unsafe impl ::std::marker::Sync for RadialControllerButtonReleasedEventArgs {}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct RadialControllerConfiguration(::windows::runtime::IInspectable);
impl RadialControllerConfiguration {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Input`, `Foundation_Collections`*"]
    pub fn SetDefaultMenuItems<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<RadialControllerSystemMenuItemKind>>>(&self, buttons: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), buttons.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn ResetToDefaultMenuItems(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn TrySelectDefaultMenuItem(&self, r#type: RadialControllerSystemMenuItemKind) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), r#type, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn GetForCurrentView() -> ::windows::runtime::Result<RadialControllerConfiguration> {
        Self::IRadialControllerConfigurationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<RadialControllerConfiguration>(result__)
        })
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetActiveControllerWhenMenuIsSuppressed<'a, Param0: ::windows::runtime::IntoParam<'a, RadialController>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IRadialControllerConfiguration2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn ActiveControllerWhenMenuIsSuppressed(&self) -> ::windows::runtime::Result<RadialController> {
        let this = &::windows::runtime::Interface::cast::<IRadialControllerConfiguration2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<RadialController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetIsMenuSuppressed(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IRadialControllerConfiguration2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn IsMenuSuppressed(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IRadialControllerConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetAppController<'a, Param0: ::windows::runtime::IntoParam<'a, RadialController>>(value: Param0) -> ::windows::runtime::Result<()> {
        Self::IRadialControllerConfigurationStatics2(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn AppController() -> ::windows::runtime::Result<RadialController> {
        Self::IRadialControllerConfigurationStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<RadialController>(result__)
        })
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetIsAppControllerEnabled(value: bool) -> ::windows::runtime::Result<()> {
        Self::IRadialControllerConfigurationStatics2(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() })
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn IsAppControllerEnabled() -> ::windows::runtime::Result<bool> {
        Self::IRadialControllerConfigurationStatics2(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IRadialControllerConfigurationStatics<R, F: FnOnce(&IRadialControllerConfigurationStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RadialControllerConfiguration, IRadialControllerConfigurationStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IRadialControllerConfigurationStatics2<R, F: FnOnce(&IRadialControllerConfigurationStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RadialControllerConfiguration, IRadialControllerConfigurationStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RadialControllerConfiguration {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerConfiguration;{a6b79ecb-6a52-4430-910c-56370a9d6b42})");
}
unsafe impl ::windows::runtime::Interface for RadialControllerConfiguration {
    type Vtable = IRadialControllerConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2797051595, 27218, 17456, [145, 12, 86, 55, 10, 157, 107, 66]);
}
impl ::windows::runtime::RuntimeName for RadialControllerConfiguration {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerConfiguration";
}
unsafe impl ::std::marker::Send for RadialControllerConfiguration {}
unsafe impl ::std::marker::Sync for RadialControllerConfiguration {}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct RadialControllerControlAcquiredEventArgs(::windows::runtime::IInspectable);
impl RadialControllerControlAcquiredEventArgs {
    #[doc = "*Required features: `UI_Input`*"]
    pub fn Contact(&self) -> ::windows::runtime::Result<RadialControllerScreenContact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<RadialControllerScreenContact>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn IsButtonPressed(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IRadialControllerControlAcquiredEventArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Devices_Haptics")]
    #[doc = "*Required features: `UI_Input`, `Devices_Haptics`*"]
    pub fn SimpleHapticsController(&self) -> ::windows::runtime::Result<super::super::Devices::Haptics::SimpleHapticsController> {
        let this = &::windows::runtime::Interface::cast::<IRadialControllerControlAcquiredEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Haptics::SimpleHapticsController>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RadialControllerControlAcquiredEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerControlAcquiredEventArgs;{206aa439-e651-11e5-bf62-2c27d7404e85})");
}
unsafe impl ::windows::runtime::Interface for RadialControllerControlAcquiredEventArgs {
    type Vtable = IRadialControllerControlAcquiredEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(543859769, 58961, 4581, [191, 98, 44, 39, 215, 64, 78, 133]);
}
impl ::windows::runtime::RuntimeName for RadialControllerControlAcquiredEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerControlAcquiredEventArgs";
}
unsafe impl ::std::marker::Send for RadialControllerControlAcquiredEventArgs {}
unsafe impl ::std::marker::Sync for RadialControllerControlAcquiredEventArgs {}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct RadialControllerMenu(::windows::runtime::IInspectable);
impl RadialControllerMenu {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Input`, `Foundation_Collections`*"]
    pub fn Items(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<RadialControllerMenuItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<RadialControllerMenuItem>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn IsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn GetSelectedMenuItem(&self) -> ::windows::runtime::Result<RadialControllerMenuItem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<RadialControllerMenuItem>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SelectMenuItem<'a, Param0: ::windows::runtime::IntoParam<'a, RadialControllerMenuItem>>(&self, menuitem: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), menuitem.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn TrySelectPreviouslySelectedMenuItem(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RadialControllerMenu {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerMenu;{8506b35d-f640-4412-aba0-bad077e5ea8a})");
}
unsafe impl ::windows::runtime::Interface for RadialControllerMenu {
    type Vtable = IRadialControllerMenu_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2231808861, 63040, 17426, [171, 160, 186, 208, 119, 229, 234, 138]);
}
impl ::windows::runtime::RuntimeName for RadialControllerMenu {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerMenu";
}
unsafe impl ::std::marker::Send for RadialControllerMenu {}
unsafe impl ::std::marker::Sync for RadialControllerMenu {}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct RadialControllerMenuItem(::windows::runtime::IInspectable);
impl RadialControllerMenuItem {
    #[doc = "*Required features: `UI_Input`*"]
    pub fn DisplayText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn Tag(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetTag<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn Invoked<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<RadialControllerMenuItem, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn RemoveInvoked<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `UI_Input`, `Storage_Streams`*"]
    pub fn CreateFromIcon<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::RandomAccessStreamReference>>(displaytext: Param0, icon: Param1) -> ::windows::runtime::Result<RadialControllerMenuItem> {
        Self::IRadialControllerMenuItemStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), displaytext.into_param().abi(), icon.into_param().abi(), &mut result__).from_abi::<RadialControllerMenuItem>(result__)
        })
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn CreateFromKnownIcon<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(displaytext: Param0, value: RadialControllerMenuKnownIcon) -> ::windows::runtime::Result<RadialControllerMenuItem> {
        Self::IRadialControllerMenuItemStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), displaytext.into_param().abi(), value, &mut result__).from_abi::<RadialControllerMenuItem>(result__)
        })
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn CreateFromFontGlyph<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(displaytext: Param0, glyph: Param1, fontfamily: Param2) -> ::windows::runtime::Result<RadialControllerMenuItem> {
        Self::IRadialControllerMenuItemStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), displaytext.into_param().abi(), glyph.into_param().abi(), fontfamily.into_param().abi(), &mut result__).from_abi::<RadialControllerMenuItem>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn CreateFromFontGlyphWithUri<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(displaytext: Param0, glyph: Param1, fontfamily: Param2, fonturi: Param3) -> ::windows::runtime::Result<RadialControllerMenuItem> {
        Self::IRadialControllerMenuItemStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), displaytext.into_param().abi(), glyph.into_param().abi(), fontfamily.into_param().abi(), fonturi.into_param().abi(), &mut result__).from_abi::<RadialControllerMenuItem>(result__)
        })
    }
    pub fn IRadialControllerMenuItemStatics<R, F: FnOnce(&IRadialControllerMenuItemStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RadialControllerMenuItem, IRadialControllerMenuItemStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IRadialControllerMenuItemStatics2<R, F: FnOnce(&IRadialControllerMenuItemStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RadialControllerMenuItem, IRadialControllerMenuItemStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RadialControllerMenuItem {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerMenuItem;{c80fc98d-ad0b-4c9c-8f2f-136a2373a6ba})");
}
unsafe impl ::windows::runtime::Interface for RadialControllerMenuItem {
    type Vtable = IRadialControllerMenuItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3356477837, 44299, 19612, [143, 47, 19, 106, 35, 115, 166, 186]);
}
impl ::windows::runtime::RuntimeName for RadialControllerMenuItem {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerMenuItem";
}
unsafe impl ::std::marker::Send for RadialControllerMenuItem {}
unsafe impl ::std::marker::Sync for RadialControllerMenuItem {}
#[doc = "*Required features: `UI_Input`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RadialControllerMenuKnownIcon(pub i32);
impl RadialControllerMenuKnownIcon {
    pub const Scroll: RadialControllerMenuKnownIcon = RadialControllerMenuKnownIcon(0i32);
    pub const Zoom: RadialControllerMenuKnownIcon = RadialControllerMenuKnownIcon(1i32);
    pub const UndoRedo: RadialControllerMenuKnownIcon = RadialControllerMenuKnownIcon(2i32);
    pub const Volume: RadialControllerMenuKnownIcon = RadialControllerMenuKnownIcon(3i32);
    pub const NextPreviousTrack: RadialControllerMenuKnownIcon = RadialControllerMenuKnownIcon(4i32);
    pub const Ruler: RadialControllerMenuKnownIcon = RadialControllerMenuKnownIcon(5i32);
    pub const InkColor: RadialControllerMenuKnownIcon = RadialControllerMenuKnownIcon(6i32);
    pub const InkThickness: RadialControllerMenuKnownIcon = RadialControllerMenuKnownIcon(7i32);
    pub const PenType: RadialControllerMenuKnownIcon = RadialControllerMenuKnownIcon(8i32);
}
impl ::std::convert::From<i32> for RadialControllerMenuKnownIcon {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RadialControllerMenuKnownIcon {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RadialControllerMenuKnownIcon {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.RadialControllerMenuKnownIcon;i4)");
}
impl ::windows::runtime::DefaultType for RadialControllerMenuKnownIcon {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct RadialControllerRotationChangedEventArgs(::windows::runtime::IInspectable);
impl RadialControllerRotationChangedEventArgs {
    #[doc = "*Required features: `UI_Input`*"]
    pub fn RotationDeltaInDegrees(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn Contact(&self) -> ::windows::runtime::Result<RadialControllerScreenContact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<RadialControllerScreenContact>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn IsButtonPressed(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IRadialControllerRotationChangedEventArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Devices_Haptics")]
    #[doc = "*Required features: `UI_Input`, `Devices_Haptics`*"]
    pub fn SimpleHapticsController(&self) -> ::windows::runtime::Result<super::super::Devices::Haptics::SimpleHapticsController> {
        let this = &::windows::runtime::Interface::cast::<IRadialControllerRotationChangedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Haptics::SimpleHapticsController>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RadialControllerRotationChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerRotationChangedEventArgs;{206aa435-e651-11e5-bf62-2c27d7404e85})");
}
unsafe impl ::windows::runtime::Interface for RadialControllerRotationChangedEventArgs {
    type Vtable = IRadialControllerRotationChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(543859765, 58961, 4581, [191, 98, 44, 39, 215, 64, 78, 133]);
}
impl ::windows::runtime::RuntimeName for RadialControllerRotationChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerRotationChangedEventArgs";
}
unsafe impl ::std::marker::Send for RadialControllerRotationChangedEventArgs {}
unsafe impl ::std::marker::Sync for RadialControllerRotationChangedEventArgs {}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct RadialControllerScreenContact(::windows::runtime::IInspectable);
impl RadialControllerScreenContact {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn Bounds(&self) -> ::windows::runtime::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RadialControllerScreenContact {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerScreenContact;{206aa434-e651-11e5-bf62-2c27d7404e85})");
}
unsafe impl ::windows::runtime::Interface for RadialControllerScreenContact {
    type Vtable = IRadialControllerScreenContact_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(543859764, 58961, 4581, [191, 98, 44, 39, 215, 64, 78, 133]);
}
impl ::windows::runtime::RuntimeName for RadialControllerScreenContact {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerScreenContact";
}
unsafe impl ::std::marker::Send for RadialControllerScreenContact {}
unsafe impl ::std::marker::Sync for RadialControllerScreenContact {}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct RadialControllerScreenContactContinuedEventArgs(::windows::runtime::IInspectable);
impl RadialControllerScreenContactContinuedEventArgs {
    #[doc = "*Required features: `UI_Input`*"]
    pub fn Contact(&self) -> ::windows::runtime::Result<RadialControllerScreenContact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<RadialControllerScreenContact>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn IsButtonPressed(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IRadialControllerScreenContactContinuedEventArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Devices_Haptics")]
    #[doc = "*Required features: `UI_Input`, `Devices_Haptics`*"]
    pub fn SimpleHapticsController(&self) -> ::windows::runtime::Result<super::super::Devices::Haptics::SimpleHapticsController> {
        let this = &::windows::runtime::Interface::cast::<IRadialControllerScreenContactContinuedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Haptics::SimpleHapticsController>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RadialControllerScreenContactContinuedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerScreenContactContinuedEventArgs;{206aa437-e651-11e5-bf62-2c27d7404e85})");
}
unsafe impl ::windows::runtime::Interface for RadialControllerScreenContactContinuedEventArgs {
    type Vtable = IRadialControllerScreenContactContinuedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(543859767, 58961, 4581, [191, 98, 44, 39, 215, 64, 78, 133]);
}
impl ::windows::runtime::RuntimeName for RadialControllerScreenContactContinuedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerScreenContactContinuedEventArgs";
}
unsafe impl ::std::marker::Send for RadialControllerScreenContactContinuedEventArgs {}
unsafe impl ::std::marker::Sync for RadialControllerScreenContactContinuedEventArgs {}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct RadialControllerScreenContactEndedEventArgs(::windows::runtime::IInspectable);
impl RadialControllerScreenContactEndedEventArgs {
    #[doc = "*Required features: `UI_Input`*"]
    pub fn IsButtonPressed(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Devices_Haptics")]
    #[doc = "*Required features: `UI_Input`, `Devices_Haptics`*"]
    pub fn SimpleHapticsController(&self) -> ::windows::runtime::Result<super::super::Devices::Haptics::SimpleHapticsController> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Haptics::SimpleHapticsController>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RadialControllerScreenContactEndedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerScreenContactEndedEventArgs;{3d577ef2-3cee-11e6-b535-001bdc06ab3b})");
}
unsafe impl ::windows::runtime::Interface for RadialControllerScreenContactEndedEventArgs {
    type Vtable = IRadialControllerScreenContactEndedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1029144306, 15598, 4582, [181, 53, 0, 27, 220, 6, 171, 59]);
}
impl ::windows::runtime::RuntimeName for RadialControllerScreenContactEndedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerScreenContactEndedEventArgs";
}
unsafe impl ::std::marker::Send for RadialControllerScreenContactEndedEventArgs {}
unsafe impl ::std::marker::Sync for RadialControllerScreenContactEndedEventArgs {}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct RadialControllerScreenContactStartedEventArgs(::windows::runtime::IInspectable);
impl RadialControllerScreenContactStartedEventArgs {
    #[doc = "*Required features: `UI_Input`*"]
    pub fn Contact(&self) -> ::windows::runtime::Result<RadialControllerScreenContact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<RadialControllerScreenContact>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn IsButtonPressed(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IRadialControllerScreenContactStartedEventArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Devices_Haptics")]
    #[doc = "*Required features: `UI_Input`, `Devices_Haptics`*"]
    pub fn SimpleHapticsController(&self) -> ::windows::runtime::Result<super::super::Devices::Haptics::SimpleHapticsController> {
        let this = &::windows::runtime::Interface::cast::<IRadialControllerScreenContactStartedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Haptics::SimpleHapticsController>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RadialControllerScreenContactStartedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerScreenContactStartedEventArgs;{206aa436-e651-11e5-bf62-2c27d7404e85})");
}
unsafe impl ::windows::runtime::Interface for RadialControllerScreenContactStartedEventArgs {
    type Vtable = IRadialControllerScreenContactStartedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(543859766, 58961, 4581, [191, 98, 44, 39, 215, 64, 78, 133]);
}
impl ::windows::runtime::RuntimeName for RadialControllerScreenContactStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerScreenContactStartedEventArgs";
}
unsafe impl ::std::marker::Send for RadialControllerScreenContactStartedEventArgs {}
unsafe impl ::std::marker::Sync for RadialControllerScreenContactStartedEventArgs {}
#[doc = "*Required features: `UI_Input`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RadialControllerSystemMenuItemKind(pub i32);
impl RadialControllerSystemMenuItemKind {
    pub const Scroll: RadialControllerSystemMenuItemKind = RadialControllerSystemMenuItemKind(0i32);
    pub const Zoom: RadialControllerSystemMenuItemKind = RadialControllerSystemMenuItemKind(1i32);
    pub const UndoRedo: RadialControllerSystemMenuItemKind = RadialControllerSystemMenuItemKind(2i32);
    pub const Volume: RadialControllerSystemMenuItemKind = RadialControllerSystemMenuItemKind(3i32);
    pub const NextPreviousTrack: RadialControllerSystemMenuItemKind = RadialControllerSystemMenuItemKind(4i32);
}
impl ::std::convert::From<i32> for RadialControllerSystemMenuItemKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RadialControllerSystemMenuItemKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RadialControllerSystemMenuItemKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.RadialControllerSystemMenuItemKind;i4)");
}
impl ::windows::runtime::DefaultType for RadialControllerSystemMenuItemKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct RightTappedEventArgs(::windows::runtime::IInspectable);
impl RightTappedEventArgs {
    #[cfg(feature = "Devices_Input")]
    #[doc = "*Required features: `UI_Input`, `Devices_Input`*"]
    pub fn PointerDeviceType(&self) -> ::windows::runtime::Result<super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::Input::PointerDeviceType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn ContactCount(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IRightTappedEventArgs2>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RightTappedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RightTappedEventArgs;{4cbf40bd-af7a-4a36-9476-b1dce141709a})");
}
unsafe impl ::windows::runtime::Interface for RightTappedEventArgs {
    type Vtable = IRightTappedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1287602365, 44922, 18998, [148, 118, 177, 220, 225, 65, 112, 154]);
}
impl ::windows::runtime::RuntimeName for RightTappedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RightTappedEventArgs";
}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct SystemButtonEventController(::windows::runtime::IInspectable);
impl SystemButtonEventController {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn SystemFunctionButtonPressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionButtonEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn RemoveSystemFunctionButtonPressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn SystemFunctionButtonReleased<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionButtonEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn RemoveSystemFunctionButtonReleased<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn SystemFunctionLockChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionLockChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn RemoveSystemFunctionLockChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn SystemFunctionLockIndicatorChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionLockIndicatorChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn RemoveSystemFunctionLockIndicatorChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Input`, `System`*"]
    pub fn CreateForDispatcherQueue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::DispatcherQueue>>(queue: Param0) -> ::windows::runtime::Result<SystemButtonEventController> {
        Self::ISystemButtonEventControllerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), queue.into_param().abi(), &mut result__).from_abi::<SystemButtonEventController>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn ISystemButtonEventControllerStatics<R, F: FnOnce(&ISystemButtonEventControllerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SystemButtonEventController, ISystemButtonEventControllerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SystemButtonEventController {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.SystemButtonEventController;{59b893a9-73bc-52b5-ba41-82511b2cb46c})");
}
unsafe impl ::windows::runtime::Interface for SystemButtonEventController {
    type Vtable = ISystemButtonEventController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1505268649, 29628, 21173, [186, 65, 130, 81, 27, 44, 180, 108]);
}
impl ::windows::runtime::RuntimeName for SystemButtonEventController {
    const NAME: &'static str = "Windows.UI.Input.SystemButtonEventController";
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<SystemButtonEventController> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SystemButtonEventController) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&SystemButtonEventController> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SystemButtonEventController) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for SystemButtonEventController {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &SystemButtonEventController {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<SystemButtonEventController> for AttachableInputObject {
    fn from(value: SystemButtonEventController) -> Self {
        ::std::convert::Into::<AttachableInputObject>::into(&value)
    }
}
impl ::std::convert::From<&SystemButtonEventController> for AttachableInputObject {
    fn from(value: &SystemButtonEventController) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, AttachableInputObject> for SystemButtonEventController {
    fn into_param(self) -> ::windows::runtime::Param<'a, AttachableInputObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<AttachableInputObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, AttachableInputObject> for &SystemButtonEventController {
    fn into_param(self) -> ::windows::runtime::Param<'a, AttachableInputObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<AttachableInputObject>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for SystemButtonEventController {}
unsafe impl ::std::marker::Sync for SystemButtonEventController {}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct SystemFunctionButtonEventArgs(::windows::runtime::IInspectable);
impl SystemFunctionButtonEventArgs {
    #[doc = "*Required features: `UI_Input`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SystemFunctionButtonEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.SystemFunctionButtonEventArgs;{4833896f-80d1-5dd6-92a7-62a508ffef5a})");
}
unsafe impl ::windows::runtime::Interface for SystemFunctionButtonEventArgs {
    type Vtable = ISystemFunctionButtonEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1211337071, 32977, 24022, [146, 167, 98, 165, 8, 255, 239, 90]);
}
impl ::windows::runtime::RuntimeName for SystemFunctionButtonEventArgs {
    const NAME: &'static str = "Windows.UI.Input.SystemFunctionButtonEventArgs";
}
unsafe impl ::std::marker::Send for SystemFunctionButtonEventArgs {}
unsafe impl ::std::marker::Sync for SystemFunctionButtonEventArgs {}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct SystemFunctionLockChangedEventArgs(::windows::runtime::IInspectable);
impl SystemFunctionLockChangedEventArgs {
    #[doc = "*Required features: `UI_Input`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn IsLocked(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SystemFunctionLockChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.SystemFunctionLockChangedEventArgs;{cd040608-fcf9-585c-beab-f1d2eaf364ab})");
}
unsafe impl ::windows::runtime::Interface for SystemFunctionLockChangedEventArgs {
    type Vtable = ISystemFunctionLockChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3439592968, 64761, 22620, [190, 171, 241, 210, 234, 243, 100, 171]);
}
impl ::windows::runtime::RuntimeName for SystemFunctionLockChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.SystemFunctionLockChangedEventArgs";
}
unsafe impl ::std::marker::Send for SystemFunctionLockChangedEventArgs {}
unsafe impl ::std::marker::Sync for SystemFunctionLockChangedEventArgs {}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct SystemFunctionLockIndicatorChangedEventArgs(::windows::runtime::IInspectable);
impl SystemFunctionLockIndicatorChangedEventArgs {
    #[doc = "*Required features: `UI_Input`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn IsIndicatorOn(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SystemFunctionLockIndicatorChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.SystemFunctionLockIndicatorChangedEventArgs;{b212b94e-7a6f-58ae-b304-bae61d0371b9})");
}
unsafe impl ::windows::runtime::Interface for SystemFunctionLockIndicatorChangedEventArgs {
    type Vtable = ISystemFunctionLockIndicatorChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2987571534, 31343, 22702, [179, 4, 186, 230, 29, 3, 113, 185]);
}
impl ::windows::runtime::RuntimeName for SystemFunctionLockIndicatorChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.SystemFunctionLockIndicatorChangedEventArgs";
}
unsafe impl ::std::marker::Send for SystemFunctionLockIndicatorChangedEventArgs {}
unsafe impl ::std::marker::Sync for SystemFunctionLockIndicatorChangedEventArgs {}
#[doc = "*Required features: `UI_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct TappedEventArgs(::windows::runtime::IInspectable);
impl TappedEventArgs {
    #[cfg(feature = "Devices_Input")]
    #[doc = "*Required features: `UI_Input`, `Devices_Input`*"]
    pub fn PointerDeviceType(&self) -> ::windows::runtime::Result<super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::Input::PointerDeviceType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input`, `Foundation`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn TapCount(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input`*"]
    pub fn ContactCount(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<ITappedEventArgs2>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TappedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.TappedEventArgs;{cfa126e4-253a-4c3c-953b-395c37aed309})");
}
unsafe impl ::windows::runtime::Interface for TappedEventArgs {
    type Vtable = ITappedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3483444964, 9530, 19516, [149, 59, 57, 92, 55, 174, 211, 9]);
}
impl ::windows::runtime::RuntimeName for TappedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.TappedEventArgs";
}
