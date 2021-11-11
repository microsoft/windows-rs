#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Perception_People`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EyesPose(pub ::windows::core::IInspectable);
impl EyesPose {
    #[doc = "*Required features: `Perception_People`*"]
    pub fn IsCalibrationValid(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    #[doc = "*Required features: `Perception_People`, `Foundation`, `Foundation_Numerics`, `Perception_Spatial`*"]
    pub fn Gaze(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Spatial::SpatialRay>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::Spatial::SpatialRay>>(result__)
        }
    }
    #[doc = "*Required features: `Perception_People`*"]
    pub fn UpdateTimestamp(&self) -> ::windows::core::Result<super::PerceptionTimestamp> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::PerceptionTimestamp>(result__)
        }
    }
    #[doc = "*Required features: `Perception_People`*"]
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IEyesPoseStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    #[doc = "*Required features: `Perception_People`, `Foundation`, `UI_Input`*"]
    pub fn RequestAccessAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::UI::Input::GazeInputAccessStatus>> {
        Self::IEyesPoseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::UI::Input::GazeInputAccessStatus>>(result__)
        })
    }
    pub fn IEyesPoseStatics<R, F: FnOnce(&IEyesPoseStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EyesPose, IEyesPoseStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for EyesPose {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.People.EyesPose;{682a9b23-8a1e-5b86-a060-906ffacb62a4})");
}
unsafe impl ::windows::core::Interface for EyesPose {
    type Vtable = IEyesPose_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x682a9b23_8a1e_5b86_a060_906ffacb62a4);
}
impl ::windows::core::RuntimeName for EyesPose {
    const NAME: &'static str = "Windows.Perception.People.EyesPose";
}
impl ::core::convert::From<EyesPose> for ::windows::core::IUnknown {
    fn from(value: EyesPose) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EyesPose> for ::windows::core::IUnknown {
    fn from(value: &EyesPose) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EyesPose {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EyesPose {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EyesPose> for ::windows::core::IInspectable {
    fn from(value: EyesPose) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EyesPose> for ::windows::core::IInspectable {
    fn from(value: &EyesPose) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EyesPose {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EyesPose {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
unsafe impl ::windows::core::Abi for HandJointKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for HandJointKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Perception.People.HandJointKind;i4)");
}
impl ::windows::core::DefaultType for HandJointKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `Perception_People`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HandMeshObserver(pub ::windows::core::IInspectable);
impl HandMeshObserver {
    #[cfg(feature = "UI_Input_Spatial")]
    #[doc = "*Required features: `Perception_People`, `UI_Input_Spatial`*"]
    pub fn Source(&self) -> ::windows::core::Result<super::super::UI::Input::Spatial::SpatialInteractionSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Input::Spatial::SpatialInteractionSource>(result__)
        }
    }
    #[doc = "*Required features: `Perception_People`*"]
    pub fn TriangleIndexCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Perception_People`*"]
    pub fn VertexCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Perception_People`*"]
    pub fn GetTriangleIndices(&self, indices: &mut [<u16 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), indices.len() as u32, ::core::mem::transmute_copy(&indices)).ok() }
    }
    #[doc = "*Required features: `Perception_People`*"]
    pub fn GetVertexStateForPose<'a, Param0: ::windows::core::IntoParam<'a, HandPose>>(&self, handpose: Param0) -> ::windows::core::Result<HandMeshVertexState> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handpose.into_param().abi(), &mut result__).from_abi::<HandMeshVertexState>(result__)
        }
    }
    #[doc = "*Required features: `Perception_People`*"]
    pub fn NeutralPose(&self) -> ::windows::core::Result<HandPose> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HandPose>(result__)
        }
    }
    #[doc = "*Required features: `Perception_People`*"]
    pub fn NeutralPoseVersion(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Perception_People`*"]
    pub fn ModelId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for HandMeshObserver {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.People.HandMeshObserver;{85ae30cb-6fc3-55c4-a7b4-29e33896ca69})");
}
unsafe impl ::windows::core::Interface for HandMeshObserver {
    type Vtable = IHandMeshObserver_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85ae30cb_6fc3_55c4_a7b4_29e33896ca69);
}
impl ::windows::core::RuntimeName for HandMeshObserver {
    const NAME: &'static str = "Windows.Perception.People.HandMeshObserver";
}
impl ::core::convert::From<HandMeshObserver> for ::windows::core::IUnknown {
    fn from(value: HandMeshObserver) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HandMeshObserver> for ::windows::core::IUnknown {
    fn from(value: &HandMeshObserver) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HandMeshObserver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HandMeshObserver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HandMeshObserver> for ::windows::core::IInspectable {
    fn from(value: HandMeshObserver) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HandMeshObserver> for ::windows::core::IInspectable {
    fn from(value: &HandMeshObserver) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HandMeshObserver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HandMeshObserver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
unsafe impl ::windows::core::Abi for HandMeshVertex {
    type Abi = Self;
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::RuntimeType for HandMeshVertex {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Perception.People.HandMeshVertex;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4))");
}
#[cfg(feature = "Foundation_Numerics")]
impl ::windows::core::DefaultType for HandMeshVertex {
    type DefaultType = Self;
}
#[doc = "*Required features: `Perception_People`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HandMeshVertexState(pub ::windows::core::IInspectable);
impl HandMeshVertexState {
    #[cfg(feature = "Perception_Spatial")]
    #[doc = "*Required features: `Perception_People`, `Perception_Spatial`*"]
    pub fn CoordinateSystem(&self) -> ::windows::core::Result<super::Spatial::SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Spatial::SpatialCoordinateSystem>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Perception_People`, `Foundation_Numerics`*"]
    pub fn GetVertices(&self, vertices: &mut [<HandMeshVertex as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), vertices.len() as u32, ::core::mem::transmute_copy(&vertices)).ok() }
    }
    #[doc = "*Required features: `Perception_People`*"]
    pub fn UpdateTimestamp(&self) -> ::windows::core::Result<super::PerceptionTimestamp> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::PerceptionTimestamp>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for HandMeshVertexState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.People.HandMeshVertexState;{046c5fef-1d8b-55de-ab2c-1cd424886d8f})");
}
unsafe impl ::windows::core::Interface for HandMeshVertexState {
    type Vtable = IHandMeshVertexState_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x046c5fef_1d8b_55de_ab2c_1cd424886d8f);
}
impl ::windows::core::RuntimeName for HandMeshVertexState {
    const NAME: &'static str = "Windows.Perception.People.HandMeshVertexState";
}
impl ::core::convert::From<HandMeshVertexState> for ::windows::core::IUnknown {
    fn from(value: HandMeshVertexState) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HandMeshVertexState> for ::windows::core::IUnknown {
    fn from(value: &HandMeshVertexState) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HandMeshVertexState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HandMeshVertexState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HandMeshVertexState> for ::windows::core::IInspectable {
    fn from(value: HandMeshVertexState) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HandMeshVertexState> for ::windows::core::IInspectable {
    fn from(value: &HandMeshVertexState) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HandMeshVertexState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HandMeshVertexState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HandMeshVertexState {}
unsafe impl ::core::marker::Sync for HandMeshVertexState {}
#[doc = "*Required features: `Perception_People`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HandPose(pub ::windows::core::IInspectable);
impl HandPose {
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    #[doc = "*Required features: `Perception_People`, `Foundation_Numerics`, `Perception_Spatial`*"]
    pub fn TryGetJoint<'a, Param0: ::windows::core::IntoParam<'a, super::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0, joint: HandJointKind, jointpose: &mut JointPose) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), joint, jointpose, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    #[doc = "*Required features: `Perception_People`, `Foundation_Numerics`, `Perception_Spatial`*"]
    pub fn TryGetJoints<'a, Param0: ::windows::core::IntoParam<'a, super::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0, joints: &[<HandJointKind as ::windows::core::DefaultType>::DefaultType], jointposes: &mut [<JointPose as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), joints.len() as u32, ::core::mem::transmute(joints.as_ptr()), jointposes.len() as u32, ::core::mem::transmute_copy(&jointposes), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Perception_People`, `Foundation_Numerics`*"]
    pub fn GetRelativeJoint(&self, joint: HandJointKind, referencejoint: HandJointKind) -> ::windows::core::Result<JointPose> {
        let this = self;
        unsafe {
            let mut result__: JointPose = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), joint, referencejoint, &mut result__).from_abi::<JointPose>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Perception_People`, `Foundation_Numerics`*"]
    pub fn GetRelativeJoints(&self, joints: &[<HandJointKind as ::windows::core::DefaultType>::DefaultType], referencejoints: &[<HandJointKind as ::windows::core::DefaultType>::DefaultType], jointposes: &mut [<JointPose as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), joints.len() as u32, ::core::mem::transmute(joints.as_ptr()), referencejoints.len() as u32, ::core::mem::transmute(referencejoints.as_ptr()), jointposes.len() as u32, ::core::mem::transmute_copy(&jointposes)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for HandPose {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.People.HandPose;{4d98e79a-bb08-5d09-91de-df0dd3fae46c})");
}
unsafe impl ::windows::core::Interface for HandPose {
    type Vtable = IHandPose_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d98e79a_bb08_5d09_91de_df0dd3fae46c);
}
impl ::windows::core::RuntimeName for HandPose {
    const NAME: &'static str = "Windows.Perception.People.HandPose";
}
impl ::core::convert::From<HandPose> for ::windows::core::IUnknown {
    fn from(value: HandPose) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HandPose> for ::windows::core::IUnknown {
    fn from(value: &HandPose) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HandPose {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HandPose {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HandPose> for ::windows::core::IInspectable {
    fn from(value: HandPose) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HandPose> for ::windows::core::IInspectable {
    fn from(value: &HandPose) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HandPose {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HandPose {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HandPose {}
unsafe impl ::core::marker::Sync for HandPose {}
#[doc = "*Required features: `Perception_People`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HeadPose(pub ::windows::core::IInspectable);
impl HeadPose {
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Perception_People`, `Foundation_Numerics`*"]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Perception_People`, `Foundation_Numerics`*"]
    pub fn ForwardDirection(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Perception_People`, `Foundation_Numerics`*"]
    pub fn UpDirection(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for HeadPose {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.People.HeadPose;{7f5ac5a5-49db-379f-9429-32a2faf34fa6})");
}
unsafe impl ::windows::core::Interface for HeadPose {
    type Vtable = IHeadPose_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f5ac5a5_49db_379f_9429_32a2faf34fa6);
}
impl ::windows::core::RuntimeName for HeadPose {
    const NAME: &'static str = "Windows.Perception.People.HeadPose";
}
impl ::core::convert::From<HeadPose> for ::windows::core::IUnknown {
    fn from(value: HeadPose) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HeadPose> for ::windows::core::IUnknown {
    fn from(value: &HeadPose) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HeadPose {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HeadPose {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HeadPose> for ::windows::core::IInspectable {
    fn from(value: HeadPose) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HeadPose> for ::windows::core::IInspectable {
    fn from(value: &HeadPose) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HeadPose {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HeadPose {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HeadPose {}
unsafe impl ::core::marker::Sync for HeadPose {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IEyesPose(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEyesPose {
    type Vtable = IEyesPose_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x682a9b23_8a1e_5b86_a060_906ffacb62a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEyesPose_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEyesPoseStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEyesPoseStatics {
    type Vtable = IEyesPoseStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1cff7413_b21f_54c0_80c1_e60d994ca58c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEyesPoseStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHandMeshObserver(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IHandMeshObserver {
    type Vtable = IHandMeshObserver_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85ae30cb_6fc3_55c4_a7b4_29e33896ca69);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHandMeshObserver_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Input_Spatial")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Input_Spatial"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, indices_array_size: u32, indices: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handpose: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHandMeshVertexState(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IHandMeshVertexState {
    type Vtable = IHandMeshVertexState_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x046c5fef_1d8b_55de_ab2c_1cd424886d8f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHandMeshVertexState_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Perception_Spatial")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, vertices_array_size: u32, vertices: *mut HandMeshVertex) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHandPose(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IHandPose {
    type Vtable = IHandPose_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d98e79a_bb08_5d09_91de_df0dd3fae46c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHandPose_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, coordinatesystem: ::windows::core::RawPtr, joint: HandJointKind, jointpose: *mut JointPose, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))] usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, coordinatesystem: ::windows::core::RawPtr, joints_array_size: u32, joints: *const HandJointKind, jointPoses_array_size: u32, jointposes: *mut JointPose, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, joint: HandJointKind, referencejoint: HandJointKind, result__: *mut JointPose) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, joints_array_size: u32, joints: *const HandJointKind, referenceJoints_array_size: u32, referencejoints: *const HandJointKind, jointPoses_array_size: u32, jointposes: *mut JointPose) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHeadPose(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IHeadPose {
    type Vtable = IHeadPose_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f5ac5a5_49db_379f_9429_32a2faf34fa6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHeadPose_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
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
unsafe impl ::windows::core::Abi for JointPose {
    type Abi = Self;
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::RuntimeType for JointPose {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Perception.People.JointPose;struct(Windows.Foundation.Numerics.Quaternion;f4;f4;f4;f4);struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4;enum(Windows.Perception.People.JointPoseAccuracy;i4))");
}
#[cfg(feature = "Foundation_Numerics")]
impl ::windows::core::DefaultType for JointPose {
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
unsafe impl ::windows::core::Abi for JointPoseAccuracy {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for JointPoseAccuracy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Perception.People.JointPoseAccuracy;i4)");
}
impl ::windows::core::DefaultType for JointPoseAccuracy {
    type DefaultType = Self;
}
