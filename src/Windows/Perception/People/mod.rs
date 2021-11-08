#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Perception_People`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EyesPose(pub ::windows::runtime::IInspectable);
impl EyesPose {
    #[doc = "*Required features: `Perception_People`*"]
    pub fn IsCalibrationValid(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    #[doc = "*Required features: `Perception_People`, `Foundation`, `Foundation_Numerics`, `Perception_Spatial`*"]
    pub fn Gaze(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<super::Spatial::SpatialRay>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::Spatial::SpatialRay>>(result__)
        }
    }
    #[doc = "*Required features: `Perception_People`*"]
    pub fn UpdateTimestamp(&self) -> ::windows::runtime::Result<super::PerceptionTimestamp> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::PerceptionTimestamp>(result__)
        }
    }
    #[doc = "*Required features: `Perception_People`*"]
    pub fn IsSupported() -> ::windows::runtime::Result<bool> {
        Self::IEyesPoseStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    #[doc = "*Required features: `Perception_People`, `Foundation`, `UI_Input`*"]
    pub fn RequestAccessAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::UI::Input::GazeInputAccessStatus>> {
        Self::IEyesPoseStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::UI::Input::GazeInputAccessStatus>>(result__)
        })
    }
    pub fn IEyesPoseStatics<R, F: FnOnce(&IEyesPoseStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<EyesPose, IEyesPoseStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EyesPose {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Perception.People.EyesPose;{682a9b23-8a1e-5b86-a060-906ffacb62a4})");
}
unsafe impl ::windows::runtime::Interface for EyesPose {
    type Vtable = IEyesPose_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1747622691, 35358, 23430, [160, 96, 144, 111, 250, 203, 98, 164]);
}
impl ::windows::runtime::RuntimeName for EyesPose {
    const NAME: &'static str = "Windows.Perception.People.EyesPose";
}
impl ::core::convert::From<EyesPose> for ::windows::runtime::IUnknown {
    fn from(value: EyesPose) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EyesPose> for ::windows::runtime::IUnknown {
    fn from(value: &EyesPose) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EyesPose {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a EyesPose {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EyesPose> for ::windows::runtime::IInspectable {
    fn from(value: EyesPose) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EyesPose> for ::windows::runtime::IInspectable {
    fn from(value: &EyesPose) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EyesPose {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EyesPose {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EyesPose {}
unsafe impl ::core::marker::Sync for EyesPose {}
#[doc = "*Required features: `Perception_People`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct HandJointKind(pub i32);
impl HandJointKind {
    pub const Palm: HandJointKind = HandJointKind(0i32);
    pub const Wrist: HandJointKind = HandJointKind(1i32);
    pub const ThumbMetacarpal: HandJointKind = HandJointKind(2i32);
    pub const ThumbProximal: HandJointKind = HandJointKind(3i32);
    pub const ThumbDistal: HandJointKind = HandJointKind(4i32);
    pub const ThumbTip: HandJointKind = HandJointKind(5i32);
    pub const IndexMetacarpal: HandJointKind = HandJointKind(6i32);
    pub const IndexProximal: HandJointKind = HandJointKind(7i32);
    pub const IndexIntermediate: HandJointKind = HandJointKind(8i32);
    pub const IndexDistal: HandJointKind = HandJointKind(9i32);
    pub const IndexTip: HandJointKind = HandJointKind(10i32);
    pub const MiddleMetacarpal: HandJointKind = HandJointKind(11i32);
    pub const MiddleProximal: HandJointKind = HandJointKind(12i32);
    pub const MiddleIntermediate: HandJointKind = HandJointKind(13i32);
    pub const MiddleDistal: HandJointKind = HandJointKind(14i32);
    pub const MiddleTip: HandJointKind = HandJointKind(15i32);
    pub const RingMetacarpal: HandJointKind = HandJointKind(16i32);
    pub const RingProximal: HandJointKind = HandJointKind(17i32);
    pub const RingIntermediate: HandJointKind = HandJointKind(18i32);
    pub const RingDistal: HandJointKind = HandJointKind(19i32);
    pub const RingTip: HandJointKind = HandJointKind(20i32);
    pub const LittleMetacarpal: HandJointKind = HandJointKind(21i32);
    pub const LittleProximal: HandJointKind = HandJointKind(22i32);
    pub const LittleIntermediate: HandJointKind = HandJointKind(23i32);
    pub const LittleDistal: HandJointKind = HandJointKind(24i32);
    pub const LittleTip: HandJointKind = HandJointKind(25i32);
}
impl ::core::convert::From<i32> for HandJointKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HandJointKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for HandJointKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Perception.People.HandJointKind;i4)");
}
impl ::windows::runtime::DefaultType for HandJointKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `Perception_People`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HandMeshObserver(pub ::windows::runtime::IInspectable);
impl HandMeshObserver {
    #[cfg(feature = "UI_Input_Spatial")]
    #[doc = "*Required features: `Perception_People`, `UI_Input_Spatial`*"]
    pub fn Source(&self) -> ::windows::runtime::Result<super::super::UI::Input::Spatial::SpatialInteractionSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Input::Spatial::SpatialInteractionSource>(result__)
        }
    }
    #[doc = "*Required features: `Perception_People`*"]
    pub fn TriangleIndexCount(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Perception_People`*"]
    pub fn VertexCount(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Perception_People`*"]
    pub fn GetTriangleIndices(&self, indices: &mut [<u16 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), indices.len() as u32, ::core::mem::transmute_copy(&indices)).ok() }
    }
    #[doc = "*Required features: `Perception_People`*"]
    pub fn GetVertexStateForPose<'a, Param0: ::windows::runtime::IntoParam<'a, HandPose>>(&self, handpose: Param0) -> ::windows::runtime::Result<HandMeshVertexState> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handpose.into_param().abi(), &mut result__).from_abi::<HandMeshVertexState>(result__)
        }
    }
    #[doc = "*Required features: `Perception_People`*"]
    pub fn NeutralPose(&self) -> ::windows::runtime::Result<HandPose> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HandPose>(result__)
        }
    }
    #[doc = "*Required features: `Perception_People`*"]
    pub fn NeutralPoseVersion(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Perception_People`*"]
    pub fn ModelId(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HandMeshObserver {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Perception.People.HandMeshObserver;{85ae30cb-6fc3-55c4-a7b4-29e33896ca69})");
}
unsafe impl ::windows::runtime::Interface for HandMeshObserver {
    type Vtable = IHandMeshObserver_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2242785483, 28611, 21956, [167, 180, 41, 227, 56, 150, 202, 105]);
}
impl ::windows::runtime::RuntimeName for HandMeshObserver {
    const NAME: &'static str = "Windows.Perception.People.HandMeshObserver";
}
impl ::core::convert::From<HandMeshObserver> for ::windows::runtime::IUnknown {
    fn from(value: HandMeshObserver) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HandMeshObserver> for ::windows::runtime::IUnknown {
    fn from(value: &HandMeshObserver) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HandMeshObserver {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a HandMeshObserver {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HandMeshObserver> for ::windows::runtime::IInspectable {
    fn from(value: HandMeshObserver) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HandMeshObserver> for ::windows::runtime::IInspectable {
    fn from(value: &HandMeshObserver) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HandMeshObserver {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HandMeshObserver {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HandMeshObserver {}
unsafe impl ::core::marker::Sync for HandMeshObserver {}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
#[doc = "*Required features: `Perception_People`, `Foundation_Numerics`*"]
pub struct HandMeshVertex {
    pub Position: super::super::Foundation::Numerics::Vector3,
    pub Normal: super::super::Foundation::Numerics::Vector3,
}
#[cfg(feature = "Foundation_Numerics")]
impl HandMeshVertex {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for HandMeshVertex {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for HandMeshVertex {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("HandMeshVertex").field("Position", &self.Position).field("Normal", &self.Normal).finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for HandMeshVertex {
    fn eq(&self, other: &Self) -> bool {
        self.Position == other.Position && self.Normal == other.Normal
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for HandMeshVertex {}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::runtime::Abi for HandMeshVertex {
    type Abi = Self;
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::runtime::RuntimeType for HandMeshVertex {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Perception.People.HandMeshVertex;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4))");
}
#[cfg(feature = "Foundation_Numerics")]
impl ::windows::runtime::DefaultType for HandMeshVertex {
    type DefaultType = Self;
}
#[doc = "*Required features: `Perception_People`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HandMeshVertexState(pub ::windows::runtime::IInspectable);
impl HandMeshVertexState {
    #[cfg(feature = "Perception_Spatial")]
    #[doc = "*Required features: `Perception_People`, `Perception_Spatial`*"]
    pub fn CoordinateSystem(&self) -> ::windows::runtime::Result<super::Spatial::SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Spatial::SpatialCoordinateSystem>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Perception_People`, `Foundation_Numerics`*"]
    pub fn GetVertices(&self, vertices: &mut [<HandMeshVertex as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), vertices.len() as u32, ::core::mem::transmute_copy(&vertices)).ok() }
    }
    #[doc = "*Required features: `Perception_People`*"]
    pub fn UpdateTimestamp(&self) -> ::windows::runtime::Result<super::PerceptionTimestamp> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::PerceptionTimestamp>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HandMeshVertexState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Perception.People.HandMeshVertexState;{046c5fef-1d8b-55de-ab2c-1cd424886d8f})");
}
unsafe impl ::windows::runtime::Interface for HandMeshVertexState {
    type Vtable = IHandMeshVertexState_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(74211311, 7563, 21982, [171, 44, 28, 212, 36, 136, 109, 143]);
}
impl ::windows::runtime::RuntimeName for HandMeshVertexState {
    const NAME: &'static str = "Windows.Perception.People.HandMeshVertexState";
}
impl ::core::convert::From<HandMeshVertexState> for ::windows::runtime::IUnknown {
    fn from(value: HandMeshVertexState) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HandMeshVertexState> for ::windows::runtime::IUnknown {
    fn from(value: &HandMeshVertexState) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HandMeshVertexState {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a HandMeshVertexState {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HandMeshVertexState> for ::windows::runtime::IInspectable {
    fn from(value: HandMeshVertexState) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HandMeshVertexState> for ::windows::runtime::IInspectable {
    fn from(value: &HandMeshVertexState) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HandMeshVertexState {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HandMeshVertexState {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HandMeshVertexState {}
unsafe impl ::core::marker::Sync for HandMeshVertexState {}
#[doc = "*Required features: `Perception_People`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HandPose(pub ::windows::runtime::IInspectable);
impl HandPose {
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    #[doc = "*Required features: `Perception_People`, `Foundation_Numerics`, `Perception_Spatial`*"]
    pub fn TryGetJoint<'a, Param0: ::windows::runtime::IntoParam<'a, super::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0, joint: HandJointKind, jointpose: &mut JointPose) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), joint, jointpose, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    #[doc = "*Required features: `Perception_People`, `Foundation_Numerics`, `Perception_Spatial`*"]
    pub fn TryGetJoints<'a, Param0: ::windows::runtime::IntoParam<'a, super::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0, joints: &[<HandJointKind as ::windows::runtime::DefaultType>::DefaultType], jointposes: &mut [<JointPose as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), joints.len() as u32, ::core::mem::transmute(joints.as_ptr()), jointposes.len() as u32, ::core::mem::transmute_copy(&jointposes), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Perception_People`, `Foundation_Numerics`*"]
    pub fn GetRelativeJoint(&self, joint: HandJointKind, referencejoint: HandJointKind) -> ::windows::runtime::Result<JointPose> {
        let this = self;
        unsafe {
            let mut result__: JointPose = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), joint, referencejoint, &mut result__).from_abi::<JointPose>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Perception_People`, `Foundation_Numerics`*"]
    pub fn GetRelativeJoints(&self, joints: &[<HandJointKind as ::windows::runtime::DefaultType>::DefaultType], referencejoints: &[<HandJointKind as ::windows::runtime::DefaultType>::DefaultType], jointposes: &mut [<JointPose as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), joints.len() as u32, ::core::mem::transmute(joints.as_ptr()), referencejoints.len() as u32, ::core::mem::transmute(referencejoints.as_ptr()), jointposes.len() as u32, ::core::mem::transmute_copy(&jointposes)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HandPose {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Perception.People.HandPose;{4d98e79a-bb08-5d09-91de-df0dd3fae46c})");
}
unsafe impl ::windows::runtime::Interface for HandPose {
    type Vtable = IHandPose_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1301866394, 47880, 23817, [145, 222, 223, 13, 211, 250, 228, 108]);
}
impl ::windows::runtime::RuntimeName for HandPose {
    const NAME: &'static str = "Windows.Perception.People.HandPose";
}
impl ::core::convert::From<HandPose> for ::windows::runtime::IUnknown {
    fn from(value: HandPose) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HandPose> for ::windows::runtime::IUnknown {
    fn from(value: &HandPose) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HandPose {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a HandPose {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HandPose> for ::windows::runtime::IInspectable {
    fn from(value: HandPose) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HandPose> for ::windows::runtime::IInspectable {
    fn from(value: &HandPose) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HandPose {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HandPose {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HandPose {}
unsafe impl ::core::marker::Sync for HandPose {}
#[doc = "*Required features: `Perception_People`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HeadPose(pub ::windows::runtime::IInspectable);
impl HeadPose {
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Perception_People`, `Foundation_Numerics`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Perception_People`, `Foundation_Numerics`*"]
    pub fn ForwardDirection(&self) -> ::windows::runtime::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Perception_People`, `Foundation_Numerics`*"]
    pub fn UpDirection(&self) -> ::windows::runtime::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HeadPose {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Perception.People.HeadPose;{7f5ac5a5-49db-379f-9429-32a2faf34fa6})");
}
unsafe impl ::windows::runtime::Interface for HeadPose {
    type Vtable = IHeadPose_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2136655269, 18907, 14239, [148, 41, 50, 162, 250, 243, 79, 166]);
}
impl ::windows::runtime::RuntimeName for HeadPose {
    const NAME: &'static str = "Windows.Perception.People.HeadPose";
}
impl ::core::convert::From<HeadPose> for ::windows::runtime::IUnknown {
    fn from(value: HeadPose) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HeadPose> for ::windows::runtime::IUnknown {
    fn from(value: &HeadPose) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HeadPose {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a HeadPose {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HeadPose> for ::windows::runtime::IInspectable {
    fn from(value: HeadPose) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HeadPose> for ::windows::runtime::IInspectable {
    fn from(value: &HeadPose) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HeadPose {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HeadPose {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HeadPose {}
unsafe impl ::core::marker::Sync for HeadPose {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IEyesPose(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEyesPose {
    type Vtable = IEyesPose_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1747622691, 35358, 23430, [160, 96, 144, 111, 250, 203, 98, 164]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEyesPose_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEyesPoseStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEyesPoseStatics {
    type Vtable = IEyesPoseStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(486503443, 45599, 21696, [128, 193, 230, 13, 153, 76, 165, 140]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEyesPoseStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHandMeshObserver(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHandMeshObserver {
    type Vtable = IHandMeshObserver_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2242785483, 28611, 21956, [167, 180, 41, 227, 56, 150, 202, 105]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHandMeshObserver_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Input_Spatial")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Input_Spatial"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, indices_array_size: u32, indices: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handpose: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHandMeshVertexState(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHandMeshVertexState {
    type Vtable = IHandMeshVertexState_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(74211311, 7563, 21982, [171, 44, 28, 212, 36, 136, 109, 143]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHandMeshVertexState_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Perception_Spatial")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vertices_array_size: u32, vertices: *mut HandMeshVertex) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHandPose(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHandPose {
    type Vtable = IHandPose_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1301866394, 47880, 23817, [145, 222, 223, 13, 211, 250, 228, 108]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHandPose_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, coordinatesystem: ::windows::runtime::RawPtr, joint: HandJointKind, jointpose: *mut JointPose, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))] usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, coordinatesystem: ::windows::runtime::RawPtr, joints_array_size: u32, joints: *const HandJointKind, jointPoses_array_size: u32, jointposes: *mut JointPose, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, joint: HandJointKind, referencejoint: HandJointKind, result__: *mut JointPose) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, joints_array_size: u32, joints: *const HandJointKind, referenceJoints_array_size: u32, referencejoints: *const HandJointKind, jointPoses_array_size: u32, jointposes: *mut JointPose) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHeadPose(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHeadPose {
    type Vtable = IHeadPose_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2136655269, 18907, 14239, [148, 41, 50, 162, 250, 243, 79, 166]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHeadPose_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
#[doc = "*Required features: `Perception_People`, `Foundation_Numerics`*"]
pub struct JointPose {
    pub Orientation: super::super::Foundation::Numerics::Quaternion,
    pub Position: super::super::Foundation::Numerics::Vector3,
    pub Radius: f32,
    pub Accuracy: JointPoseAccuracy,
}
#[cfg(feature = "Foundation_Numerics")]
impl JointPose {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for JointPose {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for JointPose {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("JointPose").field("Orientation", &self.Orientation).field("Position", &self.Position).field("Radius", &self.Radius).field("Accuracy", &self.Accuracy).finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for JointPose {
    fn eq(&self, other: &Self) -> bool {
        self.Orientation == other.Orientation && self.Position == other.Position && self.Radius == other.Radius && self.Accuracy == other.Accuracy
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for JointPose {}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::runtime::Abi for JointPose {
    type Abi = Self;
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::runtime::RuntimeType for JointPose {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Perception.People.JointPose;struct(Windows.Foundation.Numerics.Quaternion;f4;f4;f4;f4);struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4;enum(Windows.Perception.People.JointPoseAccuracy;i4))");
}
#[cfg(feature = "Foundation_Numerics")]
impl ::windows::runtime::DefaultType for JointPose {
    type DefaultType = Self;
}
#[doc = "*Required features: `Perception_People`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct JointPoseAccuracy(pub i32);
impl JointPoseAccuracy {
    pub const High: JointPoseAccuracy = JointPoseAccuracy(0i32);
    pub const Approximate: JointPoseAccuracy = JointPoseAccuracy(1i32);
}
impl ::core::convert::From<i32> for JointPoseAccuracy {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for JointPoseAccuracy {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for JointPoseAccuracy {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Perception.People.JointPoseAccuracy;i4)");
}
impl ::windows::runtime::DefaultType for JointPoseAccuracy {
    type DefaultType = Self;
}
