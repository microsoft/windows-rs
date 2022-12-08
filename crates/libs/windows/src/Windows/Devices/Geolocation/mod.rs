#[cfg(feature = "Devices_Geolocation_Geofencing")]
pub mod Geofencing;
#[cfg(feature = "Devices_Geolocation_Provider")]
pub mod Provider;
#[doc(hidden)]
#[repr(transparent)]
pub struct ICivicAddress(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICivicAddress {
    type Vtable = ICivicAddress_Vtbl;
}
unsafe impl ::windows::core::Interface for ICivicAddress {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8567a1a_64f4_4d48_bcea_f6b008eca34c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICivicAddress_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Country: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub City: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PostalCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeoboundingBox(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGeoboundingBox {
    type Vtable = IGeoboundingBox_Vtbl;
}
unsafe impl ::windows::core::Interface for IGeoboundingBox {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0896c80b_274f_43da_9a06_cbfcdaeb4ec2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeoboundingBox_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub NorthwestCorner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BasicGeoposition) -> ::windows::core::HRESULT,
    pub SoutheastCorner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BasicGeoposition) -> ::windows::core::HRESULT,
    pub Center: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BasicGeoposition) -> ::windows::core::HRESULT,
    pub MinAltitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub MaxAltitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeoboundingBoxFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGeoboundingBoxFactory {
    type Vtable = IGeoboundingBoxFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IGeoboundingBoxFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4dfba589_0411_4abc_b3b5_5bbccb57d98c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeoboundingBoxFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateWithAltitudeReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateWithAltitudeReferenceAndSpatialReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeoboundingBoxStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGeoboundingBoxStatics {
    type Vtable = IGeoboundingBoxStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IGeoboundingBoxStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x67b80708_e61a_4cd0_841b_93233792b5ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeoboundingBoxStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub TryCompute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, positions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryCompute: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TryComputeWithAltitudeReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, positions: *mut ::core::ffi::c_void, altituderefsystem: AltitudeReferenceSystem, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryComputeWithAltitudeReference: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TryComputeWithAltitudeReferenceAndSpatialReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, positions: *mut ::core::ffi::c_void, altituderefsystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryComputeWithAltitudeReferenceAndSpatialReference: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeocircle(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGeocircle {
    type Vtable = IGeocircle_Vtbl;
}
unsafe impl ::windows::core::Interface for IGeocircle {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39e45843_a7f9_4e63_92a7_ba0c28d124b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocircle_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Center: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BasicGeoposition) -> ::windows::core::HRESULT,
    pub Radius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeocircleFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGeocircleFactory {
    type Vtable = IGeocircleFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IGeocircleFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xafd6531f_72b1_4f7d_87cc_4ed4c9849c05);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocircleFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: BasicGeoposition, radius: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateWithAltitudeReferenceSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: BasicGeoposition, radius: f64, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateWithAltitudeReferenceSystemAndSpatialReferenceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: BasicGeoposition, radius: f64, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeocoordinate(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGeocoordinate {
    type Vtable = IGeocoordinate_Vtbl;
}
unsafe impl ::windows::core::Interface for IGeocoordinate {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee21a3aa_976a_4c70_803d_083ea55bcbc4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinate_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Latitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Latitude: usize,
    #[cfg(feature = "deprecated")]
    pub Longitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Longitude: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Altitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Altitude: usize,
    pub Accuracy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AltitudeAccuracy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AltitudeAccuracy: usize,
    #[cfg(feature = "Foundation")]
    pub Heading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Heading: usize,
    #[cfg(feature = "Foundation")]
    pub Speed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Speed: usize,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeocoordinateSatelliteData(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGeocoordinateSatelliteData {
    type Vtable = IGeocoordinateSatelliteData_Vtbl;
}
unsafe impl ::windows::core::Interface for IGeocoordinateSatelliteData {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc32a74d9_2608_474c_912c_06dd490f4af7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinateSatelliteData_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PositionDilutionOfPrecision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PositionDilutionOfPrecision: usize,
    #[cfg(feature = "Foundation")]
    pub HorizontalDilutionOfPrecision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HorizontalDilutionOfPrecision: usize,
    #[cfg(feature = "Foundation")]
    pub VerticalDilutionOfPrecision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VerticalDilutionOfPrecision: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeocoordinateSatelliteData2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGeocoordinateSatelliteData2 {
    type Vtable = IGeocoordinateSatelliteData2_Vtbl;
}
unsafe impl ::windows::core::Interface for IGeocoordinateSatelliteData2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x761c8cfd_a19d_5a51_80f5_71676115483e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinateSatelliteData2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GeometricDilutionOfPrecision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GeometricDilutionOfPrecision: usize,
    #[cfg(feature = "Foundation")]
    pub TimeDilutionOfPrecision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimeDilutionOfPrecision: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeocoordinateWithPoint(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGeocoordinateWithPoint {
    type Vtable = IGeocoordinateWithPoint_Vtbl;
}
unsafe impl ::windows::core::Interface for IGeocoordinateWithPoint {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfeea0525_d22c_4d46_b527_0b96066fc7db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinateWithPoint_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Point: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeocoordinateWithPositionData(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGeocoordinateWithPositionData {
    type Vtable = IGeocoordinateWithPositionData_Vtbl;
}
unsafe impl ::windows::core::Interface for IGeocoordinateWithPositionData {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95e634be_dbd6_40ac_b8f2_a65c0340d9a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinateWithPositionData_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PositionSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PositionSource) -> ::windows::core::HRESULT,
    pub SatelliteData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeocoordinateWithPositionSourceTimestamp(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGeocoordinateWithPositionSourceTimestamp {
    type Vtable = IGeocoordinateWithPositionSourceTimestamp_Vtbl;
}
unsafe impl ::windows::core::Interface for IGeocoordinateWithPositionSourceTimestamp {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8543fc02_c9f1_4610_afe0_8bc3a6a87036);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinateWithPositionSourceTimestamp_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PositionSourceTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PositionSourceTimestamp: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeocoordinateWithRemoteSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGeocoordinateWithRemoteSource {
    type Vtable = IGeocoordinateWithRemoteSource_Vtbl;
}
unsafe impl ::windows::core::Interface for IGeocoordinateWithRemoteSource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x397cebd7_ee38_5f3b_8900_c4a7bc9cf953);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinateWithRemoteSource_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsRemoteSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeolocator(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGeolocator {
    type Vtable = IGeolocator_Vtbl;
}
unsafe impl ::windows::core::Interface for IGeolocator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9c3bf62_4524_4989_8aa9_de019d2e551f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeolocator_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DesiredAccuracy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PositionAccuracy) -> ::windows::core::HRESULT,
    pub SetDesiredAccuracy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PositionAccuracy) -> ::windows::core::HRESULT,
    pub MovementThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetMovementThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub LocationStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PositionStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetGeopositionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetGeopositionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetGeopositionAsyncWithAgeAndTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maximumage: super::super::Foundation::TimeSpan, timeout: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetGeopositionAsyncWithAgeAndTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub PositionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PositionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePositionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePositionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub StatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeolocator2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGeolocator2 {
    type Vtable = IGeolocator2_Vtbl;
}
unsafe impl ::windows::core::Interface for IGeolocator2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1b42e6d_8891_43b4_ad36_27c6fe9a97b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeolocator2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AllowFallbackToConsentlessPositions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeolocatorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGeolocatorStatics {
    type Vtable = IGeolocatorStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IGeolocatorStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a8e7571_2df5_4591_9f87_eb5fd894e9b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeolocatorStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetGeopositionHistoryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetGeopositionHistoryAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetGeopositionHistoryWithDurationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetGeopositionHistoryWithDurationAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeolocatorStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGeolocatorStatics2 {
    type Vtable = IGeolocatorStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IGeolocatorStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x993011a2_fa1c_4631_a71d_0dbeb1250d9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeolocatorStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsDefaultGeopositionRecommended: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetDefaultGeoposition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDefaultGeoposition: usize,
    #[cfg(feature = "Foundation")]
    pub DefaultGeoposition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DefaultGeoposition: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeolocatorWithScalarAccuracy(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGeolocatorWithScalarAccuracy {
    type Vtable = IGeolocatorWithScalarAccuracy_Vtbl;
}
unsafe impl ::windows::core::Interface for IGeolocatorWithScalarAccuracy {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96f5d3c1_b80f_460a_994d_a96c47a51aa4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeolocatorWithScalarAccuracy_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub DesiredAccuracyInMeters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DesiredAccuracyInMeters: usize,
    #[cfg(feature = "Foundation")]
    pub SetDesiredAccuracyInMeters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDesiredAccuracyInMeters: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeopath(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGeopath {
    type Vtable = IGeopath_Vtbl;
}
unsafe impl ::windows::core::Interface for IGeopath {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe53fd7b9_2da4_4714_a652_de8593289898);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeopath_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Positions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Positions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeopathFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGeopathFactory {
    type Vtable = IGeopathFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IGeopathFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27bea9c8_c7e7_4359_9b9b_fca3e05ef593);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeopathFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, positions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithAltitudeReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, positions: *mut ::core::ffi::c_void, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithAltitudeReference: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithAltitudeReferenceAndSpatialReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, positions: *mut ::core::ffi::c_void, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithAltitudeReferenceAndSpatialReference: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeopoint(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGeopoint {
    type Vtable = IGeopoint_Vtbl;
}
unsafe impl ::windows::core::Interface for IGeopoint {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6bfa00eb_e56e_49bb_9caf_cbaa78a8bcef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeopoint_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BasicGeoposition) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeopointFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGeopointFactory {
    type Vtable = IGeopointFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IGeopointFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb6b8d33_76bd_4e30_8af7_a844dc37b7a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeopointFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: BasicGeoposition, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateWithAltitudeReferenceSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateWithAltitudeReferenceSystemAndSpatialReferenceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeoposition(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGeoposition {
    type Vtable = IGeoposition_Vtbl;
}
unsafe impl ::windows::core::Interface for IGeoposition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc18d0454_7d41_4ff7_a957_9dffb4ef7f5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeoposition_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Coordinate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CivicAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeoposition2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGeoposition2 {
    type Vtable = IGeoposition2_Vtbl;
}
unsafe impl ::windows::core::Interface for IGeoposition2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f62f697_8671_4b0d_86f8_474a8496187c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeoposition2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub VenueData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
#[repr(transparent)]
pub struct IGeoshape(::windows::core::IUnknown);
impl IGeoshape {
    pub fn GeoshapeType(&self) -> ::windows::core::Result<GeoshapeType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GeoshapeType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SpatialReferenceId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SpatialReferenceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AltitudeReferenceSystem(&self) -> ::windows::core::Result<AltitudeReferenceSystem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AltitudeReferenceSystem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IGeoshape, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IGeoshape {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGeoshape {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGeoshape {}
impl ::core::fmt::Debug for IGeoshape {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGeoshape").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IGeoshape {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{c99ca2af-c729-43c1-8fab-d6dec914df7e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IGeoshape {
    type Vtable = IGeoshape_Vtbl;
}
unsafe impl ::windows::core::Interface for IGeoshape {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc99ca2af_c729_43c1_8fab_d6dec914df7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeoshape_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GeoshapeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GeoshapeType) -> ::windows::core::HRESULT,
    pub SpatialReferenceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub AltitudeReferenceSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AltitudeReferenceSystem) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeovisit(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGeovisit {
    type Vtable = IGeovisit_Vtbl;
}
unsafe impl ::windows::core::Interface for IGeovisit {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1877a76_9ef6_41ab_a0dd_793ece76e2de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeovisit_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VisitStateChange) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeovisitMonitor(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGeovisitMonitor {
    type Vtable = IGeovisitMonitor_Vtbl;
}
unsafe impl ::windows::core::Interface for IGeovisitMonitor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80118aaf_5944_4591_83c1_396647f54f2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeovisitMonitor_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub MonitoringScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VisitMonitoringScope) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: VisitMonitoringScope) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub VisitStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VisitStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVisitStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVisitStateChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeovisitMonitorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGeovisitMonitorStatics {
    type Vtable = IGeovisitMonitorStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IGeovisitMonitorStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbcf976a7_bbf2_4cdd_95cf_554c82edfb87);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeovisitMonitorStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetLastReportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetLastReportAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeovisitStateChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGeovisitStateChangedEventArgs {
    type Vtable = IGeovisitStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IGeovisitStateChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xceb4d1ff_8b53_4968_beed_4cecd029ce15);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeovisitStateChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Visit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeovisitTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGeovisitTriggerDetails {
    type Vtable = IGeovisitTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for IGeovisitTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea770d9e_d1c9_454b_99b7_b2f8cdd2482f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeovisitTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadReports: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadReports: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPositionChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPositionChangedEventArgs {
    type Vtable = IPositionChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IPositionChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x37859ce5_9d1e_46c5_bf3b_6ad8cac1a093);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPositionChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStatusChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStatusChangedEventArgs {
    type Vtable = IStatusChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IStatusChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3453d2da_8c93_4111_a205_9aecfc9be5c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStatusChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PositionStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVenueData(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IVenueData {
    type Vtable = IVenueData_Vtbl;
}
unsafe impl ::windows::core::Interface for IVenueData {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66f39187_60e3_4b2f_b527_4f53f1c3c677);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVenueData_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Level: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
#[repr(transparent)]
pub struct CivicAddress(::windows::core::IUnknown);
impl CivicAddress {
    pub fn Country(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Country)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn State(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).State)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn City(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).City)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PostalCode(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PostalCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Timestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for CivicAddress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CivicAddress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CivicAddress {}
impl ::core::fmt::Debug for CivicAddress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CivicAddress").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CivicAddress {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.CivicAddress;{a8567a1a-64f4-4d48-bcea-f6b008eca34c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CivicAddress {
    type Vtable = ICivicAddress_Vtbl;
}
unsafe impl ::windows::core::Interface for CivicAddress {
    const IID: ::windows::core::GUID = <ICivicAddress as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CivicAddress {
    const NAME: &'static str = "Windows.Devices.Geolocation.CivicAddress";
}
::windows::core::interface_hierarchy!(CivicAddress, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CivicAddress {}
unsafe impl ::core::marker::Sync for CivicAddress {}
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
#[repr(transparent)]
pub struct GeoboundingBox(::windows::core::IUnknown);
impl GeoboundingBox {
    pub fn NorthwestCorner(&self) -> ::windows::core::Result<BasicGeoposition> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NorthwestCorner)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SoutheastCorner(&self) -> ::windows::core::Result<BasicGeoposition> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SoutheastCorner)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Center(&self) -> ::windows::core::Result<BasicGeoposition> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Center)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn MinAltitude(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MinAltitude)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn MaxAltitude(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxAltitude)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Create(northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition) -> ::windows::core::Result<GeoboundingBox> {
        Self::IGeoboundingBoxFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), northwestcorner, southeastcorner, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateWithAltitudeReference(northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem) -> ::windows::core::Result<GeoboundingBox> {
        Self::IGeoboundingBoxFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWithAltitudeReference)(::windows::core::Vtable::as_raw(this), northwestcorner, southeastcorner, altitudereferencesystem, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateWithAltitudeReferenceAndSpatialReference(northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows::core::Result<GeoboundingBox> {
        Self::IGeoboundingBoxFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWithAltitudeReferenceAndSpatialReference)(::windows::core::Vtable::as_raw(this), northwestcorner, southeastcorner, altitudereferencesystem, spatialreferenceid, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TryCompute<P0, E0>(positions: P0) -> ::windows::core::Result<GeoboundingBox>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<BasicGeoposition>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IGeoboundingBoxStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryCompute)(::windows::core::Vtable::as_raw(this), positions.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TryComputeWithAltitudeReference<P0, E0>(positions: P0, altituderefsystem: AltitudeReferenceSystem) -> ::windows::core::Result<GeoboundingBox>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<BasicGeoposition>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IGeoboundingBoxStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryComputeWithAltitudeReference)(::windows::core::Vtable::as_raw(this), positions.try_into().map_err(|e| e.into())?.abi(), altituderefsystem, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TryComputeWithAltitudeReferenceAndSpatialReference<P0, E0>(positions: P0, altituderefsystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows::core::Result<GeoboundingBox>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<BasicGeoposition>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IGeoboundingBoxStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryComputeWithAltitudeReferenceAndSpatialReference)(::windows::core::Vtable::as_raw(this), positions.try_into().map_err(|e| e.into())?.abi(), altituderefsystem, spatialreferenceid, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GeoshapeType(&self) -> ::windows::core::Result<GeoshapeType> {
        let this = &::windows::core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GeoshapeType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SpatialReferenceId(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SpatialReferenceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AltitudeReferenceSystem(&self) -> ::windows::core::Result<AltitudeReferenceSystem> {
        let this = &::windows::core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AltitudeReferenceSystem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IGeoboundingBoxFactory<R, F: FnOnce(&IGeoboundingBoxFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<GeoboundingBox, IGeoboundingBoxFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGeoboundingBoxStatics<R, F: FnOnce(&IGeoboundingBoxStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<GeoboundingBox, IGeoboundingBoxStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for GeoboundingBox {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GeoboundingBox {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeoboundingBox {}
impl ::core::fmt::Debug for GeoboundingBox {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeoboundingBox").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GeoboundingBox {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.GeoboundingBox;{0896c80b-274f-43da-9a06-cbfcdaeb4ec2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GeoboundingBox {
    type Vtable = IGeoboundingBox_Vtbl;
}
unsafe impl ::windows::core::Interface for GeoboundingBox {
    const IID: ::windows::core::GUID = <IGeoboundingBox as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GeoboundingBox {
    const NAME: &'static str = "Windows.Devices.Geolocation.GeoboundingBox";
}
::windows::core::interface_hierarchy!(GeoboundingBox, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<GeoboundingBox> for IGeoshape {
    type Error = ::windows::core::Error;
    fn try_from(value: GeoboundingBox) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&GeoboundingBox> for IGeoshape {
    type Error = ::windows::core::Error;
    fn try_from(value: &GeoboundingBox) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&GeoboundingBox> for ::windows::core::InParam<IGeoshape> {
    type Error = ::windows::core::Error;
    fn try_from(value: &GeoboundingBox) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for GeoboundingBox {}
unsafe impl ::core::marker::Sync for GeoboundingBox {}
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
#[repr(transparent)]
pub struct Geocircle(::windows::core::IUnknown);
impl Geocircle {
    pub fn Center(&self) -> ::windows::core::Result<BasicGeoposition> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Center)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Radius(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Radius)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Create(position: BasicGeoposition, radius: f64) -> ::windows::core::Result<Geocircle> {
        Self::IGeocircleFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), position, radius, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateWithAltitudeReferenceSystem(position: BasicGeoposition, radius: f64, altitudereferencesystem: AltitudeReferenceSystem) -> ::windows::core::Result<Geocircle> {
        Self::IGeocircleFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWithAltitudeReferenceSystem)(::windows::core::Vtable::as_raw(this), position, radius, altitudereferencesystem, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateWithAltitudeReferenceSystemAndSpatialReferenceId(position: BasicGeoposition, radius: f64, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows::core::Result<Geocircle> {
        Self::IGeocircleFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWithAltitudeReferenceSystemAndSpatialReferenceId)(::windows::core::Vtable::as_raw(this), position, radius, altitudereferencesystem, spatialreferenceid, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GeoshapeType(&self) -> ::windows::core::Result<GeoshapeType> {
        let this = &::windows::core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GeoshapeType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SpatialReferenceId(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SpatialReferenceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AltitudeReferenceSystem(&self) -> ::windows::core::Result<AltitudeReferenceSystem> {
        let this = &::windows::core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AltitudeReferenceSystem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IGeocircleFactory<R, F: FnOnce(&IGeocircleFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Geocircle, IGeocircleFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for Geocircle {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Geocircle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Geocircle {}
impl ::core::fmt::Debug for Geocircle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Geocircle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Geocircle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geocircle;{39e45843-a7f9-4e63-92a7-ba0c28d124b1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for Geocircle {
    type Vtable = IGeocircle_Vtbl;
}
unsafe impl ::windows::core::Interface for Geocircle {
    const IID: ::windows::core::GUID = <IGeocircle as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Geocircle {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geocircle";
}
::windows::core::interface_hierarchy!(Geocircle, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<Geocircle> for IGeoshape {
    type Error = ::windows::core::Error;
    fn try_from(value: Geocircle) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&Geocircle> for IGeoshape {
    type Error = ::windows::core::Error;
    fn try_from(value: &Geocircle) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&Geocircle> for ::windows::core::InParam<IGeoshape> {
    type Error = ::windows::core::Error;
    fn try_from(value: &Geocircle) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for Geocircle {}
unsafe impl ::core::marker::Sync for Geocircle {}
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
#[repr(transparent)]
pub struct Geocoordinate(::windows::core::IUnknown);
impl Geocoordinate {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Latitude(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Latitude)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Longitude(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Longitude)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Altitude(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Altitude)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Accuracy(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Accuracy)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AltitudeAccuracy(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AltitudeAccuracy)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Heading(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Heading)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Speed(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Speed)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Timestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Point(&self) -> ::windows::core::Result<Geopoint> {
        let this = &::windows::core::Interface::cast::<IGeocoordinateWithPoint>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Point)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PositionSource(&self) -> ::windows::core::Result<PositionSource> {
        let this = &::windows::core::Interface::cast::<IGeocoordinateWithPositionData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PositionSource)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SatelliteData(&self) -> ::windows::core::Result<GeocoordinateSatelliteData> {
        let this = &::windows::core::Interface::cast::<IGeocoordinateWithPositionData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SatelliteData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PositionSourceTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = &::windows::core::Interface::cast::<IGeocoordinateWithPositionSourceTimestamp>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PositionSourceTimestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsRemoteSource(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IGeocoordinateWithRemoteSource>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsRemoteSource)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for Geocoordinate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Geocoordinate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Geocoordinate {}
impl ::core::fmt::Debug for Geocoordinate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Geocoordinate").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Geocoordinate {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geocoordinate;{ee21a3aa-976a-4c70-803d-083ea55bcbc4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for Geocoordinate {
    type Vtable = IGeocoordinate_Vtbl;
}
unsafe impl ::windows::core::Interface for Geocoordinate {
    const IID: ::windows::core::GUID = <IGeocoordinate as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Geocoordinate {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geocoordinate";
}
::windows::core::interface_hierarchy!(Geocoordinate, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Geocoordinate {}
unsafe impl ::core::marker::Sync for Geocoordinate {}
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
#[repr(transparent)]
pub struct GeocoordinateSatelliteData(::windows::core::IUnknown);
impl GeocoordinateSatelliteData {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PositionDilutionOfPrecision(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PositionDilutionOfPrecision)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HorizontalDilutionOfPrecision(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HorizontalDilutionOfPrecision)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn VerticalDilutionOfPrecision(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VerticalDilutionOfPrecision)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GeometricDilutionOfPrecision(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows::core::Interface::cast::<IGeocoordinateSatelliteData2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GeometricDilutionOfPrecision)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeDilutionOfPrecision(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows::core::Interface::cast::<IGeocoordinateSatelliteData2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TimeDilutionOfPrecision)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for GeocoordinateSatelliteData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GeocoordinateSatelliteData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeocoordinateSatelliteData {}
impl ::core::fmt::Debug for GeocoordinateSatelliteData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeocoordinateSatelliteData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GeocoordinateSatelliteData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.GeocoordinateSatelliteData;{c32a74d9-2608-474c-912c-06dd490f4af7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GeocoordinateSatelliteData {
    type Vtable = IGeocoordinateSatelliteData_Vtbl;
}
unsafe impl ::windows::core::Interface for GeocoordinateSatelliteData {
    const IID: ::windows::core::GUID = <IGeocoordinateSatelliteData as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GeocoordinateSatelliteData {
    const NAME: &'static str = "Windows.Devices.Geolocation.GeocoordinateSatelliteData";
}
::windows::core::interface_hierarchy!(GeocoordinateSatelliteData, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GeocoordinateSatelliteData {}
unsafe impl ::core::marker::Sync for GeocoordinateSatelliteData {}
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
#[repr(transparent)]
pub struct Geolocator(::windows::core::IUnknown);
impl Geolocator {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Geolocator, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn DesiredAccuracy(&self) -> ::windows::core::Result<PositionAccuracy> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DesiredAccuracy)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDesiredAccuracy(&self, value: PositionAccuracy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDesiredAccuracy)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn MovementThreshold(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MovementThreshold)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetMovementThreshold(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMovementThreshold)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportInterval)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetReportInterval)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn LocationStatus(&self) -> ::windows::core::Result<PositionStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LocationStatus)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetGeopositionAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Geoposition>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetGeopositionAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetGeopositionAsyncWithAgeAndTimeout(&self, maximumage: super::super::Foundation::TimeSpan, timeout: super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Geoposition>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetGeopositionAsyncWithAgeAndTimeout)(::windows::core::Vtable::as_raw(this), maximumage, timeout, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PositionChanged(&self, handler: &super::super::Foundation::TypedEventHandler<Geolocator, PositionChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PositionChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePositionChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePositionChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StatusChanged(&self, handler: &super::super::Foundation::TypedEventHandler<Geolocator, StatusChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StatusChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveStatusChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn AllowFallbackToConsentlessPositions(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGeolocator2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).AllowFallbackToConsentlessPositions)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GeolocationAccessStatus>> {
        Self::IGeolocatorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestAccessAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetGeopositionHistoryAsync(starttime: super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Geoposition>>> {
        Self::IGeolocatorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetGeopositionHistoryAsync)(::windows::core::Vtable::as_raw(this), starttime, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetGeopositionHistoryWithDurationAsync(starttime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Geoposition>>> {
        Self::IGeolocatorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetGeopositionHistoryWithDurationAsync)(::windows::core::Vtable::as_raw(this), starttime, duration, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn IsDefaultGeopositionRecommended() -> ::windows::core::Result<bool> {
        Self::IGeolocatorStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDefaultGeopositionRecommended)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDefaultGeoposition<P0, E0>(value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<BasicGeoposition>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IGeolocatorStatics2(|this| unsafe { (::windows::core::Vtable::vtable(this).SetDefaultGeoposition)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DefaultGeoposition() -> ::windows::core::Result<super::super::Foundation::IReference<BasicGeoposition>> {
        Self::IGeolocatorStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DefaultGeoposition)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DesiredAccuracyInMeters(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = &::windows::core::Interface::cast::<IGeolocatorWithScalarAccuracy>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DesiredAccuracyInMeters)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDesiredAccuracyInMeters<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<u32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IGeolocatorWithScalarAccuracy>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetDesiredAccuracyInMeters)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc(hidden)]
    pub fn IGeolocatorStatics<R, F: FnOnce(&IGeolocatorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Geolocator, IGeolocatorStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGeolocatorStatics2<R, F: FnOnce(&IGeolocatorStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Geolocator, IGeolocatorStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for Geolocator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Geolocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Geolocator {}
impl ::core::fmt::Debug for Geolocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Geolocator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Geolocator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geolocator;{a9c3bf62-4524-4989-8aa9-de019d2e551f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for Geolocator {
    type Vtable = IGeolocator_Vtbl;
}
unsafe impl ::windows::core::Interface for Geolocator {
    const IID: ::windows::core::GUID = <IGeolocator as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Geolocator {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geolocator";
}
::windows::core::interface_hierarchy!(Geolocator, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Geolocator {}
unsafe impl ::core::marker::Sync for Geolocator {}
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
#[repr(transparent)]
pub struct Geopath(::windows::core::IUnknown);
impl Geopath {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Positions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<BasicGeoposition>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Positions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create<P0, E0>(positions: P0) -> ::windows::core::Result<Geopath>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<BasicGeoposition>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IGeopathFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), positions.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithAltitudeReference<P0, E0>(positions: P0, altitudereferencesystem: AltitudeReferenceSystem) -> ::windows::core::Result<Geopath>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<BasicGeoposition>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IGeopathFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWithAltitudeReference)(::windows::core::Vtable::as_raw(this), positions.try_into().map_err(|e| e.into())?.abi(), altitudereferencesystem, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithAltitudeReferenceAndSpatialReference<P0, E0>(positions: P0, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows::core::Result<Geopath>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<BasicGeoposition>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IGeopathFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWithAltitudeReferenceAndSpatialReference)(::windows::core::Vtable::as_raw(this), positions.try_into().map_err(|e| e.into())?.abi(), altitudereferencesystem, spatialreferenceid, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GeoshapeType(&self) -> ::windows::core::Result<GeoshapeType> {
        let this = &::windows::core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GeoshapeType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SpatialReferenceId(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SpatialReferenceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AltitudeReferenceSystem(&self) -> ::windows::core::Result<AltitudeReferenceSystem> {
        let this = &::windows::core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AltitudeReferenceSystem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IGeopathFactory<R, F: FnOnce(&IGeopathFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Geopath, IGeopathFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for Geopath {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Geopath {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Geopath {}
impl ::core::fmt::Debug for Geopath {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Geopath").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Geopath {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geopath;{e53fd7b9-2da4-4714-a652-de8593289898})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for Geopath {
    type Vtable = IGeopath_Vtbl;
}
unsafe impl ::windows::core::Interface for Geopath {
    const IID: ::windows::core::GUID = <IGeopath as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Geopath {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geopath";
}
::windows::core::interface_hierarchy!(Geopath, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<Geopath> for IGeoshape {
    type Error = ::windows::core::Error;
    fn try_from(value: Geopath) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&Geopath> for IGeoshape {
    type Error = ::windows::core::Error;
    fn try_from(value: &Geopath) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&Geopath> for ::windows::core::InParam<IGeoshape> {
    type Error = ::windows::core::Error;
    fn try_from(value: &Geopath) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for Geopath {}
unsafe impl ::core::marker::Sync for Geopath {}
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
#[repr(transparent)]
pub struct Geopoint(::windows::core::IUnknown);
impl Geopoint {
    pub fn Position(&self) -> ::windows::core::Result<BasicGeoposition> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Position)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Create(position: BasicGeoposition) -> ::windows::core::Result<Geopoint> {
        Self::IGeopointFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), position, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateWithAltitudeReferenceSystem(position: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem) -> ::windows::core::Result<Geopoint> {
        Self::IGeopointFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWithAltitudeReferenceSystem)(::windows::core::Vtable::as_raw(this), position, altitudereferencesystem, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateWithAltitudeReferenceSystemAndSpatialReferenceId(position: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows::core::Result<Geopoint> {
        Self::IGeopointFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWithAltitudeReferenceSystemAndSpatialReferenceId)(::windows::core::Vtable::as_raw(this), position, altitudereferencesystem, spatialreferenceid, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GeoshapeType(&self) -> ::windows::core::Result<GeoshapeType> {
        let this = &::windows::core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GeoshapeType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SpatialReferenceId(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SpatialReferenceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AltitudeReferenceSystem(&self) -> ::windows::core::Result<AltitudeReferenceSystem> {
        let this = &::windows::core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AltitudeReferenceSystem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IGeopointFactory<R, F: FnOnce(&IGeopointFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Geopoint, IGeopointFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for Geopoint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Geopoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Geopoint {}
impl ::core::fmt::Debug for Geopoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Geopoint").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Geopoint {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geopoint;{6bfa00eb-e56e-49bb-9caf-cbaa78a8bcef})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for Geopoint {
    type Vtable = IGeopoint_Vtbl;
}
unsafe impl ::windows::core::Interface for Geopoint {
    const IID: ::windows::core::GUID = <IGeopoint as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Geopoint {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geopoint";
}
::windows::core::interface_hierarchy!(Geopoint, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<Geopoint> for IGeoshape {
    type Error = ::windows::core::Error;
    fn try_from(value: Geopoint) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&Geopoint> for IGeoshape {
    type Error = ::windows::core::Error;
    fn try_from(value: &Geopoint) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&Geopoint> for ::windows::core::InParam<IGeoshape> {
    type Error = ::windows::core::Error;
    fn try_from(value: &Geopoint) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for Geopoint {}
unsafe impl ::core::marker::Sync for Geopoint {}
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
#[repr(transparent)]
pub struct Geoposition(::windows::core::IUnknown);
impl Geoposition {
    pub fn Coordinate(&self) -> ::windows::core::Result<Geocoordinate> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Coordinate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CivicAddress(&self) -> ::windows::core::Result<CivicAddress> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CivicAddress)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn VenueData(&self) -> ::windows::core::Result<VenueData> {
        let this = &::windows::core::Interface::cast::<IGeoposition2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VenueData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for Geoposition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Geoposition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Geoposition {}
impl ::core::fmt::Debug for Geoposition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Geoposition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Geoposition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geoposition;{c18d0454-7d41-4ff7-a957-9dffb4ef7f5b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for Geoposition {
    type Vtable = IGeoposition_Vtbl;
}
unsafe impl ::windows::core::Interface for Geoposition {
    const IID: ::windows::core::GUID = <IGeoposition as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Geoposition {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geoposition";
}
::windows::core::interface_hierarchy!(Geoposition, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Geoposition {}
unsafe impl ::core::marker::Sync for Geoposition {}
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
#[repr(transparent)]
pub struct Geovisit(::windows::core::IUnknown);
impl Geovisit {
    pub fn Position(&self) -> ::windows::core::Result<Geoposition> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Position)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StateChange(&self) -> ::windows::core::Result<VisitStateChange> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StateChange)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Timestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for Geovisit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Geovisit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Geovisit {}
impl ::core::fmt::Debug for Geovisit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Geovisit").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Geovisit {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geovisit;{b1877a76-9ef6-41ab-a0dd-793ece76e2de})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for Geovisit {
    type Vtable = IGeovisit_Vtbl;
}
unsafe impl ::windows::core::Interface for Geovisit {
    const IID: ::windows::core::GUID = <IGeovisit as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Geovisit {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geovisit";
}
::windows::core::interface_hierarchy!(Geovisit, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Geovisit {}
unsafe impl ::core::marker::Sync for Geovisit {}
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
#[repr(transparent)]
pub struct GeovisitMonitor(::windows::core::IUnknown);
impl GeovisitMonitor {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<GeovisitMonitor, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn MonitoringScope(&self) -> ::windows::core::Result<VisitMonitoringScope> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MonitoringScope)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Start(&self, value: VisitMonitoringScope) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Start)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Stop)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn VisitStateChanged(&self, handler: &super::super::Foundation::TypedEventHandler<GeovisitMonitor, GeovisitStateChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VisitStateChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveVisitStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveVisitStateChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetLastReportAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Geovisit>> {
        Self::IGeovisitMonitorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetLastReportAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGeovisitMonitorStatics<R, F: FnOnce(&IGeovisitMonitorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<GeovisitMonitor, IGeovisitMonitorStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for GeovisitMonitor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GeovisitMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeovisitMonitor {}
impl ::core::fmt::Debug for GeovisitMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeovisitMonitor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GeovisitMonitor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.GeovisitMonitor;{80118aaf-5944-4591-83c1-396647f54f2c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GeovisitMonitor {
    type Vtable = IGeovisitMonitor_Vtbl;
}
unsafe impl ::windows::core::Interface for GeovisitMonitor {
    const IID: ::windows::core::GUID = <IGeovisitMonitor as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GeovisitMonitor {
    const NAME: &'static str = "Windows.Devices.Geolocation.GeovisitMonitor";
}
::windows::core::interface_hierarchy!(GeovisitMonitor, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GeovisitMonitor {}
unsafe impl ::core::marker::Sync for GeovisitMonitor {}
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
#[repr(transparent)]
pub struct GeovisitStateChangedEventArgs(::windows::core::IUnknown);
impl GeovisitStateChangedEventArgs {
    pub fn Visit(&self) -> ::windows::core::Result<Geovisit> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Visit)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for GeovisitStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GeovisitStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeovisitStateChangedEventArgs {}
impl ::core::fmt::Debug for GeovisitStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeovisitStateChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GeovisitStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.GeovisitStateChangedEventArgs;{ceb4d1ff-8b53-4968-beed-4cecd029ce15})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GeovisitStateChangedEventArgs {
    type Vtable = IGeovisitStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for GeovisitStateChangedEventArgs {
    const IID: ::windows::core::GUID = <IGeovisitStateChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GeovisitStateChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Geolocation.GeovisitStateChangedEventArgs";
}
::windows::core::interface_hierarchy!(GeovisitStateChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GeovisitStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for GeovisitStateChangedEventArgs {}
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
#[repr(transparent)]
pub struct GeovisitTriggerDetails(::windows::core::IUnknown);
impl GeovisitTriggerDetails {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadReports(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<Geovisit>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadReports)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for GeovisitTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GeovisitTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeovisitTriggerDetails {}
impl ::core::fmt::Debug for GeovisitTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeovisitTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GeovisitTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.GeovisitTriggerDetails;{ea770d9e-d1c9-454b-99b7-b2f8cdd2482f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GeovisitTriggerDetails {
    type Vtable = IGeovisitTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for GeovisitTriggerDetails {
    const IID: ::windows::core::GUID = <IGeovisitTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GeovisitTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Geolocation.GeovisitTriggerDetails";
}
::windows::core::interface_hierarchy!(GeovisitTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GeovisitTriggerDetails {}
unsafe impl ::core::marker::Sync for GeovisitTriggerDetails {}
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
#[repr(transparent)]
pub struct PositionChangedEventArgs(::windows::core::IUnknown);
impl PositionChangedEventArgs {
    pub fn Position(&self) -> ::windows::core::Result<Geoposition> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Position)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for PositionChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PositionChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PositionChangedEventArgs {}
impl ::core::fmt::Debug for PositionChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PositionChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PositionChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.PositionChangedEventArgs;{37859ce5-9d1e-46c5-bf3b-6ad8cac1a093})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PositionChangedEventArgs {
    type Vtable = IPositionChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for PositionChangedEventArgs {
    const IID: ::windows::core::GUID = <IPositionChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PositionChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Geolocation.PositionChangedEventArgs";
}
::windows::core::interface_hierarchy!(PositionChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PositionChangedEventArgs {}
unsafe impl ::core::marker::Sync for PositionChangedEventArgs {}
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
#[repr(transparent)]
pub struct StatusChangedEventArgs(::windows::core::IUnknown);
impl StatusChangedEventArgs {
    pub fn Status(&self) -> ::windows::core::Result<PositionStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for StatusChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StatusChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StatusChangedEventArgs {}
impl ::core::fmt::Debug for StatusChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StatusChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StatusChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.StatusChangedEventArgs;{3453d2da-8c93-4111-a205-9aecfc9be5c0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StatusChangedEventArgs {
    type Vtable = IStatusChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for StatusChangedEventArgs {
    const IID: ::windows::core::GUID = <IStatusChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StatusChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Geolocation.StatusChangedEventArgs";
}
::windows::core::interface_hierarchy!(StatusChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StatusChangedEventArgs {}
unsafe impl ::core::marker::Sync for StatusChangedEventArgs {}
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
#[repr(transparent)]
pub struct VenueData(::windows::core::IUnknown);
impl VenueData {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Level(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Level)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for VenueData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VenueData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VenueData {}
impl ::core::fmt::Debug for VenueData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VenueData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VenueData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.VenueData;{66f39187-60e3-4b2f-b527-4f53f1c3c677})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for VenueData {
    type Vtable = IVenueData_Vtbl;
}
unsafe impl ::windows::core::Interface for VenueData {
    const IID: ::windows::core::GUID = <IVenueData as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for VenueData {
    const NAME: &'static str = "Windows.Devices.Geolocation.VenueData";
}
::windows::core::interface_hierarchy!(VenueData, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for VenueData {}
unsafe impl ::core::marker::Sync for VenueData {}
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AltitudeReferenceSystem(pub i32);
impl AltitudeReferenceSystem {
    pub const Unspecified: Self = Self(0i32);
    pub const Terrain: Self = Self(1i32);
    pub const Ellipsoid: Self = Self(2i32);
    pub const Geoid: Self = Self(3i32);
    pub const Surface: Self = Self(4i32);
}
impl ::core::marker::Copy for AltitudeReferenceSystem {}
impl ::core::clone::Clone for AltitudeReferenceSystem {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AltitudeReferenceSystem {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AltitudeReferenceSystem {
    type Abi = Self;
}
impl ::core::fmt::Debug for AltitudeReferenceSystem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AltitudeReferenceSystem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AltitudeReferenceSystem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.AltitudeReferenceSystem;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GeolocationAccessStatus(pub i32);
impl GeolocationAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const Denied: Self = Self(2i32);
}
impl ::core::marker::Copy for GeolocationAccessStatus {}
impl ::core::clone::Clone for GeolocationAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GeolocationAccessStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GeolocationAccessStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for GeolocationAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeolocationAccessStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GeolocationAccessStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.GeolocationAccessStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GeoshapeType(pub i32);
impl GeoshapeType {
    pub const Geopoint: Self = Self(0i32);
    pub const Geocircle: Self = Self(1i32);
    pub const Geopath: Self = Self(2i32);
    pub const GeoboundingBox: Self = Self(3i32);
}
impl ::core::marker::Copy for GeoshapeType {}
impl ::core::clone::Clone for GeoshapeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GeoshapeType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GeoshapeType {
    type Abi = Self;
}
impl ::core::fmt::Debug for GeoshapeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeoshapeType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GeoshapeType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.GeoshapeType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PositionAccuracy(pub i32);
impl PositionAccuracy {
    pub const Default: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
impl ::core::marker::Copy for PositionAccuracy {}
impl ::core::clone::Clone for PositionAccuracy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PositionAccuracy {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PositionAccuracy {
    type Abi = Self;
}
impl ::core::fmt::Debug for PositionAccuracy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PositionAccuracy").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PositionAccuracy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.PositionAccuracy;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PositionSource(pub i32);
impl PositionSource {
    pub const Cellular: Self = Self(0i32);
    pub const Satellite: Self = Self(1i32);
    pub const WiFi: Self = Self(2i32);
    pub const IPAddress: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
    pub const Default: Self = Self(5i32);
    pub const Obfuscated: Self = Self(6i32);
}
impl ::core::marker::Copy for PositionSource {}
impl ::core::clone::Clone for PositionSource {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PositionSource {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PositionSource {
    type Abi = Self;
}
impl ::core::fmt::Debug for PositionSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PositionSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PositionSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.PositionSource;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PositionStatus(pub i32);
impl PositionStatus {
    pub const Ready: Self = Self(0i32);
    pub const Initializing: Self = Self(1i32);
    pub const NoData: Self = Self(2i32);
    pub const Disabled: Self = Self(3i32);
    pub const NotInitialized: Self = Self(4i32);
    pub const NotAvailable: Self = Self(5i32);
}
impl ::core::marker::Copy for PositionStatus {}
impl ::core::clone::Clone for PositionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PositionStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PositionStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for PositionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PositionStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PositionStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.PositionStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VisitMonitoringScope(pub i32);
impl VisitMonitoringScope {
    pub const Venue: Self = Self(0i32);
    pub const City: Self = Self(1i32);
}
impl ::core::marker::Copy for VisitMonitoringScope {}
impl ::core::clone::Clone for VisitMonitoringScope {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VisitMonitoringScope {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VisitMonitoringScope {
    type Abi = Self;
}
impl ::core::fmt::Debug for VisitMonitoringScope {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VisitMonitoringScope").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VisitMonitoringScope {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.VisitMonitoringScope;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VisitStateChange(pub i32);
impl VisitStateChange {
    pub const TrackingLost: Self = Self(0i32);
    pub const Arrived: Self = Self(1i32);
    pub const Departed: Self = Self(2i32);
    pub const OtherMovement: Self = Self(3i32);
}
impl ::core::marker::Copy for VisitStateChange {}
impl ::core::clone::Clone for VisitStateChange {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VisitStateChange {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VisitStateChange {
    type Abi = Self;
}
impl ::core::fmt::Debug for VisitStateChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VisitStateChange").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VisitStateChange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.VisitStateChange;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Devices_Geolocation\"`*"]
pub struct BasicGeoposition {
    pub Latitude: f64,
    pub Longitude: f64,
    pub Altitude: f64,
}
impl ::core::marker::Copy for BasicGeoposition {}
impl ::core::clone::Clone for BasicGeoposition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BasicGeoposition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BasicGeoposition").field("Latitude", &self.Latitude).field("Longitude", &self.Longitude).field("Altitude", &self.Altitude).finish()
    }
}
unsafe impl ::windows::core::Abi for BasicGeoposition {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BasicGeoposition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Devices.Geolocation.BasicGeoposition;f8;f8;f8)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for BasicGeoposition {
    fn eq(&self, other: &Self) -> bool {
        self.Latitude == other.Latitude && self.Longitude == other.Longitude && self.Altitude == other.Altitude
    }
}
impl ::core::cmp::Eq for BasicGeoposition {}
impl ::core::default::Default for BasicGeoposition {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
