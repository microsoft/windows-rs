#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Services_Maps_Guidance")]
pub mod Guidance;
#[cfg(feature = "Services_Maps_LocalSearch")]
pub mod LocalSearch;
#[cfg(feature = "Services_Maps_OfflineMaps")]
pub mod OfflineMaps;
#[doc = "*Required features: `Services_Maps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EnhancedWaypoint(pub ::windows::runtime::IInspectable);
impl EnhancedWaypoint {
    #[cfg(feature = "Devices_Geolocation")]
    #[doc = "*Required features: `Services_Maps`, `Devices_Geolocation`*"]
    pub fn Point(&self) -> ::windows::runtime::Result<super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Geolocation::Geopoint>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<WaypointKind> {
        let this = self;
        unsafe {
            let mut result__: WaypointKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WaypointKind>(result__)
        }
    }
    #[cfg(feature = "Devices_Geolocation")]
    #[doc = "*Required features: `Services_Maps`, `Devices_Geolocation`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Devices::Geolocation::Geopoint>>(point: Param0, kind: WaypointKind) -> ::windows::runtime::Result<EnhancedWaypoint> {
        Self::IEnhancedWaypointFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), point.into_param().abi(), kind, &mut result__).from_abi::<EnhancedWaypoint>(result__)
        })
    }
    pub fn IEnhancedWaypointFactory<R, F: FnOnce(&IEnhancedWaypointFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<EnhancedWaypoint, IEnhancedWaypointFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EnhancedWaypoint {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.EnhancedWaypoint;{ed268c74-5913-11e6-8b77-86f30ca893d3})");
}
unsafe impl ::windows::runtime::Interface for EnhancedWaypoint {
    type Vtable = IEnhancedWaypoint_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xed268c74_5913_11e6_8b77_86f30ca893d3);
}
impl ::windows::runtime::RuntimeName for EnhancedWaypoint {
    const NAME: &'static str = "Windows.Services.Maps.EnhancedWaypoint";
}
impl ::core::convert::From<EnhancedWaypoint> for ::windows::runtime::IUnknown {
    fn from(value: EnhancedWaypoint) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EnhancedWaypoint> for ::windows::runtime::IUnknown {
    fn from(value: &EnhancedWaypoint) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EnhancedWaypoint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a EnhancedWaypoint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EnhancedWaypoint> for ::windows::runtime::IInspectable {
    fn from(value: EnhancedWaypoint) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EnhancedWaypoint> for ::windows::runtime::IInspectable {
    fn from(value: &EnhancedWaypoint) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EnhancedWaypoint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EnhancedWaypoint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EnhancedWaypoint {}
unsafe impl ::core::marker::Sync for EnhancedWaypoint {}
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct GuidanceContract(pub u8);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEnhancedWaypoint(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEnhancedWaypoint {
    type Vtable = IEnhancedWaypoint_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xed268c74_5913_11e6_8b77_86f30ca893d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnhancedWaypoint_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Geolocation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut WaypointKind) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEnhancedWaypointFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEnhancedWaypointFactory {
    type Vtable = IEnhancedWaypointFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xaf868477_a2aa_46dd_b645_23b31b8aa6c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnhancedWaypointFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Geolocation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, point: ::windows::runtime::RawPtr, kind: WaypointKind, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IManeuverWarning(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IManeuverWarning {
    type Vtable = IManeuverWarning_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc1a36d8a_2630_4378_9e4a_6e44253dceba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManeuverWarning_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ManeuverWarningKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ManeuverWarningSeverity) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMapAddress(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMapAddress {
    type Vtable = IMapAddress_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xcfa7a973_a3b4_4494_b3ff_cba94db69699);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapAddress_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMapAddress2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMapAddress2 {
    type Vtable = IMapAddress2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x75cd6df1_e5ad_45a9_bf40_6cf256c1dd13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapAddress2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMapLocation(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMapLocation {
    type Vtable = IMapLocation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3c073f57_0da4_42e8_9ee2_a96fcf2371dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapLocation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Geolocation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMapLocationFinderResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMapLocationFinderResult {
    type Vtable = IMapLocationFinderResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x43f1f179_e8cc_45f6_bed2_54ccbf965d9a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapLocationFinderResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MapLocationFinderStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMapLocationFinderStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMapLocationFinderStatics {
    type Vtable = IMapLocationFinderStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x318adb5d_1c5d_4f35_a2df_aaca94959517);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapLocationFinderStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, querypoint: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, searchtext: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, referencepoint: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, searchtext: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, referencepoint: ::windows::runtime::RawPtr, maxcount: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMapLocationFinderStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMapLocationFinderStatics2 {
    type Vtable = IMapLocationFinderStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x959a8b96_6485_4dfd_851a_33ac317e3af6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapLocationFinderStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, querypoint: ::windows::runtime::RawPtr, accuracy: MapLocationDesiredAccuracy, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMapManagerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMapManagerStatics {
    type Vtable = IMapManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x37e3e515_82b4_4d54_8fd9_af2624b3011c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMapRoute(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMapRoute {
    type Vtable = IMapRoute_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfb07b732_584d_4583_9c60_641fea274349);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRoute_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Geolocation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Devices_Geolocation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMapRoute2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMapRoute2 {
    type Vtable = IMapRoute2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd1c5d40c_2213_4ab0_a260_46b38169beac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRoute2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MapRouteRestrictions) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMapRoute3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMapRoute3 {
    type Vtable = IMapRoute3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x858d1eae_f2ad_429f_bb37_cd21094ffc92);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRoute3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut TrafficCongestion) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMapRoute4(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMapRoute4 {
    type Vtable = IMapRoute4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x366c8ca5_3053_4fa1_80ff_d475f3ed1e6e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRoute4_abi(
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
pub struct IMapRouteDrivingOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMapRouteDrivingOptions {
    type Vtable = IMapRouteDrivingOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6815364d_c6dc_4697_a452_b18f8f0b67a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteDrivingOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MapRouteOptimization) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: MapRouteOptimization) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MapRouteRestrictions) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: MapRouteRestrictions) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMapRouteDrivingOptions2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMapRouteDrivingOptions2 {
    type Vtable = IMapRouteDrivingOptions2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x35dc8670_c298_48d0_b5ad_825460645603);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteDrivingOptions2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMapRouteFinderResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMapRouteFinderResult {
    type Vtable = IMapRouteFinderResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa868a31a_9422_46ac_8ca1_b1614d4bfbe2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteFinderResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MapRouteFinderStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMapRouteFinderResult2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMapRouteFinderResult2 {
    type Vtable = IMapRouteFinderResult2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x20709c6d_d90c_46c8_91c6_7d4be4efb215);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteFinderResult2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMapRouteFinderStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMapRouteFinderStatics {
    type Vtable = IMapRouteFinderStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb8a5c50f_1c64_4c3a_81eb_1f7c152afbbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteFinderStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, startpoint: ::windows::runtime::RawPtr, endpoint: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, startpoint: ::windows::runtime::RawPtr, endpoint: ::windows::runtime::RawPtr, optimization: MapRouteOptimization, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, startpoint: ::windows::runtime::RawPtr, endpoint: ::windows::runtime::RawPtr, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, startpoint: ::windows::runtime::RawPtr, endpoint: ::windows::runtime::RawPtr, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, headingindegrees: f64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, waypoints: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, waypoints: ::windows::runtime::RawPtr, optimization: MapRouteOptimization, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, waypoints: ::windows::runtime::RawPtr, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, waypoints: ::windows::runtime::RawPtr, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, headingindegrees: f64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, startpoint: ::windows::runtime::RawPtr, endpoint: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, waypoints: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMapRouteFinderStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMapRouteFinderStatics2 {
    type Vtable = IMapRouteFinderStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xafcc2c73_7760_49af_b3bd_baf135b703e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteFinderStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, startpoint: ::windows::runtime::RawPtr, endpoint: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMapRouteFinderStatics3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMapRouteFinderStatics3 {
    type Vtable = IMapRouteFinderStatics3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf6098134_5913_11e6_8b77_86f30ca893d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteFinderStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, waypoints: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, waypoints: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMapRouteLeg(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMapRouteLeg {
    type Vtable = IMapRouteLeg_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x96f8b2f6_5bba_4d17_9db6_1a263fec7471);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteLeg_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Geolocation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))] usize,
    #[cfg(feature = "Devices_Geolocation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMapRouteLeg2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMapRouteLeg2 {
    type Vtable = IMapRouteLeg2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x02e2062d_c9c6_45b8_8e54_1a10b57a17e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteLeg2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut TrafficCongestion) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMapRouteManeuver(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMapRouteManeuver {
    type Vtable = IMapRouteManeuver_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xed5c17f0_a6ab_4d65_a086_fa8a7e340df2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteManeuver_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Geolocation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MapRouteManeuverKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MapManeuverNotices) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMapRouteManeuver2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMapRouteManeuver2 {
    type Vtable = IMapRouteManeuver2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5d7bcd9c_7c9b_41df_838b_eae21e4b05a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteManeuver2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMapRouteManeuver3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMapRouteManeuver3 {
    type Vtable = IMapRouteManeuver3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa6a138df_0483_4166_85be_b99336c11875);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteManeuver3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMapServiceStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMapServiceStatics {
    type Vtable = IMapServiceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0144ad85_c04c_4cdd_871a_a0726d097cd4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapServiceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMapServiceStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMapServiceStatics2 {
    type Vtable = IMapServiceStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf8193eed_9c85_40a9_8896_0fc3fd2b7c2a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapServiceStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMapServiceStatics3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMapServiceStatics3 {
    type Vtable = IMapServiceStatics3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0a11ce20_63a7_4854_b355_d6dcda223d1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapServiceStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMapServiceStatics4(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMapServiceStatics4 {
    type Vtable = IMapServiceStatics4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x088a2862_6abc_420e_945f_4cfd89c67356);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapServiceStatics4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: MapServiceDataUsagePreference) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MapServiceDataUsagePreference) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlaceInfo(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlaceInfo {
    type Vtable = IPlaceInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9a0810b6_31c8_4f6a_9f18_950b4c38951a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaceInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, selection: super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Geolocation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlaceInfoCreateOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlaceInfoCreateOptions {
    type Vtable = IPlaceInfoCreateOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xcd33c125_67f1_4bb3_9907_ecce939b0399);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaceInfoCreateOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlaceInfoStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlaceInfoStatics {
    type Vtable = IPlaceInfoStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x82b9ff71_6cd0_48a4_afd9_5ed82097936b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaceInfoStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Geolocation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, referencepoint: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))] usize,
    #[cfg(feature = "Devices_Geolocation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, referencepoint: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, identifier: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Geolocation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, identifier: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, defaultpoint: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, location: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlaceInfoStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlaceInfoStatics2 {
    type Vtable = IPlaceInfoStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x730f0249_4047_44a3_8f81_2550a5216370);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaceInfoStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, displayaddress: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, displayaddress: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct LocalSearchContract(pub u8);
#[doc = "*Required features: `Services_Maps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ManeuverWarning(pub ::windows::runtime::IInspectable);
impl ManeuverWarning {
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ManeuverWarningKind> {
        let this = self;
        unsafe {
            let mut result__: ManeuverWarningKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ManeuverWarningKind>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn Severity(&self) -> ::windows::runtime::Result<ManeuverWarningSeverity> {
        let this = self;
        unsafe {
            let mut result__: ManeuverWarningSeverity = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ManeuverWarningSeverity>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ManeuverWarning {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.ManeuverWarning;{c1a36d8a-2630-4378-9e4a-6e44253dceba})");
}
unsafe impl ::windows::runtime::Interface for ManeuverWarning {
    type Vtable = IManeuverWarning_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc1a36d8a_2630_4378_9e4a_6e44253dceba);
}
impl ::windows::runtime::RuntimeName for ManeuverWarning {
    const NAME: &'static str = "Windows.Services.Maps.ManeuverWarning";
}
impl ::core::convert::From<ManeuverWarning> for ::windows::runtime::IUnknown {
    fn from(value: ManeuverWarning) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ManeuverWarning> for ::windows::runtime::IUnknown {
    fn from(value: &ManeuverWarning) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ManeuverWarning {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ManeuverWarning {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ManeuverWarning> for ::windows::runtime::IInspectable {
    fn from(value: ManeuverWarning) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ManeuverWarning> for ::windows::runtime::IInspectable {
    fn from(value: &ManeuverWarning) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ManeuverWarning {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ManeuverWarning {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ManeuverWarning {}
unsafe impl ::core::marker::Sync for ManeuverWarning {}
#[doc = "*Required features: `Services_Maps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ManeuverWarningKind(pub i32);
impl ManeuverWarningKind {
    pub const None: ManeuverWarningKind = ManeuverWarningKind(0i32);
    pub const Accident: ManeuverWarningKind = ManeuverWarningKind(1i32);
    pub const AdministrativeDivisionChange: ManeuverWarningKind = ManeuverWarningKind(2i32);
    pub const Alert: ManeuverWarningKind = ManeuverWarningKind(3i32);
    pub const BlockedRoad: ManeuverWarningKind = ManeuverWarningKind(4i32);
    pub const CheckTimetable: ManeuverWarningKind = ManeuverWarningKind(5i32);
    pub const Congestion: ManeuverWarningKind = ManeuverWarningKind(6i32);
    pub const Construction: ManeuverWarningKind = ManeuverWarningKind(7i32);
    pub const CountryChange: ManeuverWarningKind = ManeuverWarningKind(8i32);
    pub const DisabledVehicle: ManeuverWarningKind = ManeuverWarningKind(9i32);
    pub const GateAccess: ManeuverWarningKind = ManeuverWarningKind(10i32);
    pub const GetOffTransit: ManeuverWarningKind = ManeuverWarningKind(11i32);
    pub const GetOnTransit: ManeuverWarningKind = ManeuverWarningKind(12i32);
    pub const IllegalUTurn: ManeuverWarningKind = ManeuverWarningKind(13i32);
    pub const MassTransit: ManeuverWarningKind = ManeuverWarningKind(14i32);
    pub const Miscellaneous: ManeuverWarningKind = ManeuverWarningKind(15i32);
    pub const NoIncident: ManeuverWarningKind = ManeuverWarningKind(16i32);
    pub const Other: ManeuverWarningKind = ManeuverWarningKind(17i32);
    pub const OtherNews: ManeuverWarningKind = ManeuverWarningKind(18i32);
    pub const OtherTrafficIncidents: ManeuverWarningKind = ManeuverWarningKind(19i32);
    pub const PlannedEvent: ManeuverWarningKind = ManeuverWarningKind(20i32);
    pub const PrivateRoad: ManeuverWarningKind = ManeuverWarningKind(21i32);
    pub const RestrictedTurn: ManeuverWarningKind = ManeuverWarningKind(22i32);
    pub const RoadClosures: ManeuverWarningKind = ManeuverWarningKind(23i32);
    pub const RoadHazard: ManeuverWarningKind = ManeuverWarningKind(24i32);
    pub const ScheduledConstruction: ManeuverWarningKind = ManeuverWarningKind(25i32);
    pub const SeasonalClosures: ManeuverWarningKind = ManeuverWarningKind(26i32);
    pub const Tollbooth: ManeuverWarningKind = ManeuverWarningKind(27i32);
    pub const TollRoad: ManeuverWarningKind = ManeuverWarningKind(28i32);
    pub const TollZoneEnter: ManeuverWarningKind = ManeuverWarningKind(29i32);
    pub const TollZoneExit: ManeuverWarningKind = ManeuverWarningKind(30i32);
    pub const TrafficFlow: ManeuverWarningKind = ManeuverWarningKind(31i32);
    pub const TransitLineChange: ManeuverWarningKind = ManeuverWarningKind(32i32);
    pub const UnpavedRoad: ManeuverWarningKind = ManeuverWarningKind(33i32);
    pub const UnscheduledConstruction: ManeuverWarningKind = ManeuverWarningKind(34i32);
    pub const Weather: ManeuverWarningKind = ManeuverWarningKind(35i32);
}
impl ::core::convert::From<i32> for ManeuverWarningKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ManeuverWarningKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ManeuverWarningKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.ManeuverWarningKind;i4)");
}
impl ::windows::runtime::DefaultType for ManeuverWarningKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `Services_Maps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ManeuverWarningSeverity(pub i32);
impl ManeuverWarningSeverity {
    pub const None: ManeuverWarningSeverity = ManeuverWarningSeverity(0i32);
    pub const LowImpact: ManeuverWarningSeverity = ManeuverWarningSeverity(1i32);
    pub const Minor: ManeuverWarningSeverity = ManeuverWarningSeverity(2i32);
    pub const Moderate: ManeuverWarningSeverity = ManeuverWarningSeverity(3i32);
    pub const Serious: ManeuverWarningSeverity = ManeuverWarningSeverity(4i32);
}
impl ::core::convert::From<i32> for ManeuverWarningSeverity {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ManeuverWarningSeverity {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ManeuverWarningSeverity {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.ManeuverWarningSeverity;i4)");
}
impl ::windows::runtime::DefaultType for ManeuverWarningSeverity {
    type DefaultType = Self;
}
#[doc = "*Required features: `Services_Maps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MapAddress(pub ::windows::runtime::IInspectable);
impl MapAddress {
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn BuildingName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn BuildingFloor(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn BuildingRoom(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn BuildingWing(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn StreetNumber(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn Street(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn Neighborhood(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn District(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn Town(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn Region(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn RegionCode(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn Country(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn CountryCode(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn PostCode(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn Continent(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn FormattedAddress(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IMapAddress2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MapAddress {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.MapAddress;{cfa7a973-a3b4-4494-b3ff-cba94db69699})");
}
unsafe impl ::windows::runtime::Interface for MapAddress {
    type Vtable = IMapAddress_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xcfa7a973_a3b4_4494_b3ff_cba94db69699);
}
impl ::windows::runtime::RuntimeName for MapAddress {
    const NAME: &'static str = "Windows.Services.Maps.MapAddress";
}
impl ::core::convert::From<MapAddress> for ::windows::runtime::IUnknown {
    fn from(value: MapAddress) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MapAddress> for ::windows::runtime::IUnknown {
    fn from(value: &MapAddress) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MapAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MapAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MapAddress> for ::windows::runtime::IInspectable {
    fn from(value: MapAddress) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MapAddress> for ::windows::runtime::IInspectable {
    fn from(value: &MapAddress) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MapAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MapAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MapAddress {}
unsafe impl ::core::marker::Sync for MapAddress {}
#[doc = "*Required features: `Services_Maps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MapLocation(pub ::windows::runtime::IInspectable);
impl MapLocation {
    #[cfg(feature = "Devices_Geolocation")]
    #[doc = "*Required features: `Services_Maps`, `Devices_Geolocation`*"]
    pub fn Point(&self) -> ::windows::runtime::Result<super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Geolocation::Geopoint>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn Description(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn Address(&self) -> ::windows::runtime::Result<MapAddress> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapAddress>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MapLocation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.MapLocation;{3c073f57-0da4-42e8-9ee2-a96fcf2371dc})");
}
unsafe impl ::windows::runtime::Interface for MapLocation {
    type Vtable = IMapLocation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3c073f57_0da4_42e8_9ee2_a96fcf2371dc);
}
impl ::windows::runtime::RuntimeName for MapLocation {
    const NAME: &'static str = "Windows.Services.Maps.MapLocation";
}
impl ::core::convert::From<MapLocation> for ::windows::runtime::IUnknown {
    fn from(value: MapLocation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MapLocation> for ::windows::runtime::IUnknown {
    fn from(value: &MapLocation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MapLocation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MapLocation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MapLocation> for ::windows::runtime::IInspectable {
    fn from(value: MapLocation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MapLocation> for ::windows::runtime::IInspectable {
    fn from(value: &MapLocation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MapLocation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MapLocation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MapLocation {}
unsafe impl ::core::marker::Sync for MapLocation {}
#[doc = "*Required features: `Services_Maps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MapLocationDesiredAccuracy(pub i32);
impl MapLocationDesiredAccuracy {
    pub const High: MapLocationDesiredAccuracy = MapLocationDesiredAccuracy(0i32);
    pub const Low: MapLocationDesiredAccuracy = MapLocationDesiredAccuracy(1i32);
}
impl ::core::convert::From<i32> for MapLocationDesiredAccuracy {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MapLocationDesiredAccuracy {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MapLocationDesiredAccuracy {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.MapLocationDesiredAccuracy;i4)");
}
impl ::windows::runtime::DefaultType for MapLocationDesiredAccuracy {
    type DefaultType = Self;
}
#[doc = "*Required features: `Services_Maps`*"]
pub struct MapLocationFinder {}
impl MapLocationFinder {
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    #[doc = "*Required features: `Services_Maps`, `Devices_Geolocation`, `Foundation`*"]
    pub fn FindLocationsAtAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Devices::Geolocation::Geopoint>>(querypoint: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MapLocationFinderResult>> {
        Self::IMapLocationFinderStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), querypoint.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MapLocationFinderResult>>(result__)
        })
    }
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    #[doc = "*Required features: `Services_Maps`, `Devices_Geolocation`, `Foundation`*"]
    pub fn FindLocationsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Devices::Geolocation::Geopoint>>(searchtext: Param0, referencepoint: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MapLocationFinderResult>> {
        Self::IMapLocationFinderStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), searchtext.into_param().abi(), referencepoint.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MapLocationFinderResult>>(result__)
        })
    }
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    #[doc = "*Required features: `Services_Maps`, `Devices_Geolocation`, `Foundation`*"]
    pub fn FindLocationsWithMaxCountAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Devices::Geolocation::Geopoint>>(searchtext: Param0, referencepoint: Param1, maxcount: u32) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MapLocationFinderResult>> {
        Self::IMapLocationFinderStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), searchtext.into_param().abi(), referencepoint.into_param().abi(), maxcount, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MapLocationFinderResult>>(result__)
        })
    }
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    #[doc = "*Required features: `Services_Maps`, `Devices_Geolocation`, `Foundation`*"]
    pub fn FindLocationsAtWithAccuracyAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Devices::Geolocation::Geopoint>>(querypoint: Param0, accuracy: MapLocationDesiredAccuracy) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MapLocationFinderResult>> {
        Self::IMapLocationFinderStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), querypoint.into_param().abi(), accuracy, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MapLocationFinderResult>>(result__)
        })
    }
    pub fn IMapLocationFinderStatics<R, F: FnOnce(&IMapLocationFinderStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MapLocationFinder, IMapLocationFinderStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMapLocationFinderStatics2<R, F: FnOnce(&IMapLocationFinderStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MapLocationFinder, IMapLocationFinderStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for MapLocationFinder {
    const NAME: &'static str = "Windows.Services.Maps.MapLocationFinder";
}
#[doc = "*Required features: `Services_Maps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MapLocationFinderResult(pub ::windows::runtime::IInspectable);
impl MapLocationFinderResult {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Services_Maps`, `Foundation_Collections`*"]
    pub fn Locations(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<MapLocation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MapLocation>>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<MapLocationFinderStatus> {
        let this = self;
        unsafe {
            let mut result__: MapLocationFinderStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapLocationFinderStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MapLocationFinderResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.MapLocationFinderResult;{43f1f179-e8cc-45f6-bed2-54ccbf965d9a})");
}
unsafe impl ::windows::runtime::Interface for MapLocationFinderResult {
    type Vtable = IMapLocationFinderResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x43f1f179_e8cc_45f6_bed2_54ccbf965d9a);
}
impl ::windows::runtime::RuntimeName for MapLocationFinderResult {
    const NAME: &'static str = "Windows.Services.Maps.MapLocationFinderResult";
}
impl ::core::convert::From<MapLocationFinderResult> for ::windows::runtime::IUnknown {
    fn from(value: MapLocationFinderResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MapLocationFinderResult> for ::windows::runtime::IUnknown {
    fn from(value: &MapLocationFinderResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MapLocationFinderResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MapLocationFinderResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MapLocationFinderResult> for ::windows::runtime::IInspectable {
    fn from(value: MapLocationFinderResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MapLocationFinderResult> for ::windows::runtime::IInspectable {
    fn from(value: &MapLocationFinderResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MapLocationFinderResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MapLocationFinderResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MapLocationFinderResult {}
unsafe impl ::core::marker::Sync for MapLocationFinderResult {}
#[doc = "*Required features: `Services_Maps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MapLocationFinderStatus(pub i32);
impl MapLocationFinderStatus {
    pub const Success: MapLocationFinderStatus = MapLocationFinderStatus(0i32);
    pub const UnknownError: MapLocationFinderStatus = MapLocationFinderStatus(1i32);
    pub const InvalidCredentials: MapLocationFinderStatus = MapLocationFinderStatus(2i32);
    pub const BadLocation: MapLocationFinderStatus = MapLocationFinderStatus(3i32);
    pub const IndexFailure: MapLocationFinderStatus = MapLocationFinderStatus(4i32);
    pub const NetworkFailure: MapLocationFinderStatus = MapLocationFinderStatus(5i32);
    pub const NotSupported: MapLocationFinderStatus = MapLocationFinderStatus(6i32);
}
impl ::core::convert::From<i32> for MapLocationFinderStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MapLocationFinderStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MapLocationFinderStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.MapLocationFinderStatus;i4)");
}
impl ::windows::runtime::DefaultType for MapLocationFinderStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Services_Maps`*"]
pub struct MapManager {}
impl MapManager {
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn ShowDownloadedMapsUI() -> ::windows::runtime::Result<()> {
        Self::IMapManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn ShowMapsUpdateUI() -> ::windows::runtime::Result<()> {
        Self::IMapManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() })
    }
    pub fn IMapManagerStatics<R, F: FnOnce(&IMapManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MapManager, IMapManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for MapManager {
    const NAME: &'static str = "Windows.Services.Maps.MapManager";
}
#[doc = "*Required features: `Services_Maps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MapManeuverNotices(pub u32);
impl MapManeuverNotices {
    pub const None: MapManeuverNotices = MapManeuverNotices(0u32);
    pub const Toll: MapManeuverNotices = MapManeuverNotices(1u32);
    pub const Unpaved: MapManeuverNotices = MapManeuverNotices(2u32);
}
impl ::core::convert::From<u32> for MapManeuverNotices {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MapManeuverNotices {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MapManeuverNotices {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.MapManeuverNotices;u4)");
}
impl ::windows::runtime::DefaultType for MapManeuverNotices {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for MapManeuverNotices {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for MapManeuverNotices {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for MapManeuverNotices {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for MapManeuverNotices {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for MapManeuverNotices {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Services_Maps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MapRoute(pub ::windows::runtime::IInspectable);
impl MapRoute {
    #[cfg(feature = "Devices_Geolocation")]
    #[doc = "*Required features: `Services_Maps`, `Devices_Geolocation`*"]
    pub fn BoundingBox(&self) -> ::windows::runtime::Result<super::super::Devices::Geolocation::GeoboundingBox> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Geolocation::GeoboundingBox>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn LengthInMeters(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Maps`, `Foundation`*"]
    pub fn EstimatedDuration(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Devices_Geolocation")]
    #[doc = "*Required features: `Services_Maps`, `Devices_Geolocation`*"]
    pub fn Path(&self) -> ::windows::runtime::Result<super::super::Devices::Geolocation::Geopath> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Geolocation::Geopath>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Services_Maps`, `Foundation_Collections`*"]
    pub fn Legs(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<MapRouteLeg>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MapRouteLeg>>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn IsTrafficBased(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn ViolatedRestrictions(&self) -> ::windows::runtime::Result<MapRouteRestrictions> {
        let this = &::windows::runtime::Interface::cast::<IMapRoute2>(self)?;
        unsafe {
            let mut result__: MapRouteRestrictions = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapRouteRestrictions>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn HasBlockedRoads(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IMapRoute2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Maps`, `Foundation`*"]
    pub fn DurationWithoutTraffic(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::runtime::Interface::cast::<IMapRoute3>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn TrafficCongestion(&self) -> ::windows::runtime::Result<TrafficCongestion> {
        let this = &::windows::runtime::Interface::cast::<IMapRoute3>(self)?;
        unsafe {
            let mut result__: TrafficCongestion = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TrafficCongestion>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn IsScenic(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IMapRoute4>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MapRoute {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.MapRoute;{fb07b732-584d-4583-9c60-641fea274349})");
}
unsafe impl ::windows::runtime::Interface for MapRoute {
    type Vtable = IMapRoute_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfb07b732_584d_4583_9c60_641fea274349);
}
impl ::windows::runtime::RuntimeName for MapRoute {
    const NAME: &'static str = "Windows.Services.Maps.MapRoute";
}
impl ::core::convert::From<MapRoute> for ::windows::runtime::IUnknown {
    fn from(value: MapRoute) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MapRoute> for ::windows::runtime::IUnknown {
    fn from(value: &MapRoute) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MapRoute {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MapRoute {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MapRoute> for ::windows::runtime::IInspectable {
    fn from(value: MapRoute) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MapRoute> for ::windows::runtime::IInspectable {
    fn from(value: &MapRoute) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MapRoute {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MapRoute {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MapRoute {}
unsafe impl ::core::marker::Sync for MapRoute {}
#[doc = "*Required features: `Services_Maps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MapRouteDrivingOptions(pub ::windows::runtime::IInspectable);
impl MapRouteDrivingOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MapRouteDrivingOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn MaxAlternateRouteCount(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn SetMaxAlternateRouteCount(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Maps`, `Foundation`*"]
    pub fn InitialHeading(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Maps`, `Foundation`*"]
    pub fn SetInitialHeading<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<f64>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn RouteOptimization(&self) -> ::windows::runtime::Result<MapRouteOptimization> {
        let this = self;
        unsafe {
            let mut result__: MapRouteOptimization = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapRouteOptimization>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn SetRouteOptimization(&self, value: MapRouteOptimization) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn RouteRestrictions(&self) -> ::windows::runtime::Result<MapRouteRestrictions> {
        let this = self;
        unsafe {
            let mut result__: MapRouteRestrictions = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapRouteRestrictions>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn SetRouteRestrictions(&self, value: MapRouteRestrictions) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Maps`, `Foundation`*"]
    pub fn DepartureTime(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = &::windows::runtime::Interface::cast::<IMapRouteDrivingOptions2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Maps`, `Foundation`*"]
    pub fn SetDepartureTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMapRouteDrivingOptions2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MapRouteDrivingOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.MapRouteDrivingOptions;{6815364d-c6dc-4697-a452-b18f8f0b67a1})");
}
unsafe impl ::windows::runtime::Interface for MapRouteDrivingOptions {
    type Vtable = IMapRouteDrivingOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6815364d_c6dc_4697_a452_b18f8f0b67a1);
}
impl ::windows::runtime::RuntimeName for MapRouteDrivingOptions {
    const NAME: &'static str = "Windows.Services.Maps.MapRouteDrivingOptions";
}
impl ::core::convert::From<MapRouteDrivingOptions> for ::windows::runtime::IUnknown {
    fn from(value: MapRouteDrivingOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MapRouteDrivingOptions> for ::windows::runtime::IUnknown {
    fn from(value: &MapRouteDrivingOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MapRouteDrivingOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MapRouteDrivingOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MapRouteDrivingOptions> for ::windows::runtime::IInspectable {
    fn from(value: MapRouteDrivingOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MapRouteDrivingOptions> for ::windows::runtime::IInspectable {
    fn from(value: &MapRouteDrivingOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MapRouteDrivingOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MapRouteDrivingOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MapRouteDrivingOptions {}
unsafe impl ::core::marker::Sync for MapRouteDrivingOptions {}
#[doc = "*Required features: `Services_Maps`*"]
pub struct MapRouteFinder {}
impl MapRouteFinder {
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    #[doc = "*Required features: `Services_Maps`, `Devices_Geolocation`, `Foundation`*"]
    pub fn GetDrivingRouteAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Devices::Geolocation::Geopoint>, Param1: ::windows::runtime::IntoParam<'a, super::super::Devices::Geolocation::Geopoint>>(startpoint: Param0, endpoint: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>> {
        Self::IMapRouteFinderStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), startpoint.into_param().abi(), endpoint.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>(result__)
        })
    }
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    #[doc = "*Required features: `Services_Maps`, `Devices_Geolocation`, `Foundation`*"]
    pub fn GetDrivingRouteWithOptimizationAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Devices::Geolocation::Geopoint>, Param1: ::windows::runtime::IntoParam<'a, super::super::Devices::Geolocation::Geopoint>>(startpoint: Param0, endpoint: Param1, optimization: MapRouteOptimization) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>> {
        Self::IMapRouteFinderStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), startpoint.into_param().abi(), endpoint.into_param().abi(), optimization, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>(result__)
        })
    }
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    #[doc = "*Required features: `Services_Maps`, `Devices_Geolocation`, `Foundation`*"]
    pub fn GetDrivingRouteWithOptimizationAndRestrictionsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Devices::Geolocation::Geopoint>, Param1: ::windows::runtime::IntoParam<'a, super::super::Devices::Geolocation::Geopoint>>(startpoint: Param0, endpoint: Param1, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>> {
        Self::IMapRouteFinderStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), startpoint.into_param().abi(), endpoint.into_param().abi(), optimization, restrictions, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>(result__)
        })
    }
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    #[doc = "*Required features: `Services_Maps`, `Devices_Geolocation`, `Foundation`*"]
    pub fn GetDrivingRouteWithOptimizationRestrictionsAndHeadingAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Devices::Geolocation::Geopoint>, Param1: ::windows::runtime::IntoParam<'a, super::super::Devices::Geolocation::Geopoint>>(startpoint: Param0, endpoint: Param1, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, headingindegrees: f64) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>> {
        Self::IMapRouteFinderStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), startpoint.into_param().abi(), endpoint.into_param().abi(), optimization, restrictions, headingindegrees, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>(result__)
        })
    }
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Services_Maps`, `Devices_Geolocation`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetDrivingRouteFromWaypointsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint>>>(waypoints: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>> {
        Self::IMapRouteFinderStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), waypoints.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>(result__)
        })
    }
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Services_Maps`, `Devices_Geolocation`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetDrivingRouteFromWaypointsAndOptimizationAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint>>>(waypoints: Param0, optimization: MapRouteOptimization) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>> {
        Self::IMapRouteFinderStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), waypoints.into_param().abi(), optimization, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>(result__)
        })
    }
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Services_Maps`, `Devices_Geolocation`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetDrivingRouteFromWaypointsOptimizationAndRestrictionsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint>>>(waypoints: Param0, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>> {
        Self::IMapRouteFinderStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), waypoints.into_param().abi(), optimization, restrictions, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>(result__)
        })
    }
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Services_Maps`, `Devices_Geolocation`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetDrivingRouteFromWaypointsOptimizationRestrictionsAndHeadingAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint>>>(waypoints: Param0, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, headingindegrees: f64) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>> {
        Self::IMapRouteFinderStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), waypoints.into_param().abi(), optimization, restrictions, headingindegrees, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>(result__)
        })
    }
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    #[doc = "*Required features: `Services_Maps`, `Devices_Geolocation`, `Foundation`*"]
    pub fn GetWalkingRouteAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Devices::Geolocation::Geopoint>, Param1: ::windows::runtime::IntoParam<'a, super::super::Devices::Geolocation::Geopoint>>(startpoint: Param0, endpoint: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>> {
        Self::IMapRouteFinderStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), startpoint.into_param().abi(), endpoint.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>(result__)
        })
    }
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Services_Maps`, `Devices_Geolocation`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetWalkingRouteFromWaypointsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint>>>(waypoints: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>> {
        Self::IMapRouteFinderStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), waypoints.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>(result__)
        })
    }
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    #[doc = "*Required features: `Services_Maps`, `Devices_Geolocation`, `Foundation`*"]
    pub fn GetDrivingRouteWithOptionsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Devices::Geolocation::Geopoint>, Param1: ::windows::runtime::IntoParam<'a, super::super::Devices::Geolocation::Geopoint>, Param2: ::windows::runtime::IntoParam<'a, MapRouteDrivingOptions>>(startpoint: Param0, endpoint: Param1, options: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>> {
        Self::IMapRouteFinderStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), startpoint.into_param().abi(), endpoint.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Services_Maps`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetDrivingRouteFromEnhancedWaypointsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<EnhancedWaypoint>>>(waypoints: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>> {
        Self::IMapRouteFinderStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), waypoints.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Services_Maps`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetDrivingRouteFromEnhancedWaypointsWithOptionsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<EnhancedWaypoint>>, Param1: ::windows::runtime::IntoParam<'a, MapRouteDrivingOptions>>(waypoints: Param0, options: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>> {
        Self::IMapRouteFinderStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), waypoints.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>(result__)
        })
    }
    pub fn IMapRouteFinderStatics<R, F: FnOnce(&IMapRouteFinderStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MapRouteFinder, IMapRouteFinderStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMapRouteFinderStatics2<R, F: FnOnce(&IMapRouteFinderStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MapRouteFinder, IMapRouteFinderStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMapRouteFinderStatics3<R, F: FnOnce(&IMapRouteFinderStatics3) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MapRouteFinder, IMapRouteFinderStatics3> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for MapRouteFinder {
    const NAME: &'static str = "Windows.Services.Maps.MapRouteFinder";
}
#[doc = "*Required features: `Services_Maps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MapRouteFinderResult(pub ::windows::runtime::IInspectable);
impl MapRouteFinderResult {
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn Route(&self) -> ::windows::runtime::Result<MapRoute> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapRoute>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<MapRouteFinderStatus> {
        let this = self;
        unsafe {
            let mut result__: MapRouteFinderStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapRouteFinderStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Services_Maps`, `Foundation_Collections`*"]
    pub fn AlternateRoutes(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<MapRoute>> {
        let this = &::windows::runtime::Interface::cast::<IMapRouteFinderResult2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MapRoute>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MapRouteFinderResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.MapRouteFinderResult;{a868a31a-9422-46ac-8ca1-b1614d4bfbe2})");
}
unsafe impl ::windows::runtime::Interface for MapRouteFinderResult {
    type Vtable = IMapRouteFinderResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa868a31a_9422_46ac_8ca1_b1614d4bfbe2);
}
impl ::windows::runtime::RuntimeName for MapRouteFinderResult {
    const NAME: &'static str = "Windows.Services.Maps.MapRouteFinderResult";
}
impl ::core::convert::From<MapRouteFinderResult> for ::windows::runtime::IUnknown {
    fn from(value: MapRouteFinderResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MapRouteFinderResult> for ::windows::runtime::IUnknown {
    fn from(value: &MapRouteFinderResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MapRouteFinderResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MapRouteFinderResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MapRouteFinderResult> for ::windows::runtime::IInspectable {
    fn from(value: MapRouteFinderResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MapRouteFinderResult> for ::windows::runtime::IInspectable {
    fn from(value: &MapRouteFinderResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MapRouteFinderResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MapRouteFinderResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MapRouteFinderResult {}
unsafe impl ::core::marker::Sync for MapRouteFinderResult {}
#[doc = "*Required features: `Services_Maps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MapRouteFinderStatus(pub i32);
impl MapRouteFinderStatus {
    pub const Success: MapRouteFinderStatus = MapRouteFinderStatus(0i32);
    pub const UnknownError: MapRouteFinderStatus = MapRouteFinderStatus(1i32);
    pub const InvalidCredentials: MapRouteFinderStatus = MapRouteFinderStatus(2i32);
    pub const NoRouteFound: MapRouteFinderStatus = MapRouteFinderStatus(3i32);
    pub const NoRouteFoundWithGivenOptions: MapRouteFinderStatus = MapRouteFinderStatus(4i32);
    pub const StartPointNotFound: MapRouteFinderStatus = MapRouteFinderStatus(5i32);
    pub const EndPointNotFound: MapRouteFinderStatus = MapRouteFinderStatus(6i32);
    pub const NoPedestrianRouteFound: MapRouteFinderStatus = MapRouteFinderStatus(7i32);
    pub const NetworkFailure: MapRouteFinderStatus = MapRouteFinderStatus(8i32);
    pub const NotSupported: MapRouteFinderStatus = MapRouteFinderStatus(9i32);
}
impl ::core::convert::From<i32> for MapRouteFinderStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MapRouteFinderStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MapRouteFinderStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.MapRouteFinderStatus;i4)");
}
impl ::windows::runtime::DefaultType for MapRouteFinderStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Services_Maps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MapRouteLeg(pub ::windows::runtime::IInspectable);
impl MapRouteLeg {
    #[cfg(feature = "Devices_Geolocation")]
    #[doc = "*Required features: `Services_Maps`, `Devices_Geolocation`*"]
    pub fn BoundingBox(&self) -> ::windows::runtime::Result<super::super::Devices::Geolocation::GeoboundingBox> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Geolocation::GeoboundingBox>(result__)
        }
    }
    #[cfg(feature = "Devices_Geolocation")]
    #[doc = "*Required features: `Services_Maps`, `Devices_Geolocation`*"]
    pub fn Path(&self) -> ::windows::runtime::Result<super::super::Devices::Geolocation::Geopath> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Geolocation::Geopath>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn LengthInMeters(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Maps`, `Foundation`*"]
    pub fn EstimatedDuration(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Services_Maps`, `Foundation_Collections`*"]
    pub fn Maneuvers(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<MapRouteManeuver>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MapRouteManeuver>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Maps`, `Foundation`*"]
    pub fn DurationWithoutTraffic(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::runtime::Interface::cast::<IMapRouteLeg2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn TrafficCongestion(&self) -> ::windows::runtime::Result<TrafficCongestion> {
        let this = &::windows::runtime::Interface::cast::<IMapRouteLeg2>(self)?;
        unsafe {
            let mut result__: TrafficCongestion = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TrafficCongestion>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MapRouteLeg {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.MapRouteLeg;{96f8b2f6-5bba-4d17-9db6-1a263fec7471})");
}
unsafe impl ::windows::runtime::Interface for MapRouteLeg {
    type Vtable = IMapRouteLeg_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x96f8b2f6_5bba_4d17_9db6_1a263fec7471);
}
impl ::windows::runtime::RuntimeName for MapRouteLeg {
    const NAME: &'static str = "Windows.Services.Maps.MapRouteLeg";
}
impl ::core::convert::From<MapRouteLeg> for ::windows::runtime::IUnknown {
    fn from(value: MapRouteLeg) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MapRouteLeg> for ::windows::runtime::IUnknown {
    fn from(value: &MapRouteLeg) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MapRouteLeg {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MapRouteLeg {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MapRouteLeg> for ::windows::runtime::IInspectable {
    fn from(value: MapRouteLeg) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MapRouteLeg> for ::windows::runtime::IInspectable {
    fn from(value: &MapRouteLeg) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MapRouteLeg {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MapRouteLeg {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MapRouteLeg {}
unsafe impl ::core::marker::Sync for MapRouteLeg {}
#[doc = "*Required features: `Services_Maps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MapRouteManeuver(pub ::windows::runtime::IInspectable);
impl MapRouteManeuver {
    #[cfg(feature = "Devices_Geolocation")]
    #[doc = "*Required features: `Services_Maps`, `Devices_Geolocation`*"]
    pub fn StartingPoint(&self) -> ::windows::runtime::Result<super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Geolocation::Geopoint>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn LengthInMeters(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn InstructionText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<MapRouteManeuverKind> {
        let this = self;
        unsafe {
            let mut result__: MapRouteManeuverKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapRouteManeuverKind>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn ExitNumber(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn ManeuverNotices(&self) -> ::windows::runtime::Result<MapManeuverNotices> {
        let this = self;
        unsafe {
            let mut result__: MapManeuverNotices = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapManeuverNotices>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn StartHeading(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<IMapRouteManeuver2>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn EndHeading(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<IMapRouteManeuver2>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn StreetName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IMapRouteManeuver2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Services_Maps`, `Foundation_Collections`*"]
    pub fn Warnings(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<ManeuverWarning>> {
        let this = &::windows::runtime::Interface::cast::<IMapRouteManeuver3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ManeuverWarning>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MapRouteManeuver {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.MapRouteManeuver;{ed5c17f0-a6ab-4d65-a086-fa8a7e340df2})");
}
unsafe impl ::windows::runtime::Interface for MapRouteManeuver {
    type Vtable = IMapRouteManeuver_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xed5c17f0_a6ab_4d65_a086_fa8a7e340df2);
}
impl ::windows::runtime::RuntimeName for MapRouteManeuver {
    const NAME: &'static str = "Windows.Services.Maps.MapRouteManeuver";
}
impl ::core::convert::From<MapRouteManeuver> for ::windows::runtime::IUnknown {
    fn from(value: MapRouteManeuver) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MapRouteManeuver> for ::windows::runtime::IUnknown {
    fn from(value: &MapRouteManeuver) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MapRouteManeuver {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MapRouteManeuver {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MapRouteManeuver> for ::windows::runtime::IInspectable {
    fn from(value: MapRouteManeuver) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MapRouteManeuver> for ::windows::runtime::IInspectable {
    fn from(value: &MapRouteManeuver) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MapRouteManeuver {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MapRouteManeuver {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MapRouteManeuver {}
unsafe impl ::core::marker::Sync for MapRouteManeuver {}
#[doc = "*Required features: `Services_Maps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MapRouteManeuverKind(pub i32);
impl MapRouteManeuverKind {
    pub const None: MapRouteManeuverKind = MapRouteManeuverKind(0i32);
    pub const Start: MapRouteManeuverKind = MapRouteManeuverKind(1i32);
    pub const Stopover: MapRouteManeuverKind = MapRouteManeuverKind(2i32);
    pub const StopoverResume: MapRouteManeuverKind = MapRouteManeuverKind(3i32);
    pub const End: MapRouteManeuverKind = MapRouteManeuverKind(4i32);
    pub const GoStraight: MapRouteManeuverKind = MapRouteManeuverKind(5i32);
    pub const UTurnLeft: MapRouteManeuverKind = MapRouteManeuverKind(6i32);
    pub const UTurnRight: MapRouteManeuverKind = MapRouteManeuverKind(7i32);
    pub const TurnKeepLeft: MapRouteManeuverKind = MapRouteManeuverKind(8i32);
    pub const TurnKeepRight: MapRouteManeuverKind = MapRouteManeuverKind(9i32);
    pub const TurnLightLeft: MapRouteManeuverKind = MapRouteManeuverKind(10i32);
    pub const TurnLightRight: MapRouteManeuverKind = MapRouteManeuverKind(11i32);
    pub const TurnLeft: MapRouteManeuverKind = MapRouteManeuverKind(12i32);
    pub const TurnRight: MapRouteManeuverKind = MapRouteManeuverKind(13i32);
    pub const TurnHardLeft: MapRouteManeuverKind = MapRouteManeuverKind(14i32);
    pub const TurnHardRight: MapRouteManeuverKind = MapRouteManeuverKind(15i32);
    pub const FreewayEnterLeft: MapRouteManeuverKind = MapRouteManeuverKind(16i32);
    pub const FreewayEnterRight: MapRouteManeuverKind = MapRouteManeuverKind(17i32);
    pub const FreewayLeaveLeft: MapRouteManeuverKind = MapRouteManeuverKind(18i32);
    pub const FreewayLeaveRight: MapRouteManeuverKind = MapRouteManeuverKind(19i32);
    pub const FreewayContinueLeft: MapRouteManeuverKind = MapRouteManeuverKind(20i32);
    pub const FreewayContinueRight: MapRouteManeuverKind = MapRouteManeuverKind(21i32);
    pub const TrafficCircleLeft: MapRouteManeuverKind = MapRouteManeuverKind(22i32);
    pub const TrafficCircleRight: MapRouteManeuverKind = MapRouteManeuverKind(23i32);
    pub const TakeFerry: MapRouteManeuverKind = MapRouteManeuverKind(24i32);
}
impl ::core::convert::From<i32> for MapRouteManeuverKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MapRouteManeuverKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MapRouteManeuverKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.MapRouteManeuverKind;i4)");
}
impl ::windows::runtime::DefaultType for MapRouteManeuverKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `Services_Maps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MapRouteOptimization(pub i32);
impl MapRouteOptimization {
    pub const Time: MapRouteOptimization = MapRouteOptimization(0i32);
    pub const Distance: MapRouteOptimization = MapRouteOptimization(1i32);
    pub const TimeWithTraffic: MapRouteOptimization = MapRouteOptimization(2i32);
    pub const Scenic: MapRouteOptimization = MapRouteOptimization(3i32);
}
impl ::core::convert::From<i32> for MapRouteOptimization {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MapRouteOptimization {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MapRouteOptimization {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.MapRouteOptimization;i4)");
}
impl ::windows::runtime::DefaultType for MapRouteOptimization {
    type DefaultType = Self;
}
#[doc = "*Required features: `Services_Maps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MapRouteRestrictions(pub u32);
impl MapRouteRestrictions {
    pub const None: MapRouteRestrictions = MapRouteRestrictions(0u32);
    pub const Highways: MapRouteRestrictions = MapRouteRestrictions(1u32);
    pub const TollRoads: MapRouteRestrictions = MapRouteRestrictions(2u32);
    pub const Ferries: MapRouteRestrictions = MapRouteRestrictions(4u32);
    pub const Tunnels: MapRouteRestrictions = MapRouteRestrictions(8u32);
    pub const DirtRoads: MapRouteRestrictions = MapRouteRestrictions(16u32);
    pub const Motorail: MapRouteRestrictions = MapRouteRestrictions(32u32);
}
impl ::core::convert::From<u32> for MapRouteRestrictions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MapRouteRestrictions {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MapRouteRestrictions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.MapRouteRestrictions;u4)");
}
impl ::windows::runtime::DefaultType for MapRouteRestrictions {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for MapRouteRestrictions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for MapRouteRestrictions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for MapRouteRestrictions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for MapRouteRestrictions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for MapRouteRestrictions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Services_Maps`*"]
pub struct MapService {}
impl MapService {
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn SetServiceToken<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(value: Param0) -> ::windows::runtime::Result<()> {
        Self::IMapServiceStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn ServiceToken() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMapServiceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn WorldViewRegionCode() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMapServiceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn DataAttributions() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMapServiceStatics3(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn SetDataUsagePreference(value: MapServiceDataUsagePreference) -> ::windows::runtime::Result<()> {
        Self::IMapServiceStatics4(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() })
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn DataUsagePreference() -> ::windows::runtime::Result<MapServiceDataUsagePreference> {
        Self::IMapServiceStatics4(|this| unsafe {
            let mut result__: MapServiceDataUsagePreference = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapServiceDataUsagePreference>(result__)
        })
    }
    pub fn IMapServiceStatics<R, F: FnOnce(&IMapServiceStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MapService, IMapServiceStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMapServiceStatics2<R, F: FnOnce(&IMapServiceStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MapService, IMapServiceStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMapServiceStatics3<R, F: FnOnce(&IMapServiceStatics3) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MapService, IMapServiceStatics3> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMapServiceStatics4<R, F: FnOnce(&IMapServiceStatics4) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MapService, IMapServiceStatics4> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for MapService {
    const NAME: &'static str = "Windows.Services.Maps.MapService";
}
#[doc = "*Required features: `Services_Maps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MapServiceDataUsagePreference(pub i32);
impl MapServiceDataUsagePreference {
    pub const Default: MapServiceDataUsagePreference = MapServiceDataUsagePreference(0i32);
    pub const OfflineMapDataOnly: MapServiceDataUsagePreference = MapServiceDataUsagePreference(1i32);
}
impl ::core::convert::From<i32> for MapServiceDataUsagePreference {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MapServiceDataUsagePreference {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MapServiceDataUsagePreference {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.MapServiceDataUsagePreference;i4)");
}
impl ::windows::runtime::DefaultType for MapServiceDataUsagePreference {
    type DefaultType = Self;
}
#[doc = "*Required features: `Services_Maps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PlaceInfo(pub ::windows::runtime::IInspectable);
impl PlaceInfo {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Maps`, `Foundation`*"]
    pub fn Show<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Rect>>(&self, selection: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), selection.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    #[doc = "*Required features: `Services_Maps`, `Foundation`, `UI_Popups`*"]
    pub fn ShowWithPreferredPlacement<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Rect>>(&self, selection: Param0, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), selection.into_param().abi(), preferredplacement).ok() }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn Identifier(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn DisplayAddress(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Devices_Geolocation")]
    #[doc = "*Required features: `Services_Maps`, `Devices_Geolocation`*"]
    pub fn Geoshape(&self) -> ::windows::runtime::Result<super::super::Devices::Geolocation::IGeoshape> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Geolocation::IGeoshape>(result__)
        }
    }
    #[cfg(feature = "Devices_Geolocation")]
    #[doc = "*Required features: `Services_Maps`, `Devices_Geolocation`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Devices::Geolocation::Geopoint>>(referencepoint: Param0) -> ::windows::runtime::Result<PlaceInfo> {
        Self::IPlaceInfoStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), referencepoint.into_param().abi(), &mut result__).from_abi::<PlaceInfo>(result__)
        })
    }
    #[cfg(feature = "Devices_Geolocation")]
    #[doc = "*Required features: `Services_Maps`, `Devices_Geolocation`*"]
    pub fn CreateWithGeopointAndOptions<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Devices::Geolocation::Geopoint>, Param1: ::windows::runtime::IntoParam<'a, PlaceInfoCreateOptions>>(referencepoint: Param0, options: Param1) -> ::windows::runtime::Result<PlaceInfo> {
        Self::IPlaceInfoStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), referencepoint.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<PlaceInfo>(result__)
        })
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn CreateFromIdentifier<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(identifier: Param0) -> ::windows::runtime::Result<PlaceInfo> {
        Self::IPlaceInfoStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), identifier.into_param().abi(), &mut result__).from_abi::<PlaceInfo>(result__)
        })
    }
    #[cfg(feature = "Devices_Geolocation")]
    #[doc = "*Required features: `Services_Maps`, `Devices_Geolocation`*"]
    pub fn CreateFromIdentifierWithOptions<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Devices::Geolocation::Geopoint>, Param2: ::windows::runtime::IntoParam<'a, PlaceInfoCreateOptions>>(identifier: Param0, defaultpoint: Param1, options: Param2) -> ::windows::runtime::Result<PlaceInfo> {
        Self::IPlaceInfoStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), identifier.into_param().abi(), defaultpoint.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<PlaceInfo>(result__)
        })
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn CreateFromMapLocation<'a, Param0: ::windows::runtime::IntoParam<'a, MapLocation>>(location: Param0) -> ::windows::runtime::Result<PlaceInfo> {
        Self::IPlaceInfoStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), location.into_param().abi(), &mut result__).from_abi::<PlaceInfo>(result__)
        })
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn IsShowSupported() -> ::windows::runtime::Result<bool> {
        Self::IPlaceInfoStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn CreateFromAddress<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(displayaddress: Param0) -> ::windows::runtime::Result<PlaceInfo> {
        Self::IPlaceInfoStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), displayaddress.into_param().abi(), &mut result__).from_abi::<PlaceInfo>(result__)
        })
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn CreateFromAddressWithName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(displayaddress: Param0, displayname: Param1) -> ::windows::runtime::Result<PlaceInfo> {
        Self::IPlaceInfoStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), displayaddress.into_param().abi(), displayname.into_param().abi(), &mut result__).from_abi::<PlaceInfo>(result__)
        })
    }
    pub fn IPlaceInfoStatics<R, F: FnOnce(&IPlaceInfoStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PlaceInfo, IPlaceInfoStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPlaceInfoStatics2<R, F: FnOnce(&IPlaceInfoStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PlaceInfo, IPlaceInfoStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PlaceInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.PlaceInfo;{9a0810b6-31c8-4f6a-9f18-950b4c38951a})");
}
unsafe impl ::windows::runtime::Interface for PlaceInfo {
    type Vtable = IPlaceInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9a0810b6_31c8_4f6a_9f18_950b4c38951a);
}
impl ::windows::runtime::RuntimeName for PlaceInfo {
    const NAME: &'static str = "Windows.Services.Maps.PlaceInfo";
}
impl ::core::convert::From<PlaceInfo> for ::windows::runtime::IUnknown {
    fn from(value: PlaceInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PlaceInfo> for ::windows::runtime::IUnknown {
    fn from(value: &PlaceInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PlaceInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PlaceInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PlaceInfo> for ::windows::runtime::IInspectable {
    fn from(value: PlaceInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PlaceInfo> for ::windows::runtime::IInspectable {
    fn from(value: &PlaceInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PlaceInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PlaceInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PlaceInfo {}
unsafe impl ::core::marker::Sync for PlaceInfo {}
#[doc = "*Required features: `Services_Maps`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PlaceInfoCreateOptions(pub ::windows::runtime::IInspectable);
impl PlaceInfoCreateOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PlaceInfoCreateOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn SetDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn SetDisplayAddress<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Services_Maps`*"]
    pub fn DisplayAddress(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PlaceInfoCreateOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.PlaceInfoCreateOptions;{cd33c125-67f1-4bb3-9907-ecce939b0399})");
}
unsafe impl ::windows::runtime::Interface for PlaceInfoCreateOptions {
    type Vtable = IPlaceInfoCreateOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xcd33c125_67f1_4bb3_9907_ecce939b0399);
}
impl ::windows::runtime::RuntimeName for PlaceInfoCreateOptions {
    const NAME: &'static str = "Windows.Services.Maps.PlaceInfoCreateOptions";
}
impl ::core::convert::From<PlaceInfoCreateOptions> for ::windows::runtime::IUnknown {
    fn from(value: PlaceInfoCreateOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PlaceInfoCreateOptions> for ::windows::runtime::IUnknown {
    fn from(value: &PlaceInfoCreateOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PlaceInfoCreateOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PlaceInfoCreateOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PlaceInfoCreateOptions> for ::windows::runtime::IInspectable {
    fn from(value: PlaceInfoCreateOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PlaceInfoCreateOptions> for ::windows::runtime::IInspectable {
    fn from(value: &PlaceInfoCreateOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PlaceInfoCreateOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PlaceInfoCreateOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PlaceInfoCreateOptions {}
unsafe impl ::core::marker::Sync for PlaceInfoCreateOptions {}
#[doc = "*Required features: `Services_Maps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TrafficCongestion(pub i32);
impl TrafficCongestion {
    pub const Unknown: TrafficCongestion = TrafficCongestion(0i32);
    pub const Light: TrafficCongestion = TrafficCongestion(1i32);
    pub const Mild: TrafficCongestion = TrafficCongestion(2i32);
    pub const Medium: TrafficCongestion = TrafficCongestion(3i32);
    pub const Heavy: TrafficCongestion = TrafficCongestion(4i32);
}
impl ::core::convert::From<i32> for TrafficCongestion {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TrafficCongestion {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TrafficCongestion {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.TrafficCongestion;i4)");
}
impl ::windows::runtime::DefaultType for TrafficCongestion {
    type DefaultType = Self;
}
#[doc = "*Required features: `Services_Maps`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WaypointKind(pub i32);
impl WaypointKind {
    pub const Stop: WaypointKind = WaypointKind(0i32);
    pub const Via: WaypointKind = WaypointKind(1i32);
}
impl ::core::convert::From<i32> for WaypointKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WaypointKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WaypointKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.WaypointKind;i4)");
}
impl ::windows::runtime::DefaultType for WaypointKind {
    type DefaultType = Self;
}
