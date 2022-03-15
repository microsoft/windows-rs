#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "UI_Input_Core")]
pub mod Core;
#[cfg(feature = "UI_Input_Inking")]
pub mod Inking;
#[cfg(feature = "UI_Input_Preview")]
pub mod Preview;
#[cfg(feature = "UI_Input_Spatial")]
pub mod Spatial;
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct AttachableInputObject(::windows::core::IUnknown);
impl AttachableInputObject {
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for AttachableInputObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AttachableInputObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AttachableInputObject {}
impl ::core::fmt::Debug for AttachableInputObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AttachableInputObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AttachableInputObject {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.AttachableInputObject;{9b822734-a3c1-542a-b2f4-0e32b773fb07})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AttachableInputObject {
    type Vtable = IAttachableInputObject_Vtbl;
    const IID: ::windows::core::GUID = <IAttachableInputObject as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AttachableInputObject {
    const NAME: &'static str = "Windows.UI.Input.AttachableInputObject";
}
impl ::core::convert::From<AttachableInputObject> for ::windows::core::IUnknown {
    fn from(value: AttachableInputObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AttachableInputObject> for ::windows::core::IUnknown {
    fn from(value: &AttachableInputObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AttachableInputObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AttachableInputObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AttachableInputObject> for ::windows::core::IInspectable {
    fn from(value: AttachableInputObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AttachableInputObject> for ::windows::core::IInspectable {
    fn from(value: &AttachableInputObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AttachableInputObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AttachableInputObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<AttachableInputObject> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: AttachableInputObject) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&AttachableInputObject> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &AttachableInputObject) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for AttachableInputObject {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &AttachableInputObject {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AttachableInputObject {}
unsafe impl ::core::marker::Sync for AttachableInputObject {}
#[repr(C)]
#[doc = "*Required features: `\"UI_Input\"`*"]
pub struct CrossSlideThresholds {
    pub SelectionStart: f32,
    pub SpeedBumpStart: f32,
    pub SpeedBumpEnd: f32,
    pub RearrangeStart: f32,
}
impl ::core::marker::Copy for CrossSlideThresholds {}
impl ::core::clone::Clone for CrossSlideThresholds {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CrossSlideThresholds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CrossSlideThresholds").field("SelectionStart", &self.SelectionStart).field("SpeedBumpStart", &self.SpeedBumpStart).field("SpeedBumpEnd", &self.SpeedBumpEnd).field("RearrangeStart", &self.RearrangeStart).finish()
    }
}
unsafe impl ::windows::core::Abi for CrossSlideThresholds {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CrossSlideThresholds {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.UI.Input.CrossSlideThresholds;f4;f4;f4;f4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for CrossSlideThresholds {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CrossSlideThresholds>()) == 0 }
    }
}
impl ::core::cmp::Eq for CrossSlideThresholds {}
impl ::core::default::Default for CrossSlideThresholds {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct CrossSlidingEventArgs(::windows::core::IUnknown);
impl CrossSlidingEventArgs {
    #[doc = "*Required features: `\"UI_Input\"`, `\"Devices_Input\"`*"]
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerDeviceType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn CrossSlidingState(&self) -> ::windows::core::Result<CrossSlidingState> {
        let this = self;
        unsafe {
            let mut result__: CrossSlidingState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CrossSlidingState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CrossSlidingState>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn ContactCount(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICrossSlidingEventArgs2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContactCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for CrossSlidingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CrossSlidingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CrossSlidingEventArgs {}
impl ::core::fmt::Debug for CrossSlidingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CrossSlidingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CrossSlidingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.CrossSlidingEventArgs;{e9374738-6f88-41d9-8720-78e08e398349})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CrossSlidingEventArgs {
    type Vtable = ICrossSlidingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ICrossSlidingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CrossSlidingEventArgs {
    const NAME: &'static str = "Windows.UI.Input.CrossSlidingEventArgs";
}
impl ::core::convert::From<CrossSlidingEventArgs> for ::windows::core::IUnknown {
    fn from(value: CrossSlidingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CrossSlidingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CrossSlidingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CrossSlidingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CrossSlidingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CrossSlidingEventArgs> for ::windows::core::IInspectable {
    fn from(value: CrossSlidingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CrossSlidingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CrossSlidingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CrossSlidingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CrossSlidingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CrossSlidingState(pub i32);
impl CrossSlidingState {
    pub const Started: Self = Self(0i32);
    pub const Dragging: Self = Self(1i32);
    pub const Selecting: Self = Self(2i32);
    pub const SelectSpeedBumping: Self = Self(3i32);
    pub const SpeedBumping: Self = Self(4i32);
    pub const Rearranging: Self = Self(5i32);
    pub const Completed: Self = Self(6i32);
}
impl ::core::marker::Copy for CrossSlidingState {}
impl ::core::clone::Clone for CrossSlidingState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CrossSlidingState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CrossSlidingState {
    type Abi = Self;
}
impl ::core::fmt::Debug for CrossSlidingState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CrossSlidingState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CrossSlidingState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.CrossSlidingState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct DraggingEventArgs(::windows::core::IUnknown);
impl DraggingEventArgs {
    #[doc = "*Required features: `\"UI_Input\"`, `\"Devices_Input\"`*"]
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerDeviceType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn DraggingState(&self) -> ::windows::core::Result<DraggingState> {
        let this = self;
        unsafe {
            let mut result__: DraggingState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DraggingState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DraggingState>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn ContactCount(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IDraggingEventArgs2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContactCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for DraggingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DraggingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DraggingEventArgs {}
impl ::core::fmt::Debug for DraggingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DraggingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DraggingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.DraggingEventArgs;{1c905384-083c-4bd3-b559-179cddeb33ec})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DraggingEventArgs {
    type Vtable = IDraggingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IDraggingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DraggingEventArgs {
    const NAME: &'static str = "Windows.UI.Input.DraggingEventArgs";
}
impl ::core::convert::From<DraggingEventArgs> for ::windows::core::IUnknown {
    fn from(value: DraggingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DraggingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DraggingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DraggingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DraggingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DraggingEventArgs> for ::windows::core::IInspectable {
    fn from(value: DraggingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DraggingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DraggingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DraggingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DraggingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DraggingState(pub i32);
impl DraggingState {
    pub const Started: Self = Self(0i32);
    pub const Continuing: Self = Self(1i32);
    pub const Completed: Self = Self(2i32);
}
impl ::core::marker::Copy for DraggingState {}
impl ::core::clone::Clone for DraggingState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DraggingState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DraggingState {
    type Abi = Self;
}
impl ::core::fmt::Debug for DraggingState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DraggingState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DraggingState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.DraggingState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct EdgeGesture(::windows::core::IUnknown);
impl EdgeGesture {
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Starting<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<EdgeGesture, EdgeGestureEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Starting)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStarting<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStarting)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Completed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<EdgeGesture, EdgeGestureEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Completed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCompleted)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Canceled<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<EdgeGesture, EdgeGestureEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Canceled)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCanceled<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCanceled)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn GetForCurrentView() -> ::windows::core::Result<EdgeGesture> {
        Self::IEdgeGestureStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EdgeGesture>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IEdgeGestureStatics<R, F: FnOnce(&IEdgeGestureStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EdgeGesture, IEdgeGestureStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for EdgeGesture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EdgeGesture {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EdgeGesture {}
impl ::core::fmt::Debug for EdgeGesture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EdgeGesture").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EdgeGesture {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.EdgeGesture;{580d5292-2ab1-49aa-a7f0-33bd3f8df9f1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for EdgeGesture {
    type Vtable = IEdgeGesture_Vtbl;
    const IID: ::windows::core::GUID = <IEdgeGesture as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EdgeGesture {
    const NAME: &'static str = "Windows.UI.Input.EdgeGesture";
}
impl ::core::convert::From<EdgeGesture> for ::windows::core::IUnknown {
    fn from(value: EdgeGesture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EdgeGesture> for ::windows::core::IUnknown {
    fn from(value: &EdgeGesture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EdgeGesture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EdgeGesture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EdgeGesture> for ::windows::core::IInspectable {
    fn from(value: EdgeGesture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EdgeGesture> for ::windows::core::IInspectable {
    fn from(value: &EdgeGesture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EdgeGesture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EdgeGesture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct EdgeGestureEventArgs(::windows::core::IUnknown);
impl EdgeGestureEventArgs {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<EdgeGestureKind> {
        let this = self;
        unsafe {
            let mut result__: EdgeGestureKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EdgeGestureKind>(result__)
        }
    }
}
impl ::core::clone::Clone for EdgeGestureEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EdgeGestureEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EdgeGestureEventArgs {}
impl ::core::fmt::Debug for EdgeGestureEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EdgeGestureEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EdgeGestureEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.EdgeGestureEventArgs;{44fa4a24-2d09-42e1-8b5e-368208796a4c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for EdgeGestureEventArgs {
    type Vtable = IEdgeGestureEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IEdgeGestureEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EdgeGestureEventArgs {
    const NAME: &'static str = "Windows.UI.Input.EdgeGestureEventArgs";
}
impl ::core::convert::From<EdgeGestureEventArgs> for ::windows::core::IUnknown {
    fn from(value: EdgeGestureEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EdgeGestureEventArgs> for ::windows::core::IUnknown {
    fn from(value: &EdgeGestureEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EdgeGestureEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EdgeGestureEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EdgeGestureEventArgs> for ::windows::core::IInspectable {
    fn from(value: EdgeGestureEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EdgeGestureEventArgs> for ::windows::core::IInspectable {
    fn from(value: &EdgeGestureEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EdgeGestureEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EdgeGestureEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EdgeGestureKind(pub i32);
impl EdgeGestureKind {
    pub const Touch: Self = Self(0i32);
    pub const Keyboard: Self = Self(1i32);
    pub const Mouse: Self = Self(2i32);
}
impl ::core::marker::Copy for EdgeGestureKind {}
impl ::core::clone::Clone for EdgeGestureKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EdgeGestureKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EdgeGestureKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for EdgeGestureKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EdgeGestureKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EdgeGestureKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.EdgeGestureKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GazeInputAccessStatus(pub i32);
impl GazeInputAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
impl ::core::marker::Copy for GazeInputAccessStatus {}
impl ::core::clone::Clone for GazeInputAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GazeInputAccessStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GazeInputAccessStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for GazeInputAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GazeInputAccessStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GazeInputAccessStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.GazeInputAccessStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct GestureRecognizer(::windows::core::IUnknown);
impl GestureRecognizer {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GestureRecognizer, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn GestureSettings(&self) -> ::windows::core::Result<GestureSettings> {
        let this = self;
        unsafe {
            let mut result__: GestureSettings = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GestureSettings)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GestureSettings>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetGestureSettings(&self, value: GestureSettings) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetGestureSettings)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsInertial(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsInertial)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsActive(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsActive)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn ShowGestureFeedback(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowGestureFeedback)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetShowGestureFeedback(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetShowGestureFeedback)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PivotCenter(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PivotCenter)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPivotCenter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPivotCenter)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn PivotRadius(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PivotRadius)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetPivotRadius(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPivotRadius)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn InertiaTranslationDeceleration(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InertiaTranslationDeceleration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetInertiaTranslationDeceleration(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInertiaTranslationDeceleration)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn InertiaRotationDeceleration(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InertiaRotationDeceleration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetInertiaRotationDeceleration(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInertiaRotationDeceleration)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn InertiaExpansionDeceleration(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InertiaExpansionDeceleration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetInertiaExpansionDeceleration(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInertiaExpansionDeceleration)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn InertiaTranslationDisplacement(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InertiaTranslationDisplacement)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetInertiaTranslationDisplacement(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInertiaTranslationDisplacement)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn InertiaRotationAngle(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InertiaRotationAngle)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetInertiaRotationAngle(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInertiaRotationAngle)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn InertiaExpansion(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InertiaExpansion)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetInertiaExpansion(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInertiaExpansion)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn ManipulationExact(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ManipulationExact)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetManipulationExact(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetManipulationExact)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn CrossSlideThresholds(&self) -> ::windows::core::Result<CrossSlideThresholds> {
        let this = self;
        unsafe {
            let mut result__: CrossSlideThresholds = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CrossSlideThresholds)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CrossSlideThresholds>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetCrossSlideThresholds<'a, Param0: ::windows::core::IntoParam<'a, CrossSlideThresholds>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCrossSlideThresholds)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn CrossSlideHorizontally(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CrossSlideHorizontally)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetCrossSlideHorizontally(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCrossSlideHorizontally)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn CrossSlideExact(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CrossSlideExact)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetCrossSlideExact(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCrossSlideExact)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn AutoProcessInertia(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AutoProcessInertia)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetAutoProcessInertia(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAutoProcessInertia)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn MouseWheelParameters(&self) -> ::windows::core::Result<MouseWheelParameters> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MouseWheelParameters)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MouseWheelParameters>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn CanBeDoubleTap<'a, Param0: ::windows::core::IntoParam<'a, PointerPoint>>(&self, value: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanBeDoubleTap)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn ProcessDownEvent<'a, Param0: ::windows::core::IntoParam<'a, PointerPoint>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ProcessDownEvent)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProcessMoveEvents<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVector<PointerPoint>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ProcessMoveEvents)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn ProcessUpEvent<'a, Param0: ::windows::core::IntoParam<'a, PointerPoint>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ProcessUpEvent)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn ProcessMouseWheelEvent<'a, Param0: ::windows::core::IntoParam<'a, PointerPoint>>(&self, value: Param0, isshiftkeydown: bool, iscontrolkeydown: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ProcessMouseWheelEvent)(::core::mem::transmute_copy(this), value.into_param().abi(), isshiftkeydown, iscontrolkeydown).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn ProcessInertia(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ProcessInertia)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn CompleteGesture(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).CompleteGesture)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Tapped<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<GestureRecognizer, TappedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Tapped)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTapped<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveTapped)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RightTapped<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<GestureRecognizer, RightTappedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RightTapped)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRightTapped<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveRightTapped)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Holding<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<GestureRecognizer, HoldingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Holding)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveHolding<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveHolding)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Dragging<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<GestureRecognizer, DraggingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Dragging)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDragging<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDragging)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ManipulationStarted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationStartedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ManipulationStarted)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveManipulationStarted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveManipulationStarted)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ManipulationUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ManipulationUpdated)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveManipulationUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveManipulationUpdated)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ManipulationInertiaStarting<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationInertiaStartingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ManipulationInertiaStarting)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveManipulationInertiaStarting<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveManipulationInertiaStarting)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ManipulationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ManipulationCompleted)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveManipulationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveManipulationCompleted)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CrossSliding<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<GestureRecognizer, CrossSlidingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CrossSliding)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCrossSliding<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCrossSliding)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn TapMinContactCount(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TapMinContactCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetTapMinContactCount(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetTapMinContactCount)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn TapMaxContactCount(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TapMaxContactCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetTapMaxContactCount(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetTapMaxContactCount)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn HoldMinContactCount(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HoldMinContactCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetHoldMinContactCount(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetHoldMinContactCount)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn HoldMaxContactCount(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HoldMaxContactCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetHoldMaxContactCount(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetHoldMaxContactCount)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn HoldRadius(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HoldRadius)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetHoldRadius(&self, value: f32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetHoldRadius)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HoldStartDelay(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HoldStartDelay)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetHoldStartDelay<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetHoldStartDelay)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn TranslationMinContactCount(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TranslationMinContactCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetTranslationMinContactCount(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetTranslationMinContactCount)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn TranslationMaxContactCount(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TranslationMaxContactCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetTranslationMaxContactCount(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetTranslationMaxContactCount)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for GestureRecognizer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GestureRecognizer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GestureRecognizer {}
impl ::core::fmt::Debug for GestureRecognizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GestureRecognizer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GestureRecognizer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.GestureRecognizer;{b47a37bf-3d6b-4f88-83e8-6dcb4012ffb0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for GestureRecognizer {
    type Vtable = IGestureRecognizer_Vtbl;
    const IID: ::windows::core::GUID = <IGestureRecognizer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GestureRecognizer {
    const NAME: &'static str = "Windows.UI.Input.GestureRecognizer";
}
impl ::core::convert::From<GestureRecognizer> for ::windows::core::IUnknown {
    fn from(value: GestureRecognizer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GestureRecognizer> for ::windows::core::IUnknown {
    fn from(value: &GestureRecognizer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GestureRecognizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GestureRecognizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GestureRecognizer> for ::windows::core::IInspectable {
    fn from(value: GestureRecognizer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GestureRecognizer> for ::windows::core::IInspectable {
    fn from(value: &GestureRecognizer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GestureRecognizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GestureRecognizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GestureSettings(pub u32);
impl GestureSettings {
    pub const None: Self = Self(0u32);
    pub const Tap: Self = Self(1u32);
    pub const DoubleTap: Self = Self(2u32);
    pub const Hold: Self = Self(4u32);
    pub const HoldWithMouse: Self = Self(8u32);
    pub const RightTap: Self = Self(16u32);
    pub const Drag: Self = Self(32u32);
    pub const ManipulationTranslateX: Self = Self(64u32);
    pub const ManipulationTranslateY: Self = Self(128u32);
    pub const ManipulationTranslateRailsX: Self = Self(256u32);
    pub const ManipulationTranslateRailsY: Self = Self(512u32);
    pub const ManipulationRotate: Self = Self(1024u32);
    pub const ManipulationScale: Self = Self(2048u32);
    pub const ManipulationTranslateInertia: Self = Self(4096u32);
    pub const ManipulationRotateInertia: Self = Self(8192u32);
    pub const ManipulationScaleInertia: Self = Self(16384u32);
    pub const CrossSlide: Self = Self(32768u32);
    pub const ManipulationMultipleFingerPanning: Self = Self(65536u32);
}
impl ::core::marker::Copy for GestureSettings {}
impl ::core::clone::Clone for GestureSettings {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GestureSettings {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GestureSettings {
    type Abi = Self;
}
impl ::core::fmt::Debug for GestureSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GestureSettings").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for GestureSettings {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GestureSettings {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GestureSettings {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GestureSettings {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GestureSettings {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for GestureSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.GestureSettings;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct HoldingEventArgs(::windows::core::IUnknown);
impl HoldingEventArgs {
    #[doc = "*Required features: `\"UI_Input\"`, `\"Devices_Input\"`*"]
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerDeviceType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn HoldingState(&self) -> ::windows::core::Result<HoldingState> {
        let this = self;
        unsafe {
            let mut result__: HoldingState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HoldingState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HoldingState>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn ContactCount(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IHoldingEventArgs2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContactCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn CurrentContactCount(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IHoldingEventArgs2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CurrentContactCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for HoldingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HoldingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HoldingEventArgs {}
impl ::core::fmt::Debug for HoldingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HoldingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HoldingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.HoldingEventArgs;{2bf755c5-e799-41b4-bb40-242f40959b71})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HoldingEventArgs {
    type Vtable = IHoldingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IHoldingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HoldingEventArgs {
    const NAME: &'static str = "Windows.UI.Input.HoldingEventArgs";
}
impl ::core::convert::From<HoldingEventArgs> for ::windows::core::IUnknown {
    fn from(value: HoldingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HoldingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &HoldingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HoldingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HoldingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HoldingEventArgs> for ::windows::core::IInspectable {
    fn from(value: HoldingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HoldingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &HoldingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HoldingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HoldingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HoldingState(pub i32);
impl HoldingState {
    pub const Started: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const Canceled: Self = Self(2i32);
}
impl ::core::marker::Copy for HoldingState {}
impl ::core::clone::Clone for HoldingState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HoldingState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HoldingState {
    type Abi = Self;
}
impl ::core::fmt::Debug for HoldingState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HoldingState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HoldingState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.HoldingState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAttachableInputObject(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAttachableInputObject {
    type Vtable = IAttachableInputObject_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b822734_a3c1_542a_b2f4_0e32b773fb07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAttachableInputObject_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAttachableInputObjectFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAttachableInputObjectFactory {
    type Vtable = IAttachableInputObjectFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4c54c4e_42bc_58fa_a640_ea1516f4c06b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAttachableInputObjectFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICrossSlidingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICrossSlidingEventArgs {
    type Vtable = ICrossSlidingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9374738_6f88_41d9_8720_78e08e398349);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrossSlidingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    pub CrossSlidingState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CrossSlidingState) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICrossSlidingEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICrossSlidingEventArgs2 {
    type Vtable = ICrossSlidingEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeefb7d48_c070_59f3_8dab_bcaf621d8687);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrossSlidingEventArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDraggingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDraggingEventArgs {
    type Vtable = IDraggingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c905384_083c_4bd3_b559_179cddeb33ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDraggingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    pub DraggingState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DraggingState) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDraggingEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDraggingEventArgs2 {
    type Vtable = IDraggingEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71efdbf9_382a_55ca_b4b9_008123c1bf1a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDraggingEventArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEdgeGesture(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEdgeGesture {
    type Vtable = IEdgeGesture_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x580d5292_2ab1_49aa_a7f0_33bd3f8df9f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEdgeGesture_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Starting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Starting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStarting: usize,
    #[cfg(feature = "Foundation")]
    pub Completed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Completed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Canceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Canceled: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCanceled: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEdgeGestureEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEdgeGestureEventArgs {
    type Vtable = IEdgeGestureEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x44fa4a24_2d09_42e1_8b5e_368208796a4c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEdgeGestureEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EdgeGestureKind) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEdgeGestureStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEdgeGestureStatics {
    type Vtable = IEdgeGestureStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc6a8519_18ee_4043_9839_4fc584d60a14);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEdgeGestureStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGestureRecognizer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGestureRecognizer {
    type Vtable = IGestureRecognizer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb47a37bf_3d6b_4f88_83e8_6dcb4012ffb0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGestureRecognizer_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GestureSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GestureSettings) -> ::windows::core::HRESULT,
    pub SetGestureSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: GestureSettings) -> ::windows::core::HRESULT,
    pub IsInertial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ShowGestureFeedback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetShowGestureFeedback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PivotCenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PivotCenter: usize,
    #[cfg(feature = "Foundation")]
    pub SetPivotCenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPivotCenter: usize,
    pub PivotRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetPivotRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub InertiaTranslationDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetInertiaTranslationDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub InertiaRotationDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetInertiaRotationDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub InertiaExpansionDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetInertiaExpansionDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub InertiaTranslationDisplacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetInertiaTranslationDisplacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub InertiaRotationAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetInertiaRotationAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub InertiaExpansion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetInertiaExpansion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub ManipulationExact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetManipulationExact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CrossSlideThresholds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CrossSlideThresholds) -> ::windows::core::HRESULT,
    pub SetCrossSlideThresholds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CrossSlideThresholds) -> ::windows::core::HRESULT,
    pub CrossSlideHorizontally: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCrossSlideHorizontally: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CrossSlideExact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCrossSlideExact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AutoProcessInertia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAutoProcessInertia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub MouseWheelParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CanBeDoubleTap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ProcessDownEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ProcessMoveEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProcessMoveEvents: usize,
    pub ProcessUpEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ProcessMouseWheelEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, isshiftkeydown: bool, iscontrolkeydown: bool) -> ::windows::core::HRESULT,
    pub ProcessInertia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CompleteGesture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Tapped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Tapped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTapped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTapped: usize,
    #[cfg(feature = "Foundation")]
    pub RightTapped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RightTapped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRightTapped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRightTapped: usize,
    #[cfg(feature = "Foundation")]
    pub Holding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Holding: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHolding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHolding: usize,
    #[cfg(feature = "Foundation")]
    pub Dragging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Dragging: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDragging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDragging: usize,
    #[cfg(feature = "Foundation")]
    pub ManipulationStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ManipulationStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveManipulationStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveManipulationStarted: usize,
    #[cfg(feature = "Foundation")]
    pub ManipulationUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ManipulationUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveManipulationUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveManipulationUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub ManipulationInertiaStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ManipulationInertiaStarting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveManipulationInertiaStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveManipulationInertiaStarting: usize,
    #[cfg(feature = "Foundation")]
    pub ManipulationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ManipulationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveManipulationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveManipulationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub CrossSliding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CrossSliding: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCrossSliding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCrossSliding: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGestureRecognizer2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGestureRecognizer2 {
    type Vtable = IGestureRecognizer2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd646097f_6ef7_5746_8ba8_8ff2206e6f3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGestureRecognizer2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TapMinContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetTapMinContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub TapMaxContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetTapMaxContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub HoldMinContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetHoldMinContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub HoldMaxContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetHoldMaxContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub HoldRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetHoldRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub HoldStartDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HoldStartDelay: usize,
    #[cfg(feature = "Foundation")]
    pub SetHoldStartDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetHoldStartDelay: usize,
    pub TranslationMinContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetTranslationMinContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub TranslationMaxContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetTranslationMaxContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHoldingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHoldingEventArgs {
    type Vtable = IHoldingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2bf755c5_e799_41b4_bb40_242f40959b71);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHoldingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    pub HoldingState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HoldingState) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHoldingEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHoldingEventArgs2 {
    type Vtable = IHoldingEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x141da9ea_4c79_5674_afea_493fdeb91f19);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHoldingEventArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub CurrentContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputActivationListener(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInputActivationListener {
    type Vtable = IInputActivationListener_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d6d4ed2_28c7_5ae3_aa74_c918a9f243ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputActivationListener_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InputActivationState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub InputActivationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InputActivationChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveInputActivationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveInputActivationChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputActivationListenerActivationChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInputActivationListenerActivationChangedEventArgs {
    type Vtable = IInputActivationListenerActivationChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7699b465_1dcf_5791_b4b9_6cafbeed2056);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputActivationListenerActivationChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InputActivationState) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyboardDeliveryInterceptor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyboardDeliveryInterceptor {
    type Vtable = IKeyboardDeliveryInterceptor_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4baf068_8f49_446c_8db5_8c0ffe85cc9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardDeliveryInterceptor_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsInterceptionEnabledWhenInForeground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsInterceptionEnabledWhenInForeground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub KeyDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    KeyDown: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveKeyDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveKeyDown: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub KeyUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    KeyUp: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveKeyUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveKeyUp: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyboardDeliveryInterceptorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyboardDeliveryInterceptorStatics {
    type Vtable = IKeyboardDeliveryInterceptorStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9f63ba2_ceba_4755_8a7e_14c0ffecd239);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardDeliveryInterceptorStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationCompletedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IManipulationCompletedEventArgs {
    type Vtable = IManipulationCompletedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb34ab22b_d19b_46ff_9f38_dec7754bb9e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationCompletedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub Cumulative: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ManipulationDelta) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Cumulative: usize,
    #[cfg(feature = "Foundation")]
    pub Velocities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ManipulationVelocities) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Velocities: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationCompletedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IManipulationCompletedEventArgs2 {
    type Vtable = IManipulationCompletedEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0c0dce7_30a9_5b96_886f_6560a85e4757);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationCompletedEventArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub CurrentContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationInertiaStartingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IManipulationInertiaStartingEventArgs {
    type Vtable = IManipulationInertiaStartingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd37a898_26bf_467a_9ce5_ccf3fb11371e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationInertiaStartingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub Delta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ManipulationDelta) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Delta: usize,
    #[cfg(feature = "Foundation")]
    pub Cumulative: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ManipulationDelta) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Cumulative: usize,
    #[cfg(feature = "Foundation")]
    pub Velocities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ManipulationVelocities) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Velocities: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationInertiaStartingEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IManipulationInertiaStartingEventArgs2 {
    type Vtable = IManipulationInertiaStartingEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc25409b8_f9fa_5a45_bd97_dcbbb2201860);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationInertiaStartingEventArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationStartedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IManipulationStartedEventArgs {
    type Vtable = IManipulationStartedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xddec873e_cfce_4932_8c1d_3c3d011a34c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationStartedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub Cumulative: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ManipulationDelta) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Cumulative: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationStartedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IManipulationStartedEventArgs2 {
    type Vtable = IManipulationStartedEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2da3db4e_e583_5055_afaa_16fd986531a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationStartedEventArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationUpdatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IManipulationUpdatedEventArgs {
    type Vtable = IManipulationUpdatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb354ce5_abb8_4f9f_b3ce_8181aa61ad82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationUpdatedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub Delta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ManipulationDelta) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Delta: usize,
    #[cfg(feature = "Foundation")]
    pub Cumulative: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ManipulationDelta) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Cumulative: usize,
    #[cfg(feature = "Foundation")]
    pub Velocities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ManipulationVelocities) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Velocities: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationUpdatedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IManipulationUpdatedEventArgs2 {
    type Vtable = IManipulationUpdatedEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3dfb96a_3306_5903_a1c5_ff9757a8689e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationUpdatedEventArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub CurrentContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMouseWheelParameters(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMouseWheelParameters {
    type Vtable = IMouseWheelParameters_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xead0ca44_9ded_4037_8149_5e4cc2564468);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMouseWheelParameters_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CharTranslation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CharTranslation: usize,
    #[cfg(feature = "Foundation")]
    pub SetCharTranslation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCharTranslation: usize,
    pub DeltaScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetDeltaScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub DeltaRotationAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetDeltaRotationAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PageTranslation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PageTranslation: usize,
    #[cfg(feature = "Foundation")]
    pub SetPageTranslation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPageTranslation: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerPoint(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPointerPoint {
    type Vtable = IPointerPoint_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe995317d_7296_42d9_8233_c5be73b74a4a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPoint_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Input")]
    pub PointerDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDevice: usize,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub RawPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RawPosition: usize,
    pub PointerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub FrameId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub IsInContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerPointProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPointerPointProperties {
    type Vtable = IPointerPointProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc79d8a4b_c163_4ee7_803f_67ce79f9972d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPointProperties_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Pressure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub IsInverted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsEraser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Orientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub XTilt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub YTilt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub Twist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ContactRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContactRect: usize,
    #[cfg(feature = "Foundation")]
    pub ContactRectRaw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContactRectRaw: usize,
    pub TouchConfidence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsLeftButtonPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsRightButtonPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsMiddleButtonPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub MouseWheelDelta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub IsHorizontalMouseWheel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsPrimary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsInRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsBarrelButtonPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsXButton1Pressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsXButton2Pressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub PointerUpdateKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PointerUpdateKind) -> ::windows::core::HRESULT,
    pub HasUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usagepage: u32, usageid: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub GetUsageValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usagepage: u32, usageid: u32, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerPointProperties2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPointerPointProperties2 {
    type Vtable = IPointerPointProperties2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22c3433a_c83b_41c0_a296_5e232d64d6af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPointProperties2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ZDistance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ZDistance: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerPointStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPointerPointStatics {
    type Vtable = IPointerPointStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa506638d_2a1a_413e_bc75_9f38381cc069);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPointStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetCurrentPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetIntermediatePoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetIntermediatePoints: usize,
    pub GetCurrentPointTransformed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerid: u32, transform: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetIntermediatePointsTransformed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerid: u32, transform: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetIntermediatePointsTransformed: usize,
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct IPointerPointTransform(::windows::core::IUnknown);
impl IPointerPointTransform {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Inverse(&self) -> ::windows::core::Result<IPointerPointTransform> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Inverse)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IPointerPointTransform>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryTransform<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Point>>(&self, inpoint: Param0, outpoint: &mut super::super::Foundation::Point) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryTransform)(::core::mem::transmute_copy(this), inpoint.into_param().abi(), outpoint, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TransformBounds<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, rect: Param0) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TransformBounds)(::core::mem::transmute_copy(this), rect.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
}
impl ::core::convert::From<IPointerPointTransform> for ::windows::core::IUnknown {
    fn from(value: IPointerPointTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPointerPointTransform> for ::windows::core::IUnknown {
    fn from(value: &IPointerPointTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPointerPointTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPointerPointTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPointerPointTransform> for ::windows::core::IInspectable {
    fn from(value: IPointerPointTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPointerPointTransform> for ::windows::core::IInspectable {
    fn from(value: &IPointerPointTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPointerPointTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IPointerPointTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPointerPointTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPointerPointTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPointerPointTransform {}
impl ::core::fmt::Debug for IPointerPointTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPointerPointTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IPointerPointTransform {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{4d5fe14f-b87c-4028-bc9c-59e9947fb056}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IPointerPointTransform {
    type Vtable = IPointerPointTransform_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d5fe14f_b87c_4028_bc9c_59e9947fb056);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPointTransform_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Inverse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inpoint: super::super::Foundation::Point, outpoint: *mut super::super::Foundation::Point, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryTransform: usize,
    #[cfg(feature = "Foundation")]
    pub TransformBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rect: super::super::Foundation::Rect, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TransformBounds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerVisualizationSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPointerVisualizationSettings {
    type Vtable = IPointerVisualizationSettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d1e6461_84f7_499d_bd91_2a36e2b7aaa2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerVisualizationSettings_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetIsContactFeedbackEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsContactFeedbackEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsBarrelButtonFeedbackEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsBarrelButtonFeedbackEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerVisualizationSettingsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPointerVisualizationSettingsStatics {
    type Vtable = IPointerVisualizationSettingsStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68870edb_165b_4214_b4f3_584eca8c8a69);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerVisualizationSettingsStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialController(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRadialController {
    type Vtable = IRadialController_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3055d1c8_df51_43d4_b23b_0e1037467a09);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialController_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Menu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RotationResolutionInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetRotationResolutionInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub UseAutomaticHapticFeedback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetUseAutomaticHapticFeedback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ScreenContactStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScreenContactStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveScreenContactStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveScreenContactStarted: usize,
    #[cfg(feature = "Foundation")]
    pub ScreenContactEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScreenContactEnded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveScreenContactEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveScreenContactEnded: usize,
    #[cfg(feature = "Foundation")]
    pub ScreenContactContinued: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScreenContactContinued: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveScreenContactContinued: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveScreenContactContinued: usize,
    #[cfg(feature = "Foundation")]
    pub ControlLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ControlLost: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveControlLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveControlLost: usize,
    #[cfg(feature = "Foundation")]
    pub RotationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RotationChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRotationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRotationChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonClicked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonClicked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveButtonClicked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveButtonClicked: usize,
    #[cfg(feature = "Foundation")]
    pub ControlAcquired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ControlAcquired: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveControlAcquired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveControlAcquired: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialController2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRadialController2 {
    type Vtable = IRadialController2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d577eff_4cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialController2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ButtonPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveButtonPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveButtonPressed: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonHolding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonHolding: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveButtonHolding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveButtonHolding: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonReleased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonReleased: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveButtonReleased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveButtonReleased: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerButtonClickedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRadialControllerButtonClickedEventArgs {
    type Vtable = IRadialControllerButtonClickedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x206aa438_e651_11e5_bf62_2c27d7404e85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerButtonClickedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerButtonClickedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRadialControllerButtonClickedEventArgs2 {
    type Vtable = IRadialControllerButtonClickedEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d577ef3_3cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerButtonClickedEventArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerButtonHoldingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRadialControllerButtonHoldingEventArgs {
    type Vtable = IRadialControllerButtonHoldingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d577eee_3cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerButtonHoldingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerButtonPressedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRadialControllerButtonPressedEventArgs {
    type Vtable = IRadialControllerButtonPressedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d577eed_4cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerButtonPressedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerButtonReleasedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRadialControllerButtonReleasedEventArgs {
    type Vtable = IRadialControllerButtonReleasedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d577eef_3cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerButtonReleasedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRadialControllerConfiguration {
    type Vtable = IRadialControllerConfiguration_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6b79ecb_6a52_4430_910c_56370a9d6b42);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerConfiguration_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SetDefaultMenuItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buttons: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetDefaultMenuItems: usize,
    pub ResetToDefaultMenuItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TrySelectDefaultMenuItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: RadialControllerSystemMenuItemKind, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerConfiguration2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRadialControllerConfiguration2 {
    type Vtable = IRadialControllerConfiguration2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d577ef7_3cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerConfiguration2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetActiveControllerWhenMenuIsSuppressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ActiveControllerWhenMenuIsSuppressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetIsMenuSuppressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsMenuSuppressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerConfigurationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRadialControllerConfigurationStatics {
    type Vtable = IRadialControllerConfigurationStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79b6b0e5_069a_4486_a99d_8db772b9642f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerConfigurationStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerConfigurationStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRadialControllerConfigurationStatics2 {
    type Vtable = IRadialControllerConfigurationStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53e08b17_e205_48d3_9caf_80ff47c4d7c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerConfigurationStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetAppController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AppController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetIsAppControllerEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsAppControllerEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerControlAcquiredEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRadialControllerControlAcquiredEventArgs {
    type Vtable = IRadialControllerControlAcquiredEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x206aa439_e651_11e5_bf62_2c27d7404e85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerControlAcquiredEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerControlAcquiredEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRadialControllerControlAcquiredEventArgs2 {
    type Vtable = IRadialControllerControlAcquiredEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d577ef4_3cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerControlAcquiredEventArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsButtonPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerMenu(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRadialControllerMenu {
    type Vtable = IRadialControllerMenu_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8506b35d_f640_4412_aba0_bad077e5ea8a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerMenu_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub GetSelectedMenuItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SelectMenuItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, menuitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub TrySelectPreviouslySelectedMenuItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerMenuItem(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRadialControllerMenuItem {
    type Vtable = IRadialControllerMenuItem_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc80fc98d_ad0b_4c9c_8f2f_136a2373a6ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerMenuItem_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DisplayText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Tag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Invoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Invoked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveInvoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveInvoked: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerMenuItemStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRadialControllerMenuItemStatics {
    type Vtable = IRadialControllerMenuItemStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x249e0887_d842_4524_9df8_e0d647edc887);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerMenuItemStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displaytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, icon: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromIcon: usize,
    pub CreateFromKnownIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displaytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: RadialControllerMenuKnownIcon, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerMenuItemStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRadialControllerMenuItemStatics2 {
    type Vtable = IRadialControllerMenuItemStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cbb70be_7e3e_48bd_be04_2c7fcaa9c1ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerMenuItemStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateFromFontGlyph: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displaytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, glyph: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fontfamily: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateFromFontGlyphWithUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displaytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, glyph: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fontfamily: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fonturi: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFromFontGlyphWithUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerRotationChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRadialControllerRotationChangedEventArgs {
    type Vtable = IRadialControllerRotationChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x206aa435_e651_11e5_bf62_2c27d7404e85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerRotationChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub RotationDeltaInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerRotationChangedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRadialControllerRotationChangedEventArgs2 {
    type Vtable = IRadialControllerRotationChangedEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d577eec_4cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerRotationChangedEventArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsButtonPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerScreenContact(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRadialControllerScreenContact {
    type Vtable = IRadialControllerScreenContact_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x206aa434_e651_11e5_bf62_2c27d7404e85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerScreenContact_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Bounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Bounds: usize,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerScreenContactContinuedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRadialControllerScreenContactContinuedEventArgs {
    type Vtable = IRadialControllerScreenContactContinuedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x206aa437_e651_11e5_bf62_2c27d7404e85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerScreenContactContinuedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerScreenContactContinuedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRadialControllerScreenContactContinuedEventArgs2 {
    type Vtable = IRadialControllerScreenContactContinuedEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d577ef1_3cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerScreenContactContinuedEventArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsButtonPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerScreenContactEndedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRadialControllerScreenContactEndedEventArgs {
    type Vtable = IRadialControllerScreenContactEndedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d577ef2_3cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerScreenContactEndedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsButtonPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerScreenContactStartedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRadialControllerScreenContactStartedEventArgs {
    type Vtable = IRadialControllerScreenContactStartedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x206aa436_e651_11e5_bf62_2c27d7404e85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerScreenContactStartedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerScreenContactStartedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRadialControllerScreenContactStartedEventArgs2 {
    type Vtable = IRadialControllerScreenContactStartedEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d577ef0_3cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerScreenContactStartedEventArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsButtonPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRadialControllerStatics {
    type Vtable = IRadialControllerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfaded0b7_b84c_4894_87aa_8f25aa5f288b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CreateForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRightTappedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRightTappedEventArgs {
    type Vtable = IRightTappedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4cbf40bd_af7a_4a36_9476_b1dce141709a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRightTappedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRightTappedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRightTappedEventArgs2 {
    type Vtable = IRightTappedEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61c7b7bb_9f57_5857_a33c_c58c3dfa959e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRightTappedEventArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemButtonEventController(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemButtonEventController {
    type Vtable = ISystemButtonEventController_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59b893a9_73bc_52b5_ba41_82511b2cb46c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemButtonEventController_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub SystemFunctionButtonPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemFunctionButtonPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSystemFunctionButtonPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSystemFunctionButtonPressed: usize,
    #[cfg(feature = "Foundation")]
    pub SystemFunctionButtonReleased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemFunctionButtonReleased: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSystemFunctionButtonReleased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSystemFunctionButtonReleased: usize,
    #[cfg(feature = "Foundation")]
    pub SystemFunctionLockChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemFunctionLockChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSystemFunctionLockChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSystemFunctionLockChanged: usize,
    #[cfg(feature = "Foundation")]
    pub SystemFunctionLockIndicatorChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemFunctionLockIndicatorChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSystemFunctionLockIndicatorChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSystemFunctionLockIndicatorChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemButtonEventControllerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemButtonEventControllerStatics {
    type Vtable = ISystemButtonEventControllerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x632fb07b_20bd_5e15_af4a_00dbf2064ffa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemButtonEventControllerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub CreateForDispatcherQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queue: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    CreateForDispatcherQueue: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemFunctionButtonEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemFunctionButtonEventArgs {
    type Vtable = ISystemFunctionButtonEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4833896f_80d1_5dd6_92a7_62a508ffef5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemFunctionButtonEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemFunctionLockChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemFunctionLockChangedEventArgs {
    type Vtable = ISystemFunctionLockChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd040608_fcf9_585c_beab_f1d2eaf364ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemFunctionLockChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub IsLocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemFunctionLockIndicatorChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemFunctionLockIndicatorChangedEventArgs {
    type Vtable = ISystemFunctionLockIndicatorChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb212b94e_7a6f_58ae_b304_bae61d0371b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemFunctionLockIndicatorChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub IsIndicatorOn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITappedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITappedEventArgs {
    type Vtable = ITappedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcfa126e4_253a_4c3c_953b_395c37aed309);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITappedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    pub TapCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITappedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITappedEventArgs2 {
    type Vtable = ITappedEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x294388f2_177e_51d5_be56_ee0866fa968c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITappedEventArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ContactCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct InputActivationListener(::windows::core::IUnknown);
impl InputActivationListener {
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn State(&self) -> ::windows::core::Result<InputActivationState> {
        let this = self;
        unsafe {
            let mut result__: InputActivationState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InputActivationState>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InputActivationChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<InputActivationListener, InputActivationListenerActivationChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InputActivationChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveInputActivationChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveInputActivationChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for InputActivationListener {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InputActivationListener {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InputActivationListener {}
impl ::core::fmt::Debug for InputActivationListener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputActivationListener").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InputActivationListener {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.InputActivationListener;{5d6d4ed2-28c7-5ae3-aa74-c918a9f243ca})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for InputActivationListener {
    type Vtable = IInputActivationListener_Vtbl;
    const IID: ::windows::core::GUID = <IInputActivationListener as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InputActivationListener {
    const NAME: &'static str = "Windows.UI.Input.InputActivationListener";
}
impl ::core::convert::From<InputActivationListener> for ::windows::core::IUnknown {
    fn from(value: InputActivationListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputActivationListener> for ::windows::core::IUnknown {
    fn from(value: &InputActivationListener) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InputActivationListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InputActivationListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InputActivationListener> for ::windows::core::IInspectable {
    fn from(value: InputActivationListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputActivationListener> for ::windows::core::IInspectable {
    fn from(value: &InputActivationListener) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InputActivationListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InputActivationListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<InputActivationListener> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: InputActivationListener) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InputActivationListener> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &InputActivationListener) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for InputActivationListener {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &InputActivationListener {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<InputActivationListener> for AttachableInputObject {
    fn from(value: InputActivationListener) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InputActivationListener> for AttachableInputObject {
    fn from(value: &InputActivationListener) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, AttachableInputObject> for InputActivationListener {
    fn into_param(self) -> ::windows::core::Param<'a, AttachableInputObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, AttachableInputObject> for &InputActivationListener {
    fn into_param(self) -> ::windows::core::Param<'a, AttachableInputObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<AttachableInputObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for InputActivationListener {}
unsafe impl ::core::marker::Sync for InputActivationListener {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct InputActivationListenerActivationChangedEventArgs(::windows::core::IUnknown);
impl InputActivationListenerActivationChangedEventArgs {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn State(&self) -> ::windows::core::Result<InputActivationState> {
        let this = self;
        unsafe {
            let mut result__: InputActivationState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InputActivationState>(result__)
        }
    }
}
impl ::core::clone::Clone for InputActivationListenerActivationChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InputActivationListenerActivationChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InputActivationListenerActivationChangedEventArgs {}
impl ::core::fmt::Debug for InputActivationListenerActivationChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputActivationListenerActivationChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InputActivationListenerActivationChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.InputActivationListenerActivationChangedEventArgs;{7699b465-1dcf-5791-b4b9-6cafbeed2056})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for InputActivationListenerActivationChangedEventArgs {
    type Vtable = IInputActivationListenerActivationChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IInputActivationListenerActivationChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InputActivationListenerActivationChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.InputActivationListenerActivationChangedEventArgs";
}
impl ::core::convert::From<InputActivationListenerActivationChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: InputActivationListenerActivationChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputActivationListenerActivationChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &InputActivationListenerActivationChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InputActivationListenerActivationChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InputActivationListenerActivationChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InputActivationListenerActivationChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: InputActivationListenerActivationChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputActivationListenerActivationChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &InputActivationListenerActivationChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InputActivationListenerActivationChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InputActivationListenerActivationChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InputActivationListenerActivationChangedEventArgs {}
unsafe impl ::core::marker::Sync for InputActivationListenerActivationChangedEventArgs {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InputActivationState(pub i32);
impl InputActivationState {
    pub const None: Self = Self(0i32);
    pub const Deactivated: Self = Self(1i32);
    pub const ActivatedNotForeground: Self = Self(2i32);
    pub const ActivatedInForeground: Self = Self(3i32);
}
impl ::core::marker::Copy for InputActivationState {}
impl ::core::clone::Clone for InputActivationState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InputActivationState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for InputActivationState {
    type Abi = Self;
}
impl ::core::fmt::Debug for InputActivationState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputActivationState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InputActivationState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.InputActivationState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct KeyboardDeliveryInterceptor(::windows::core::IUnknown);
impl KeyboardDeliveryInterceptor {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsInterceptionEnabledWhenInForeground(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsInterceptionEnabledWhenInForeground)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetIsInterceptionEnabledWhenInForeground(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsInterceptionEnabledWhenInForeground)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`, `\"UI_Core\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn KeyDown<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<KeyboardDeliveryInterceptor, super::Core::KeyEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyDown)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveKeyDown<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveKeyDown)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`, `\"UI_Core\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn KeyUp<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<KeyboardDeliveryInterceptor, super::Core::KeyEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyUp)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveKeyUp<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveKeyUp)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn GetForCurrentView() -> ::windows::core::Result<KeyboardDeliveryInterceptor> {
        Self::IKeyboardDeliveryInterceptorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<KeyboardDeliveryInterceptor>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKeyboardDeliveryInterceptorStatics<R, F: FnOnce(&IKeyboardDeliveryInterceptorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KeyboardDeliveryInterceptor, IKeyboardDeliveryInterceptorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for KeyboardDeliveryInterceptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for KeyboardDeliveryInterceptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeyboardDeliveryInterceptor {}
impl ::core::fmt::Debug for KeyboardDeliveryInterceptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyboardDeliveryInterceptor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for KeyboardDeliveryInterceptor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.KeyboardDeliveryInterceptor;{b4baf068-8f49-446c-8db5-8c0ffe85cc9e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for KeyboardDeliveryInterceptor {
    type Vtable = IKeyboardDeliveryInterceptor_Vtbl;
    const IID: ::windows::core::GUID = <IKeyboardDeliveryInterceptor as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for KeyboardDeliveryInterceptor {
    const NAME: &'static str = "Windows.UI.Input.KeyboardDeliveryInterceptor";
}
impl ::core::convert::From<KeyboardDeliveryInterceptor> for ::windows::core::IUnknown {
    fn from(value: KeyboardDeliveryInterceptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyboardDeliveryInterceptor> for ::windows::core::IUnknown {
    fn from(value: &KeyboardDeliveryInterceptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for KeyboardDeliveryInterceptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a KeyboardDeliveryInterceptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<KeyboardDeliveryInterceptor> for ::windows::core::IInspectable {
    fn from(value: KeyboardDeliveryInterceptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyboardDeliveryInterceptor> for ::windows::core::IInspectable {
    fn from(value: &KeyboardDeliveryInterceptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for KeyboardDeliveryInterceptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a KeyboardDeliveryInterceptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for KeyboardDeliveryInterceptor {}
unsafe impl ::core::marker::Sync for KeyboardDeliveryInterceptor {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct ManipulationCompletedEventArgs(::windows::core::IUnknown);
impl ManipulationCompletedEventArgs {
    #[doc = "*Required features: `\"UI_Input\"`, `\"Devices_Input\"`*"]
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerDeviceType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Cumulative(&self) -> ::windows::core::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: ManipulationDelta = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Cumulative)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ManipulationDelta>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Velocities(&self) -> ::windows::core::Result<ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__: ManipulationVelocities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Velocities)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ManipulationVelocities>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn ContactCount(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IManipulationCompletedEventArgs2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContactCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn CurrentContactCount(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IManipulationCompletedEventArgs2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CurrentContactCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for ManipulationCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManipulationCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManipulationCompletedEventArgs {}
impl ::core::fmt::Debug for ManipulationCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationCompletedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ManipulationCompletedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.ManipulationCompletedEventArgs;{b34ab22b-d19b-46ff-9f38-dec7754bb9e7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ManipulationCompletedEventArgs {
    type Vtable = IManipulationCompletedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IManipulationCompletedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ManipulationCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.ManipulationCompletedEventArgs";
}
impl ::core::convert::From<ManipulationCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ManipulationCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ManipulationCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ManipulationCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ManipulationCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ManipulationCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ManipulationCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ManipulationCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ManipulationCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ManipulationCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
pub struct ManipulationDelta {
    pub Translation: super::super::Foundation::Point,
    pub Scale: f32,
    pub Rotation: f32,
    pub Expansion: f32,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for ManipulationDelta {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for ManipulationDelta {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for ManipulationDelta {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ManipulationDelta").field("Translation", &self.Translation).field("Scale", &self.Scale).field("Rotation", &self.Rotation).field("Expansion", &self.Expansion).finish()
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Abi for ManipulationDelta {
    type Abi = Self;
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::RuntimeType for ManipulationDelta {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.UI.Input.ManipulationDelta;struct(Windows.Foundation.Point;f4;f4);f4;f4;f4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for ManipulationDelta {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ManipulationDelta>()) == 0 }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for ManipulationDelta {}
#[cfg(feature = "Foundation")]
impl ::core::default::Default for ManipulationDelta {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct ManipulationInertiaStartingEventArgs(::windows::core::IUnknown);
impl ManipulationInertiaStartingEventArgs {
    #[doc = "*Required features: `\"UI_Input\"`, `\"Devices_Input\"`*"]
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerDeviceType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Delta(&self) -> ::windows::core::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: ManipulationDelta = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Delta)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ManipulationDelta>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Cumulative(&self) -> ::windows::core::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: ManipulationDelta = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Cumulative)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ManipulationDelta>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Velocities(&self) -> ::windows::core::Result<ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__: ManipulationVelocities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Velocities)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ManipulationVelocities>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn ContactCount(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IManipulationInertiaStartingEventArgs2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContactCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for ManipulationInertiaStartingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManipulationInertiaStartingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManipulationInertiaStartingEventArgs {}
impl ::core::fmt::Debug for ManipulationInertiaStartingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationInertiaStartingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ManipulationInertiaStartingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.ManipulationInertiaStartingEventArgs;{dd37a898-26bf-467a-9ce5-ccf3fb11371e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ManipulationInertiaStartingEventArgs {
    type Vtable = IManipulationInertiaStartingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IManipulationInertiaStartingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ManipulationInertiaStartingEventArgs {
    const NAME: &'static str = "Windows.UI.Input.ManipulationInertiaStartingEventArgs";
}
impl ::core::convert::From<ManipulationInertiaStartingEventArgs> for ::windows::core::IUnknown {
    fn from(value: ManipulationInertiaStartingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationInertiaStartingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ManipulationInertiaStartingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ManipulationInertiaStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ManipulationInertiaStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ManipulationInertiaStartingEventArgs> for ::windows::core::IInspectable {
    fn from(value: ManipulationInertiaStartingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationInertiaStartingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ManipulationInertiaStartingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ManipulationInertiaStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ManipulationInertiaStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct ManipulationStartedEventArgs(::windows::core::IUnknown);
impl ManipulationStartedEventArgs {
    #[doc = "*Required features: `\"UI_Input\"`, `\"Devices_Input\"`*"]
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerDeviceType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Cumulative(&self) -> ::windows::core::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: ManipulationDelta = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Cumulative)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ManipulationDelta>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn ContactCount(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IManipulationStartedEventArgs2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContactCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for ManipulationStartedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManipulationStartedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManipulationStartedEventArgs {}
impl ::core::fmt::Debug for ManipulationStartedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationStartedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ManipulationStartedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.ManipulationStartedEventArgs;{ddec873e-cfce-4932-8c1d-3c3d011a34c0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ManipulationStartedEventArgs {
    type Vtable = IManipulationStartedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IManipulationStartedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ManipulationStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.ManipulationStartedEventArgs";
}
impl ::core::convert::From<ManipulationStartedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ManipulationStartedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationStartedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ManipulationStartedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ManipulationStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ManipulationStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ManipulationStartedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ManipulationStartedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationStartedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ManipulationStartedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ManipulationStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ManipulationStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct ManipulationUpdatedEventArgs(::windows::core::IUnknown);
impl ManipulationUpdatedEventArgs {
    #[doc = "*Required features: `\"UI_Input\"`, `\"Devices_Input\"`*"]
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerDeviceType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Delta(&self) -> ::windows::core::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: ManipulationDelta = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Delta)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ManipulationDelta>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Cumulative(&self) -> ::windows::core::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: ManipulationDelta = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Cumulative)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ManipulationDelta>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Velocities(&self) -> ::windows::core::Result<ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__: ManipulationVelocities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Velocities)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ManipulationVelocities>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn ContactCount(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IManipulationUpdatedEventArgs2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContactCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn CurrentContactCount(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IManipulationUpdatedEventArgs2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CurrentContactCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for ManipulationUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManipulationUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManipulationUpdatedEventArgs {}
impl ::core::fmt::Debug for ManipulationUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationUpdatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ManipulationUpdatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.ManipulationUpdatedEventArgs;{cb354ce5-abb8-4f9f-b3ce-8181aa61ad82})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ManipulationUpdatedEventArgs {
    type Vtable = IManipulationUpdatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IManipulationUpdatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ManipulationUpdatedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.ManipulationUpdatedEventArgs";
}
impl ::core::convert::From<ManipulationUpdatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ManipulationUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationUpdatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ManipulationUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ManipulationUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ManipulationUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ManipulationUpdatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ManipulationUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationUpdatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ManipulationUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ManipulationUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ManipulationUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
pub struct ManipulationVelocities {
    pub Linear: super::super::Foundation::Point,
    pub Angular: f32,
    pub Expansion: f32,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for ManipulationVelocities {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for ManipulationVelocities {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for ManipulationVelocities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ManipulationVelocities").field("Linear", &self.Linear).field("Angular", &self.Angular).field("Expansion", &self.Expansion).finish()
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Abi for ManipulationVelocities {
    type Abi = Self;
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::RuntimeType for ManipulationVelocities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.UI.Input.ManipulationVelocities;struct(Windows.Foundation.Point;f4;f4);f4;f4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for ManipulationVelocities {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ManipulationVelocities>()) == 0 }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for ManipulationVelocities {}
#[cfg(feature = "Foundation")]
impl ::core::default::Default for ManipulationVelocities {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct MouseWheelParameters(::windows::core::IUnknown);
impl MouseWheelParameters {
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CharTranslation(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CharTranslation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCharTranslation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCharTranslation)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn DeltaScale(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeltaScale)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetDeltaScale(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDeltaScale)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn DeltaRotationAngle(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeltaRotationAngle)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetDeltaRotationAngle(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDeltaRotationAngle)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PageTranslation(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PageTranslation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPageTranslation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPageTranslation)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for MouseWheelParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MouseWheelParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MouseWheelParameters {}
impl ::core::fmt::Debug for MouseWheelParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MouseWheelParameters").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MouseWheelParameters {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.MouseWheelParameters;{ead0ca44-9ded-4037-8149-5e4cc2564468})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MouseWheelParameters {
    type Vtable = IMouseWheelParameters_Vtbl;
    const IID: ::windows::core::GUID = <IMouseWheelParameters as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MouseWheelParameters {
    const NAME: &'static str = "Windows.UI.Input.MouseWheelParameters";
}
impl ::core::convert::From<MouseWheelParameters> for ::windows::core::IUnknown {
    fn from(value: MouseWheelParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MouseWheelParameters> for ::windows::core::IUnknown {
    fn from(value: &MouseWheelParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MouseWheelParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MouseWheelParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MouseWheelParameters> for ::windows::core::IInspectable {
    fn from(value: MouseWheelParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MouseWheelParameters> for ::windows::core::IInspectable {
    fn from(value: &MouseWheelParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MouseWheelParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MouseWheelParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct PointerPoint(::windows::core::IUnknown);
impl PointerPoint {
    #[doc = "*Required features: `\"UI_Input\"`, `\"Devices_Input\"`*"]
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDevice(&self) -> ::windows::core::Result<super::super::Devices::Input::PointerDevice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerDevice)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Input::PointerDevice>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RawPosition(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RawPosition)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn PointerId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn FrameId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FrameId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Timestamp(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Timestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsInContact(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsInContact)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Properties(&self) -> ::windows::core::Result<PointerPointProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PointerPointProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn GetCurrentPoint(pointerid: u32) -> ::windows::core::Result<PointerPoint> {
        Self::IPointerPointStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentPoint)(::core::mem::transmute_copy(this), pointerid, &mut result__).from_abi::<PointerPoint>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetIntermediatePoints(pointerid: u32) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<PointerPoint>> {
        Self::IPointerPointStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetIntermediatePoints)(::core::mem::transmute_copy(this), pointerid, &mut result__).from_abi::<super::super::Foundation::Collections::IVector<PointerPoint>>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn GetCurrentPointTransformed<'a, Param1: ::windows::core::IntoParam<'a, IPointerPointTransform>>(pointerid: u32, transform: Param1) -> ::windows::core::Result<PointerPoint> {
        Self::IPointerPointStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentPointTransformed)(::core::mem::transmute_copy(this), pointerid, transform.into_param().abi(), &mut result__).from_abi::<PointerPoint>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetIntermediatePointsTransformed<'a, Param1: ::windows::core::IntoParam<'a, IPointerPointTransform>>(pointerid: u32, transform: Param1) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<PointerPoint>> {
        Self::IPointerPointStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetIntermediatePointsTransformed)(::core::mem::transmute_copy(this), pointerid, transform.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<PointerPoint>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPointerPointStatics<R, F: FnOnce(&IPointerPointStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PointerPoint, IPointerPointStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PointerPoint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PointerPoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PointerPoint {}
impl ::core::fmt::Debug for PointerPoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointerPoint").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PointerPoint {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.PointerPoint;{e995317d-7296-42d9-8233-c5be73b74a4a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PointerPoint {
    type Vtable = IPointerPoint_Vtbl;
    const IID: ::windows::core::GUID = <IPointerPoint as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PointerPoint {
    const NAME: &'static str = "Windows.UI.Input.PointerPoint";
}
impl ::core::convert::From<PointerPoint> for ::windows::core::IUnknown {
    fn from(value: PointerPoint) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointerPoint> for ::windows::core::IUnknown {
    fn from(value: &PointerPoint) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PointerPoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PointerPoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointerPoint> for ::windows::core::IInspectable {
    fn from(value: PointerPoint) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointerPoint> for ::windows::core::IInspectable {
    fn from(value: &PointerPoint) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PointerPoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PointerPoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct PointerPointProperties(::windows::core::IUnknown);
impl PointerPointProperties {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Pressure(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Pressure)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsInverted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsInverted)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsEraser(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEraser)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Orientation(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Orientation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn XTilt(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XTilt)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn YTilt(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).YTilt)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Twist(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Twist)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContactRect(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContactRect)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContactRectRaw(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContactRectRaw)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn TouchConfidence(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TouchConfidence)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsLeftButtonPressed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsLeftButtonPressed)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsRightButtonPressed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsRightButtonPressed)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsMiddleButtonPressed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsMiddleButtonPressed)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn MouseWheelDelta(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MouseWheelDelta)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsHorizontalMouseWheel(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsHorizontalMouseWheel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsPrimary(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPrimary)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsInRange(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsInRange)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsCanceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCanceled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsBarrelButtonPressed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBarrelButtonPressed)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsXButton1Pressed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsXButton1Pressed)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsXButton2Pressed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsXButton2Pressed)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn PointerUpdateKind(&self) -> ::windows::core::Result<PointerUpdateKind> {
        let this = self;
        unsafe {
            let mut result__: PointerUpdateKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerUpdateKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PointerUpdateKind>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn HasUsage(&self, usagepage: u32, usageid: u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasUsage)(::core::mem::transmute_copy(this), usagepage, usageid, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn GetUsageValue(&self, usagepage: u32, usageid: u32) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetUsageValue)(::core::mem::transmute_copy(this), usagepage, usageid, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ZDistance(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f32>> {
        let this = &::windows::core::Interface::cast::<IPointerPointProperties2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ZDistance)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f32>>(result__)
        }
    }
}
impl ::core::clone::Clone for PointerPointProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PointerPointProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PointerPointProperties {}
impl ::core::fmt::Debug for PointerPointProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointerPointProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PointerPointProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.PointerPointProperties;{c79d8a4b-c163-4ee7-803f-67ce79f9972d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PointerPointProperties {
    type Vtable = IPointerPointProperties_Vtbl;
    const IID: ::windows::core::GUID = <IPointerPointProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PointerPointProperties {
    const NAME: &'static str = "Windows.UI.Input.PointerPointProperties";
}
impl ::core::convert::From<PointerPointProperties> for ::windows::core::IUnknown {
    fn from(value: PointerPointProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointerPointProperties> for ::windows::core::IUnknown {
    fn from(value: &PointerPointProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PointerPointProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PointerPointProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointerPointProperties> for ::windows::core::IInspectable {
    fn from(value: PointerPointProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointerPointProperties> for ::windows::core::IInspectable {
    fn from(value: &PointerPointProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PointerPointProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PointerPointProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PointerUpdateKind(pub i32);
impl PointerUpdateKind {
    pub const Other: Self = Self(0i32);
    pub const LeftButtonPressed: Self = Self(1i32);
    pub const LeftButtonReleased: Self = Self(2i32);
    pub const RightButtonPressed: Self = Self(3i32);
    pub const RightButtonReleased: Self = Self(4i32);
    pub const MiddleButtonPressed: Self = Self(5i32);
    pub const MiddleButtonReleased: Self = Self(6i32);
    pub const XButton1Pressed: Self = Self(7i32);
    pub const XButton1Released: Self = Self(8i32);
    pub const XButton2Pressed: Self = Self(9i32);
    pub const XButton2Released: Self = Self(10i32);
}
impl ::core::marker::Copy for PointerUpdateKind {}
impl ::core::clone::Clone for PointerUpdateKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PointerUpdateKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PointerUpdateKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for PointerUpdateKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointerUpdateKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PointerUpdateKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.PointerUpdateKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct PointerVisualizationSettings(::windows::core::IUnknown);
impl PointerVisualizationSettings {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetIsContactFeedbackEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsContactFeedbackEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsContactFeedbackEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsContactFeedbackEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetIsBarrelButtonFeedbackEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsBarrelButtonFeedbackEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsBarrelButtonFeedbackEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBarrelButtonFeedbackEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn GetForCurrentView() -> ::windows::core::Result<PointerVisualizationSettings> {
        Self::IPointerVisualizationSettingsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PointerVisualizationSettings>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPointerVisualizationSettingsStatics<R, F: FnOnce(&IPointerVisualizationSettingsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PointerVisualizationSettings, IPointerVisualizationSettingsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PointerVisualizationSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PointerVisualizationSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PointerVisualizationSettings {}
impl ::core::fmt::Debug for PointerVisualizationSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointerVisualizationSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PointerVisualizationSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.PointerVisualizationSettings;{4d1e6461-84f7-499d-bd91-2a36e2b7aaa2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PointerVisualizationSettings {
    type Vtable = IPointerVisualizationSettings_Vtbl;
    const IID: ::windows::core::GUID = <IPointerVisualizationSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PointerVisualizationSettings {
    const NAME: &'static str = "Windows.UI.Input.PointerVisualizationSettings";
}
impl ::core::convert::From<PointerVisualizationSettings> for ::windows::core::IUnknown {
    fn from(value: PointerVisualizationSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointerVisualizationSettings> for ::windows::core::IUnknown {
    fn from(value: &PointerVisualizationSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PointerVisualizationSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PointerVisualizationSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointerVisualizationSettings> for ::windows::core::IInspectable {
    fn from(value: PointerVisualizationSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointerVisualizationSettings> for ::windows::core::IInspectable {
    fn from(value: &PointerVisualizationSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PointerVisualizationSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PointerVisualizationSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PointerVisualizationSettings {}
unsafe impl ::core::marker::Sync for PointerVisualizationSettings {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct RadialController(::windows::core::IUnknown);
impl RadialController {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Menu(&self) -> ::windows::core::Result<RadialControllerMenu> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Menu)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RadialControllerMenu>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn RotationResolutionInDegrees(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RotationResolutionInDegrees)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetRotationResolutionInDegrees(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRotationResolutionInDegrees)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn UseAutomaticHapticFeedback(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UseAutomaticHapticFeedback)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetUseAutomaticHapticFeedback(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetUseAutomaticHapticFeedback)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ScreenContactStarted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<RadialController, RadialControllerScreenContactStartedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ScreenContactStarted)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveScreenContactStarted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveScreenContactStarted)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ScreenContactEnded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<RadialController, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ScreenContactEnded)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveScreenContactEnded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveScreenContactEnded)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ScreenContactContinued<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<RadialController, RadialControllerScreenContactContinuedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ScreenContactContinued)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveScreenContactContinued<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveScreenContactContinued)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ControlLost<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<RadialController, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ControlLost)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveControlLost<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveControlLost)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RotationChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<RadialController, RadialControllerRotationChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RotationChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRotationChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveRotationChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ButtonClicked<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonClickedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ButtonClicked)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveButtonClicked<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveButtonClicked)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ControlAcquired<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<RadialController, RadialControllerControlAcquiredEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ControlAcquired)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveControlAcquired<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveControlAcquired)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ButtonPressed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonPressedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IRadialController2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ButtonPressed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveButtonPressed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IRadialController2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveButtonPressed)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ButtonHolding<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonHoldingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IRadialController2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ButtonHolding)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveButtonHolding<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IRadialController2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveButtonHolding)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ButtonReleased<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonReleasedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IRadialController2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ButtonReleased)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveButtonReleased<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IRadialController2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveButtonReleased)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IRadialControllerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn CreateForCurrentView() -> ::windows::core::Result<RadialController> {
        Self::IRadialControllerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateForCurrentView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RadialController>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRadialControllerStatics<R, F: FnOnce(&IRadialControllerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RadialController, IRadialControllerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RadialController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RadialController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RadialController {}
impl ::core::fmt::Debug for RadialController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadialController").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RadialController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialController;{3055d1c8-df51-43d4-b23b-0e1037467a09})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RadialController {
    type Vtable = IRadialController_Vtbl;
    const IID: ::windows::core::GUID = <IRadialController as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RadialController {
    const NAME: &'static str = "Windows.UI.Input.RadialController";
}
impl ::core::convert::From<RadialController> for ::windows::core::IUnknown {
    fn from(value: RadialController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialController> for ::windows::core::IUnknown {
    fn from(value: &RadialController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RadialController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RadialController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RadialController> for ::windows::core::IInspectable {
    fn from(value: RadialController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialController> for ::windows::core::IInspectable {
    fn from(value: &RadialController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RadialController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RadialController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RadialController {}
unsafe impl ::core::marker::Sync for RadialController {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct RadialControllerButtonClickedEventArgs(::windows::core::IUnknown);
impl RadialControllerButtonClickedEventArgs {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Contact(&self) -> ::windows::core::Result<RadialControllerScreenContact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Contact)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RadialControllerScreenContact>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Devices_Haptics\"`*"]
    #[cfg(feature = "Devices_Haptics")]
    pub fn SimpleHapticsController(&self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController> {
        let this = &::windows::core::Interface::cast::<IRadialControllerButtonClickedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SimpleHapticsController)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Haptics::SimpleHapticsController>(result__)
        }
    }
}
impl ::core::clone::Clone for RadialControllerButtonClickedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RadialControllerButtonClickedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RadialControllerButtonClickedEventArgs {}
impl ::core::fmt::Debug for RadialControllerButtonClickedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadialControllerButtonClickedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RadialControllerButtonClickedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerButtonClickedEventArgs;{206aa438-e651-11e5-bf62-2c27d7404e85})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RadialControllerButtonClickedEventArgs {
    type Vtable = IRadialControllerButtonClickedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IRadialControllerButtonClickedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RadialControllerButtonClickedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerButtonClickedEventArgs";
}
impl ::core::convert::From<RadialControllerButtonClickedEventArgs> for ::windows::core::IUnknown {
    fn from(value: RadialControllerButtonClickedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerButtonClickedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &RadialControllerButtonClickedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RadialControllerButtonClickedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RadialControllerButtonClickedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RadialControllerButtonClickedEventArgs> for ::windows::core::IInspectable {
    fn from(value: RadialControllerButtonClickedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerButtonClickedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &RadialControllerButtonClickedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RadialControllerButtonClickedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RadialControllerButtonClickedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RadialControllerButtonClickedEventArgs {}
unsafe impl ::core::marker::Sync for RadialControllerButtonClickedEventArgs {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct RadialControllerButtonHoldingEventArgs(::windows::core::IUnknown);
impl RadialControllerButtonHoldingEventArgs {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Contact(&self) -> ::windows::core::Result<RadialControllerScreenContact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Contact)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RadialControllerScreenContact>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Devices_Haptics\"`*"]
    #[cfg(feature = "Devices_Haptics")]
    pub fn SimpleHapticsController(&self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SimpleHapticsController)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Haptics::SimpleHapticsController>(result__)
        }
    }
}
impl ::core::clone::Clone for RadialControllerButtonHoldingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RadialControllerButtonHoldingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RadialControllerButtonHoldingEventArgs {}
impl ::core::fmt::Debug for RadialControllerButtonHoldingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadialControllerButtonHoldingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RadialControllerButtonHoldingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerButtonHoldingEventArgs;{3d577eee-3cee-11e6-b535-001bdc06ab3b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RadialControllerButtonHoldingEventArgs {
    type Vtable = IRadialControllerButtonHoldingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IRadialControllerButtonHoldingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RadialControllerButtonHoldingEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerButtonHoldingEventArgs";
}
impl ::core::convert::From<RadialControllerButtonHoldingEventArgs> for ::windows::core::IUnknown {
    fn from(value: RadialControllerButtonHoldingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerButtonHoldingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &RadialControllerButtonHoldingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RadialControllerButtonHoldingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RadialControllerButtonHoldingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RadialControllerButtonHoldingEventArgs> for ::windows::core::IInspectable {
    fn from(value: RadialControllerButtonHoldingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerButtonHoldingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &RadialControllerButtonHoldingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RadialControllerButtonHoldingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RadialControllerButtonHoldingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RadialControllerButtonHoldingEventArgs {}
unsafe impl ::core::marker::Sync for RadialControllerButtonHoldingEventArgs {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct RadialControllerButtonPressedEventArgs(::windows::core::IUnknown);
impl RadialControllerButtonPressedEventArgs {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Contact(&self) -> ::windows::core::Result<RadialControllerScreenContact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Contact)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RadialControllerScreenContact>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Devices_Haptics\"`*"]
    #[cfg(feature = "Devices_Haptics")]
    pub fn SimpleHapticsController(&self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SimpleHapticsController)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Haptics::SimpleHapticsController>(result__)
        }
    }
}
impl ::core::clone::Clone for RadialControllerButtonPressedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RadialControllerButtonPressedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RadialControllerButtonPressedEventArgs {}
impl ::core::fmt::Debug for RadialControllerButtonPressedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadialControllerButtonPressedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RadialControllerButtonPressedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerButtonPressedEventArgs;{3d577eed-4cee-11e6-b535-001bdc06ab3b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RadialControllerButtonPressedEventArgs {
    type Vtable = IRadialControllerButtonPressedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IRadialControllerButtonPressedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RadialControllerButtonPressedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerButtonPressedEventArgs";
}
impl ::core::convert::From<RadialControllerButtonPressedEventArgs> for ::windows::core::IUnknown {
    fn from(value: RadialControllerButtonPressedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerButtonPressedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &RadialControllerButtonPressedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RadialControllerButtonPressedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RadialControllerButtonPressedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RadialControllerButtonPressedEventArgs> for ::windows::core::IInspectable {
    fn from(value: RadialControllerButtonPressedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerButtonPressedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &RadialControllerButtonPressedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RadialControllerButtonPressedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RadialControllerButtonPressedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RadialControllerButtonPressedEventArgs {}
unsafe impl ::core::marker::Sync for RadialControllerButtonPressedEventArgs {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct RadialControllerButtonReleasedEventArgs(::windows::core::IUnknown);
impl RadialControllerButtonReleasedEventArgs {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Contact(&self) -> ::windows::core::Result<RadialControllerScreenContact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Contact)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RadialControllerScreenContact>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Devices_Haptics\"`*"]
    #[cfg(feature = "Devices_Haptics")]
    pub fn SimpleHapticsController(&self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SimpleHapticsController)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Haptics::SimpleHapticsController>(result__)
        }
    }
}
impl ::core::clone::Clone for RadialControllerButtonReleasedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RadialControllerButtonReleasedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RadialControllerButtonReleasedEventArgs {}
impl ::core::fmt::Debug for RadialControllerButtonReleasedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadialControllerButtonReleasedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RadialControllerButtonReleasedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerButtonReleasedEventArgs;{3d577eef-3cee-11e6-b535-001bdc06ab3b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RadialControllerButtonReleasedEventArgs {
    type Vtable = IRadialControllerButtonReleasedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IRadialControllerButtonReleasedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RadialControllerButtonReleasedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerButtonReleasedEventArgs";
}
impl ::core::convert::From<RadialControllerButtonReleasedEventArgs> for ::windows::core::IUnknown {
    fn from(value: RadialControllerButtonReleasedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerButtonReleasedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &RadialControllerButtonReleasedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RadialControllerButtonReleasedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RadialControllerButtonReleasedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RadialControllerButtonReleasedEventArgs> for ::windows::core::IInspectable {
    fn from(value: RadialControllerButtonReleasedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerButtonReleasedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &RadialControllerButtonReleasedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RadialControllerButtonReleasedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RadialControllerButtonReleasedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RadialControllerButtonReleasedEventArgs {}
unsafe impl ::core::marker::Sync for RadialControllerButtonReleasedEventArgs {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct RadialControllerConfiguration(::windows::core::IUnknown);
impl RadialControllerConfiguration {
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetDefaultMenuItems<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<RadialControllerSystemMenuItemKind>>>(&self, buttons: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDefaultMenuItems)(::core::mem::transmute_copy(this), buttons.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn ResetToDefaultMenuItems(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ResetToDefaultMenuItems)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn TrySelectDefaultMenuItem(&self, r#type: RadialControllerSystemMenuItemKind) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrySelectDefaultMenuItem)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetActiveControllerWhenMenuIsSuppressed<'a, Param0: ::windows::core::IntoParam<'a, RadialController>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IRadialControllerConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetActiveControllerWhenMenuIsSuppressed)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn ActiveControllerWhenMenuIsSuppressed(&self) -> ::windows::core::Result<RadialController> {
        let this = &::windows::core::Interface::cast::<IRadialControllerConfiguration2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActiveControllerWhenMenuIsSuppressed)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RadialController>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetIsMenuSuppressed(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IRadialControllerConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsMenuSuppressed)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsMenuSuppressed(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IRadialControllerConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsMenuSuppressed)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn GetForCurrentView() -> ::windows::core::Result<RadialControllerConfiguration> {
        Self::IRadialControllerConfigurationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RadialControllerConfiguration>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetAppController<'a, Param0: ::windows::core::IntoParam<'a, RadialController>>(value: Param0) -> ::windows::core::Result<()> {
        Self::IRadialControllerConfigurationStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).SetAppController)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn AppController() -> ::windows::core::Result<RadialController> {
        Self::IRadialControllerConfigurationStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppController)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RadialController>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetIsAppControllerEnabled(value: bool) -> ::windows::core::Result<()> {
        Self::IRadialControllerConfigurationStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).SetIsAppControllerEnabled)(::core::mem::transmute_copy(this), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsAppControllerEnabled() -> ::windows::core::Result<bool> {
        Self::IRadialControllerConfigurationStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAppControllerEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRadialControllerConfigurationStatics<R, F: FnOnce(&IRadialControllerConfigurationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RadialControllerConfiguration, IRadialControllerConfigurationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IRadialControllerConfigurationStatics2<R, F: FnOnce(&IRadialControllerConfigurationStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RadialControllerConfiguration, IRadialControllerConfigurationStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RadialControllerConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RadialControllerConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RadialControllerConfiguration {}
impl ::core::fmt::Debug for RadialControllerConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadialControllerConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RadialControllerConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerConfiguration;{a6b79ecb-6a52-4430-910c-56370a9d6b42})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RadialControllerConfiguration {
    type Vtable = IRadialControllerConfiguration_Vtbl;
    const IID: ::windows::core::GUID = <IRadialControllerConfiguration as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RadialControllerConfiguration {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerConfiguration";
}
impl ::core::convert::From<RadialControllerConfiguration> for ::windows::core::IUnknown {
    fn from(value: RadialControllerConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerConfiguration> for ::windows::core::IUnknown {
    fn from(value: &RadialControllerConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RadialControllerConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RadialControllerConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RadialControllerConfiguration> for ::windows::core::IInspectable {
    fn from(value: RadialControllerConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerConfiguration> for ::windows::core::IInspectable {
    fn from(value: &RadialControllerConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RadialControllerConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RadialControllerConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RadialControllerConfiguration {}
unsafe impl ::core::marker::Sync for RadialControllerConfiguration {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct RadialControllerControlAcquiredEventArgs(::windows::core::IUnknown);
impl RadialControllerControlAcquiredEventArgs {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Contact(&self) -> ::windows::core::Result<RadialControllerScreenContact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Contact)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RadialControllerScreenContact>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsButtonPressed(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IRadialControllerControlAcquiredEventArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsButtonPressed)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Devices_Haptics\"`*"]
    #[cfg(feature = "Devices_Haptics")]
    pub fn SimpleHapticsController(&self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController> {
        let this = &::windows::core::Interface::cast::<IRadialControllerControlAcquiredEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SimpleHapticsController)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Haptics::SimpleHapticsController>(result__)
        }
    }
}
impl ::core::clone::Clone for RadialControllerControlAcquiredEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RadialControllerControlAcquiredEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RadialControllerControlAcquiredEventArgs {}
impl ::core::fmt::Debug for RadialControllerControlAcquiredEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadialControllerControlAcquiredEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RadialControllerControlAcquiredEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerControlAcquiredEventArgs;{206aa439-e651-11e5-bf62-2c27d7404e85})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RadialControllerControlAcquiredEventArgs {
    type Vtable = IRadialControllerControlAcquiredEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IRadialControllerControlAcquiredEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RadialControllerControlAcquiredEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerControlAcquiredEventArgs";
}
impl ::core::convert::From<RadialControllerControlAcquiredEventArgs> for ::windows::core::IUnknown {
    fn from(value: RadialControllerControlAcquiredEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerControlAcquiredEventArgs> for ::windows::core::IUnknown {
    fn from(value: &RadialControllerControlAcquiredEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RadialControllerControlAcquiredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RadialControllerControlAcquiredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RadialControllerControlAcquiredEventArgs> for ::windows::core::IInspectable {
    fn from(value: RadialControllerControlAcquiredEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerControlAcquiredEventArgs> for ::windows::core::IInspectable {
    fn from(value: &RadialControllerControlAcquiredEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RadialControllerControlAcquiredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RadialControllerControlAcquiredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RadialControllerControlAcquiredEventArgs {}
unsafe impl ::core::marker::Sync for RadialControllerControlAcquiredEventArgs {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct RadialControllerMenu(::windows::core::IUnknown);
impl RadialControllerMenu {
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<RadialControllerMenuItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Items)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<RadialControllerMenuItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn GetSelectedMenuItem(&self) -> ::windows::core::Result<RadialControllerMenuItem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetSelectedMenuItem)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RadialControllerMenuItem>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SelectMenuItem<'a, Param0: ::windows::core::IntoParam<'a, RadialControllerMenuItem>>(&self, menuitem: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SelectMenuItem)(::core::mem::transmute_copy(this), menuitem.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn TrySelectPreviouslySelectedMenuItem(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrySelectPreviouslySelectedMenuItem)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for RadialControllerMenu {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RadialControllerMenu {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RadialControllerMenu {}
impl ::core::fmt::Debug for RadialControllerMenu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadialControllerMenu").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RadialControllerMenu {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerMenu;{8506b35d-f640-4412-aba0-bad077e5ea8a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RadialControllerMenu {
    type Vtable = IRadialControllerMenu_Vtbl;
    const IID: ::windows::core::GUID = <IRadialControllerMenu as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RadialControllerMenu {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerMenu";
}
impl ::core::convert::From<RadialControllerMenu> for ::windows::core::IUnknown {
    fn from(value: RadialControllerMenu) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerMenu> for ::windows::core::IUnknown {
    fn from(value: &RadialControllerMenu) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RadialControllerMenu {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RadialControllerMenu {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RadialControllerMenu> for ::windows::core::IInspectable {
    fn from(value: RadialControllerMenu) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerMenu> for ::windows::core::IInspectable {
    fn from(value: &RadialControllerMenu) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RadialControllerMenu {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RadialControllerMenu {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RadialControllerMenu {}
unsafe impl ::core::marker::Sync for RadialControllerMenu {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct RadialControllerMenuItem(::windows::core::IUnknown);
impl RadialControllerMenuItem {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn DisplayText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayText)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Tag(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Tag)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetTag<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTag)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Invoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<RadialControllerMenuItem, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Invoked)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveInvoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveInvoked)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromIcon<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::RandomAccessStreamReference>>(displaytext: Param0, icon: Param1) -> ::windows::core::Result<RadialControllerMenuItem> {
        Self::IRadialControllerMenuItemStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromIcon)(::core::mem::transmute_copy(this), displaytext.into_param().abi(), icon.into_param().abi(), &mut result__).from_abi::<RadialControllerMenuItem>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn CreateFromKnownIcon<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(displaytext: Param0, value: RadialControllerMenuKnownIcon) -> ::windows::core::Result<RadialControllerMenuItem> {
        Self::IRadialControllerMenuItemStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromKnownIcon)(::core::mem::transmute_copy(this), displaytext.into_param().abi(), value, &mut result__).from_abi::<RadialControllerMenuItem>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn CreateFromFontGlyph<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(displaytext: Param0, glyph: Param1, fontfamily: Param2) -> ::windows::core::Result<RadialControllerMenuItem> {
        Self::IRadialControllerMenuItemStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromFontGlyph)(::core::mem::transmute_copy(this), displaytext.into_param().abi(), glyph.into_param().abi(), fontfamily.into_param().abi(), &mut result__).from_abi::<RadialControllerMenuItem>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFromFontGlyphWithUri<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(displaytext: Param0, glyph: Param1, fontfamily: Param2, fonturi: Param3) -> ::windows::core::Result<RadialControllerMenuItem> {
        Self::IRadialControllerMenuItemStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromFontGlyphWithUri)(::core::mem::transmute_copy(this), displaytext.into_param().abi(), glyph.into_param().abi(), fontfamily.into_param().abi(), fonturi.into_param().abi(), &mut result__).from_abi::<RadialControllerMenuItem>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRadialControllerMenuItemStatics<R, F: FnOnce(&IRadialControllerMenuItemStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RadialControllerMenuItem, IRadialControllerMenuItemStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IRadialControllerMenuItemStatics2<R, F: FnOnce(&IRadialControllerMenuItemStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RadialControllerMenuItem, IRadialControllerMenuItemStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RadialControllerMenuItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RadialControllerMenuItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RadialControllerMenuItem {}
impl ::core::fmt::Debug for RadialControllerMenuItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadialControllerMenuItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RadialControllerMenuItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerMenuItem;{c80fc98d-ad0b-4c9c-8f2f-136a2373a6ba})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RadialControllerMenuItem {
    type Vtable = IRadialControllerMenuItem_Vtbl;
    const IID: ::windows::core::GUID = <IRadialControllerMenuItem as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RadialControllerMenuItem {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerMenuItem";
}
impl ::core::convert::From<RadialControllerMenuItem> for ::windows::core::IUnknown {
    fn from(value: RadialControllerMenuItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerMenuItem> for ::windows::core::IUnknown {
    fn from(value: &RadialControllerMenuItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RadialControllerMenuItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RadialControllerMenuItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RadialControllerMenuItem> for ::windows::core::IInspectable {
    fn from(value: RadialControllerMenuItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerMenuItem> for ::windows::core::IInspectable {
    fn from(value: &RadialControllerMenuItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RadialControllerMenuItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RadialControllerMenuItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RadialControllerMenuItem {}
unsafe impl ::core::marker::Sync for RadialControllerMenuItem {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RadialControllerMenuKnownIcon(pub i32);
impl RadialControllerMenuKnownIcon {
    pub const Scroll: Self = Self(0i32);
    pub const Zoom: Self = Self(1i32);
    pub const UndoRedo: Self = Self(2i32);
    pub const Volume: Self = Self(3i32);
    pub const NextPreviousTrack: Self = Self(4i32);
    pub const Ruler: Self = Self(5i32);
    pub const InkColor: Self = Self(6i32);
    pub const InkThickness: Self = Self(7i32);
    pub const PenType: Self = Self(8i32);
}
impl ::core::marker::Copy for RadialControllerMenuKnownIcon {}
impl ::core::clone::Clone for RadialControllerMenuKnownIcon {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RadialControllerMenuKnownIcon {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RadialControllerMenuKnownIcon {
    type Abi = Self;
}
impl ::core::fmt::Debug for RadialControllerMenuKnownIcon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadialControllerMenuKnownIcon").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RadialControllerMenuKnownIcon {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.RadialControllerMenuKnownIcon;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct RadialControllerRotationChangedEventArgs(::windows::core::IUnknown);
impl RadialControllerRotationChangedEventArgs {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn RotationDeltaInDegrees(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RotationDeltaInDegrees)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Contact(&self) -> ::windows::core::Result<RadialControllerScreenContact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Contact)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RadialControllerScreenContact>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsButtonPressed(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IRadialControllerRotationChangedEventArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsButtonPressed)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Devices_Haptics\"`*"]
    #[cfg(feature = "Devices_Haptics")]
    pub fn SimpleHapticsController(&self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController> {
        let this = &::windows::core::Interface::cast::<IRadialControllerRotationChangedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SimpleHapticsController)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Haptics::SimpleHapticsController>(result__)
        }
    }
}
impl ::core::clone::Clone for RadialControllerRotationChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RadialControllerRotationChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RadialControllerRotationChangedEventArgs {}
impl ::core::fmt::Debug for RadialControllerRotationChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadialControllerRotationChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RadialControllerRotationChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerRotationChangedEventArgs;{206aa435-e651-11e5-bf62-2c27d7404e85})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RadialControllerRotationChangedEventArgs {
    type Vtable = IRadialControllerRotationChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IRadialControllerRotationChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RadialControllerRotationChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerRotationChangedEventArgs";
}
impl ::core::convert::From<RadialControllerRotationChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: RadialControllerRotationChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerRotationChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &RadialControllerRotationChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RadialControllerRotationChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RadialControllerRotationChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RadialControllerRotationChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: RadialControllerRotationChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerRotationChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &RadialControllerRotationChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RadialControllerRotationChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RadialControllerRotationChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RadialControllerRotationChangedEventArgs {}
unsafe impl ::core::marker::Sync for RadialControllerRotationChangedEventArgs {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct RadialControllerScreenContact(::windows::core::IUnknown);
impl RadialControllerScreenContact {
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Bounds(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Bounds)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
}
impl ::core::clone::Clone for RadialControllerScreenContact {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RadialControllerScreenContact {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RadialControllerScreenContact {}
impl ::core::fmt::Debug for RadialControllerScreenContact {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadialControllerScreenContact").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RadialControllerScreenContact {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerScreenContact;{206aa434-e651-11e5-bf62-2c27d7404e85})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RadialControllerScreenContact {
    type Vtable = IRadialControllerScreenContact_Vtbl;
    const IID: ::windows::core::GUID = <IRadialControllerScreenContact as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RadialControllerScreenContact {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerScreenContact";
}
impl ::core::convert::From<RadialControllerScreenContact> for ::windows::core::IUnknown {
    fn from(value: RadialControllerScreenContact) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerScreenContact> for ::windows::core::IUnknown {
    fn from(value: &RadialControllerScreenContact) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RadialControllerScreenContact {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RadialControllerScreenContact {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RadialControllerScreenContact> for ::windows::core::IInspectable {
    fn from(value: RadialControllerScreenContact) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerScreenContact> for ::windows::core::IInspectable {
    fn from(value: &RadialControllerScreenContact) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RadialControllerScreenContact {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RadialControllerScreenContact {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RadialControllerScreenContact {}
unsafe impl ::core::marker::Sync for RadialControllerScreenContact {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct RadialControllerScreenContactContinuedEventArgs(::windows::core::IUnknown);
impl RadialControllerScreenContactContinuedEventArgs {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Contact(&self) -> ::windows::core::Result<RadialControllerScreenContact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Contact)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RadialControllerScreenContact>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsButtonPressed(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IRadialControllerScreenContactContinuedEventArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsButtonPressed)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Devices_Haptics\"`*"]
    #[cfg(feature = "Devices_Haptics")]
    pub fn SimpleHapticsController(&self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController> {
        let this = &::windows::core::Interface::cast::<IRadialControllerScreenContactContinuedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SimpleHapticsController)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Haptics::SimpleHapticsController>(result__)
        }
    }
}
impl ::core::clone::Clone for RadialControllerScreenContactContinuedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RadialControllerScreenContactContinuedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RadialControllerScreenContactContinuedEventArgs {}
impl ::core::fmt::Debug for RadialControllerScreenContactContinuedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadialControllerScreenContactContinuedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RadialControllerScreenContactContinuedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerScreenContactContinuedEventArgs;{206aa437-e651-11e5-bf62-2c27d7404e85})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RadialControllerScreenContactContinuedEventArgs {
    type Vtable = IRadialControllerScreenContactContinuedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IRadialControllerScreenContactContinuedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RadialControllerScreenContactContinuedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerScreenContactContinuedEventArgs";
}
impl ::core::convert::From<RadialControllerScreenContactContinuedEventArgs> for ::windows::core::IUnknown {
    fn from(value: RadialControllerScreenContactContinuedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerScreenContactContinuedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &RadialControllerScreenContactContinuedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RadialControllerScreenContactContinuedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RadialControllerScreenContactContinuedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RadialControllerScreenContactContinuedEventArgs> for ::windows::core::IInspectable {
    fn from(value: RadialControllerScreenContactContinuedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerScreenContactContinuedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &RadialControllerScreenContactContinuedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RadialControllerScreenContactContinuedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RadialControllerScreenContactContinuedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RadialControllerScreenContactContinuedEventArgs {}
unsafe impl ::core::marker::Sync for RadialControllerScreenContactContinuedEventArgs {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct RadialControllerScreenContactEndedEventArgs(::windows::core::IUnknown);
impl RadialControllerScreenContactEndedEventArgs {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsButtonPressed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsButtonPressed)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Devices_Haptics\"`*"]
    #[cfg(feature = "Devices_Haptics")]
    pub fn SimpleHapticsController(&self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SimpleHapticsController)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Haptics::SimpleHapticsController>(result__)
        }
    }
}
impl ::core::clone::Clone for RadialControllerScreenContactEndedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RadialControllerScreenContactEndedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RadialControllerScreenContactEndedEventArgs {}
impl ::core::fmt::Debug for RadialControllerScreenContactEndedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadialControllerScreenContactEndedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RadialControllerScreenContactEndedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerScreenContactEndedEventArgs;{3d577ef2-3cee-11e6-b535-001bdc06ab3b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RadialControllerScreenContactEndedEventArgs {
    type Vtable = IRadialControllerScreenContactEndedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IRadialControllerScreenContactEndedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RadialControllerScreenContactEndedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerScreenContactEndedEventArgs";
}
impl ::core::convert::From<RadialControllerScreenContactEndedEventArgs> for ::windows::core::IUnknown {
    fn from(value: RadialControllerScreenContactEndedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerScreenContactEndedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &RadialControllerScreenContactEndedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RadialControllerScreenContactEndedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RadialControllerScreenContactEndedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RadialControllerScreenContactEndedEventArgs> for ::windows::core::IInspectable {
    fn from(value: RadialControllerScreenContactEndedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerScreenContactEndedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &RadialControllerScreenContactEndedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RadialControllerScreenContactEndedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RadialControllerScreenContactEndedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RadialControllerScreenContactEndedEventArgs {}
unsafe impl ::core::marker::Sync for RadialControllerScreenContactEndedEventArgs {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct RadialControllerScreenContactStartedEventArgs(::windows::core::IUnknown);
impl RadialControllerScreenContactStartedEventArgs {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Contact(&self) -> ::windows::core::Result<RadialControllerScreenContact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Contact)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RadialControllerScreenContact>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsButtonPressed(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IRadialControllerScreenContactStartedEventArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsButtonPressed)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Devices_Haptics\"`*"]
    #[cfg(feature = "Devices_Haptics")]
    pub fn SimpleHapticsController(&self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController> {
        let this = &::windows::core::Interface::cast::<IRadialControllerScreenContactStartedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SimpleHapticsController)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Haptics::SimpleHapticsController>(result__)
        }
    }
}
impl ::core::clone::Clone for RadialControllerScreenContactStartedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RadialControllerScreenContactStartedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RadialControllerScreenContactStartedEventArgs {}
impl ::core::fmt::Debug for RadialControllerScreenContactStartedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadialControllerScreenContactStartedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RadialControllerScreenContactStartedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RadialControllerScreenContactStartedEventArgs;{206aa436-e651-11e5-bf62-2c27d7404e85})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RadialControllerScreenContactStartedEventArgs {
    type Vtable = IRadialControllerScreenContactStartedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IRadialControllerScreenContactStartedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RadialControllerScreenContactStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerScreenContactStartedEventArgs";
}
impl ::core::convert::From<RadialControllerScreenContactStartedEventArgs> for ::windows::core::IUnknown {
    fn from(value: RadialControllerScreenContactStartedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerScreenContactStartedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &RadialControllerScreenContactStartedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RadialControllerScreenContactStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RadialControllerScreenContactStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RadialControllerScreenContactStartedEventArgs> for ::windows::core::IInspectable {
    fn from(value: RadialControllerScreenContactStartedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerScreenContactStartedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &RadialControllerScreenContactStartedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RadialControllerScreenContactStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RadialControllerScreenContactStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RadialControllerScreenContactStartedEventArgs {}
unsafe impl ::core::marker::Sync for RadialControllerScreenContactStartedEventArgs {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RadialControllerSystemMenuItemKind(pub i32);
impl RadialControllerSystemMenuItemKind {
    pub const Scroll: Self = Self(0i32);
    pub const Zoom: Self = Self(1i32);
    pub const UndoRedo: Self = Self(2i32);
    pub const Volume: Self = Self(3i32);
    pub const NextPreviousTrack: Self = Self(4i32);
}
impl ::core::marker::Copy for RadialControllerSystemMenuItemKind {}
impl ::core::clone::Clone for RadialControllerSystemMenuItemKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RadialControllerSystemMenuItemKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RadialControllerSystemMenuItemKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for RadialControllerSystemMenuItemKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadialControllerSystemMenuItemKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RadialControllerSystemMenuItemKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.RadialControllerSystemMenuItemKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct RightTappedEventArgs(::windows::core::IUnknown);
impl RightTappedEventArgs {
    #[doc = "*Required features: `\"UI_Input\"`, `\"Devices_Input\"`*"]
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerDeviceType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn ContactCount(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IRightTappedEventArgs2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContactCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for RightTappedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RightTappedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RightTappedEventArgs {}
impl ::core::fmt::Debug for RightTappedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RightTappedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RightTappedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.RightTappedEventArgs;{4cbf40bd-af7a-4a36-9476-b1dce141709a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RightTappedEventArgs {
    type Vtable = IRightTappedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IRightTappedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RightTappedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RightTappedEventArgs";
}
impl ::core::convert::From<RightTappedEventArgs> for ::windows::core::IUnknown {
    fn from(value: RightTappedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RightTappedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &RightTappedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RightTappedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RightTappedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RightTappedEventArgs> for ::windows::core::IInspectable {
    fn from(value: RightTappedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RightTappedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &RightTappedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RightTappedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RightTappedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct SystemButtonEventController(::windows::core::IUnknown);
impl SystemButtonEventController {
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SystemFunctionButtonPressed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionButtonEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SystemFunctionButtonPressed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSystemFunctionButtonPressed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSystemFunctionButtonPressed)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SystemFunctionButtonReleased<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionButtonEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SystemFunctionButtonReleased)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSystemFunctionButtonReleased<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSystemFunctionButtonReleased)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SystemFunctionLockChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionLockChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SystemFunctionLockChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSystemFunctionLockChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSystemFunctionLockChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SystemFunctionLockIndicatorChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionLockIndicatorChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SystemFunctionLockIndicatorChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSystemFunctionLockIndicatorChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSystemFunctionLockIndicatorChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn CreateForDispatcherQueue<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::DispatcherQueue>>(queue: Param0) -> ::windows::core::Result<SystemButtonEventController> {
        Self::ISystemButtonEventControllerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateForDispatcherQueue)(::core::mem::transmute_copy(this), queue.into_param().abi(), &mut result__).from_abi::<SystemButtonEventController>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISystemButtonEventControllerStatics<R, F: FnOnce(&ISystemButtonEventControllerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SystemButtonEventController, ISystemButtonEventControllerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SystemButtonEventController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemButtonEventController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemButtonEventController {}
impl ::core::fmt::Debug for SystemButtonEventController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemButtonEventController").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SystemButtonEventController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.SystemButtonEventController;{59b893a9-73bc-52b5-ba41-82511b2cb46c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SystemButtonEventController {
    type Vtable = ISystemButtonEventController_Vtbl;
    const IID: ::windows::core::GUID = <ISystemButtonEventController as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SystemButtonEventController {
    const NAME: &'static str = "Windows.UI.Input.SystemButtonEventController";
}
impl ::core::convert::From<SystemButtonEventController> for ::windows::core::IUnknown {
    fn from(value: SystemButtonEventController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemButtonEventController> for ::windows::core::IUnknown {
    fn from(value: &SystemButtonEventController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemButtonEventController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemButtonEventController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemButtonEventController> for ::windows::core::IInspectable {
    fn from(value: SystemButtonEventController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemButtonEventController> for ::windows::core::IInspectable {
    fn from(value: &SystemButtonEventController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemButtonEventController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemButtonEventController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<SystemButtonEventController> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: SystemButtonEventController) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&SystemButtonEventController> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &SystemButtonEventController) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for SystemButtonEventController {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &SystemButtonEventController {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<SystemButtonEventController> for AttachableInputObject {
    fn from(value: SystemButtonEventController) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SystemButtonEventController> for AttachableInputObject {
    fn from(value: &SystemButtonEventController) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, AttachableInputObject> for SystemButtonEventController {
    fn into_param(self) -> ::windows::core::Param<'a, AttachableInputObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, AttachableInputObject> for &SystemButtonEventController {
    fn into_param(self) -> ::windows::core::Param<'a, AttachableInputObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<AttachableInputObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SystemButtonEventController {}
unsafe impl ::core::marker::Sync for SystemButtonEventController {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct SystemFunctionButtonEventArgs(::windows::core::IUnknown);
impl SystemFunctionButtonEventArgs {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Timestamp(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Timestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for SystemFunctionButtonEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemFunctionButtonEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemFunctionButtonEventArgs {}
impl ::core::fmt::Debug for SystemFunctionButtonEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemFunctionButtonEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SystemFunctionButtonEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.SystemFunctionButtonEventArgs;{4833896f-80d1-5dd6-92a7-62a508ffef5a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SystemFunctionButtonEventArgs {
    type Vtable = ISystemFunctionButtonEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ISystemFunctionButtonEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SystemFunctionButtonEventArgs {
    const NAME: &'static str = "Windows.UI.Input.SystemFunctionButtonEventArgs";
}
impl ::core::convert::From<SystemFunctionButtonEventArgs> for ::windows::core::IUnknown {
    fn from(value: SystemFunctionButtonEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemFunctionButtonEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SystemFunctionButtonEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemFunctionButtonEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemFunctionButtonEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemFunctionButtonEventArgs> for ::windows::core::IInspectable {
    fn from(value: SystemFunctionButtonEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemFunctionButtonEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SystemFunctionButtonEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemFunctionButtonEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemFunctionButtonEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemFunctionButtonEventArgs {}
unsafe impl ::core::marker::Sync for SystemFunctionButtonEventArgs {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct SystemFunctionLockChangedEventArgs(::windows::core::IUnknown);
impl SystemFunctionLockChangedEventArgs {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Timestamp(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Timestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsLocked(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsLocked)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for SystemFunctionLockChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemFunctionLockChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemFunctionLockChangedEventArgs {}
impl ::core::fmt::Debug for SystemFunctionLockChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemFunctionLockChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SystemFunctionLockChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.SystemFunctionLockChangedEventArgs;{cd040608-fcf9-585c-beab-f1d2eaf364ab})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SystemFunctionLockChangedEventArgs {
    type Vtable = ISystemFunctionLockChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ISystemFunctionLockChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SystemFunctionLockChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.SystemFunctionLockChangedEventArgs";
}
impl ::core::convert::From<SystemFunctionLockChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SystemFunctionLockChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemFunctionLockChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SystemFunctionLockChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemFunctionLockChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemFunctionLockChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemFunctionLockChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SystemFunctionLockChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemFunctionLockChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SystemFunctionLockChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemFunctionLockChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemFunctionLockChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemFunctionLockChangedEventArgs {}
unsafe impl ::core::marker::Sync for SystemFunctionLockChangedEventArgs {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct SystemFunctionLockIndicatorChangedEventArgs(::windows::core::IUnknown);
impl SystemFunctionLockIndicatorChangedEventArgs {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Timestamp(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Timestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsIndicatorOn(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsIndicatorOn)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for SystemFunctionLockIndicatorChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemFunctionLockIndicatorChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemFunctionLockIndicatorChangedEventArgs {}
impl ::core::fmt::Debug for SystemFunctionLockIndicatorChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemFunctionLockIndicatorChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SystemFunctionLockIndicatorChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.SystemFunctionLockIndicatorChangedEventArgs;{b212b94e-7a6f-58ae-b304-bae61d0371b9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SystemFunctionLockIndicatorChangedEventArgs {
    type Vtable = ISystemFunctionLockIndicatorChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ISystemFunctionLockIndicatorChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SystemFunctionLockIndicatorChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.SystemFunctionLockIndicatorChangedEventArgs";
}
impl ::core::convert::From<SystemFunctionLockIndicatorChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SystemFunctionLockIndicatorChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemFunctionLockIndicatorChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SystemFunctionLockIndicatorChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemFunctionLockIndicatorChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemFunctionLockIndicatorChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemFunctionLockIndicatorChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SystemFunctionLockIndicatorChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemFunctionLockIndicatorChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SystemFunctionLockIndicatorChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemFunctionLockIndicatorChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemFunctionLockIndicatorChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemFunctionLockIndicatorChangedEventArgs {}
unsafe impl ::core::marker::Sync for SystemFunctionLockIndicatorChangedEventArgs {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct TappedEventArgs(::windows::core::IUnknown);
impl TappedEventArgs {
    #[doc = "*Required features: `\"UI_Input\"`, `\"Devices_Input\"`*"]
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerDeviceType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn TapCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TapCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn ContactCount(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ITappedEventArgs2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContactCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for TappedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TappedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TappedEventArgs {}
impl ::core::fmt::Debug for TappedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TappedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TappedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.TappedEventArgs;{cfa126e4-253a-4c3c-953b-395c37aed309})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TappedEventArgs {
    type Vtable = ITappedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ITappedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TappedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.TappedEventArgs";
}
impl ::core::convert::From<TappedEventArgs> for ::windows::core::IUnknown {
    fn from(value: TappedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TappedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &TappedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TappedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TappedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TappedEventArgs> for ::windows::core::IInspectable {
    fn from(value: TappedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TappedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &TappedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TappedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TappedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
