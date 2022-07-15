#[cfg(feature = "Services_Maps_Guidance")]
pub mod Guidance;
#[cfg(feature = "Services_Maps_LocalSearch")]
pub mod LocalSearch;
#[cfg(feature = "Services_Maps_OfflineMaps")]
pub mod OfflineMaps;
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
pub struct EnhancedWaypoint(::windows::core::IUnknown);
impl EnhancedWaypoint {
    #[doc = "*Required features: `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Point(&self) -> ::windows::core::Result<super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Point)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::Geolocation::Geopoint>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<WaypointKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WaypointKind>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Create<'a, P0>(point: P0, kind: WaypointKind) -> ::windows::core::Result<EnhancedWaypoint>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Devices::Geolocation::Geopoint>>,
    {
        Self::IEnhancedWaypointFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), point.into().abi(), kind, result__.as_mut_ptr()).from_abi::<EnhancedWaypoint>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IEnhancedWaypointFactory<R, F: FnOnce(&IEnhancedWaypointFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<EnhancedWaypoint, IEnhancedWaypointFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for EnhancedWaypoint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EnhancedWaypoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EnhancedWaypoint {}
impl ::core::fmt::Debug for EnhancedWaypoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnhancedWaypoint").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EnhancedWaypoint {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.EnhancedWaypoint;{ed268c74-5913-11e6-8b77-86f30ca893d3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for EnhancedWaypoint {
    type Vtable = IEnhancedWaypoint_Vtbl;
    const IID: ::windows::core::GUID = <IEnhancedWaypoint as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EnhancedWaypoint {
    const NAME: &'static str = "Windows.Services.Maps.EnhancedWaypoint";
}
impl ::core::convert::From<EnhancedWaypoint> for ::windows::core::IUnknown {
    fn from(value: EnhancedWaypoint) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EnhancedWaypoint> for ::windows::core::IUnknown {
    fn from(value: &EnhancedWaypoint) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&EnhancedWaypoint> for &::windows::core::IUnknown {
    fn from(value: &EnhancedWaypoint) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<EnhancedWaypoint> for ::windows::core::IInspectable {
    fn from(value: EnhancedWaypoint) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EnhancedWaypoint> for ::windows::core::IInspectable {
    fn from(value: &EnhancedWaypoint) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&EnhancedWaypoint> for &::windows::core::IInspectable {
    fn from(value: &EnhancedWaypoint) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for EnhancedWaypoint {}
unsafe impl ::core::marker::Sync for EnhancedWaypoint {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEnhancedWaypoint(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEnhancedWaypoint {
    type Vtable = IEnhancedWaypoint_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed268c74_5913_11e6_8b77_86f30ca893d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnhancedWaypoint_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Geolocation")]
    pub Point: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Point: usize,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WaypointKind) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEnhancedWaypointFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEnhancedWaypointFactory {
    type Vtable = IEnhancedWaypointFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf868477_a2aa_46dd_b645_23b31b8aa6c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnhancedWaypointFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Geolocation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, point: *mut ::core::ffi::c_void, kind: WaypointKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManeuverWarning(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IManeuverWarning {
    type Vtable = IManeuverWarning_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1a36d8a_2630_4378_9e4a_6e44253dceba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManeuverWarning_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ManeuverWarningKind) -> ::windows::core::HRESULT,
    pub Severity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ManeuverWarningSeverity) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapAddress(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapAddress {
    type Vtable = IMapAddress_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcfa7a973_a3b4_4494_b3ff_cba94db69699);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapAddress_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub BuildingName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BuildingFloor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BuildingRoom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BuildingWing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub StreetNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Street: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Neighborhood: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub District: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Town: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Region: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RegionCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Country: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CountryCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PostCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Continent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapAddress2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapAddress2 {
    type Vtable = IMapAddress2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75cd6df1_e5ad_45a9_bf40_6cf256c1dd13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapAddress2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub FormattedAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapLocation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapLocation {
    type Vtable = IMapLocation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c073f57_0da4_42e8_9ee2_a96fcf2371dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapLocation_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Geolocation")]
    pub Point: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Point: usize,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapLocationFinderResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapLocationFinderResult {
    type Vtable = IMapLocationFinderResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43f1f179_e8cc_45f6_bed2_54ccbf965d9a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapLocationFinderResult_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Locations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Locations: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MapLocationFinderStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapLocationFinderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapLocationFinderStatics {
    type Vtable = IMapLocationFinderStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x318adb5d_1c5d_4f35_a2df_aaca94959517);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapLocationFinderStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub FindLocationsAtAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, querypoint: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    FindLocationsAtAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub FindLocationsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, searchtext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, referencepoint: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    FindLocationsAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub FindLocationsWithMaxCountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, searchtext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, referencepoint: *mut ::core::ffi::c_void, maxcount: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    FindLocationsWithMaxCountAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapLocationFinderStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapLocationFinderStatics2 {
    type Vtable = IMapLocationFinderStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x959a8b96_6485_4dfd_851a_33ac317e3af6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapLocationFinderStatics2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub FindLocationsAtWithAccuracyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, querypoint: *mut ::core::ffi::c_void, accuracy: MapLocationDesiredAccuracy, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    FindLocationsAtWithAccuracyAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapManagerStatics {
    type Vtable = IMapManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x37e3e515_82b4_4d54_8fd9_af2624b3011c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ShowDownloadedMapsUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ShowMapsUpdateUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapRoute(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapRoute {
    type Vtable = IMapRoute_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb07b732_584d_4583_9c60_641fea274349);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRoute_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Geolocation")]
    pub BoundingBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    BoundingBox: usize,
    pub LengthInMeters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EstimatedDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EstimatedDuration: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Path: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Legs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Legs: usize,
    pub IsTrafficBased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapRoute2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapRoute2 {
    type Vtable = IMapRoute2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1c5d40c_2213_4ab0_a260_46b38169beac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRoute2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ViolatedRestrictions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MapRouteRestrictions) -> ::windows::core::HRESULT,
    pub HasBlockedRoads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapRoute3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapRoute3 {
    type Vtable = IMapRoute3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x858d1eae_f2ad_429f_bb37_cd21094ffc92);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRoute3_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub DurationWithoutTraffic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DurationWithoutTraffic: usize,
    pub TrafficCongestion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TrafficCongestion) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapRoute4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapRoute4 {
    type Vtable = IMapRoute4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x366c8ca5_3053_4fa1_80ff_d475f3ed1e6e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRoute4_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub IsScenic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapRouteDrivingOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapRouteDrivingOptions {
    type Vtable = IMapRouteDrivingOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6815364d_c6dc_4697_a452_b18f8f0b67a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteDrivingOptions_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub MaxAlternateRouteCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetMaxAlternateRouteCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub InitialHeading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InitialHeading: usize,
    #[cfg(feature = "Foundation")]
    pub SetInitialHeading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInitialHeading: usize,
    pub RouteOptimization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MapRouteOptimization) -> ::windows::core::HRESULT,
    pub SetRouteOptimization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MapRouteOptimization) -> ::windows::core::HRESULT,
    pub RouteRestrictions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MapRouteRestrictions) -> ::windows::core::HRESULT,
    pub SetRouteRestrictions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MapRouteRestrictions) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapRouteDrivingOptions2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapRouteDrivingOptions2 {
    type Vtable = IMapRouteDrivingOptions2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35dc8670_c298_48d0_b5ad_825460645603);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteDrivingOptions2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub DepartureTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DepartureTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetDepartureTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDepartureTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapRouteFinderResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapRouteFinderResult {
    type Vtable = IMapRouteFinderResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa868a31a_9422_46ac_8ca1_b1614d4bfbe2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteFinderResult_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Route: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MapRouteFinderStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapRouteFinderResult2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapRouteFinderResult2 {
    type Vtable = IMapRouteFinderResult2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20709c6d_d90c_46c8_91c6_7d4be4efb215);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteFinderResult2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AlternateRoutes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AlternateRoutes: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapRouteFinderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapRouteFinderStatics {
    type Vtable = IMapRouteFinderStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8a5c50f_1c64_4c3a_81eb_1f7c152afbbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteFinderStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub GetDrivingRouteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *mut ::core::ffi::c_void, endpoint: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    GetDrivingRouteAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub GetDrivingRouteWithOptimizationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *mut ::core::ffi::c_void, endpoint: *mut ::core::ffi::c_void, optimization: MapRouteOptimization, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    GetDrivingRouteWithOptimizationAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub GetDrivingRouteWithOptimizationAndRestrictionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *mut ::core::ffi::c_void, endpoint: *mut ::core::ffi::c_void, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    GetDrivingRouteWithOptimizationAndRestrictionsAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub GetDrivingRouteWithOptimizationRestrictionsAndHeadingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *mut ::core::ffi::c_void, endpoint: *mut ::core::ffi::c_void, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, headingindegrees: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    GetDrivingRouteWithOptimizationRestrictionsAndHeadingAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections"))]
    pub GetDrivingRouteFromWaypointsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waypoints: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation_Collections")))]
    GetDrivingRouteFromWaypointsAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections"))]
    pub GetDrivingRouteFromWaypointsAndOptimizationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waypoints: *mut ::core::ffi::c_void, optimization: MapRouteOptimization, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation_Collections")))]
    GetDrivingRouteFromWaypointsAndOptimizationAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections"))]
    pub GetDrivingRouteFromWaypointsOptimizationAndRestrictionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waypoints: *mut ::core::ffi::c_void, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation_Collections")))]
    GetDrivingRouteFromWaypointsOptimizationAndRestrictionsAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections"))]
    pub GetDrivingRouteFromWaypointsOptimizationRestrictionsAndHeadingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waypoints: *mut ::core::ffi::c_void, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, headingindegrees: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation_Collections")))]
    GetDrivingRouteFromWaypointsOptimizationRestrictionsAndHeadingAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub GetWalkingRouteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *mut ::core::ffi::c_void, endpoint: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    GetWalkingRouteAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections"))]
    pub GetWalkingRouteFromWaypointsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waypoints: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation_Collections")))]
    GetWalkingRouteFromWaypointsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapRouteFinderStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapRouteFinderStatics2 {
    type Vtable = IMapRouteFinderStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xafcc2c73_7760_49af_b3bd_baf135b703e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteFinderStatics2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub GetDrivingRouteWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *mut ::core::ffi::c_void, endpoint: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    GetDrivingRouteWithOptionsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapRouteFinderStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapRouteFinderStatics3 {
    type Vtable = IMapRouteFinderStatics3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6098134_5913_11e6_8b77_86f30ca893d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteFinderStatics3_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetDrivingRouteFromEnhancedWaypointsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waypoints: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetDrivingRouteFromEnhancedWaypointsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetDrivingRouteFromEnhancedWaypointsWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waypoints: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetDrivingRouteFromEnhancedWaypointsWithOptionsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapRouteLeg(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapRouteLeg {
    type Vtable = IMapRouteLeg_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96f8b2f6_5bba_4d17_9db6_1a263fec7471);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteLeg_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Geolocation")]
    pub BoundingBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    BoundingBox: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Path: usize,
    pub LengthInMeters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EstimatedDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EstimatedDuration: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Maneuvers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Maneuvers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapRouteLeg2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapRouteLeg2 {
    type Vtable = IMapRouteLeg2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x02e2062d_c9c6_45b8_8e54_1a10b57a17e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteLeg2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub DurationWithoutTraffic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DurationWithoutTraffic: usize,
    pub TrafficCongestion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TrafficCongestion) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapRouteManeuver(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapRouteManeuver {
    type Vtable = IMapRouteManeuver_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed5c17f0_a6ab_4d65_a086_fa8a7e340df2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteManeuver_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Geolocation")]
    pub StartingPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    StartingPoint: usize,
    pub LengthInMeters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub InstructionText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MapRouteManeuverKind) -> ::windows::core::HRESULT,
    pub ExitNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ManeuverNotices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MapManeuverNotices) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapRouteManeuver2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapRouteManeuver2 {
    type Vtable = IMapRouteManeuver2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d7bcd9c_7c9b_41df_838b_eae21e4b05a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteManeuver2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub StartHeading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub EndHeading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub StreetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapRouteManeuver3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapRouteManeuver3 {
    type Vtable = IMapRouteManeuver3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6a138df_0483_4166_85be_b99336c11875);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteManeuver3_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Warnings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Warnings: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapServiceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapServiceStatics {
    type Vtable = IMapServiceStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0144ad85_c04c_4cdd_871a_a0726d097cd4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapServiceStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub SetServiceToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ServiceToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapServiceStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapServiceStatics2 {
    type Vtable = IMapServiceStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8193eed_9c85_40a9_8896_0fc3fd2b7c2a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapServiceStatics2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub WorldViewRegionCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapServiceStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapServiceStatics3 {
    type Vtable = IMapServiceStatics3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a11ce20_63a7_4854_b355_d6dcda223d1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapServiceStatics3_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub DataAttributions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapServiceStatics4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapServiceStatics4 {
    type Vtable = IMapServiceStatics4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x088a2862_6abc_420e_945f_4cfd89c67356);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapServiceStatics4_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub SetDataUsagePreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MapServiceDataUsagePreference) -> ::windows::core::HRESULT,
    pub DataUsagePreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MapServiceDataUsagePreference) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaceInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPlaceInfo {
    type Vtable = IPlaceInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a0810b6_31c8_4f6a_9f18_950b4c38951a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaceInfo_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Show: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowWithPreferredPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowWithPreferredPlacement: usize,
    pub Identifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisplayAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Geolocation")]
    pub Geoshape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Geoshape: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaceInfoCreateOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPlaceInfoCreateOptions {
    type Vtable = IPlaceInfoCreateOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd33c125_67f1_4bb3_9907_ecce939b0399);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaceInfoCreateOptions_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDisplayAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisplayAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaceInfoStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPlaceInfoStatics {
    type Vtable = IPlaceInfoStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82b9ff71_6cd0_48a4_afd9_5ed82097936b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaceInfoStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Geolocation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, referencepoint: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Create: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub CreateWithGeopointAndOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, referencepoint: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    CreateWithGeopointAndOptions: usize,
    pub CreateFromIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identifier: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Geolocation")]
    pub CreateFromIdentifierWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identifier: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, defaultpoint: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    CreateFromIdentifierWithOptions: usize,
    pub CreateFromMapLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, location: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsShowSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaceInfoStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPlaceInfoStatics2 {
    type Vtable = IPlaceInfoStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x730f0249_4047_44a3_8f81_2550a5216370);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaceInfoStatics2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreateFromAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayaddress: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateFromAddressWithName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayaddress: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
pub struct ManeuverWarning(::windows::core::IUnknown);
impl ManeuverWarning {
    pub fn Kind(&self) -> ::windows::core::Result<ManeuverWarningKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ManeuverWarningKind>(result__)
        }
    }
    pub fn Severity(&self) -> ::windows::core::Result<ManeuverWarningSeverity> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Severity)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ManeuverWarningSeverity>(result__)
        }
    }
}
impl ::core::clone::Clone for ManeuverWarning {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManeuverWarning {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManeuverWarning {}
impl ::core::fmt::Debug for ManeuverWarning {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManeuverWarning").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ManeuverWarning {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.ManeuverWarning;{c1a36d8a-2630-4378-9e4a-6e44253dceba})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ManeuverWarning {
    type Vtable = IManeuverWarning_Vtbl;
    const IID: ::windows::core::GUID = <IManeuverWarning as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ManeuverWarning {
    const NAME: &'static str = "Windows.Services.Maps.ManeuverWarning";
}
impl ::core::convert::From<ManeuverWarning> for ::windows::core::IUnknown {
    fn from(value: ManeuverWarning) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManeuverWarning> for ::windows::core::IUnknown {
    fn from(value: &ManeuverWarning) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ManeuverWarning> for &::windows::core::IUnknown {
    fn from(value: &ManeuverWarning) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<ManeuverWarning> for ::windows::core::IInspectable {
    fn from(value: ManeuverWarning) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManeuverWarning> for ::windows::core::IInspectable {
    fn from(value: &ManeuverWarning) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ManeuverWarning> for &::windows::core::IInspectable {
    fn from(value: &ManeuverWarning) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for ManeuverWarning {}
unsafe impl ::core::marker::Sync for ManeuverWarning {}
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ManeuverWarningKind(pub i32);
impl ManeuverWarningKind {
    pub const None: Self = Self(0i32);
    pub const Accident: Self = Self(1i32);
    pub const AdministrativeDivisionChange: Self = Self(2i32);
    pub const Alert: Self = Self(3i32);
    pub const BlockedRoad: Self = Self(4i32);
    pub const CheckTimetable: Self = Self(5i32);
    pub const Congestion: Self = Self(6i32);
    pub const Construction: Self = Self(7i32);
    pub const CountryChange: Self = Self(8i32);
    pub const DisabledVehicle: Self = Self(9i32);
    pub const GateAccess: Self = Self(10i32);
    pub const GetOffTransit: Self = Self(11i32);
    pub const GetOnTransit: Self = Self(12i32);
    pub const IllegalUTurn: Self = Self(13i32);
    pub const MassTransit: Self = Self(14i32);
    pub const Miscellaneous: Self = Self(15i32);
    pub const NoIncident: Self = Self(16i32);
    pub const Other: Self = Self(17i32);
    pub const OtherNews: Self = Self(18i32);
    pub const OtherTrafficIncidents: Self = Self(19i32);
    pub const PlannedEvent: Self = Self(20i32);
    pub const PrivateRoad: Self = Self(21i32);
    pub const RestrictedTurn: Self = Self(22i32);
    pub const RoadClosures: Self = Self(23i32);
    pub const RoadHazard: Self = Self(24i32);
    pub const ScheduledConstruction: Self = Self(25i32);
    pub const SeasonalClosures: Self = Self(26i32);
    pub const Tollbooth: Self = Self(27i32);
    pub const TollRoad: Self = Self(28i32);
    pub const TollZoneEnter: Self = Self(29i32);
    pub const TollZoneExit: Self = Self(30i32);
    pub const TrafficFlow: Self = Self(31i32);
    pub const TransitLineChange: Self = Self(32i32);
    pub const UnpavedRoad: Self = Self(33i32);
    pub const UnscheduledConstruction: Self = Self(34i32);
    pub const Weather: Self = Self(35i32);
}
impl ::core::marker::Copy for ManeuverWarningKind {}
impl ::core::clone::Clone for ManeuverWarningKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ManeuverWarningKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ManeuverWarningKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ManeuverWarningKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManeuverWarningKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ManeuverWarningKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.ManeuverWarningKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ManeuverWarningSeverity(pub i32);
impl ManeuverWarningSeverity {
    pub const None: Self = Self(0i32);
    pub const LowImpact: Self = Self(1i32);
    pub const Minor: Self = Self(2i32);
    pub const Moderate: Self = Self(3i32);
    pub const Serious: Self = Self(4i32);
}
impl ::core::marker::Copy for ManeuverWarningSeverity {}
impl ::core::clone::Clone for ManeuverWarningSeverity {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ManeuverWarningSeverity {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ManeuverWarningSeverity {
    type Abi = Self;
}
impl ::core::fmt::Debug for ManeuverWarningSeverity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManeuverWarningSeverity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ManeuverWarningSeverity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.ManeuverWarningSeverity;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
pub struct MapAddress(::windows::core::IUnknown);
impl MapAddress {
    pub fn BuildingName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BuildingName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn BuildingFloor(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BuildingFloor)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn BuildingRoom(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BuildingRoom)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn BuildingWing(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BuildingWing)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn StreetNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).StreetNumber)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Street(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Street)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Neighborhood(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Neighborhood)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn District(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).District)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Town(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Town)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Region(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Region)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn RegionCode(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RegionCode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Country(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Country)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn CountryCode(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CountryCode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn PostCode(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PostCode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Continent(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Continent)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FormattedAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMapAddress2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FormattedAddress)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for MapAddress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapAddress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapAddress {}
impl ::core::fmt::Debug for MapAddress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapAddress").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapAddress {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.MapAddress;{cfa7a973-a3b4-4494-b3ff-cba94db69699})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapAddress {
    type Vtable = IMapAddress_Vtbl;
    const IID: ::windows::core::GUID = <IMapAddress as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapAddress {
    const NAME: &'static str = "Windows.Services.Maps.MapAddress";
}
impl ::core::convert::From<MapAddress> for ::windows::core::IUnknown {
    fn from(value: MapAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapAddress> for ::windows::core::IUnknown {
    fn from(value: &MapAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&MapAddress> for &::windows::core::IUnknown {
    fn from(value: &MapAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<MapAddress> for ::windows::core::IInspectable {
    fn from(value: MapAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapAddress> for ::windows::core::IInspectable {
    fn from(value: &MapAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&MapAddress> for &::windows::core::IInspectable {
    fn from(value: &MapAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for MapAddress {}
unsafe impl ::core::marker::Sync for MapAddress {}
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
pub struct MapLocation(::windows::core::IUnknown);
impl MapLocation {
    #[doc = "*Required features: `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Point(&self) -> ::windows::core::Result<super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Point)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::Geolocation::Geopoint>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Address(&self) -> ::windows::core::Result<MapAddress> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Address)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MapAddress>(result__)
        }
    }
}
impl ::core::clone::Clone for MapLocation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapLocation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapLocation {}
impl ::core::fmt::Debug for MapLocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapLocation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapLocation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.MapLocation;{3c073f57-0da4-42e8-9ee2-a96fcf2371dc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapLocation {
    type Vtable = IMapLocation_Vtbl;
    const IID: ::windows::core::GUID = <IMapLocation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapLocation {
    const NAME: &'static str = "Windows.Services.Maps.MapLocation";
}
impl ::core::convert::From<MapLocation> for ::windows::core::IUnknown {
    fn from(value: MapLocation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapLocation> for ::windows::core::IUnknown {
    fn from(value: &MapLocation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&MapLocation> for &::windows::core::IUnknown {
    fn from(value: &MapLocation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<MapLocation> for ::windows::core::IInspectable {
    fn from(value: MapLocation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapLocation> for ::windows::core::IInspectable {
    fn from(value: &MapLocation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&MapLocation> for &::windows::core::IInspectable {
    fn from(value: &MapLocation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for MapLocation {}
unsafe impl ::core::marker::Sync for MapLocation {}
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MapLocationDesiredAccuracy(pub i32);
impl MapLocationDesiredAccuracy {
    pub const High: Self = Self(0i32);
    pub const Low: Self = Self(1i32);
}
impl ::core::marker::Copy for MapLocationDesiredAccuracy {}
impl ::core::clone::Clone for MapLocationDesiredAccuracy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MapLocationDesiredAccuracy {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MapLocationDesiredAccuracy {
    type Abi = Self;
}
impl ::core::fmt::Debug for MapLocationDesiredAccuracy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapLocationDesiredAccuracy").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapLocationDesiredAccuracy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.MapLocationDesiredAccuracy;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Services_Maps\"`*"]
pub struct MapLocationFinder;
impl MapLocationFinder {
    #[doc = "*Required features: `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn FindLocationsAtAsync<'a, P0>(querypoint: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapLocationFinderResult>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Devices::Geolocation::Geopoint>>,
    {
        Self::IMapLocationFinderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FindLocationsAtAsync)(::windows::core::Interface::as_raw(this), querypoint.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<MapLocationFinderResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn FindLocationsAsync<'a, P0>(searchtext: &::windows::core::HSTRING, referencepoint: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapLocationFinderResult>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Devices::Geolocation::Geopoint>>,
    {
        Self::IMapLocationFinderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FindLocationsAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(searchtext), referencepoint.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<MapLocationFinderResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn FindLocationsWithMaxCountAsync<'a, P0>(searchtext: &::windows::core::HSTRING, referencepoint: P0, maxcount: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapLocationFinderResult>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Devices::Geolocation::Geopoint>>,
    {
        Self::IMapLocationFinderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FindLocationsWithMaxCountAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(searchtext), referencepoint.into().abi(), maxcount, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<MapLocationFinderResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn FindLocationsAtWithAccuracyAsync<'a, P0>(querypoint: P0, accuracy: MapLocationDesiredAccuracy) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapLocationFinderResult>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Devices::Geolocation::Geopoint>>,
    {
        Self::IMapLocationFinderStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FindLocationsAtWithAccuracyAsync)(::windows::core::Interface::as_raw(this), querypoint.into().abi(), accuracy, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<MapLocationFinderResult>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMapLocationFinderStatics<R, F: FnOnce(&IMapLocationFinderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MapLocationFinder, IMapLocationFinderStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMapLocationFinderStatics2<R, F: FnOnce(&IMapLocationFinderStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MapLocationFinder, IMapLocationFinderStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for MapLocationFinder {
    const NAME: &'static str = "Windows.Services.Maps.MapLocationFinder";
}
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
pub struct MapLocationFinderResult(::windows::core::IUnknown);
impl MapLocationFinderResult {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Locations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MapLocation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Locations)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<MapLocation>>(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<MapLocationFinderStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MapLocationFinderStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for MapLocationFinderResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapLocationFinderResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapLocationFinderResult {}
impl ::core::fmt::Debug for MapLocationFinderResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapLocationFinderResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapLocationFinderResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.MapLocationFinderResult;{43f1f179-e8cc-45f6-bed2-54ccbf965d9a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapLocationFinderResult {
    type Vtable = IMapLocationFinderResult_Vtbl;
    const IID: ::windows::core::GUID = <IMapLocationFinderResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapLocationFinderResult {
    const NAME: &'static str = "Windows.Services.Maps.MapLocationFinderResult";
}
impl ::core::convert::From<MapLocationFinderResult> for ::windows::core::IUnknown {
    fn from(value: MapLocationFinderResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapLocationFinderResult> for ::windows::core::IUnknown {
    fn from(value: &MapLocationFinderResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&MapLocationFinderResult> for &::windows::core::IUnknown {
    fn from(value: &MapLocationFinderResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<MapLocationFinderResult> for ::windows::core::IInspectable {
    fn from(value: MapLocationFinderResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapLocationFinderResult> for ::windows::core::IInspectable {
    fn from(value: &MapLocationFinderResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&MapLocationFinderResult> for &::windows::core::IInspectable {
    fn from(value: &MapLocationFinderResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for MapLocationFinderResult {}
unsafe impl ::core::marker::Sync for MapLocationFinderResult {}
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MapLocationFinderStatus(pub i32);
impl MapLocationFinderStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const InvalidCredentials: Self = Self(2i32);
    pub const BadLocation: Self = Self(3i32);
    pub const IndexFailure: Self = Self(4i32);
    pub const NetworkFailure: Self = Self(5i32);
    pub const NotSupported: Self = Self(6i32);
}
impl ::core::marker::Copy for MapLocationFinderStatus {}
impl ::core::clone::Clone for MapLocationFinderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MapLocationFinderStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MapLocationFinderStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for MapLocationFinderStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapLocationFinderStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapLocationFinderStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.MapLocationFinderStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Services_Maps\"`*"]
pub struct MapManager;
impl MapManager {
    pub fn ShowDownloadedMapsUI() -> ::windows::core::Result<()> {
        Self::IMapManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).ShowDownloadedMapsUI)(::windows::core::Interface::as_raw(this)).ok() })
    }
    pub fn ShowMapsUpdateUI() -> ::windows::core::Result<()> {
        Self::IMapManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).ShowMapsUpdateUI)(::windows::core::Interface::as_raw(this)).ok() })
    }
    #[doc(hidden)]
    pub fn IMapManagerStatics<R, F: FnOnce(&IMapManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MapManager, IMapManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for MapManager {
    const NAME: &'static str = "Windows.Services.Maps.MapManager";
}
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MapManeuverNotices(pub u32);
impl MapManeuverNotices {
    pub const None: Self = Self(0u32);
    pub const Toll: Self = Self(1u32);
    pub const Unpaved: Self = Self(2u32);
}
impl ::core::marker::Copy for MapManeuverNotices {}
impl ::core::clone::Clone for MapManeuverNotices {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MapManeuverNotices {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MapManeuverNotices {
    type Abi = Self;
}
impl ::core::fmt::Debug for MapManeuverNotices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapManeuverNotices").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MapManeuverNotices {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MapManeuverNotices {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MapManeuverNotices {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MapManeuverNotices {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MapManeuverNotices {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for MapManeuverNotices {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.MapManeuverNotices;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
pub struct MapRoute(::windows::core::IUnknown);
impl MapRoute {
    #[doc = "*Required features: `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn BoundingBox(&self) -> ::windows::core::Result<super::super::Devices::Geolocation::GeoboundingBox> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BoundingBox)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::Geolocation::GeoboundingBox>(result__)
        }
    }
    pub fn LengthInMeters(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LengthInMeters)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EstimatedDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).EstimatedDuration)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Path(&self) -> ::windows::core::Result<super::super::Devices::Geolocation::Geopath> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Path)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::Geolocation::Geopath>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Legs(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MapRouteLeg>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Legs)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<MapRouteLeg>>(result__)
        }
    }
    pub fn IsTrafficBased(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsTrafficBased)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ViolatedRestrictions(&self) -> ::windows::core::Result<MapRouteRestrictions> {
        let this = &::windows::core::Interface::cast::<IMapRoute2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ViolatedRestrictions)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MapRouteRestrictions>(result__)
        }
    }
    pub fn HasBlockedRoads(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMapRoute2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HasBlockedRoads)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DurationWithoutTraffic(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMapRoute3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DurationWithoutTraffic)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    pub fn TrafficCongestion(&self) -> ::windows::core::Result<TrafficCongestion> {
        let this = &::windows::core::Interface::cast::<IMapRoute3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TrafficCongestion)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TrafficCongestion>(result__)
        }
    }
    pub fn IsScenic(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMapRoute4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsScenic)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for MapRoute {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapRoute {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapRoute {}
impl ::core::fmt::Debug for MapRoute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapRoute").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapRoute {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.MapRoute;{fb07b732-584d-4583-9c60-641fea274349})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapRoute {
    type Vtable = IMapRoute_Vtbl;
    const IID: ::windows::core::GUID = <IMapRoute as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapRoute {
    const NAME: &'static str = "Windows.Services.Maps.MapRoute";
}
impl ::core::convert::From<MapRoute> for ::windows::core::IUnknown {
    fn from(value: MapRoute) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapRoute> for ::windows::core::IUnknown {
    fn from(value: &MapRoute) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&MapRoute> for &::windows::core::IUnknown {
    fn from(value: &MapRoute) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<MapRoute> for ::windows::core::IInspectable {
    fn from(value: MapRoute) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapRoute> for ::windows::core::IInspectable {
    fn from(value: &MapRoute) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&MapRoute> for &::windows::core::IInspectable {
    fn from(value: &MapRoute) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for MapRoute {}
unsafe impl ::core::marker::Sync for MapRoute {}
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
pub struct MapRouteDrivingOptions(::windows::core::IUnknown);
impl MapRouteDrivingOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MapRouteDrivingOptions, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn MaxAlternateRouteCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MaxAlternateRouteCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetMaxAlternateRouteCount(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMaxAlternateRouteCount)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InitialHeading(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InitialHeading)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetInitialHeading<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::IReference<f64>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInitialHeading)(::windows::core::Interface::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn RouteOptimization(&self) -> ::windows::core::Result<MapRouteOptimization> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RouteOptimization)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MapRouteOptimization>(result__)
        }
    }
    pub fn SetRouteOptimization(&self, value: MapRouteOptimization) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRouteOptimization)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn RouteRestrictions(&self) -> ::windows::core::Result<MapRouteRestrictions> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RouteRestrictions)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MapRouteRestrictions>(result__)
        }
    }
    pub fn SetRouteRestrictions(&self, value: MapRouteRestrictions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRouteRestrictions)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DepartureTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = &::windows::core::Interface::cast::<IMapRouteDrivingOptions2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DepartureTime)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDepartureTime<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IMapRouteDrivingOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDepartureTime)(::windows::core::Interface::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
}
impl ::core::clone::Clone for MapRouteDrivingOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapRouteDrivingOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapRouteDrivingOptions {}
impl ::core::fmt::Debug for MapRouteDrivingOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapRouteDrivingOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapRouteDrivingOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.MapRouteDrivingOptions;{6815364d-c6dc-4697-a452-b18f8f0b67a1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapRouteDrivingOptions {
    type Vtable = IMapRouteDrivingOptions_Vtbl;
    const IID: ::windows::core::GUID = <IMapRouteDrivingOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapRouteDrivingOptions {
    const NAME: &'static str = "Windows.Services.Maps.MapRouteDrivingOptions";
}
impl ::core::convert::From<MapRouteDrivingOptions> for ::windows::core::IUnknown {
    fn from(value: MapRouteDrivingOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapRouteDrivingOptions> for ::windows::core::IUnknown {
    fn from(value: &MapRouteDrivingOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&MapRouteDrivingOptions> for &::windows::core::IUnknown {
    fn from(value: &MapRouteDrivingOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<MapRouteDrivingOptions> for ::windows::core::IInspectable {
    fn from(value: MapRouteDrivingOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapRouteDrivingOptions> for ::windows::core::IInspectable {
    fn from(value: &MapRouteDrivingOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&MapRouteDrivingOptions> for &::windows::core::IInspectable {
    fn from(value: &MapRouteDrivingOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for MapRouteDrivingOptions {}
unsafe impl ::core::marker::Sync for MapRouteDrivingOptions {}
#[doc = "*Required features: `\"Services_Maps\"`*"]
pub struct MapRouteFinder;
impl MapRouteFinder {
    #[doc = "*Required features: `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn GetDrivingRouteAsync<'a, P0, P1>(startpoint: P0, endpoint: P1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Devices::Geolocation::Geopoint>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Devices::Geolocation::Geopoint>>,
    {
        Self::IMapRouteFinderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDrivingRouteAsync)(::windows::core::Interface::as_raw(this), startpoint.into().abi(), endpoint.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn GetDrivingRouteWithOptimizationAsync<'a, P0, P1>(startpoint: P0, endpoint: P1, optimization: MapRouteOptimization) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Devices::Geolocation::Geopoint>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Devices::Geolocation::Geopoint>>,
    {
        Self::IMapRouteFinderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDrivingRouteWithOptimizationAsync)(::windows::core::Interface::as_raw(this), startpoint.into().abi(), endpoint.into().abi(), optimization, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn GetDrivingRouteWithOptimizationAndRestrictionsAsync<'a, P0, P1>(startpoint: P0, endpoint: P1, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Devices::Geolocation::Geopoint>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Devices::Geolocation::Geopoint>>,
    {
        Self::IMapRouteFinderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDrivingRouteWithOptimizationAndRestrictionsAsync)(::windows::core::Interface::as_raw(this), startpoint.into().abi(), endpoint.into().abi(), optimization, restrictions, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn GetDrivingRouteWithOptimizationRestrictionsAndHeadingAsync<'a, P0, P1>(startpoint: P0, endpoint: P1, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, headingindegrees: f64) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Devices::Geolocation::Geopoint>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Devices::Geolocation::Geopoint>>,
    {
        Self::IMapRouteFinderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDrivingRouteWithOptimizationRestrictionsAndHeadingAsync)(::windows::core::Interface::as_raw(this), startpoint.into().abi(), endpoint.into().abi(), optimization, restrictions, headingindegrees, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections"))]
    pub fn GetDrivingRouteFromWaypointsAsync<'a, P0, E0>(waypoints: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IMapRouteFinderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDrivingRouteFromWaypointsAsync)(::windows::core::Interface::as_raw(this), waypoints.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections"))]
    pub fn GetDrivingRouteFromWaypointsAndOptimizationAsync<'a, P0, E0>(waypoints: P0, optimization: MapRouteOptimization) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IMapRouteFinderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDrivingRouteFromWaypointsAndOptimizationAsync)(::windows::core::Interface::as_raw(this), waypoints.try_into().map_err(|e| e.into())?.abi(), optimization, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections"))]
    pub fn GetDrivingRouteFromWaypointsOptimizationAndRestrictionsAsync<'a, P0, E0>(waypoints: P0, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IMapRouteFinderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDrivingRouteFromWaypointsOptimizationAndRestrictionsAsync)(::windows::core::Interface::as_raw(this), waypoints.try_into().map_err(|e| e.into())?.abi(), optimization, restrictions, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections"))]
    pub fn GetDrivingRouteFromWaypointsOptimizationRestrictionsAndHeadingAsync<'a, P0, E0>(waypoints: P0, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, headingindegrees: f64) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IMapRouteFinderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDrivingRouteFromWaypointsOptimizationRestrictionsAndHeadingAsync)(::windows::core::Interface::as_raw(this), waypoints.try_into().map_err(|e| e.into())?.abi(), optimization, restrictions, headingindegrees, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn GetWalkingRouteAsync<'a, P0, P1>(startpoint: P0, endpoint: P1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Devices::Geolocation::Geopoint>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Devices::Geolocation::Geopoint>>,
    {
        Self::IMapRouteFinderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetWalkingRouteAsync)(::windows::core::Interface::as_raw(this), startpoint.into().abi(), endpoint.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections"))]
    pub fn GetWalkingRouteFromWaypointsAsync<'a, P0, E0>(waypoints: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IMapRouteFinderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetWalkingRouteFromWaypointsAsync)(::windows::core::Interface::as_raw(this), waypoints.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn GetDrivingRouteWithOptionsAsync<'a, P0, P1, P2>(startpoint: P0, endpoint: P1, options: P2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Devices::Geolocation::Geopoint>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Devices::Geolocation::Geopoint>>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, MapRouteDrivingOptions>>,
    {
        Self::IMapRouteFinderStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDrivingRouteWithOptionsAsync)(::windows::core::Interface::as_raw(this), startpoint.into().abi(), endpoint.into().abi(), options.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetDrivingRouteFromEnhancedWaypointsAsync<'a, P0, E0>(waypoints: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IIterable<EnhancedWaypoint>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IMapRouteFinderStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDrivingRouteFromEnhancedWaypointsAsync)(::windows::core::Interface::as_raw(this), waypoints.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetDrivingRouteFromEnhancedWaypointsWithOptionsAsync<'a, P0, E0, P1>(waypoints: P0, options: P1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IIterable<EnhancedWaypoint>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, MapRouteDrivingOptions>>,
    {
        Self::IMapRouteFinderStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDrivingRouteFromEnhancedWaypointsWithOptionsAsync)(::windows::core::Interface::as_raw(this), waypoints.try_into().map_err(|e| e.into())?.abi(), options.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMapRouteFinderStatics<R, F: FnOnce(&IMapRouteFinderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MapRouteFinder, IMapRouteFinderStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMapRouteFinderStatics2<R, F: FnOnce(&IMapRouteFinderStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MapRouteFinder, IMapRouteFinderStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMapRouteFinderStatics3<R, F: FnOnce(&IMapRouteFinderStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MapRouteFinder, IMapRouteFinderStatics3> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for MapRouteFinder {
    const NAME: &'static str = "Windows.Services.Maps.MapRouteFinder";
}
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
pub struct MapRouteFinderResult(::windows::core::IUnknown);
impl MapRouteFinderResult {
    pub fn Route(&self) -> ::windows::core::Result<MapRoute> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Route)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MapRoute>(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<MapRouteFinderStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MapRouteFinderStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AlternateRoutes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MapRoute>> {
        let this = &::windows::core::Interface::cast::<IMapRouteFinderResult2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AlternateRoutes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<MapRoute>>(result__)
        }
    }
}
impl ::core::clone::Clone for MapRouteFinderResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapRouteFinderResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapRouteFinderResult {}
impl ::core::fmt::Debug for MapRouteFinderResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapRouteFinderResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapRouteFinderResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.MapRouteFinderResult;{a868a31a-9422-46ac-8ca1-b1614d4bfbe2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapRouteFinderResult {
    type Vtable = IMapRouteFinderResult_Vtbl;
    const IID: ::windows::core::GUID = <IMapRouteFinderResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapRouteFinderResult {
    const NAME: &'static str = "Windows.Services.Maps.MapRouteFinderResult";
}
impl ::core::convert::From<MapRouteFinderResult> for ::windows::core::IUnknown {
    fn from(value: MapRouteFinderResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapRouteFinderResult> for ::windows::core::IUnknown {
    fn from(value: &MapRouteFinderResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&MapRouteFinderResult> for &::windows::core::IUnknown {
    fn from(value: &MapRouteFinderResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<MapRouteFinderResult> for ::windows::core::IInspectable {
    fn from(value: MapRouteFinderResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapRouteFinderResult> for ::windows::core::IInspectable {
    fn from(value: &MapRouteFinderResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&MapRouteFinderResult> for &::windows::core::IInspectable {
    fn from(value: &MapRouteFinderResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for MapRouteFinderResult {}
unsafe impl ::core::marker::Sync for MapRouteFinderResult {}
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MapRouteFinderStatus(pub i32);
impl MapRouteFinderStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const InvalidCredentials: Self = Self(2i32);
    pub const NoRouteFound: Self = Self(3i32);
    pub const NoRouteFoundWithGivenOptions: Self = Self(4i32);
    pub const StartPointNotFound: Self = Self(5i32);
    pub const EndPointNotFound: Self = Self(6i32);
    pub const NoPedestrianRouteFound: Self = Self(7i32);
    pub const NetworkFailure: Self = Self(8i32);
    pub const NotSupported: Self = Self(9i32);
}
impl ::core::marker::Copy for MapRouteFinderStatus {}
impl ::core::clone::Clone for MapRouteFinderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MapRouteFinderStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MapRouteFinderStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for MapRouteFinderStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapRouteFinderStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapRouteFinderStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.MapRouteFinderStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
pub struct MapRouteLeg(::windows::core::IUnknown);
impl MapRouteLeg {
    #[doc = "*Required features: `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn BoundingBox(&self) -> ::windows::core::Result<super::super::Devices::Geolocation::GeoboundingBox> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BoundingBox)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::Geolocation::GeoboundingBox>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Path(&self) -> ::windows::core::Result<super::super::Devices::Geolocation::Geopath> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Path)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::Geolocation::Geopath>(result__)
        }
    }
    pub fn LengthInMeters(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LengthInMeters)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EstimatedDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).EstimatedDuration)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Maneuvers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MapRouteManeuver>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Maneuvers)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<MapRouteManeuver>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DurationWithoutTraffic(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMapRouteLeg2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DurationWithoutTraffic)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    pub fn TrafficCongestion(&self) -> ::windows::core::Result<TrafficCongestion> {
        let this = &::windows::core::Interface::cast::<IMapRouteLeg2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TrafficCongestion)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TrafficCongestion>(result__)
        }
    }
}
impl ::core::clone::Clone for MapRouteLeg {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapRouteLeg {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapRouteLeg {}
impl ::core::fmt::Debug for MapRouteLeg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapRouteLeg").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapRouteLeg {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.MapRouteLeg;{96f8b2f6-5bba-4d17-9db6-1a263fec7471})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapRouteLeg {
    type Vtable = IMapRouteLeg_Vtbl;
    const IID: ::windows::core::GUID = <IMapRouteLeg as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapRouteLeg {
    const NAME: &'static str = "Windows.Services.Maps.MapRouteLeg";
}
impl ::core::convert::From<MapRouteLeg> for ::windows::core::IUnknown {
    fn from(value: MapRouteLeg) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapRouteLeg> for ::windows::core::IUnknown {
    fn from(value: &MapRouteLeg) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&MapRouteLeg> for &::windows::core::IUnknown {
    fn from(value: &MapRouteLeg) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<MapRouteLeg> for ::windows::core::IInspectable {
    fn from(value: MapRouteLeg) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapRouteLeg> for ::windows::core::IInspectable {
    fn from(value: &MapRouteLeg) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&MapRouteLeg> for &::windows::core::IInspectable {
    fn from(value: &MapRouteLeg) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for MapRouteLeg {}
unsafe impl ::core::marker::Sync for MapRouteLeg {}
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
pub struct MapRouteManeuver(::windows::core::IUnknown);
impl MapRouteManeuver {
    #[doc = "*Required features: `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn StartingPoint(&self) -> ::windows::core::Result<super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).StartingPoint)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::Geolocation::Geopoint>(result__)
        }
    }
    pub fn LengthInMeters(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LengthInMeters)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn InstructionText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InstructionText)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<MapRouteManeuverKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MapRouteManeuverKind>(result__)
        }
    }
    pub fn ExitNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ExitNumber)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ManeuverNotices(&self) -> ::windows::core::Result<MapManeuverNotices> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ManeuverNotices)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MapManeuverNotices>(result__)
        }
    }
    pub fn StartHeading(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IMapRouteManeuver2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).StartHeading)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn EndHeading(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IMapRouteManeuver2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).EndHeading)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn StreetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMapRouteManeuver2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).StreetName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Warnings(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ManeuverWarning>> {
        let this = &::windows::core::Interface::cast::<IMapRouteManeuver3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Warnings)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<ManeuverWarning>>(result__)
        }
    }
}
impl ::core::clone::Clone for MapRouteManeuver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapRouteManeuver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapRouteManeuver {}
impl ::core::fmt::Debug for MapRouteManeuver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapRouteManeuver").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapRouteManeuver {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.MapRouteManeuver;{ed5c17f0-a6ab-4d65-a086-fa8a7e340df2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapRouteManeuver {
    type Vtable = IMapRouteManeuver_Vtbl;
    const IID: ::windows::core::GUID = <IMapRouteManeuver as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapRouteManeuver {
    const NAME: &'static str = "Windows.Services.Maps.MapRouteManeuver";
}
impl ::core::convert::From<MapRouteManeuver> for ::windows::core::IUnknown {
    fn from(value: MapRouteManeuver) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapRouteManeuver> for ::windows::core::IUnknown {
    fn from(value: &MapRouteManeuver) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&MapRouteManeuver> for &::windows::core::IUnknown {
    fn from(value: &MapRouteManeuver) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<MapRouteManeuver> for ::windows::core::IInspectable {
    fn from(value: MapRouteManeuver) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapRouteManeuver> for ::windows::core::IInspectable {
    fn from(value: &MapRouteManeuver) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&MapRouteManeuver> for &::windows::core::IInspectable {
    fn from(value: &MapRouteManeuver) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for MapRouteManeuver {}
unsafe impl ::core::marker::Sync for MapRouteManeuver {}
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MapRouteManeuverKind(pub i32);
impl MapRouteManeuverKind {
    pub const None: Self = Self(0i32);
    pub const Start: Self = Self(1i32);
    pub const Stopover: Self = Self(2i32);
    pub const StopoverResume: Self = Self(3i32);
    pub const End: Self = Self(4i32);
    pub const GoStraight: Self = Self(5i32);
    pub const UTurnLeft: Self = Self(6i32);
    pub const UTurnRight: Self = Self(7i32);
    pub const TurnKeepLeft: Self = Self(8i32);
    pub const TurnKeepRight: Self = Self(9i32);
    pub const TurnLightLeft: Self = Self(10i32);
    pub const TurnLightRight: Self = Self(11i32);
    pub const TurnLeft: Self = Self(12i32);
    pub const TurnRight: Self = Self(13i32);
    pub const TurnHardLeft: Self = Self(14i32);
    pub const TurnHardRight: Self = Self(15i32);
    pub const FreewayEnterLeft: Self = Self(16i32);
    pub const FreewayEnterRight: Self = Self(17i32);
    pub const FreewayLeaveLeft: Self = Self(18i32);
    pub const FreewayLeaveRight: Self = Self(19i32);
    pub const FreewayContinueLeft: Self = Self(20i32);
    pub const FreewayContinueRight: Self = Self(21i32);
    pub const TrafficCircleLeft: Self = Self(22i32);
    pub const TrafficCircleRight: Self = Self(23i32);
    pub const TakeFerry: Self = Self(24i32);
}
impl ::core::marker::Copy for MapRouteManeuverKind {}
impl ::core::clone::Clone for MapRouteManeuverKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MapRouteManeuverKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MapRouteManeuverKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for MapRouteManeuverKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapRouteManeuverKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapRouteManeuverKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.MapRouteManeuverKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MapRouteOptimization(pub i32);
impl MapRouteOptimization {
    pub const Time: Self = Self(0i32);
    pub const Distance: Self = Self(1i32);
    pub const TimeWithTraffic: Self = Self(2i32);
    pub const Scenic: Self = Self(3i32);
}
impl ::core::marker::Copy for MapRouteOptimization {}
impl ::core::clone::Clone for MapRouteOptimization {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MapRouteOptimization {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MapRouteOptimization {
    type Abi = Self;
}
impl ::core::fmt::Debug for MapRouteOptimization {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapRouteOptimization").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapRouteOptimization {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.MapRouteOptimization;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MapRouteRestrictions(pub u32);
impl MapRouteRestrictions {
    pub const None: Self = Self(0u32);
    pub const Highways: Self = Self(1u32);
    pub const TollRoads: Self = Self(2u32);
    pub const Ferries: Self = Self(4u32);
    pub const Tunnels: Self = Self(8u32);
    pub const DirtRoads: Self = Self(16u32);
    pub const Motorail: Self = Self(32u32);
}
impl ::core::marker::Copy for MapRouteRestrictions {}
impl ::core::clone::Clone for MapRouteRestrictions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MapRouteRestrictions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MapRouteRestrictions {
    type Abi = Self;
}
impl ::core::fmt::Debug for MapRouteRestrictions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapRouteRestrictions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MapRouteRestrictions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MapRouteRestrictions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MapRouteRestrictions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MapRouteRestrictions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MapRouteRestrictions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for MapRouteRestrictions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.MapRouteRestrictions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Services_Maps\"`*"]
pub struct MapService;
impl MapService {
    pub fn SetServiceToken(value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        Self::IMapServiceStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetServiceToken)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() })
    }
    pub fn ServiceToken() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapServiceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ServiceToken)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn WorldViewRegionCode() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapServiceStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).WorldViewRegionCode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn DataAttributions() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapServiceStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DataAttributions)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SetDataUsagePreference(value: MapServiceDataUsagePreference) -> ::windows::core::Result<()> {
        Self::IMapServiceStatics4(|this| unsafe { (::windows::core::Interface::vtable(this).SetDataUsagePreference)(::windows::core::Interface::as_raw(this), value).ok() })
    }
    pub fn DataUsagePreference() -> ::windows::core::Result<MapServiceDataUsagePreference> {
        Self::IMapServiceStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DataUsagePreference)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MapServiceDataUsagePreference>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMapServiceStatics<R, F: FnOnce(&IMapServiceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MapService, IMapServiceStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMapServiceStatics2<R, F: FnOnce(&IMapServiceStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MapService, IMapServiceStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMapServiceStatics3<R, F: FnOnce(&IMapServiceStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MapService, IMapServiceStatics3> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMapServiceStatics4<R, F: FnOnce(&IMapServiceStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MapService, IMapServiceStatics4> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for MapService {
    const NAME: &'static str = "Windows.Services.Maps.MapService";
}
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MapServiceDataUsagePreference(pub i32);
impl MapServiceDataUsagePreference {
    pub const Default: Self = Self(0i32);
    pub const OfflineMapDataOnly: Self = Self(1i32);
}
impl ::core::marker::Copy for MapServiceDataUsagePreference {}
impl ::core::clone::Clone for MapServiceDataUsagePreference {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MapServiceDataUsagePreference {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MapServiceDataUsagePreference {
    type Abi = Self;
}
impl ::core::fmt::Debug for MapServiceDataUsagePreference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapServiceDataUsagePreference").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapServiceDataUsagePreference {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.MapServiceDataUsagePreference;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
pub struct PlaceInfo(::windows::core::IUnknown);
impl PlaceInfo {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Show(&self, selection: super::super::Foundation::Rect) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Show)(::windows::core::Interface::as_raw(this), selection).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowWithPreferredPlacement(&self, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ShowWithPreferredPlacement)(::windows::core::Interface::as_raw(this), selection, preferredplacement).ok() }
    }
    pub fn Identifier(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Identifier)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DisplayAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DisplayAddress)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Geoshape(&self) -> ::windows::core::Result<super::super::Devices::Geolocation::IGeoshape> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Geoshape)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::Geolocation::IGeoshape>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Create<'a, P0>(referencepoint: P0) -> ::windows::core::Result<PlaceInfo>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Devices::Geolocation::Geopoint>>,
    {
        Self::IPlaceInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), referencepoint.into().abi(), result__.as_mut_ptr()).from_abi::<PlaceInfo>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn CreateWithGeopointAndOptions<'a, P0, P1>(referencepoint: P0, options: P1) -> ::windows::core::Result<PlaceInfo>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Devices::Geolocation::Geopoint>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, PlaceInfoCreateOptions>>,
    {
        Self::IPlaceInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithGeopointAndOptions)(::windows::core::Interface::as_raw(this), referencepoint.into().abi(), options.into().abi(), result__.as_mut_ptr()).from_abi::<PlaceInfo>(result__)
        })
    }
    pub fn CreateFromIdentifier(identifier: &::windows::core::HSTRING) -> ::windows::core::Result<PlaceInfo> {
        Self::IPlaceInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromIdentifier)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(identifier), result__.as_mut_ptr()).from_abi::<PlaceInfo>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn CreateFromIdentifierWithOptions<'a, P0, P1>(identifier: &::windows::core::HSTRING, defaultpoint: P0, options: P1) -> ::windows::core::Result<PlaceInfo>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Devices::Geolocation::Geopoint>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, PlaceInfoCreateOptions>>,
    {
        Self::IPlaceInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromIdentifierWithOptions)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(identifier), defaultpoint.into().abi(), options.into().abi(), result__.as_mut_ptr()).from_abi::<PlaceInfo>(result__)
        })
    }
    pub fn CreateFromMapLocation<'a, P0>(location: P0) -> ::windows::core::Result<PlaceInfo>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, MapLocation>>,
    {
        Self::IPlaceInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromMapLocation)(::windows::core::Interface::as_raw(this), location.into().abi(), result__.as_mut_ptr()).from_abi::<PlaceInfo>(result__)
        })
    }
    pub fn IsShowSupported() -> ::windows::core::Result<bool> {
        Self::IPlaceInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsShowSupported)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn CreateFromAddress(displayaddress: &::windows::core::HSTRING) -> ::windows::core::Result<PlaceInfo> {
        Self::IPlaceInfoStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromAddress)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(displayaddress), result__.as_mut_ptr()).from_abi::<PlaceInfo>(result__)
        })
    }
    pub fn CreateFromAddressWithName(displayaddress: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<PlaceInfo> {
        Self::IPlaceInfoStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromAddressWithName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(displayaddress), ::core::mem::transmute_copy(displayname), result__.as_mut_ptr()).from_abi::<PlaceInfo>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPlaceInfoStatics<R, F: FnOnce(&IPlaceInfoStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PlaceInfo, IPlaceInfoStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPlaceInfoStatics2<R, F: FnOnce(&IPlaceInfoStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PlaceInfo, IPlaceInfoStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PlaceInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlaceInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlaceInfo {}
impl ::core::fmt::Debug for PlaceInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaceInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlaceInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.PlaceInfo;{9a0810b6-31c8-4f6a-9f18-950b4c38951a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PlaceInfo {
    type Vtable = IPlaceInfo_Vtbl;
    const IID: ::windows::core::GUID = <IPlaceInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PlaceInfo {
    const NAME: &'static str = "Windows.Services.Maps.PlaceInfo";
}
impl ::core::convert::From<PlaceInfo> for ::windows::core::IUnknown {
    fn from(value: PlaceInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlaceInfo> for ::windows::core::IUnknown {
    fn from(value: &PlaceInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PlaceInfo> for &::windows::core::IUnknown {
    fn from(value: &PlaceInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PlaceInfo> for ::windows::core::IInspectable {
    fn from(value: PlaceInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlaceInfo> for ::windows::core::IInspectable {
    fn from(value: &PlaceInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PlaceInfo> for &::windows::core::IInspectable {
    fn from(value: &PlaceInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PlaceInfo {}
unsafe impl ::core::marker::Sync for PlaceInfo {}
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
pub struct PlaceInfoCreateOptions(::windows::core::IUnknown);
impl PlaceInfoCreateOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PlaceInfoCreateOptions, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDisplayName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetDisplayAddress(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDisplayAddress)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DisplayAddress)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for PlaceInfoCreateOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlaceInfoCreateOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlaceInfoCreateOptions {}
impl ::core::fmt::Debug for PlaceInfoCreateOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaceInfoCreateOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlaceInfoCreateOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.PlaceInfoCreateOptions;{cd33c125-67f1-4bb3-9907-ecce939b0399})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PlaceInfoCreateOptions {
    type Vtable = IPlaceInfoCreateOptions_Vtbl;
    const IID: ::windows::core::GUID = <IPlaceInfoCreateOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PlaceInfoCreateOptions {
    const NAME: &'static str = "Windows.Services.Maps.PlaceInfoCreateOptions";
}
impl ::core::convert::From<PlaceInfoCreateOptions> for ::windows::core::IUnknown {
    fn from(value: PlaceInfoCreateOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlaceInfoCreateOptions> for ::windows::core::IUnknown {
    fn from(value: &PlaceInfoCreateOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PlaceInfoCreateOptions> for &::windows::core::IUnknown {
    fn from(value: &PlaceInfoCreateOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PlaceInfoCreateOptions> for ::windows::core::IInspectable {
    fn from(value: PlaceInfoCreateOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlaceInfoCreateOptions> for ::windows::core::IInspectable {
    fn from(value: &PlaceInfoCreateOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PlaceInfoCreateOptions> for &::windows::core::IInspectable {
    fn from(value: &PlaceInfoCreateOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PlaceInfoCreateOptions {}
unsafe impl ::core::marker::Sync for PlaceInfoCreateOptions {}
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TrafficCongestion(pub i32);
impl TrafficCongestion {
    pub const Unknown: Self = Self(0i32);
    pub const Light: Self = Self(1i32);
    pub const Mild: Self = Self(2i32);
    pub const Medium: Self = Self(3i32);
    pub const Heavy: Self = Self(4i32);
}
impl ::core::marker::Copy for TrafficCongestion {}
impl ::core::clone::Clone for TrafficCongestion {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TrafficCongestion {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TrafficCongestion {
    type Abi = Self;
}
impl ::core::fmt::Debug for TrafficCongestion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TrafficCongestion").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TrafficCongestion {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.TrafficCongestion;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Services_Maps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WaypointKind(pub i32);
impl WaypointKind {
    pub const Stop: Self = Self(0i32);
    pub const Via: Self = Self(1i32);
}
impl ::core::marker::Copy for WaypointKind {}
impl ::core::clone::Clone for WaypointKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WaypointKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WaypointKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for WaypointKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WaypointKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WaypointKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.WaypointKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
