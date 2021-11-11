#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Services_Maps_Guidance`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GuidanceAudioMeasurementSystem(pub i32);
impl GuidanceAudioMeasurementSystem {
    pub const Meters: GuidanceAudioMeasurementSystem = GuidanceAudioMeasurementSystem(0i32);
    pub const MilesAndYards: GuidanceAudioMeasurementSystem = GuidanceAudioMeasurementSystem(1i32);
    pub const MilesAndFeet: GuidanceAudioMeasurementSystem = GuidanceAudioMeasurementSystem(2i32);
}
impl ::core::convert::From<i32> for GuidanceAudioMeasurementSystem {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GuidanceAudioMeasurementSystem {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GuidanceAudioMeasurementSystem {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.Guidance.GuidanceAudioMeasurementSystem;i4)");
}
impl ::windows::runtime::DefaultType for GuidanceAudioMeasurementSystem {
    type DefaultType = Self;
}
#[doc = "*Required features: `Services_Maps_Guidance`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GuidanceAudioNotificationKind(pub i32);
impl GuidanceAudioNotificationKind {
    pub const Maneuver: GuidanceAudioNotificationKind = GuidanceAudioNotificationKind(0i32);
    pub const Route: GuidanceAudioNotificationKind = GuidanceAudioNotificationKind(1i32);
    pub const Gps: GuidanceAudioNotificationKind = GuidanceAudioNotificationKind(2i32);
    pub const SpeedLimit: GuidanceAudioNotificationKind = GuidanceAudioNotificationKind(3i32);
    pub const Traffic: GuidanceAudioNotificationKind = GuidanceAudioNotificationKind(4i32);
    pub const TrafficCamera: GuidanceAudioNotificationKind = GuidanceAudioNotificationKind(5i32);
}
impl ::core::convert::From<i32> for GuidanceAudioNotificationKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GuidanceAudioNotificationKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GuidanceAudioNotificationKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.Guidance.GuidanceAudioNotificationKind;i4)");
}
impl ::windows::runtime::DefaultType for GuidanceAudioNotificationKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `Services_Maps_Guidance`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GuidanceAudioNotificationRequestedEventArgs(pub ::windows::runtime::IInspectable);
impl GuidanceAudioNotificationRequestedEventArgs {
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn AudioNotification(&self) -> ::windows::runtime::Result<GuidanceAudioNotificationKind> {
        let this = self;
        unsafe {
            let mut result__: GuidanceAudioNotificationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GuidanceAudioNotificationKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Services_Maps_Guidance`, `Foundation_Collections`*"]
    pub fn AudioFilePaths(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn AudioText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GuidanceAudioNotificationRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceAudioNotificationRequestedEventArgs;{ca2aa24a-c7c2-4d4c-9d7c-499576bceddb})");
}
unsafe impl ::windows::runtime::Interface for GuidanceAudioNotificationRequestedEventArgs {
    type Vtable = IGuidanceAudioNotificationRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xca2aa24a_c7c2_4d4c_9d7c_499576bceddb);
}
impl ::windows::runtime::RuntimeName for GuidanceAudioNotificationRequestedEventArgs {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceAudioNotificationRequestedEventArgs";
}
impl ::core::convert::From<GuidanceAudioNotificationRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: GuidanceAudioNotificationRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GuidanceAudioNotificationRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &GuidanceAudioNotificationRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GuidanceAudioNotificationRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a GuidanceAudioNotificationRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GuidanceAudioNotificationRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: GuidanceAudioNotificationRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GuidanceAudioNotificationRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &GuidanceAudioNotificationRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GuidanceAudioNotificationRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GuidanceAudioNotificationRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GuidanceAudioNotificationRequestedEventArgs {}
unsafe impl ::core::marker::Sync for GuidanceAudioNotificationRequestedEventArgs {}
#[doc = "*Required features: `Services_Maps_Guidance`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GuidanceAudioNotifications(pub u32);
impl GuidanceAudioNotifications {
    pub const None: GuidanceAudioNotifications = GuidanceAudioNotifications(0u32);
    pub const Maneuver: GuidanceAudioNotifications = GuidanceAudioNotifications(1u32);
    pub const Route: GuidanceAudioNotifications = GuidanceAudioNotifications(2u32);
    pub const Gps: GuidanceAudioNotifications = GuidanceAudioNotifications(4u32);
    pub const SpeedLimit: GuidanceAudioNotifications = GuidanceAudioNotifications(8u32);
    pub const Traffic: GuidanceAudioNotifications = GuidanceAudioNotifications(16u32);
    pub const TrafficCamera: GuidanceAudioNotifications = GuidanceAudioNotifications(32u32);
}
impl ::core::convert::From<u32> for GuidanceAudioNotifications {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GuidanceAudioNotifications {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GuidanceAudioNotifications {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.Guidance.GuidanceAudioNotifications;u4)");
}
impl ::windows::runtime::DefaultType for GuidanceAudioNotifications {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for GuidanceAudioNotifications {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for GuidanceAudioNotifications {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for GuidanceAudioNotifications {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for GuidanceAudioNotifications {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for GuidanceAudioNotifications {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Services_Maps_Guidance`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GuidanceLaneInfo(pub ::windows::runtime::IInspectable);
impl GuidanceLaneInfo {
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn LaneMarkers(&self) -> ::windows::runtime::Result<GuidanceLaneMarkers> {
        let this = self;
        unsafe {
            let mut result__: GuidanceLaneMarkers = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GuidanceLaneMarkers>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn IsOnRoute(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GuidanceLaneInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceLaneInfo;{8404d114-6581-43b7-ac15-c9079bf90df1})");
}
unsafe impl ::windows::runtime::Interface for GuidanceLaneInfo {
    type Vtable = IGuidanceLaneInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8404d114_6581_43b7_ac15_c9079bf90df1);
}
impl ::windows::runtime::RuntimeName for GuidanceLaneInfo {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceLaneInfo";
}
impl ::core::convert::From<GuidanceLaneInfo> for ::windows::runtime::IUnknown {
    fn from(value: GuidanceLaneInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GuidanceLaneInfo> for ::windows::runtime::IUnknown {
    fn from(value: &GuidanceLaneInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GuidanceLaneInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a GuidanceLaneInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GuidanceLaneInfo> for ::windows::runtime::IInspectable {
    fn from(value: GuidanceLaneInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GuidanceLaneInfo> for ::windows::runtime::IInspectable {
    fn from(value: &GuidanceLaneInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GuidanceLaneInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GuidanceLaneInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GuidanceLaneInfo {}
unsafe impl ::core::marker::Sync for GuidanceLaneInfo {}
#[doc = "*Required features: `Services_Maps_Guidance`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GuidanceLaneMarkers(pub u32);
impl GuidanceLaneMarkers {
    pub const None: GuidanceLaneMarkers = GuidanceLaneMarkers(0u32);
    pub const LightRight: GuidanceLaneMarkers = GuidanceLaneMarkers(1u32);
    pub const Right: GuidanceLaneMarkers = GuidanceLaneMarkers(2u32);
    pub const HardRight: GuidanceLaneMarkers = GuidanceLaneMarkers(4u32);
    pub const Straight: GuidanceLaneMarkers = GuidanceLaneMarkers(8u32);
    pub const UTurnLeft: GuidanceLaneMarkers = GuidanceLaneMarkers(16u32);
    pub const HardLeft: GuidanceLaneMarkers = GuidanceLaneMarkers(32u32);
    pub const Left: GuidanceLaneMarkers = GuidanceLaneMarkers(64u32);
    pub const LightLeft: GuidanceLaneMarkers = GuidanceLaneMarkers(128u32);
    pub const UTurnRight: GuidanceLaneMarkers = GuidanceLaneMarkers(256u32);
    pub const Unknown: GuidanceLaneMarkers = GuidanceLaneMarkers(4294967295u32);
}
impl ::core::convert::From<u32> for GuidanceLaneMarkers {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GuidanceLaneMarkers {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GuidanceLaneMarkers {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.Guidance.GuidanceLaneMarkers;u4)");
}
impl ::windows::runtime::DefaultType for GuidanceLaneMarkers {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for GuidanceLaneMarkers {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for GuidanceLaneMarkers {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for GuidanceLaneMarkers {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for GuidanceLaneMarkers {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for GuidanceLaneMarkers {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Services_Maps_Guidance`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GuidanceManeuver(pub ::windows::runtime::IInspectable);
impl GuidanceManeuver {
    #[cfg(feature = "Devices_Geolocation")]
    #[doc = "*Required features: `Services_Maps_Guidance`, `Devices_Geolocation`*"]
    pub fn StartLocation(&self) -> ::windows::runtime::Result<super::super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Geolocation::Geopoint>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn DistanceFromRouteStart(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn DistanceFromPreviousManeuver(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn DepartureRoadName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn NextRoadName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn DepartureShortRoadName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn NextShortRoadName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<GuidanceManeuverKind> {
        let this = self;
        unsafe {
            let mut result__: GuidanceManeuverKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GuidanceManeuverKind>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn StartAngle(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn EndAngle(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn RoadSignpost(&self) -> ::windows::runtime::Result<GuidanceRoadSignpost> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GuidanceRoadSignpost>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn InstructionText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GuidanceManeuver {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceManeuver;{fc09326c-ecc9-4928-a2a1-7232b99b94a1})");
}
unsafe impl ::windows::runtime::Interface for GuidanceManeuver {
    type Vtable = IGuidanceManeuver_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfc09326c_ecc9_4928_a2a1_7232b99b94a1);
}
impl ::windows::runtime::RuntimeName for GuidanceManeuver {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceManeuver";
}
impl ::core::convert::From<GuidanceManeuver> for ::windows::runtime::IUnknown {
    fn from(value: GuidanceManeuver) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GuidanceManeuver> for ::windows::runtime::IUnknown {
    fn from(value: &GuidanceManeuver) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GuidanceManeuver {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a GuidanceManeuver {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GuidanceManeuver> for ::windows::runtime::IInspectable {
    fn from(value: GuidanceManeuver) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GuidanceManeuver> for ::windows::runtime::IInspectable {
    fn from(value: &GuidanceManeuver) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GuidanceManeuver {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GuidanceManeuver {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GuidanceManeuver {}
unsafe impl ::core::marker::Sync for GuidanceManeuver {}
#[doc = "*Required features: `Services_Maps_Guidance`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GuidanceManeuverKind(pub i32);
impl GuidanceManeuverKind {
    pub const None: GuidanceManeuverKind = GuidanceManeuverKind(0i32);
    pub const GoStraight: GuidanceManeuverKind = GuidanceManeuverKind(1i32);
    pub const UTurnRight: GuidanceManeuverKind = GuidanceManeuverKind(2i32);
    pub const UTurnLeft: GuidanceManeuverKind = GuidanceManeuverKind(3i32);
    pub const TurnKeepRight: GuidanceManeuverKind = GuidanceManeuverKind(4i32);
    pub const TurnLightRight: GuidanceManeuverKind = GuidanceManeuverKind(5i32);
    pub const TurnRight: GuidanceManeuverKind = GuidanceManeuverKind(6i32);
    pub const TurnHardRight: GuidanceManeuverKind = GuidanceManeuverKind(7i32);
    pub const KeepMiddle: GuidanceManeuverKind = GuidanceManeuverKind(8i32);
    pub const TurnKeepLeft: GuidanceManeuverKind = GuidanceManeuverKind(9i32);
    pub const TurnLightLeft: GuidanceManeuverKind = GuidanceManeuverKind(10i32);
    pub const TurnLeft: GuidanceManeuverKind = GuidanceManeuverKind(11i32);
    pub const TurnHardLeft: GuidanceManeuverKind = GuidanceManeuverKind(12i32);
    pub const FreewayEnterRight: GuidanceManeuverKind = GuidanceManeuverKind(13i32);
    pub const FreewayEnterLeft: GuidanceManeuverKind = GuidanceManeuverKind(14i32);
    pub const FreewayLeaveRight: GuidanceManeuverKind = GuidanceManeuverKind(15i32);
    pub const FreewayLeaveLeft: GuidanceManeuverKind = GuidanceManeuverKind(16i32);
    pub const FreewayKeepRight: GuidanceManeuverKind = GuidanceManeuverKind(17i32);
    pub const FreewayKeepLeft: GuidanceManeuverKind = GuidanceManeuverKind(18i32);
    pub const TrafficCircleRight1: GuidanceManeuverKind = GuidanceManeuverKind(19i32);
    pub const TrafficCircleRight2: GuidanceManeuverKind = GuidanceManeuverKind(20i32);
    pub const TrafficCircleRight3: GuidanceManeuverKind = GuidanceManeuverKind(21i32);
    pub const TrafficCircleRight4: GuidanceManeuverKind = GuidanceManeuverKind(22i32);
    pub const TrafficCircleRight5: GuidanceManeuverKind = GuidanceManeuverKind(23i32);
    pub const TrafficCircleRight6: GuidanceManeuverKind = GuidanceManeuverKind(24i32);
    pub const TrafficCircleRight7: GuidanceManeuverKind = GuidanceManeuverKind(25i32);
    pub const TrafficCircleRight8: GuidanceManeuverKind = GuidanceManeuverKind(26i32);
    pub const TrafficCircleRight9: GuidanceManeuverKind = GuidanceManeuverKind(27i32);
    pub const TrafficCircleRight10: GuidanceManeuverKind = GuidanceManeuverKind(28i32);
    pub const TrafficCircleRight11: GuidanceManeuverKind = GuidanceManeuverKind(29i32);
    pub const TrafficCircleRight12: GuidanceManeuverKind = GuidanceManeuverKind(30i32);
    pub const TrafficCircleLeft1: GuidanceManeuverKind = GuidanceManeuverKind(31i32);
    pub const TrafficCircleLeft2: GuidanceManeuverKind = GuidanceManeuverKind(32i32);
    pub const TrafficCircleLeft3: GuidanceManeuverKind = GuidanceManeuverKind(33i32);
    pub const TrafficCircleLeft4: GuidanceManeuverKind = GuidanceManeuverKind(34i32);
    pub const TrafficCircleLeft5: GuidanceManeuverKind = GuidanceManeuverKind(35i32);
    pub const TrafficCircleLeft6: GuidanceManeuverKind = GuidanceManeuverKind(36i32);
    pub const TrafficCircleLeft7: GuidanceManeuverKind = GuidanceManeuverKind(37i32);
    pub const TrafficCircleLeft8: GuidanceManeuverKind = GuidanceManeuverKind(38i32);
    pub const TrafficCircleLeft9: GuidanceManeuverKind = GuidanceManeuverKind(39i32);
    pub const TrafficCircleLeft10: GuidanceManeuverKind = GuidanceManeuverKind(40i32);
    pub const TrafficCircleLeft11: GuidanceManeuverKind = GuidanceManeuverKind(41i32);
    pub const TrafficCircleLeft12: GuidanceManeuverKind = GuidanceManeuverKind(42i32);
    pub const Start: GuidanceManeuverKind = GuidanceManeuverKind(43i32);
    pub const End: GuidanceManeuverKind = GuidanceManeuverKind(44i32);
    pub const TakeFerry: GuidanceManeuverKind = GuidanceManeuverKind(45i32);
    pub const PassTransitStation: GuidanceManeuverKind = GuidanceManeuverKind(46i32);
    pub const LeaveTransitStation: GuidanceManeuverKind = GuidanceManeuverKind(47i32);
}
impl ::core::convert::From<i32> for GuidanceManeuverKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GuidanceManeuverKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GuidanceManeuverKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.Guidance.GuidanceManeuverKind;i4)");
}
impl ::windows::runtime::DefaultType for GuidanceManeuverKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `Services_Maps_Guidance`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GuidanceMapMatchedCoordinate(pub ::windows::runtime::IInspectable);
impl GuidanceMapMatchedCoordinate {
    #[cfg(feature = "Devices_Geolocation")]
    #[doc = "*Required features: `Services_Maps_Guidance`, `Devices_Geolocation`*"]
    pub fn Location(&self) -> ::windows::runtime::Result<super::super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Geolocation::Geopoint>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn CurrentHeading(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn CurrentSpeed(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn IsOnStreet(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn Road(&self) -> ::windows::runtime::Result<GuidanceRoadSegment> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GuidanceRoadSegment>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GuidanceMapMatchedCoordinate {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceMapMatchedCoordinate;{b7acb168-2912-4a99-aff1-798609b981fe})");
}
unsafe impl ::windows::runtime::Interface for GuidanceMapMatchedCoordinate {
    type Vtable = IGuidanceMapMatchedCoordinate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb7acb168_2912_4a99_aff1_798609b981fe);
}
impl ::windows::runtime::RuntimeName for GuidanceMapMatchedCoordinate {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceMapMatchedCoordinate";
}
impl ::core::convert::From<GuidanceMapMatchedCoordinate> for ::windows::runtime::IUnknown {
    fn from(value: GuidanceMapMatchedCoordinate) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GuidanceMapMatchedCoordinate> for ::windows::runtime::IUnknown {
    fn from(value: &GuidanceMapMatchedCoordinate) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GuidanceMapMatchedCoordinate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a GuidanceMapMatchedCoordinate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GuidanceMapMatchedCoordinate> for ::windows::runtime::IInspectable {
    fn from(value: GuidanceMapMatchedCoordinate) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GuidanceMapMatchedCoordinate> for ::windows::runtime::IInspectable {
    fn from(value: &GuidanceMapMatchedCoordinate) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GuidanceMapMatchedCoordinate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GuidanceMapMatchedCoordinate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GuidanceMapMatchedCoordinate {}
unsafe impl ::core::marker::Sync for GuidanceMapMatchedCoordinate {}
#[doc = "*Required features: `Services_Maps_Guidance`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GuidanceMode(pub i32);
impl GuidanceMode {
    pub const None: GuidanceMode = GuidanceMode(0i32);
    pub const Simulation: GuidanceMode = GuidanceMode(1i32);
    pub const Navigation: GuidanceMode = GuidanceMode(2i32);
    pub const Tracking: GuidanceMode = GuidanceMode(3i32);
}
impl ::core::convert::From<i32> for GuidanceMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GuidanceMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GuidanceMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.Guidance.GuidanceMode;i4)");
}
impl ::windows::runtime::DefaultType for GuidanceMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Services_Maps_Guidance`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GuidanceNavigator(pub ::windows::runtime::IInspectable);
impl GuidanceNavigator {
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn StartNavigating<'a, Param0: ::windows::runtime::IntoParam<'a, GuidanceRoute>>(&self, route: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), route.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn StartSimulating<'a, Param0: ::windows::runtime::IntoParam<'a, GuidanceRoute>>(&self, route: Param0, speedinmeterspersecond: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), route.into_param().abi(), speedinmeterspersecond).ok() }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn StartTracking(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn Pause(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn Resume(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn RepeatLastAudioNotification(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn AudioMeasurementSystem(&self) -> ::windows::runtime::Result<GuidanceAudioMeasurementSystem> {
        let this = self;
        unsafe {
            let mut result__: GuidanceAudioMeasurementSystem = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GuidanceAudioMeasurementSystem>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn SetAudioMeasurementSystem(&self, value: GuidanceAudioMeasurementSystem) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn AudioNotifications(&self) -> ::windows::runtime::Result<GuidanceAudioNotifications> {
        let this = self;
        unsafe {
            let mut result__: GuidanceAudioNotifications = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GuidanceAudioNotifications>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn SetAudioNotifications(&self, value: GuidanceAudioNotifications) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Maps_Guidance`, `Foundation`*"]
    pub fn GuidanceUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, GuidanceUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Maps_Guidance`, `Foundation`*"]
    pub fn RemoveGuidanceUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Maps_Guidance`, `Foundation`*"]
    pub fn DestinationReached<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Maps_Guidance`, `Foundation`*"]
    pub fn RemoveDestinationReached<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Maps_Guidance`, `Foundation`*"]
    pub fn Rerouting<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Maps_Guidance`, `Foundation`*"]
    pub fn RemoveRerouting<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Maps_Guidance`, `Foundation`*"]
    pub fn Rerouted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, GuidanceReroutedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Maps_Guidance`, `Foundation`*"]
    pub fn RemoveRerouted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).24)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Maps_Guidance`, `Foundation`*"]
    pub fn RerouteFailed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Maps_Guidance`, `Foundation`*"]
    pub fn RemoveRerouteFailed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).26)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Maps_Guidance`, `Foundation`*"]
    pub fn UserLocationLost<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Maps_Guidance`, `Foundation`*"]
    pub fn RemoveUserLocationLost<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Maps_Guidance`, `Foundation`*"]
    pub fn UserLocationRestored<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Maps_Guidance`, `Foundation`*"]
    pub fn RemoveUserLocationRestored<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).30)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn SetGuidanceVoice<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, voiceid: i32, voicefolder: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).31)(::core::mem::transmute_copy(this), voiceid, voicefolder.into_param().abi()).ok() }
    }
    #[cfg(feature = "Devices_Geolocation")]
    #[doc = "*Required features: `Services_Maps_Guidance`, `Devices_Geolocation`*"]
    pub fn UpdateUserLocation<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Devices::Geolocation::Geocoordinate>>(&self, userlocation: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).32)(::core::mem::transmute_copy(this), userlocation.into_param().abi()).ok() }
    }
    #[cfg(feature = "Devices_Geolocation")]
    #[doc = "*Required features: `Services_Maps_Guidance`, `Devices_Geolocation`*"]
    pub fn UpdateUserLocationWithPositionOverride<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Devices::Geolocation::Geocoordinate>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Devices::Geolocation::BasicGeoposition>>(&self, userlocation: Param0, positionoverride: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).33)(::core::mem::transmute_copy(this), userlocation.into_param().abi(), positionoverride.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn GetCurrent() -> ::windows::runtime::Result<GuidanceNavigator> {
        Self::IGuidanceNavigatorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GuidanceNavigator>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Maps_Guidance`, `Foundation`*"]
    pub fn AudioNotificationRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, GuidanceAudioNotificationRequestedEventArgs>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IGuidanceNavigator2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Maps_Guidance`, `Foundation`*"]
    pub fn RemoveAudioNotificationRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGuidanceNavigator2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn IsGuidanceAudioMuted(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IGuidanceNavigator2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn SetIsGuidanceAudioMuted(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGuidanceNavigator2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn UseAppProvidedVoice() -> ::windows::runtime::Result<bool> {
        Self::IGuidanceNavigatorStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IGuidanceNavigatorStatics<R, F: FnOnce(&IGuidanceNavigatorStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<GuidanceNavigator, IGuidanceNavigatorStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IGuidanceNavigatorStatics2<R, F: FnOnce(&IGuidanceNavigatorStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<GuidanceNavigator, IGuidanceNavigatorStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GuidanceNavigator {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceNavigator;{08f17ef7-8e3f-4d9a-be8a-108f9a012c67})");
}
unsafe impl ::windows::runtime::Interface for GuidanceNavigator {
    type Vtable = IGuidanceNavigator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x08f17ef7_8e3f_4d9a_be8a_108f9a012c67);
}
impl ::windows::runtime::RuntimeName for GuidanceNavigator {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceNavigator";
}
impl ::core::convert::From<GuidanceNavigator> for ::windows::runtime::IUnknown {
    fn from(value: GuidanceNavigator) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GuidanceNavigator> for ::windows::runtime::IUnknown {
    fn from(value: &GuidanceNavigator) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GuidanceNavigator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a GuidanceNavigator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GuidanceNavigator> for ::windows::runtime::IInspectable {
    fn from(value: GuidanceNavigator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GuidanceNavigator> for ::windows::runtime::IInspectable {
    fn from(value: &GuidanceNavigator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GuidanceNavigator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GuidanceNavigator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GuidanceNavigator {}
unsafe impl ::core::marker::Sync for GuidanceNavigator {}
#[doc = "*Required features: `Services_Maps_Guidance`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GuidanceReroutedEventArgs(pub ::windows::runtime::IInspectable);
impl GuidanceReroutedEventArgs {
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn Route(&self) -> ::windows::runtime::Result<GuidanceRoute> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GuidanceRoute>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GuidanceReroutedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceReroutedEventArgs;{115d4008-d528-454e-bb94-a50341d2c9f1})");
}
unsafe impl ::windows::runtime::Interface for GuidanceReroutedEventArgs {
    type Vtable = IGuidanceReroutedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x115d4008_d528_454e_bb94_a50341d2c9f1);
}
impl ::windows::runtime::RuntimeName for GuidanceReroutedEventArgs {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceReroutedEventArgs";
}
impl ::core::convert::From<GuidanceReroutedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: GuidanceReroutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GuidanceReroutedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &GuidanceReroutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GuidanceReroutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a GuidanceReroutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GuidanceReroutedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: GuidanceReroutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GuidanceReroutedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &GuidanceReroutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GuidanceReroutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GuidanceReroutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GuidanceReroutedEventArgs {}
unsafe impl ::core::marker::Sync for GuidanceReroutedEventArgs {}
#[doc = "*Required features: `Services_Maps_Guidance`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GuidanceRoadSegment(pub ::windows::runtime::IInspectable);
impl GuidanceRoadSegment {
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn RoadName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn ShortRoadName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn SpeedLimit(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Maps_Guidance`, `Foundation`*"]
    pub fn TravelTime(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Devices_Geolocation")]
    #[doc = "*Required features: `Services_Maps_Guidance`, `Devices_Geolocation`*"]
    pub fn Path(&self) -> ::windows::runtime::Result<super::super::super::Devices::Geolocation::Geopath> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Geolocation::Geopath>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn IsHighway(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn IsTunnel(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn IsTollRoad(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn IsScenic(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IGuidanceRoadSegment2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GuidanceRoadSegment {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceRoadSegment;{b32758a6-be78-4c63-afe7-6c2957479b3e})");
}
unsafe impl ::windows::runtime::Interface for GuidanceRoadSegment {
    type Vtable = IGuidanceRoadSegment_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb32758a6_be78_4c63_afe7_6c2957479b3e);
}
impl ::windows::runtime::RuntimeName for GuidanceRoadSegment {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceRoadSegment";
}
impl ::core::convert::From<GuidanceRoadSegment> for ::windows::runtime::IUnknown {
    fn from(value: GuidanceRoadSegment) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GuidanceRoadSegment> for ::windows::runtime::IUnknown {
    fn from(value: &GuidanceRoadSegment) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GuidanceRoadSegment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a GuidanceRoadSegment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GuidanceRoadSegment> for ::windows::runtime::IInspectable {
    fn from(value: GuidanceRoadSegment) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GuidanceRoadSegment> for ::windows::runtime::IInspectable {
    fn from(value: &GuidanceRoadSegment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GuidanceRoadSegment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GuidanceRoadSegment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GuidanceRoadSegment {}
unsafe impl ::core::marker::Sync for GuidanceRoadSegment {}
#[doc = "*Required features: `Services_Maps_Guidance`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GuidanceRoadSignpost(pub ::windows::runtime::IInspectable);
impl GuidanceRoadSignpost {
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn ExitNumber(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn Exit(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Services_Maps_Guidance`, `UI`*"]
    pub fn BackgroundColor(&self) -> ::windows::runtime::Result<super::super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::UI::Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::UI::Color>(result__)
        }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Services_Maps_Guidance`, `UI`*"]
    pub fn ForegroundColor(&self) -> ::windows::runtime::Result<super::super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::UI::Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::UI::Color>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Services_Maps_Guidance`, `Foundation_Collections`*"]
    pub fn ExitDirections(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GuidanceRoadSignpost {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceRoadSignpost;{f1a728b6-f77a-4742-8312-53300f9845f0})");
}
unsafe impl ::windows::runtime::Interface for GuidanceRoadSignpost {
    type Vtable = IGuidanceRoadSignpost_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf1a728b6_f77a_4742_8312_53300f9845f0);
}
impl ::windows::runtime::RuntimeName for GuidanceRoadSignpost {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceRoadSignpost";
}
impl ::core::convert::From<GuidanceRoadSignpost> for ::windows::runtime::IUnknown {
    fn from(value: GuidanceRoadSignpost) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GuidanceRoadSignpost> for ::windows::runtime::IUnknown {
    fn from(value: &GuidanceRoadSignpost) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GuidanceRoadSignpost {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a GuidanceRoadSignpost {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GuidanceRoadSignpost> for ::windows::runtime::IInspectable {
    fn from(value: GuidanceRoadSignpost) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GuidanceRoadSignpost> for ::windows::runtime::IInspectable {
    fn from(value: &GuidanceRoadSignpost) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GuidanceRoadSignpost {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GuidanceRoadSignpost {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GuidanceRoadSignpost {}
unsafe impl ::core::marker::Sync for GuidanceRoadSignpost {}
#[doc = "*Required features: `Services_Maps_Guidance`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GuidanceRoute(pub ::windows::runtime::IInspectable);
impl GuidanceRoute {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Maps_Guidance`, `Foundation`*"]
    pub fn Duration(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn Distance(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Services_Maps_Guidance`, `Foundation_Collections`*"]
    pub fn Maneuvers(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<GuidanceManeuver>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<GuidanceManeuver>>(result__)
        }
    }
    #[cfg(feature = "Devices_Geolocation")]
    #[doc = "*Required features: `Services_Maps_Guidance`, `Devices_Geolocation`*"]
    pub fn BoundingBox(&self) -> ::windows::runtime::Result<super::super::super::Devices::Geolocation::GeoboundingBox> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Geolocation::GeoboundingBox>(result__)
        }
    }
    #[cfg(feature = "Devices_Geolocation")]
    #[doc = "*Required features: `Services_Maps_Guidance`, `Devices_Geolocation`*"]
    pub fn Path(&self) -> ::windows::runtime::Result<super::super::super::Devices::Geolocation::Geopath> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Geolocation::Geopath>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Services_Maps_Guidance`, `Foundation_Collections`*"]
    pub fn RoadSegments(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<GuidanceRoadSegment>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<GuidanceRoadSegment>>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn ConvertToMapRoute(&self) -> ::windows::runtime::Result<super::MapRoute> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MapRoute>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn CanCreateFromMapRoute<'a, Param0: ::windows::runtime::IntoParam<'a, super::MapRoute>>(maproute: Param0) -> ::windows::runtime::Result<bool> {
        Self::IGuidanceRouteStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), maproute.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn TryCreateFromMapRoute<'a, Param0: ::windows::runtime::IntoParam<'a, super::MapRoute>>(maproute: Param0) -> ::windows::runtime::Result<GuidanceRoute> {
        Self::IGuidanceRouteStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), maproute.into_param().abi(), &mut result__).from_abi::<GuidanceRoute>(result__)
        })
    }
    pub fn IGuidanceRouteStatics<R, F: FnOnce(&IGuidanceRouteStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<GuidanceRoute, IGuidanceRouteStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GuidanceRoute {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceRoute;{3a14545d-801a-40bd-a286-afb2010cce6c})");
}
unsafe impl ::windows::runtime::Interface for GuidanceRoute {
    type Vtable = IGuidanceRoute_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3a14545d_801a_40bd_a286_afb2010cce6c);
}
impl ::windows::runtime::RuntimeName for GuidanceRoute {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceRoute";
}
impl ::core::convert::From<GuidanceRoute> for ::windows::runtime::IUnknown {
    fn from(value: GuidanceRoute) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GuidanceRoute> for ::windows::runtime::IUnknown {
    fn from(value: &GuidanceRoute) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GuidanceRoute {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a GuidanceRoute {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GuidanceRoute> for ::windows::runtime::IInspectable {
    fn from(value: GuidanceRoute) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GuidanceRoute> for ::windows::runtime::IInspectable {
    fn from(value: &GuidanceRoute) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GuidanceRoute {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GuidanceRoute {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GuidanceRoute {}
unsafe impl ::core::marker::Sync for GuidanceRoute {}
#[doc = "*Required features: `Services_Maps_Guidance`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GuidanceTelemetryCollector(pub ::windows::runtime::IInspectable);
impl GuidanceTelemetryCollector {
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn Enabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn SetEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn ClearLocalData(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn SpeedTrigger(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn SetSpeedTrigger(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn UploadFrequency(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn SetUploadFrequency(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn GetCurrent() -> ::windows::runtime::Result<GuidanceTelemetryCollector> {
        Self::IGuidanceTelemetryCollectorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GuidanceTelemetryCollector>(result__)
        })
    }
    pub fn IGuidanceTelemetryCollectorStatics<R, F: FnOnce(&IGuidanceTelemetryCollectorStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<GuidanceTelemetryCollector, IGuidanceTelemetryCollectorStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GuidanceTelemetryCollector {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceTelemetryCollector;{db1f8da5-b878-4d92-98dd-347d23d38262})");
}
unsafe impl ::windows::runtime::Interface for GuidanceTelemetryCollector {
    type Vtable = IGuidanceTelemetryCollector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdb1f8da5_b878_4d92_98dd_347d23d38262);
}
impl ::windows::runtime::RuntimeName for GuidanceTelemetryCollector {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceTelemetryCollector";
}
impl ::core::convert::From<GuidanceTelemetryCollector> for ::windows::runtime::IUnknown {
    fn from(value: GuidanceTelemetryCollector) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GuidanceTelemetryCollector> for ::windows::runtime::IUnknown {
    fn from(value: &GuidanceTelemetryCollector) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GuidanceTelemetryCollector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a GuidanceTelemetryCollector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GuidanceTelemetryCollector> for ::windows::runtime::IInspectable {
    fn from(value: GuidanceTelemetryCollector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GuidanceTelemetryCollector> for ::windows::runtime::IInspectable {
    fn from(value: &GuidanceTelemetryCollector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GuidanceTelemetryCollector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GuidanceTelemetryCollector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GuidanceTelemetryCollector {}
unsafe impl ::core::marker::Sync for GuidanceTelemetryCollector {}
#[doc = "*Required features: `Services_Maps_Guidance`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GuidanceUpdatedEventArgs(pub ::windows::runtime::IInspectable);
impl GuidanceUpdatedEventArgs {
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn Mode(&self) -> ::windows::runtime::Result<GuidanceMode> {
        let this = self;
        unsafe {
            let mut result__: GuidanceMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GuidanceMode>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn NextManeuver(&self) -> ::windows::runtime::Result<GuidanceManeuver> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GuidanceManeuver>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn NextManeuverDistance(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn AfterNextManeuver(&self) -> ::windows::runtime::Result<GuidanceManeuver> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GuidanceManeuver>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn AfterNextManeuverDistance(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn DistanceToDestination(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn ElapsedDistance(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Maps_Guidance`, `Foundation`*"]
    pub fn ElapsedTime(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Maps_Guidance`, `Foundation`*"]
    pub fn TimeToDestination(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn RoadName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn Route(&self) -> ::windows::runtime::Result<GuidanceRoute> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GuidanceRoute>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn CurrentLocation(&self) -> ::windows::runtime::Result<GuidanceMapMatchedCoordinate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GuidanceMapMatchedCoordinate>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_Guidance`*"]
    pub fn IsNewManeuver(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Services_Maps_Guidance`, `Foundation_Collections`*"]
    pub fn LaneInfo(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<GuidanceLaneInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<GuidanceLaneInfo>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GuidanceUpdatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceUpdatedEventArgs;{fdac160b-9e8d-4de3-a9fa-b06321d18db9})");
}
unsafe impl ::windows::runtime::Interface for GuidanceUpdatedEventArgs {
    type Vtable = IGuidanceUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfdac160b_9e8d_4de3_a9fa_b06321d18db9);
}
impl ::windows::runtime::RuntimeName for GuidanceUpdatedEventArgs {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceUpdatedEventArgs";
}
impl ::core::convert::From<GuidanceUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: GuidanceUpdatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GuidanceUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &GuidanceUpdatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GuidanceUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a GuidanceUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GuidanceUpdatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: GuidanceUpdatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GuidanceUpdatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &GuidanceUpdatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GuidanceUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GuidanceUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GuidanceUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for GuidanceUpdatedEventArgs {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IGuidanceAudioNotificationRequestedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGuidanceAudioNotificationRequestedEventArgs {
    type Vtable = IGuidanceAudioNotificationRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xca2aa24a_c7c2_4d4c_9d7c_499576bceddb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceAudioNotificationRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut GuidanceAudioNotificationKind) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGuidanceLaneInfo(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGuidanceLaneInfo {
    type Vtable = IGuidanceLaneInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8404d114_6581_43b7_ac15_c9079bf90df1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceLaneInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut GuidanceLaneMarkers) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGuidanceManeuver(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGuidanceManeuver {
    type Vtable = IGuidanceManeuver_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfc09326c_ecc9_4928_a2a1_7232b99b94a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceManeuver_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Geolocation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut GuidanceManeuverKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGuidanceMapMatchedCoordinate(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGuidanceMapMatchedCoordinate {
    type Vtable = IGuidanceMapMatchedCoordinate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb7acb168_2912_4a99_aff1_798609b981fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceMapMatchedCoordinate_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Geolocation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGuidanceNavigator(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGuidanceNavigator {
    type Vtable = IGuidanceNavigator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x08f17ef7_8e3f_4d9a_be8a_108f9a012c67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceNavigator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, route: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, route: ::windows::runtime::RawPtr, speedinmeterspersecond: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut GuidanceAudioMeasurementSystem) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: GuidanceAudioMeasurementSystem) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut GuidanceAudioNotifications) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: GuidanceAudioNotifications) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, voiceid: i32, voicefolder: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Geolocation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, userlocation: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))] usize,
    #[cfg(feature = "Devices_Geolocation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, userlocation: ::windows::runtime::RawPtr, positionoverride: super::super::super::Devices::Geolocation::BasicGeoposition) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGuidanceNavigator2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGuidanceNavigator2 {
    type Vtable = IGuidanceNavigator2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6cdc50d1_041c_4bf3_b633_a101fc2f6b57);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceNavigator2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGuidanceNavigatorStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGuidanceNavigatorStatics {
    type Vtable = IGuidanceNavigatorStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x00fd9513_4456_4e66_a143_3add6be08426);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceNavigatorStatics_abi(
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
pub struct IGuidanceNavigatorStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGuidanceNavigatorStatics2 {
    type Vtable = IGuidanceNavigatorStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x54c5c3e2_7784_4c85_8c95_d0c6efb43965);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceNavigatorStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGuidanceReroutedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGuidanceReroutedEventArgs {
    type Vtable = IGuidanceReroutedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x115d4008_d528_454e_bb94_a50341d2c9f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceReroutedEventArgs_abi(
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
pub struct IGuidanceRoadSegment(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGuidanceRoadSegment {
    type Vtable = IGuidanceRoadSegment_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb32758a6_be78_4c63_afe7_6c2957479b3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceRoadSegment_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Devices_Geolocation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGuidanceRoadSegment2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGuidanceRoadSegment2 {
    type Vtable = IGuidanceRoadSegment2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2474a61d_1723_49f1_895b_47a2c4aa9c55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceRoadSegment2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGuidanceRoadSignpost(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGuidanceRoadSignpost {
    type Vtable = IGuidanceRoadSignpost_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf1a728b6_f77a_4742_8312_53300f9845f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceRoadSignpost_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGuidanceRoute(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGuidanceRoute {
    type Vtable = IGuidanceRoute_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3a14545d_801a_40bd_a286_afb2010cce6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceRoute_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Devices_Geolocation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))] usize,
    #[cfg(feature = "Devices_Geolocation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGuidanceRouteStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGuidanceRouteStatics {
    type Vtable = IGuidanceRouteStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf56d926a_55ed_49c1_b09c_4b8223b50db3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceRouteStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, maproute: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, maproute: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGuidanceTelemetryCollector(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGuidanceTelemetryCollector {
    type Vtable = IGuidanceTelemetryCollector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdb1f8da5_b878_4d92_98dd_347d23d38262);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceTelemetryCollector_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGuidanceTelemetryCollectorStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGuidanceTelemetryCollectorStatics {
    type Vtable = IGuidanceTelemetryCollectorStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x36532047_f160_44fb_b578_94577ca05990);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceTelemetryCollectorStatics_abi(
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
pub struct IGuidanceUpdatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGuidanceUpdatedEventArgs {
    type Vtable = IGuidanceUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfdac160b_9e8d_4de3_a9fa_b06321d18db9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceUpdatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut GuidanceMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
