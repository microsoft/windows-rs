#[cfg(feature = "Devices_PointOfService_Provider")]
pub mod Provider;
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScanner(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBarcodeScanner {
    type Vtable = IBarcodeScanner_Vtbl;
}
unsafe impl ::windows::core::Interface for IBarcodeScanner {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbea33e06_b264_4f03_a9c1_45b20f01134f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScanner_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ClaimScannerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClaimScannerAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CheckHealthAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: UnifiedPosHealthCheckLevel, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckHealthAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedSymbologiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedSymbologiesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub IsSymbologySupportedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, barcodesymbology: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsSymbologySupportedAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub RetrieveStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statisticscategories: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    RetrieveStatisticsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedProfiles: usize,
    pub IsProfileSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profile: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StatusUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusUpdated: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScanner2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBarcodeScanner2 {
    type Vtable = IBarcodeScanner2_Vtbl;
}
unsafe impl ::windows::core::Interface for IBarcodeScanner2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89215167_8cee_436d_89ab_8dfb43bb4286);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScanner2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub VideoDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBarcodeScannerCapabilities {
    type Vtable = IBarcodeScannerCapabilities_Vtbl;
}
unsafe impl ::windows::core::Interface for IBarcodeScannerCapabilities {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc60691e4_f2c8_4420_a307_b12ef6622857);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerCapabilities_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PowerReportingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosPowerReportingType) -> ::windows::core::HRESULT,
    pub IsStatisticsReportingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsStatisticsUpdatingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsImagePreviewSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerCapabilities1(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBarcodeScannerCapabilities1 {
    type Vtable = IBarcodeScannerCapabilities1_Vtbl;
}
unsafe impl ::windows::core::Interface for IBarcodeScannerCapabilities1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8e5ab3e9_0e2c_472f_a1cc_ee8054b6a684);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerCapabilities1_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsSoftwareTriggerSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerCapabilities2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBarcodeScannerCapabilities2 {
    type Vtable = IBarcodeScannerCapabilities2_Vtbl;
}
unsafe impl ::windows::core::Interface for IBarcodeScannerCapabilities2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf211cfec_e1a1_4ea8_9abc_92b1596270ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerCapabilities2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsVideoPreviewSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerDataReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBarcodeScannerDataReceivedEventArgs {
    type Vtable = IBarcodeScannerDataReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IBarcodeScannerDataReceivedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4234a7e2_ed97_467d_ad2b_01e44313a929);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerDataReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Report: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerErrorOccurredEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBarcodeScannerErrorOccurredEventArgs {
    type Vtable = IBarcodeScannerErrorOccurredEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IBarcodeScannerErrorOccurredEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cd2602f_cf3a_4002_a75a_c5ec468f0a20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerErrorOccurredEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PartialInputData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsRetriable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ErrorData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerImagePreviewReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBarcodeScannerImagePreviewReceivedEventArgs {
    type Vtable = IBarcodeScannerImagePreviewReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IBarcodeScannerImagePreviewReceivedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3b7de85_6e8b_434e_9f58_06ef26bc4baf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerImagePreviewReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Preview: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Preview: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerReport(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBarcodeScannerReport {
    type Vtable = IBarcodeScannerReport_Vtbl;
}
unsafe impl ::windows::core::Interface for IBarcodeScannerReport {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ce4d8b0_a489_4b96_86c4_f0bf8a37753d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerReport_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ScanDataType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ScanData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ScanData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ScanDataLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ScanDataLabel: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerReportFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBarcodeScannerReportFactory {
    type Vtable = IBarcodeScannerReportFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IBarcodeScannerReportFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2547326_2013_457c_8963_49c15dca78ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerReportFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scandatatype: u32, scandata: *mut ::core::ffi::c_void, scandatalabel: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateInstance: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBarcodeScannerStatics {
    type Vtable = IBarcodeScannerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IBarcodeScannerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d115f6f_da49_41e8_8c8c_f0cb62a9c4fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBarcodeScannerStatics2 {
    type Vtable = IBarcodeScannerStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IBarcodeScannerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8652473_a36f_4007_b1d0_279ebe92a656);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeviceSelectorWithConnectionTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectiontypes: PosConnectionTypes, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerStatusUpdatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBarcodeScannerStatusUpdatedEventArgs {
    type Vtable = IBarcodeScannerStatusUpdatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IBarcodeScannerStatusUpdatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x355d8586_9c43_462b_a91a_816dc97f452c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStatusUpdatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BarcodeScannerStatus) -> ::windows::core::HRESULT,
    pub ExtendedStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeSymbologiesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBarcodeSymbologiesStatics {
    type Vtable = IBarcodeSymbologiesStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IBarcodeSymbologiesStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca8549bb_06d2_43f4_a44b_c620679fd8d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeSymbologiesStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Unknown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Ean8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Ean8Add2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Ean8Add5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Eanv: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub EanvAdd2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub EanvAdd5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Ean13: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Ean13Add2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Ean13Add5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Isbn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub IsbnAdd5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Ismn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub IsmnAdd2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub IsmnAdd5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Issn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub IssnAdd2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub IssnAdd5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Ean99: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Ean99Add2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Ean99Add5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Upca: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub UpcaAdd2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub UpcaAdd5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Upce: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub UpceAdd2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub UpceAdd5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub UpcCoupon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub TfStd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub TfDis: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub TfInt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub TfInd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub TfMat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub TfIata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Gs1DatabarType1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Gs1DatabarType2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Gs1DatabarType3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Code39: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Code39Ex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Trioptic39: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Code32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Pzn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Code93: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Code93Ex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Code128: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Gs1128: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Gs1128Coupon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub UccEan128: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Sisac: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Isbt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Codabar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Code11: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Msi: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Plessey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Telepen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Code16k: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub CodablockA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub CodablockF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Codablock128: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Code49: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Aztec: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub DataCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub DataMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub HanXin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Maxicode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub MicroPdf417: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub MicroQr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Pdf417: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Qr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub MsTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Ccab: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Ccc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Tlc39: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub AusPost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub CanPost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ChinaPost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub DutchKix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub InfoMail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ItalianPost25: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ItalianPost39: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub JapanPost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub KoreanPost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SwedenPost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub UkPost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub UsIntelligent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub UsIntelligentPkg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub UsPlanet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub UsPostNet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Us4StateFics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub OcrA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub OcrB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Micr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ExtendedBase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scandatatype: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeSymbologiesStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBarcodeSymbologiesStatics2 {
    type Vtable = IBarcodeSymbologiesStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IBarcodeSymbologiesStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8b7518f4_99d0_40bf_9424_b91d6dd4c6e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeSymbologiesStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Gs1DWCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeSymbologyAttributes(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBarcodeSymbologyAttributes {
    type Vtable = IBarcodeSymbologyAttributes_Vtbl;
}
unsafe impl ::windows::core::Interface for IBarcodeSymbologyAttributes {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66413a78_ab7a_4ada_8ece_936014b2ead7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeSymbologyAttributes_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsCheckDigitValidationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsCheckDigitValidationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsCheckDigitValidationSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsCheckDigitTransmissionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsCheckDigitTransmissionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsCheckDigitTransmissionSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub DecodeLength1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetDecodeLength1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub DecodeLength2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetDecodeLength2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub DecodeLengthKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BarcodeSymbologyDecodeLengthKind) -> ::windows::core::HRESULT,
    pub SetDecodeLengthKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: BarcodeSymbologyDecodeLengthKind) -> ::windows::core::HRESULT,
    pub IsDecodeLengthSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICashDrawer(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICashDrawer {
    type Vtable = ICashDrawer_Vtbl;
}
unsafe impl ::windows::core::Interface for ICashDrawer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f88f5c8_de54_4aee_a890_920bcbfe30fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsDrawerOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub DrawerEventSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ClaimDrawerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClaimDrawerAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CheckHealthAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: UnifiedPosHealthCheckLevel, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckHealthAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statisticscategories: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetStatisticsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StatusUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusUpdated: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICashDrawerCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICashDrawerCapabilities {
    type Vtable = ICashDrawerCapabilities_Vtbl;
}
unsafe impl ::windows::core::Interface for ICashDrawerCapabilities {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0bc6de0b_e8e7_4b1f_b1d1_3e501ad08247);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerCapabilities_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PowerReportingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosPowerReportingType) -> ::windows::core::HRESULT,
    pub IsStatisticsReportingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsStatisticsUpdatingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsStatusReportingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsStatusMultiDrawerDetectSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsDrawerOpenSensorAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICashDrawerCloseAlarm(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICashDrawerCloseAlarm {
    type Vtable = ICashDrawerCloseAlarm_Vtbl;
}
unsafe impl ::windows::core::Interface for ICashDrawerCloseAlarm {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6bf88cc7_6f63_430e_ab3b_95d75ffbe87f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerCloseAlarm_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SetAlarmTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAlarmTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub AlarmTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AlarmTimeout: usize,
    pub SetBeepFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub BeepFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetBeepDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBeepDuration: usize,
    #[cfg(feature = "Foundation")]
    pub BeepDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BeepDuration: usize,
    #[cfg(feature = "Foundation")]
    pub SetBeepDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBeepDelay: usize,
    #[cfg(feature = "Foundation")]
    pub BeepDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BeepDelay: usize,
    #[cfg(feature = "Foundation")]
    pub AlarmTimeoutExpired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AlarmTimeoutExpired: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAlarmTimeoutExpired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAlarmTimeoutExpired: usize,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICashDrawerEventSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICashDrawerEventSource {
    type Vtable = ICashDrawerEventSource_Vtbl;
}
unsafe impl ::windows::core::Interface for ICashDrawerEventSource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe006e46c_f2f9_442f_8dd6_06c10a4227ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerEventSource_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub DrawerClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DrawerClosed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDrawerClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDrawerClosed: usize,
    #[cfg(feature = "Foundation")]
    pub DrawerOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DrawerOpened: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDrawerOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDrawerOpened: usize,
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ICashDrawerEventSourceEventArgs(::windows::core::IUnknown);
impl ICashDrawerEventSourceEventArgs {
    pub fn CashDrawer(&self) -> ::windows::core::Result<CashDrawer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CashDrawer)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(ICashDrawerEventSourceEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for ICashDrawerEventSourceEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICashDrawerEventSourceEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICashDrawerEventSourceEventArgs {}
impl ::core::fmt::Debug for ICashDrawerEventSourceEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICashDrawerEventSourceEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICashDrawerEventSourceEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{69cb3bc1-147f-421c-9c23-090123bb786c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ICashDrawerEventSourceEventArgs {
    type Vtable = ICashDrawerEventSourceEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ICashDrawerEventSourceEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69cb3bc1_147f_421c_9c23_090123bb786c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerEventSourceEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CashDrawer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICashDrawerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICashDrawerStatics {
    type Vtable = ICashDrawerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICashDrawerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdfa0955a_d437_4fff_b547_dda969a4f883);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICashDrawerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICashDrawerStatics2 {
    type Vtable = ICashDrawerStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for ICashDrawerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e818121_8c42_40e8_9c0e_40297048104c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeviceSelectorWithConnectionTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectiontypes: PosConnectionTypes, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICashDrawerStatus(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICashDrawerStatus {
    type Vtable = ICashDrawerStatus_Vtbl;
}
unsafe impl ::windows::core::Interface for ICashDrawerStatus {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6bbd78bf_dca1_4e06_99eb_5af6a5aec108);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerStatus_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub StatusKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CashDrawerStatusKind) -> ::windows::core::HRESULT,
    pub ExtendedStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICashDrawerStatusUpdatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICashDrawerStatusUpdatedEventArgs {
    type Vtable = ICashDrawerStatusUpdatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ICashDrawerStatusUpdatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30aae98a_0d70_459c_9553_87e124c52488);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerStatusUpdatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedBarcodeScanner(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IClaimedBarcodeScanner {
    type Vtable = IClaimedBarcodeScanner_Vtbl;
}
unsafe impl ::windows::core::Interface for IClaimedBarcodeScanner {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a63b49c_8fa4_4332_bb26_945d11d81e0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsDisabledOnDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsDisabledOnDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsDecodeDataEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsDecodeDataEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EnableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DisableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisableAsync: usize,
    pub RetainDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetActiveSymbologiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, symbologies: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetActiveSymbologiesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ResetStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statisticscategories: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResetStatisticsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UpdateStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statistics: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UpdateStatisticsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SetActiveProfileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetActiveProfileAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DataReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDataReceived: usize,
    #[cfg(feature = "Foundation")]
    pub TriggerPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TriggerPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTriggerPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTriggerPressed: usize,
    #[cfg(feature = "Foundation")]
    pub TriggerReleased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TriggerReleased: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTriggerReleased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTriggerReleased: usize,
    #[cfg(feature = "Foundation")]
    pub ReleaseDeviceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReleaseDeviceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReleaseDeviceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReleaseDeviceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub ImagePreviewReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImagePreviewReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveImagePreviewReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveImagePreviewReceived: usize,
    #[cfg(feature = "Foundation")]
    pub ErrorOccurred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ErrorOccurred: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveErrorOccurred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveErrorOccurred: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedBarcodeScanner1(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IClaimedBarcodeScanner1 {
    type Vtable = IClaimedBarcodeScanner1_Vtbl;
}
unsafe impl ::windows::core::Interface for IClaimedBarcodeScanner1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf61aad0c_8551_42b4_998c_970c20210a22);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner1_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub StartSoftwareTriggerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartSoftwareTriggerAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopSoftwareTriggerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopSoftwareTriggerAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedBarcodeScanner2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IClaimedBarcodeScanner2 {
    type Vtable = IClaimedBarcodeScanner2_Vtbl;
}
unsafe impl ::windows::core::Interface for IClaimedBarcodeScanner2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3b59e8c_2d8b_4f70_8af3_3448bedd5fe2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetSymbologyAttributesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, barcodesymbology: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetSymbologyAttributesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SetSymbologyAttributesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, barcodesymbology: u32, attributes: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSymbologyAttributesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedBarcodeScanner3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IClaimedBarcodeScanner3 {
    type Vtable = IClaimedBarcodeScanner3_Vtbl;
}
unsafe impl ::windows::core::Interface for IClaimedBarcodeScanner3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6ceb430_712e_45fc_8b86_cd55f5aef79d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ShowVideoPreviewAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowVideoPreviewAsync: usize,
    pub HideVideoPreview: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetIsVideoPreviewShownOnEnable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsVideoPreviewShownOnEnable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedBarcodeScanner4(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IClaimedBarcodeScanner4 {
    type Vtable = IClaimedBarcodeScanner4_Vtbl;
}
unsafe impl ::windows::core::Interface for IClaimedBarcodeScanner4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d501f97_376a_41a8_a230_2f37c1949dde);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedBarcodeScannerClosedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IClaimedBarcodeScannerClosedEventArgs {
    type Vtable = IClaimedBarcodeScannerClosedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IClaimedBarcodeScannerClosedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf7d5489_a22c_4c65_a901_88d77d833954);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedBarcodeScannerClosedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedCashDrawer(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IClaimedCashDrawer {
    type Vtable = IClaimedCashDrawer_Vtbl;
}
unsafe impl ::windows::core::Interface for IClaimedCashDrawer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca3f99af_abb8_42c1_8a84_5c66512f5a75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedCashDrawer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsDrawerOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CloseAlarm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub OpenDrawerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenDrawerAsync: usize,
    #[cfg(feature = "Foundation")]
    pub EnableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DisableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RetainDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RetainDeviceAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ResetStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statisticscategories: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResetStatisticsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UpdateStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statistics: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UpdateStatisticsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReleaseDeviceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReleaseDeviceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReleaseDeviceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReleaseDeviceRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedCashDrawer2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IClaimedCashDrawer2 {
    type Vtable = IClaimedCashDrawer2_Vtbl;
}
unsafe impl ::windows::core::Interface for IClaimedCashDrawer2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9cbab5a2_de42_4d5b_b0c1_9b57a2ba89c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedCashDrawer2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedCashDrawerClosedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IClaimedCashDrawerClosedEventArgs {
    type Vtable = IClaimedCashDrawerClosedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IClaimedCashDrawerClosedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc573f33_3f34_4c5c_baae_deadf16cd7fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedCashDrawerClosedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedJournalPrinter(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IClaimedJournalPrinter {
    type Vtable = IClaimedJournalPrinter_Vtbl;
}
unsafe impl ::windows::core::Interface for IClaimedJournalPrinter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x67ea0630_517d_487f_9fdf_d2e0a0a264a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedJournalPrinter_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedLineDisplay(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IClaimedLineDisplay {
    type Vtable = IClaimedLineDisplay_Vtbl;
}
unsafe impl ::windows::core::Interface for IClaimedLineDisplay {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x120ac970_9a75_4acf_aae7_09972bcf8794);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedLineDisplay_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PhysicalDeviceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PhysicalDeviceDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DeviceControlDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DeviceControlVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DeviceServiceVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DefaultWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RetainDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReleaseDeviceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReleaseDeviceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReleaseDeviceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReleaseDeviceRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedLineDisplay2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IClaimedLineDisplay2 {
    type Vtable = IClaimedLineDisplay2_Vtbl;
}
unsafe impl ::windows::core::Interface for IClaimedLineDisplay2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa31c75ed_41f5_4e76_a074_795e47a46e97);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedLineDisplay2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statisticscategories: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetStatisticsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CheckHealthAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: UnifiedPosHealthCheckLevel, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckHealthAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CheckPowerStatusAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckPowerStatusAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StatusUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusUpdated: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedScreenSizesInCharacters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedScreenSizesInCharacters: usize,
    #[cfg(feature = "Foundation")]
    pub MaxBitmapSizeInPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxBitmapSizeInPixels: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCharacterSets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCharacterSets: usize,
    pub CustomGlyphs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryUpdateAttributesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributes: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryUpdateAttributesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TrySetDescriptorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptor: u32, descriptorstate: LineDisplayDescriptorState, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySetDescriptorAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryClearDescriptorsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryClearDescriptorsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryCreateWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: super::super::Foundation::Rect, windowsize: super::super::Foundation::Size, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryCreateWindowAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub TryStoreStorageFileBitmapAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    TryStoreStorageFileBitmapAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub TryStoreStorageFileBitmapWithAlignmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    TryStoreStorageFileBitmapWithAlignmentAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub TryStoreStorageFileBitmapWithAlignmentAndWidthAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, widthinpixels: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    TryStoreStorageFileBitmapWithAlignmentAndWidthAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedLineDisplay3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IClaimedLineDisplay3 {
    type Vtable = IClaimedLineDisplay3_Vtbl;
}
unsafe impl ::windows::core::Interface for IClaimedLineDisplay3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x642ecd92_e9d4_4ecc_af75_329c274cd18f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedLineDisplay3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedLineDisplayClosedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IClaimedLineDisplayClosedEventArgs {
    type Vtable = IClaimedLineDisplayClosedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IClaimedLineDisplayClosedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf915f364_d3d5_4f10_b511_90939edfacd8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedLineDisplayClosedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedLineDisplayStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IClaimedLineDisplayStatics {
    type Vtable = IClaimedLineDisplayStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IClaimedLineDisplayStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78ca98fb_8b6b_4973_86f0_3e570c351825);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedLineDisplayStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDeviceSelectorWithConnectionTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectiontypes: PosConnectionTypes, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedMagneticStripeReader(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IClaimedMagneticStripeReader {
    type Vtable = IClaimedMagneticStripeReader_Vtbl;
}
unsafe impl ::windows::core::Interface for IClaimedMagneticStripeReader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x475ca8f3_9417_48bc_b9d7_4163a7844c02);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedMagneticStripeReader_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsDisabledOnDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsDisabledOnDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsDecodeDataEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsDecodeDataEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsDeviceAuthenticated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetDataEncryptionAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub DataEncryptionAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetTracksToRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MagneticStripeReaderTrackIds) -> ::windows::core::HRESULT,
    pub TracksToRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderTrackIds) -> ::windows::core::HRESULT,
    pub SetIsTransmitSentinelsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsTransmitSentinelsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EnableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DisableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisableAsync: usize,
    pub RetainDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetErrorReportingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MagneticStripeReaderErrorReportingType) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub RetrieveDeviceAuthenticationDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    RetrieveDeviceAuthenticationDataAsync: usize,
    #[cfg(feature = "Foundation")]
    pub AuthenticateDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, responseToken_array_size: u32, responsetoken: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AuthenticateDeviceAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeAuthenticateDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, responseToken_array_size: u32, responsetoken: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeAuthenticateDeviceAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateKeyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut ::core::ffi::c_void, keyname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateKeyAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ResetStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statisticscategories: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResetStatisticsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UpdateStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statistics: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UpdateStatisticsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub BankCardDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BankCardDataReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBankCardDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBankCardDataReceived: usize,
    #[cfg(feature = "Foundation")]
    pub AamvaCardDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AamvaCardDataReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAamvaCardDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAamvaCardDataReceived: usize,
    #[cfg(feature = "Foundation")]
    pub VendorSpecificDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VendorSpecificDataReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVendorSpecificDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVendorSpecificDataReceived: usize,
    #[cfg(feature = "Foundation")]
    pub ReleaseDeviceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReleaseDeviceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReleaseDeviceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReleaseDeviceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub ErrorOccurred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ErrorOccurred: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveErrorOccurred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveErrorOccurred: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedMagneticStripeReader2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IClaimedMagneticStripeReader2 {
    type Vtable = IClaimedMagneticStripeReader2_Vtbl;
}
unsafe impl ::windows::core::Interface for IClaimedMagneticStripeReader2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x236fafdf_e2dc_4d7d_9c78_060df2bf2928);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedMagneticStripeReader2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedMagneticStripeReaderClosedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IClaimedMagneticStripeReaderClosedEventArgs {
    type Vtable = IClaimedMagneticStripeReaderClosedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IClaimedMagneticStripeReaderClosedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14ada93a_adcd_4c80_acda_c3eaed2647e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedMagneticStripeReaderClosedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedPosPrinter(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IClaimedPosPrinter {
    type Vtable = IClaimedPosPrinter_Vtbl;
}
unsafe impl ::windows::core::Interface for IClaimedPosPrinter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d64ce0c_e03e_4b14_a38e_c28c34b86353);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedPosPrinter_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCharacterSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub CharacterSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub IsCoverOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsCharacterSetMappingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsCharacterSetMappingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetMapMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PosPrinterMapMode) -> ::windows::core::HRESULT,
    pub MapMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterMapMode) -> ::windows::core::HRESULT,
    pub Receipt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Slip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Journal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EnableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DisableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RetainDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RetainDeviceAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ResetStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statisticscategories: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResetStatisticsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UpdateStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statistics: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UpdateStatisticsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReleaseDeviceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReleaseDeviceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReleaseDeviceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReleaseDeviceRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedPosPrinter2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IClaimedPosPrinter2 {
    type Vtable = IClaimedPosPrinter2_Vtbl;
}
unsafe impl ::windows::core::Interface for IClaimedPosPrinter2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5bf7a3d5_5198_437a_82df_589993fa77e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedPosPrinter2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedPosPrinterClosedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IClaimedPosPrinterClosedEventArgs {
    type Vtable = IClaimedPosPrinterClosedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IClaimedPosPrinterClosedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2b7a27b_4d40_471d_92ed_63375b18c788);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedPosPrinterClosedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedReceiptPrinter(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IClaimedReceiptPrinter {
    type Vtable = IClaimedReceiptPrinter_Vtbl;
}
unsafe impl ::windows::core::Interface for IClaimedReceiptPrinter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ad27a74_dd61_4ee2_9837_5b5d72d538b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedReceiptPrinter_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SidewaysMaxLines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SidewaysMaxChars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub LinesToPaperCut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PageSize: usize,
    #[cfg(feature = "Foundation")]
    pub PrintArea: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrintArea: usize,
    pub CreateJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedSlipPrinter(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IClaimedSlipPrinter {
    type Vtable = IClaimedSlipPrinter_Vtbl;
}
unsafe impl ::windows::core::Interface for IClaimedSlipPrinter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd5deff2_af90_4e8a_b77b_e3ae9ca63a7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedSlipPrinter_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SidewaysMaxLines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SidewaysMaxChars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub MaxLines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub LinesNearEndToEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub PrintSide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterPrintSide) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PageSize: usize,
    #[cfg(feature = "Foundation")]
    pub PrintArea: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrintArea: usize,
    pub OpenJaws: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CloseJaws: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub InsertSlipAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeout: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InsertSlipAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSlipAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeout: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSlipAsync: usize,
    pub ChangePrintSide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printside: PosPrinterPrintSide) -> ::windows::core::HRESULT,
    pub CreateJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ICommonClaimedPosPrinterStation(::windows::core::IUnknown);
impl ICommonClaimedPosPrinterStation {
    pub fn SetCharactersPerLine(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCharactersPerLine)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn CharactersPerLine(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CharactersPerLine)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLineHeight(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLineHeight)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn LineHeight(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LineHeight)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLineSpacing(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLineSpacing)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn LineSpacing(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LineSpacing)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LineWidth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LineWidth)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsLetterQuality(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsLetterQuality)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsLetterQuality(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsLetterQuality)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsPaperNearEnd(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPaperNearEnd)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetColorCartridge(&self, value: PosPrinterColorCartridge) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetColorCartridge)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ColorCartridge(&self) -> ::windows::core::Result<PosPrinterColorCartridge> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ColorCartridge)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsCoverOpen(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCoverOpen)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsCartridgeRemoved(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCartridgeRemoved)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsCartridgeEmpty(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCartridgeEmpty)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsHeadCleaning(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsHeadCleaning)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsPaperEmpty(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPaperEmpty)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsReadyToPrint(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsReadyToPrint)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ValidateData(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ValidateData)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(data), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(ICommonClaimedPosPrinterStation, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for ICommonClaimedPosPrinterStation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICommonClaimedPosPrinterStation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICommonClaimedPosPrinterStation {}
impl ::core::fmt::Debug for ICommonClaimedPosPrinterStation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICommonClaimedPosPrinterStation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICommonClaimedPosPrinterStation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{b7eb66a8-fe8a-4cfb-8b42-e35b280cb27c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ICommonClaimedPosPrinterStation {
    type Vtable = ICommonClaimedPosPrinterStation_Vtbl;
}
unsafe impl ::windows::core::Interface for ICommonClaimedPosPrinterStation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7eb66a8_fe8a_4cfb_8b42_e35b280cb27c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommonClaimedPosPrinterStation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetCharactersPerLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub CharactersPerLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetLineHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub LineHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetLineSpacing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub LineSpacing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub LineWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetIsLetterQuality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsLetterQuality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsPaperNearEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetColorCartridge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PosPrinterColorCartridge) -> ::windows::core::HRESULT,
    pub ColorCartridge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterColorCartridge) -> ::windows::core::HRESULT,
    pub IsCoverOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsCartridgeRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsCartridgeEmpty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsHeadCleaning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsPaperEmpty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsReadyToPrint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ValidateData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ICommonPosPrintStationCapabilities(::windows::core::IUnknown);
impl ICommonPosPrintStationCapabilities {
    pub fn IsPrinterPresent(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPrinterPresent)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsDualColorSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDualColorSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ColorCartridgeCapabilities(&self) -> ::windows::core::Result<PosPrinterColorCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ColorCartridgeCapabilities)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CartridgeSensors(&self) -> ::windows::core::Result<PosPrinterCartridgeSensors> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CartridgeSensors)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsBoldSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsBoldSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsItalicSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsItalicSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsUnderlineSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsUnderlineSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsDoubleHighPrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDoubleHighPrintSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDoubleWidePrintSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsDoubleHighDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDoubleHighDoubleWidePrintSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsPaperEmptySensorSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPaperEmptySensorSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsPaperNearEndSensorSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPaperNearEndSensorSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCharactersPerLine(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedCharactersPerLine)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(ICommonPosPrintStationCapabilities, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for ICommonPosPrintStationCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICommonPosPrintStationCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICommonPosPrintStationCapabilities {}
impl ::core::fmt::Debug for ICommonPosPrintStationCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICommonPosPrintStationCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICommonPosPrintStationCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{de5b52ca-e02e-40e9-9e5e-1b488e6aacfc}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ICommonPosPrintStationCapabilities {
    type Vtable = ICommonPosPrintStationCapabilities_Vtbl;
}
unsafe impl ::windows::core::Interface for ICommonPosPrintStationCapabilities {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde5b52ca_e02e_40e9_9e5e_1b488e6aacfc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommonPosPrintStationCapabilities_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsPrinterPresent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsDualColorSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ColorCartridgeCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterColorCapabilities) -> ::windows::core::HRESULT,
    pub CartridgeSensors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterCartridgeSensors) -> ::windows::core::HRESULT,
    pub IsBoldSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsItalicSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsUnderlineSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsDoubleHighPrintSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsDoubleWidePrintSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsDoubleHighDoubleWidePrintSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsPaperEmptySensorSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsPaperNearEndSensorSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCharactersPerLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCharactersPerLine: usize,
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ICommonReceiptSlipCapabilities(::windows::core::IUnknown);
impl ICommonReceiptSlipCapabilities {
    pub fn IsBarcodeSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsBarcodeSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsBitmapSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsBitmapSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsLeft90RotationSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsLeft90RotationSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsRight90RotationSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsRight90RotationSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Is180RotationSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Is180RotationSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsPrintAreaSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPrintAreaSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RuledLineCapabilities(&self) -> ::windows::core::Result<PosPrinterRuledLineCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RuledLineCapabilities)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedBarcodeRotations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedBarcodeRotations)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedBitmapRotations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedBitmapRotations)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsPrinterPresent(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPrinterPresent)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsDualColorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDualColorSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ColorCartridgeCapabilities(&self) -> ::windows::core::Result<PosPrinterColorCapabilities> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ColorCartridgeCapabilities)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CartridgeSensors(&self) -> ::windows::core::Result<PosPrinterCartridgeSensors> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CartridgeSensors)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsBoldSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsBoldSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsItalicSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsItalicSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsUnderlineSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsUnderlineSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsDoubleHighPrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDoubleHighPrintSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDoubleWidePrintSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsDoubleHighDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDoubleHighDoubleWidePrintSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsPaperEmptySensorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPaperEmptySensorSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsPaperNearEndSensorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPaperNearEndSensorSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCharactersPerLine(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedCharactersPerLine)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(ICommonReceiptSlipCapabilities, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<ICommonReceiptSlipCapabilities> for ICommonPosPrintStationCapabilities {
    type Error = ::windows::core::Error;
    fn try_from(value: ICommonReceiptSlipCapabilities) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ICommonReceiptSlipCapabilities> for ICommonPosPrintStationCapabilities {
    type Error = ::windows::core::Error;
    fn try_from(value: &ICommonReceiptSlipCapabilities) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&ICommonReceiptSlipCapabilities> for ::windows::core::InParam<ICommonPosPrintStationCapabilities> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ICommonReceiptSlipCapabilities) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for ICommonReceiptSlipCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICommonReceiptSlipCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICommonReceiptSlipCapabilities {}
impl ::core::fmt::Debug for ICommonReceiptSlipCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICommonReceiptSlipCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICommonReceiptSlipCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{09286b8b-9873-4d05-bfbe-4727a6038f69}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ICommonReceiptSlipCapabilities {
    type Vtable = ICommonReceiptSlipCapabilities_Vtbl;
}
unsafe impl ::windows::core::Interface for ICommonReceiptSlipCapabilities {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09286b8b_9873_4d05_bfbe_4727a6038f69);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommonReceiptSlipCapabilities_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsBarcodeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsBitmapSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsLeft90RotationSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsRight90RotationSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Is180RotationSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsPrintAreaSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub RuledLineCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterRuledLineCapabilities) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedBarcodeRotations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedBarcodeRotations: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedBitmapRotations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedBitmapRotations: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJournalPrintJob(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IJournalPrintJob {
    type Vtable = IJournalPrintJob_Vtbl;
}
unsafe impl ::windows::core::Interface for IJournalPrintJob {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f4f2864_f3f0_55d0_8c39_74cc91783eed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJournalPrintJob_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Print: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, printoptions: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FeedPaperByLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linecount: i32) -> ::windows::core::HRESULT,
    pub FeedPaperByMapModeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, distance: i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJournalPrinterCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IJournalPrinterCapabilities {
    type Vtable = IJournalPrinterCapabilities_Vtbl;
}
unsafe impl ::windows::core::Interface for IJournalPrinterCapabilities {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b5ccc43_e047_4463_bb58_17b5ba1d8056);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJournalPrinterCapabilities_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJournalPrinterCapabilities2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IJournalPrinterCapabilities2 {
    type Vtable = IJournalPrinterCapabilities2_Vtbl;
}
unsafe impl ::windows::core::Interface for IJournalPrinterCapabilities2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03b0b645_33b8_533b_baaa_a4389283ab0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJournalPrinterCapabilities2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsReverseVideoSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsStrikethroughSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsSuperscriptSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsSubscriptSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsReversePaperFeedByLineSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsReversePaperFeedByMapModeUnitSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplay(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILineDisplay {
    type Vtable = ILineDisplay_Vtbl;
}
unsafe impl ::windows::core::Interface for ILineDisplay {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24f5df4e_3c99_44e2_b73f_e51be3637a8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplay_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PhysicalDeviceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PhysicalDeviceDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DeviceControlDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DeviceControlVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DeviceServiceVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ClaimAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClaimAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplay2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILineDisplay2 {
    type Vtable = ILineDisplay2_Vtbl;
}
unsafe impl ::windows::core::Interface for ILineDisplay2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc296a628_ef44_40f3_bd1c_b04c6a5cdc7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplay2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CheckPowerStatusAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckPowerStatusAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayAttributes(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILineDisplayAttributes {
    type Vtable = ILineDisplayAttributes_Vtbl;
}
unsafe impl ::windows::core::Interface for ILineDisplayAttributes {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc17de99c_229a_4c14_a6f1_b4e4b1fead92);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayAttributes_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsPowerNotifyEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsPowerNotifyEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Brightness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetBrightness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub BlinkRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BlinkRate: usize,
    #[cfg(feature = "Foundation")]
    pub SetBlinkRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBlinkRate: usize,
    #[cfg(feature = "Foundation")]
    pub ScreenSizeInCharacters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScreenSizeInCharacters: usize,
    #[cfg(feature = "Foundation")]
    pub SetScreenSizeInCharacters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetScreenSizeInCharacters: usize,
    pub CharacterSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetCharacterSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub IsCharacterSetMappingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsCharacterSetMappingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CurrentWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCurrentWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILineDisplayCapabilities {
    type Vtable = ILineDisplayCapabilities_Vtbl;
}
unsafe impl ::windows::core::Interface for ILineDisplayCapabilities {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a15b5d1_8dc5_4b9c_9172_303e47b70c55);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayCapabilities_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsStatisticsReportingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsStatisticsUpdatingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub PowerReportingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosPowerReportingType) -> ::windows::core::HRESULT,
    pub CanChangeScreenSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanDisplayBitmaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanReadCharacterAtCursor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanMapCharacterSets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanDisplayCustomGlyphs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanReverse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LineDisplayTextAttributeGranularity) -> ::windows::core::HRESULT,
    pub CanBlink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LineDisplayTextAttributeGranularity) -> ::windows::core::HRESULT,
    pub CanChangeBlinkRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsBrightnessSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsCursorSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsHorizontalMarqueeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsVerticalMarqueeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsInterCharacterWaitSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SupportedDescriptors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SupportedWindows: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayCursor(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILineDisplayCursor {
    type Vtable = ILineDisplayCursor_Vtbl;
}
unsafe impl ::windows::core::Interface for ILineDisplayCursor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecdffc45_754a_4e3b_ab2b_151181085605);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayCursor_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CanCustomize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsBlinkSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsBlockSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsHalfBlockSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsUnderlineSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsReverseSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsOtherSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub GetAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryUpdateAttributesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributes: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryUpdateAttributesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayCursorAttributes(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILineDisplayCursorAttributes {
    type Vtable = ILineDisplayCursorAttributes_Vtbl;
}
unsafe impl ::windows::core::Interface for ILineDisplayCursorAttributes {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e2d54fe_4ffd_4190_aae1_ce285f20c896);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayCursorAttributes_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsBlinkEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsBlinkEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CursorType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LineDisplayCursorType) -> ::windows::core::HRESULT,
    pub SetCursorType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: LineDisplayCursorType) -> ::windows::core::HRESULT,
    pub IsAutoAdvanceEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsAutoAdvanceEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPosition: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayCustomGlyphs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILineDisplayCustomGlyphs {
    type Vtable = ILineDisplayCustomGlyphs_Vtbl;
}
unsafe impl ::windows::core::Interface for ILineDisplayCustomGlyphs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2257f63c_f263_44f1_a1a0_e750a6a0ec54);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayCustomGlyphs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SizeInPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SizeInPixels: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedGlyphCodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedGlyphCodes: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub TryRedefineAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphcode: u32, glyphdata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    TryRedefineAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayMarquee(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILineDisplayMarquee {
    type Vtable = ILineDisplayMarquee_Vtbl;
}
unsafe impl ::windows::core::Interface for ILineDisplayMarquee {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3d33e3e_f46a_4b7a_bc21_53eb3b57f8b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayMarquee_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LineDisplayMarqueeFormat) -> ::windows::core::HRESULT,
    pub SetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: LineDisplayMarqueeFormat) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RepeatWaitInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RepeatWaitInterval: usize,
    #[cfg(feature = "Foundation")]
    pub SetRepeatWaitInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRepeatWaitInterval: usize,
    #[cfg(feature = "Foundation")]
    pub ScrollWaitInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScrollWaitInterval: usize,
    #[cfg(feature = "Foundation")]
    pub SetScrollWaitInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetScrollWaitInterval: usize,
    #[cfg(feature = "Foundation")]
    pub TryStartScrollingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, direction: LineDisplayScrollDirection, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryStartScrollingAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryStopScrollingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryStopScrollingAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILineDisplayStatics {
    type Vtable = ILineDisplayStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ILineDisplayStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x022dc0b6_11b0_4690_9547_0b39c5af2114);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDeviceSelectorWithConnectionTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectiontypes: PosConnectionTypes, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILineDisplayStatics2 {
    type Vtable = ILineDisplayStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for ILineDisplayStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x600c3f1c_77ab_4968_a7de_c02ff169f2cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub StatisticsCategorySelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayStatisticsCategorySelector(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILineDisplayStatisticsCategorySelector {
    type Vtable = ILineDisplayStatisticsCategorySelector_Vtbl;
}
unsafe impl ::windows::core::Interface for ILineDisplayStatisticsCategorySelector {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb521c46b_9274_4d24_94f3_b6017b832444);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayStatisticsCategorySelector_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AllStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UnifiedPosStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ManufacturerStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayStatusUpdatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILineDisplayStatusUpdatedEventArgs {
    type Vtable = ILineDisplayStatusUpdatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ILineDisplayStatusUpdatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xddd57c1a_86fb_4eba_93d1_6f5eda52b752);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayStatusUpdatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LineDisplayPowerStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayStoredBitmap(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILineDisplayStoredBitmap {
    type Vtable = ILineDisplayStoredBitmap_Vtbl;
}
unsafe impl ::windows::core::Interface for ILineDisplayStoredBitmap {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf621515b_d81e_43ba_bf1b_bcfa3c785ba0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayStoredBitmap_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub EscapeSequence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryDeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryDeleteAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayWindow(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILineDisplayWindow {
    type Vtable = ILineDisplayWindow_Vtbl;
}
unsafe impl ::windows::core::Interface for ILineDisplayWindow {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd21feef4_2364_4be5_bee1_851680af4964);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayWindow_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SizeInCharacters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SizeInCharacters: usize,
    #[cfg(feature = "Foundation")]
    pub InterCharacterWaitInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InterCharacterWaitInterval: usize,
    #[cfg(feature = "Foundation")]
    pub SetInterCharacterWaitInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInterCharacterWaitInterval: usize,
    #[cfg(feature = "Foundation")]
    pub TryRefreshAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryRefreshAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryDisplayTextAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: *mut ::core::ffi::c_void, displayattribute: LineDisplayTextAttribute, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryDisplayTextAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryDisplayTextAtPositionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: *mut ::core::ffi::c_void, displayattribute: LineDisplayTextAttribute, startposition: super::super::Foundation::Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryDisplayTextAtPositionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryDisplayTextNormalAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryDisplayTextNormalAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryScrollTextAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, direction: LineDisplayScrollDirection, numberofcolumnsorrows: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryScrollTextAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryClearTextAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryClearTextAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayWindow2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILineDisplayWindow2 {
    type Vtable = ILineDisplayWindow2_Vtbl;
}
unsafe impl ::windows::core::Interface for ILineDisplayWindow2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa95ce2e6_bdd8_4365_8e11_de94de8dff02);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayWindow2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Cursor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Marquee: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadCharacterAtCursorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadCharacterAtCursorAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryDisplayStoredBitmapAtCursorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryDisplayStoredBitmapAtCursorAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub TryDisplayStorageFileBitmapAtCursorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    TryDisplayStorageFileBitmapAtCursorAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub TryDisplayStorageFileBitmapAtCursorWithAlignmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    TryDisplayStorageFileBitmapAtCursorWithAlignmentAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub TryDisplayStorageFileBitmapAtCursorWithAlignmentAndWidthAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, widthinpixels: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    TryDisplayStorageFileBitmapAtCursorWithAlignmentAndWidthAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub TryDisplayStorageFileBitmapAtPointAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, offsetinpixels: super::super::Foundation::Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    TryDisplayStorageFileBitmapAtPointAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub TryDisplayStorageFileBitmapAtPointWithWidthAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, offsetinpixels: super::super::Foundation::Point, widthinpixels: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    TryDisplayStorageFileBitmapAtPointWithWidthAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReader(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMagneticStripeReader {
    type Vtable = IMagneticStripeReader_Vtbl;
}
unsafe impl ::windows::core::Interface for IMagneticStripeReader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a92b015_47c3_468a_9333_0c6517574883);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReader_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SupportedCardTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u32) -> ::windows::core::HRESULT,
    pub DeviceAuthenticationProtocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderAuthenticationProtocol) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CheckHealthAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: UnifiedPosHealthCheckLevel, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckHealthAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ClaimReaderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClaimReaderAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub RetrieveStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statisticscategories: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    RetrieveStatisticsAsync: usize,
    pub GetErrorReportingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderErrorReportingType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StatusUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusUpdated: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderAamvaCardDataReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMagneticStripeReaderAamvaCardDataReceivedEventArgs {
    type Vtable = IMagneticStripeReaderAamvaCardDataReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IMagneticStripeReaderAamvaCardDataReceivedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a4bbd51_c316_4910_87f3_7a62ba862d31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderAamvaCardDataReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Report: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LicenseNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Restrictions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Class: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Endorsements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BirthDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FirstName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Surname: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Suffix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Gender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HairColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EyeColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Weight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub City: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PostalCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderBankCardDataReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMagneticStripeReaderBankCardDataReceivedEventArgs {
    type Vtable = IMagneticStripeReaderBankCardDataReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IMagneticStripeReaderBankCardDataReceivedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e958823_a31a_4763_882c_23725e39b08e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderBankCardDataReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Report: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AccountNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ServiceCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FirstName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MiddleInitial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Surname: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Suffix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMagneticStripeReaderCapabilities {
    type Vtable = IMagneticStripeReaderCapabilities_Vtbl;
}
unsafe impl ::windows::core::Interface for IMagneticStripeReaderCapabilities {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7128809c_c440_44a2_a467_469175d02896);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderCapabilities_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CardAuthentication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SupportedEncryptionAlgorithms: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub AuthenticationLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderAuthenticationLevel) -> ::windows::core::HRESULT,
    pub IsIsoSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsJisOneSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsJisTwoSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub PowerReportingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosPowerReportingType) -> ::windows::core::HRESULT,
    pub IsStatisticsReportingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsStatisticsUpdatingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsTrackDataMaskingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsTransmitSentinelsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderCardTypesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMagneticStripeReaderCardTypesStatics {
    type Vtable = IMagneticStripeReaderCardTypesStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IMagneticStripeReaderCardTypesStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x528f2c5d_2986_474f_8454_7ccd05928d5f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderCardTypesStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Unknown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Bank: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Aamva: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ExtendedBase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderEncryptionAlgorithmsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMagneticStripeReaderEncryptionAlgorithmsStatics {
    type Vtable = IMagneticStripeReaderEncryptionAlgorithmsStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IMagneticStripeReaderEncryptionAlgorithmsStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53b57350_c3db_4754_9c00_41392374a109);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderEncryptionAlgorithmsStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub None: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub TripleDesDukpt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ExtendedBase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderErrorOccurredEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMagneticStripeReaderErrorOccurredEventArgs {
    type Vtable = IMagneticStripeReaderErrorOccurredEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IMagneticStripeReaderErrorOccurredEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1fedf95d_2c84_41ad_b778_f2356a789ab1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderErrorOccurredEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Track1Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderTrackErrorType) -> ::windows::core::HRESULT,
    pub Track2Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderTrackErrorType) -> ::windows::core::HRESULT,
    pub Track3Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderTrackErrorType) -> ::windows::core::HRESULT,
    pub Track4Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderTrackErrorType) -> ::windows::core::HRESULT,
    pub ErrorData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PartialInputData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderReport(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMagneticStripeReaderReport {
    type Vtable = IMagneticStripeReaderReport_Vtbl;
}
unsafe impl ::windows::core::Interface for IMagneticStripeReaderReport {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a5b6047_99b0_4188_bef1_eddf79f78fe6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderReport_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CardType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Track1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Track2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Track3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Track4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CardAuthenticationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CardAuthenticationData: usize,
    pub CardAuthenticationDataLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub AdditionalSecurityInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AdditionalSecurityInformation: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMagneticStripeReaderStatics {
    type Vtable = IMagneticStripeReaderStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IMagneticStripeReaderStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc45fab4a_efd7_4760_a5ce_15b0e47e94eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMagneticStripeReaderStatics2 {
    type Vtable = IMagneticStripeReaderStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IMagneticStripeReaderStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8cadc362_d667_48fa_86bc_f5ae1189262b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeviceSelectorWithConnectionTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectiontypes: PosConnectionTypes, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderStatusUpdatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMagneticStripeReaderStatusUpdatedEventArgs {
    type Vtable = IMagneticStripeReaderStatusUpdatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IMagneticStripeReaderStatusUpdatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09cc6bb0_3262_401d_9e8a_e80d6358906b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderStatusUpdatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderStatus) -> ::windows::core::HRESULT,
    pub ExtendedStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderTrackData(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMagneticStripeReaderTrackData {
    type Vtable = IMagneticStripeReaderTrackData_Vtbl;
}
unsafe impl ::windows::core::Interface for IMagneticStripeReaderTrackData {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x104cf671_4a9d_446e_abc5_20402307ba36);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderTrackData_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    #[cfg(feature = "Storage_Streams")]
    pub DiscretionaryData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DiscretionaryData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub EncryptedData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    EncryptedData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    type Vtable = IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf0a5514_59cc_4a60_99e8_99a53dace5aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Report: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPosPrinter(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPosPrinter {
    type Vtable = IPosPrinter_Vtbl;
}
unsafe impl ::windows::core::Interface for IPosPrinter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a03c10e_9a19_4a01_994f_12dfad6adcbf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinter_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCharacterSets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCharacterSets: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedTypeFaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedTypeFaces: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ClaimPrinterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClaimPrinterAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CheckHealthAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: UnifiedPosHealthCheckLevel, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckHealthAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statisticscategories: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetStatisticsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StatusUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusUpdated: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPosPrinter2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPosPrinter2 {
    type Vtable = IPosPrinter2_Vtbl;
}
unsafe impl ::windows::core::Interface for IPosPrinter2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x248475e8_8b98_5517_8e48_760e86f68987);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinter2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedBarcodeSymbologies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedBarcodeSymbologies: usize,
    pub GetFontProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, typeface: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPosPrinterCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPosPrinterCapabilities {
    type Vtable = IPosPrinterCapabilities_Vtbl;
}
unsafe impl ::windows::core::Interface for IPosPrinterCapabilities {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcde95721_4380_4985_adc5_39db30cd93bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterCapabilities_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PowerReportingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosPowerReportingType) -> ::windows::core::HRESULT,
    pub IsStatisticsReportingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsStatisticsUpdatingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub DefaultCharacterSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub HasCoverSensor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanMapCharacterSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsTransactionSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Receipt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Slip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Journal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPosPrinterCharacterSetIdsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPosPrinterCharacterSetIdsStatics {
    type Vtable = IPosPrinterCharacterSetIdsStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IPosPrinterCharacterSetIdsStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c709eff_709a_4fe7_b215_06a748a38b39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterCharacterSetIdsStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Utf16LE: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Ascii: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Ansi: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPosPrinterFontProperty(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPosPrinterFontProperty {
    type Vtable = IPosPrinterFontProperty_Vtbl;
}
unsafe impl ::windows::core::Interface for IPosPrinterFontProperty {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7f4e93a_f8ac_5f04_84d2_29b16d8a633c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterFontProperty_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TypeFace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsScalableToAnySize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CharacterSizes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CharacterSizes: usize,
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct IPosPrinterJob(::windows::core::IUnknown);
impl IPosPrinterJob {
    pub fn Print(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Print)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn PrintLine(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).PrintLine)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn PrintNewline(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).PrintNewline)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExecuteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExecuteAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IPosPrinterJob, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IPosPrinterJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPosPrinterJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPosPrinterJob {}
impl ::core::fmt::Debug for IPosPrinterJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPosPrinterJob").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IPosPrinterJob {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{9a94005c-0615-4591-a58f-30f87edfe2e4}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IPosPrinterJob {
    type Vtable = IPosPrinterJob_Vtbl;
}
unsafe impl ::windows::core::Interface for IPosPrinterJob {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a94005c_0615_4591_a58f_30f87edfe2e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterJob_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Print: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PrintLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PrintNewline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ExecuteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExecuteAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPosPrinterPrintOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPosPrinterPrintOptions {
    type Vtable = IPosPrinterPrintOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for IPosPrinterPrintOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a2e16fd_1d02_5a58_9d59_bfcde76fde86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterPrintOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TypeFace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTypeFace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CharacterHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetCharacterHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub Bold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetBold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Italic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetItalic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Underline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetUnderline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ReverseVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetReverseVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Strikethrough: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetStrikethrough: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Superscript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetSuperscript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Subscript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetSubscript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub DoubleWide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetDoubleWide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub DoubleHigh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetDoubleHigh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Alignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterAlignment) -> ::windows::core::HRESULT,
    pub SetAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PosPrinterAlignment) -> ::windows::core::HRESULT,
    pub CharacterSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetCharacterSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPosPrinterReleaseDeviceRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPosPrinterReleaseDeviceRequestedEventArgs {
    type Vtable = IPosPrinterReleaseDeviceRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IPosPrinterReleaseDeviceRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2bcba359_1cef_40b2_9ecb_f927f856ae3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterReleaseDeviceRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPosPrinterStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPosPrinterStatics {
    type Vtable = IPosPrinterStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IPosPrinterStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ce0d4ea_132f_4cdf_a64a_2d0d7c96a85b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPosPrinterStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPosPrinterStatics2 {
    type Vtable = IPosPrinterStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IPosPrinterStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeecd2c1c_b0d0_42e7_b137_b89b16244d41);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeviceSelectorWithConnectionTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectiontypes: PosConnectionTypes, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPosPrinterStatus(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPosPrinterStatus {
    type Vtable = IPosPrinterStatus_Vtbl;
}
unsafe impl ::windows::core::Interface for IPosPrinterStatus {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1f0c730_da40_4328_bf76_5156fa33b747);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterStatus_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub StatusKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterStatusKind) -> ::windows::core::HRESULT,
    pub ExtendedStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPosPrinterStatusUpdatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPosPrinterStatusUpdatedEventArgs {
    type Vtable = IPosPrinterStatusUpdatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IPosPrinterStatusUpdatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2edb87df_13a6_428d_ba81_b0e7c3e5a3cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterStatusUpdatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct IReceiptOrSlipJob(::windows::core::IUnknown);
impl IReceiptOrSlipJob {
    pub fn SetBarcodeRotation(&self, value: PosPrinterRotation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBarcodeRotation)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn SetPrintRotation(&self, value: PosPrinterRotation, includebitmaps: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPrintRotation)(::windows::core::Vtable::as_raw(this), value, includebitmaps).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPrintArea(&self, value: super::super::Foundation::Rect) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPrintArea)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetBitmap(&self, bitmapnumber: u32, bitmap: &super::super::Graphics::Imaging::BitmapFrame, alignment: PosPrinterAlignment) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBitmap)(::windows::core::Vtable::as_raw(this), bitmapnumber, ::core::mem::transmute_copy(bitmap), alignment).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetBitmapCustomWidthStandardAlign(&self, bitmapnumber: u32, bitmap: &super::super::Graphics::Imaging::BitmapFrame, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBitmapCustomWidthStandardAlign)(::windows::core::Vtable::as_raw(this), bitmapnumber, ::core::mem::transmute_copy(bitmap), alignment, width).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetCustomAlignedBitmap(&self, bitmapnumber: u32, bitmap: &super::super::Graphics::Imaging::BitmapFrame, alignmentdistance: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCustomAlignedBitmap)(::windows::core::Vtable::as_raw(this), bitmapnumber, ::core::mem::transmute_copy(bitmap), alignmentdistance).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetBitmapCustomWidthCustomAlign(&self, bitmapnumber: u32, bitmap: &super::super::Graphics::Imaging::BitmapFrame, alignmentdistance: u32, width: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBitmapCustomWidthCustomAlign)(::windows::core::Vtable::as_raw(this), bitmapnumber, ::core::mem::transmute_copy(bitmap), alignmentdistance, width).ok() }
    }
    pub fn PrintSavedBitmap(&self, bitmapnumber: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).PrintSavedBitmap)(::windows::core::Vtable::as_raw(this), bitmapnumber).ok() }
    }
    pub fn DrawRuledLine(&self, positionlist: &::windows::core::HSTRING, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).DrawRuledLine)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(positionlist), linedirection, linewidth, linestyle, linecolor).ok() }
    }
    pub fn PrintBarcode(&self, data: &::windows::core::HSTRING, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).PrintBarcode)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(data), symbology, height, width, textposition, alignment).ok() }
    }
    pub fn PrintBarcodeCustomAlign(&self, data: &::windows::core::HSTRING, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).PrintBarcodeCustomAlign)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(data), symbology, height, width, textposition, alignmentdistance).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintBitmap(&self, bitmap: &super::super::Graphics::Imaging::BitmapFrame, alignment: PosPrinterAlignment) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).PrintBitmap)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(bitmap), alignment).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintBitmapCustomWidthStandardAlign(&self, bitmap: &super::super::Graphics::Imaging::BitmapFrame, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).PrintBitmapCustomWidthStandardAlign)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(bitmap), alignment, width).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintCustomAlignedBitmap(&self, bitmap: &super::super::Graphics::Imaging::BitmapFrame, alignmentdistance: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).PrintCustomAlignedBitmap)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(bitmap), alignmentdistance).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintBitmapCustomWidthCustomAlign(&self, bitmap: &super::super::Graphics::Imaging::BitmapFrame, alignmentdistance: u32, width: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).PrintBitmapCustomWidthCustomAlign)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(bitmap), alignmentdistance, width).ok() }
    }
    pub fn Print(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Print)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn PrintLine(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).PrintLine)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn PrintNewline(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).PrintNewline)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExecuteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExecuteAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IReceiptOrSlipJob, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<IReceiptOrSlipJob> for IPosPrinterJob {
    type Error = ::windows::core::Error;
    fn try_from(value: IReceiptOrSlipJob) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IReceiptOrSlipJob> for IPosPrinterJob {
    type Error = ::windows::core::Error;
    fn try_from(value: &IReceiptOrSlipJob) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&IReceiptOrSlipJob> for ::windows::core::InParam<IPosPrinterJob> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IReceiptOrSlipJob) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IReceiptOrSlipJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IReceiptOrSlipJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReceiptOrSlipJob {}
impl ::core::fmt::Debug for IReceiptOrSlipJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReceiptOrSlipJob").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IReceiptOrSlipJob {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{532199be-c8c3-4dc2-89e9-5c4a37b34ddc}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IReceiptOrSlipJob {
    type Vtable = IReceiptOrSlipJob_Vtbl;
}
unsafe impl ::windows::core::Interface for IReceiptOrSlipJob {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x532199be_c8c3_4dc2_89e9_5c4a37b34ddc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReceiptOrSlipJob_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetBarcodeRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PosPrinterRotation) -> ::windows::core::HRESULT,
    pub SetPrintRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PosPrinterRotation, includebitmaps: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetPrintArea: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPrintArea: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmapnumber: u32, bitmap: *mut ::core::ffi::c_void, alignment: PosPrinterAlignment) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetBitmap: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetBitmapCustomWidthStandardAlign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmapnumber: u32, bitmap: *mut ::core::ffi::c_void, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetBitmapCustomWidthStandardAlign: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetCustomAlignedBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmapnumber: u32, bitmap: *mut ::core::ffi::c_void, alignmentdistance: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetCustomAlignedBitmap: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetBitmapCustomWidthCustomAlign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmapnumber: u32, bitmap: *mut ::core::ffi::c_void, alignmentdistance: u32, width: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetBitmapCustomWidthCustomAlign: usize,
    pub PrintSavedBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmapnumber: u32) -> ::windows::core::HRESULT,
    pub DrawRuledLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, positionlist: *mut ::core::ffi::c_void, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> ::windows::core::HRESULT,
    pub PrintBarcode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> ::windows::core::HRESULT,
    pub PrintBarcodeCustomAlign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")]
    pub PrintBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, alignment: PosPrinterAlignment) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    PrintBitmap: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub PrintBitmapCustomWidthStandardAlign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    PrintBitmapCustomWidthStandardAlign: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub PrintCustomAlignedBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, alignmentdistance: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    PrintCustomAlignedBitmap: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub PrintBitmapCustomWidthCustomAlign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, alignmentdistance: u32, width: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    PrintBitmapCustomWidthCustomAlign: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IReceiptPrintJob(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IReceiptPrintJob {
    type Vtable = IReceiptPrintJob_Vtbl;
}
unsafe impl ::windows::core::Interface for IReceiptPrintJob {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa96066e_acad_4b79_9d0f_c0cfc08dc77b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReceiptPrintJob_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub MarkFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: PosPrinterMarkFeedKind) -> ::windows::core::HRESULT,
    pub CutPaper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, percentage: f64) -> ::windows::core::HRESULT,
    pub CutPaperDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IReceiptPrintJob2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IReceiptPrintJob2 {
    type Vtable = IReceiptPrintJob2_Vtbl;
}
unsafe impl ::windows::core::Interface for IReceiptPrintJob2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cbc12e3_9e29_5179_bcd8_1811d3b9a10e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReceiptPrintJob2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub StampPaper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Print: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, printoptions: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FeedPaperByLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linecount: i32) -> ::windows::core::HRESULT,
    pub FeedPaperByMapModeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, distance: i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IReceiptPrinterCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IReceiptPrinterCapabilities {
    type Vtable = IReceiptPrinterCapabilities_Vtbl;
}
unsafe impl ::windows::core::Interface for IReceiptPrinterCapabilities {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8f0b58f_51a8_43fc_9bd5_8de272a6415b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReceiptPrinterCapabilities_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CanCutPaper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsStampSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub MarkFeedCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterMarkFeedCapabilities) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IReceiptPrinterCapabilities2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IReceiptPrinterCapabilities2 {
    type Vtable = IReceiptPrinterCapabilities2_Vtbl;
}
unsafe impl ::windows::core::Interface for IReceiptPrinterCapabilities2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20030638_8a2c_55ac_9a7b_7576d8869e99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReceiptPrinterCapabilities2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsReverseVideoSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsStrikethroughSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsSuperscriptSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsSubscriptSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsReversePaperFeedByLineSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsReversePaperFeedByMapModeUnitSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISlipPrintJob(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISlipPrintJob {
    type Vtable = ISlipPrintJob_Vtbl;
}
unsafe impl ::windows::core::Interface for ISlipPrintJob {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d88f95d_6131_5a4b_b7d5_8ef2da7b4165);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISlipPrintJob_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Print: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, printoptions: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FeedPaperByLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linecount: i32) -> ::windows::core::HRESULT,
    pub FeedPaperByMapModeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, distance: i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISlipPrinterCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISlipPrinterCapabilities {
    type Vtable = ISlipPrinterCapabilities_Vtbl;
}
unsafe impl ::windows::core::Interface for ISlipPrinterCapabilities {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99b16399_488c_4157_8ac2_9f57f708d3db);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISlipPrinterCapabilities_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsFullLengthSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsBothSidesPrintingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISlipPrinterCapabilities2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISlipPrinterCapabilities2 {
    type Vtable = ISlipPrinterCapabilities2_Vtbl;
}
unsafe impl ::windows::core::Interface for ISlipPrinterCapabilities2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ff89671_2d1a_5000_87c2_b0851bfdf07e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISlipPrinterCapabilities2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsReverseVideoSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsStrikethroughSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsSuperscriptSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsSubscriptSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsReversePaperFeedByLineSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsReversePaperFeedByMapModeUnitSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUnifiedPosErrorData(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUnifiedPosErrorData {
    type Vtable = IUnifiedPosErrorData_Vtbl;
}
unsafe impl ::windows::core::Interface for IUnifiedPosErrorData {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b998c3a_555c_4889_8ed8_c599bb3a712a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnifiedPosErrorData_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Severity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosErrorSeverity) -> ::windows::core::HRESULT,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosErrorReason) -> ::windows::core::HRESULT,
    pub ExtendedReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUnifiedPosErrorDataFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUnifiedPosErrorDataFactory {
    type Vtable = IUnifiedPosErrorDataFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IUnifiedPosErrorDataFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b982551_1ffe_451b_a368_63e0ce465f5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnifiedPosErrorDataFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, severity: UnifiedPosErrorSeverity, reason: UnifiedPosErrorReason, extendedreason: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct BarcodeScanner(::windows::core::IUnknown);
impl BarcodeScanner {
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeviceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Capabilities(&self) -> ::windows::core::Result<BarcodeScannerCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Capabilities)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ClaimScannerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedBarcodeScanner>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ClaimScannerAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CheckHealthAsync)(::windows::core::Vtable::as_raw(this), level, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedSymbologiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<u32>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSupportedSymbologiesAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsSymbologySupportedAsync(&self, barcodesymbology: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSymbologySupportedAsync)(::windows::core::Vtable::as_raw(this), barcodesymbology, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn RetrieveStatisticsAsync<P0, E0>(&self, statisticscategories: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RetrieveStatisticsAsync)(::windows::core::Vtable::as_raw(this), statisticscategories.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedProfiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSupportedProfiles)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsProfileSupported(&self, profile: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsProfileSupported)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(profile), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StatusUpdated(&self, handler: &super::super::Foundation::TypedEventHandler<BarcodeScanner, BarcodeScannerStatusUpdatedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StatusUpdated)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusUpdated(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveStatusUpdated)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn VideoDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IBarcodeScanner2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VideoDeviceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BarcodeScanner>> {
        Self::IBarcodeScannerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefaultAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BarcodeScanner>> {
        Self::IBarcodeScannerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FromIdAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(deviceid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBarcodeScannerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceSelector)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBarcodeScannerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceSelectorWithConnectionTypes)(::windows::core::Vtable::as_raw(this), connectiontypes, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc(hidden)]
    pub fn IBarcodeScannerStatics<R, F: FnOnce(&IBarcodeScannerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<BarcodeScanner, IBarcodeScannerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IBarcodeScannerStatics2<R, F: FnOnce(&IBarcodeScannerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<BarcodeScanner, IBarcodeScannerStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for BarcodeScanner {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScanner {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScanner {}
impl ::core::fmt::Debug for BarcodeScanner {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScanner").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BarcodeScanner {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScanner;{bea33e06-b264-4f03-a9c1-45b20f01134f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for BarcodeScanner {
    type Vtable = IBarcodeScanner_Vtbl;
}
unsafe impl ::windows::core::Interface for BarcodeScanner {
    const IID: ::windows::core::GUID = <IBarcodeScanner as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BarcodeScanner {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScanner";
}
::windows::core::interface_hierarchy!(BarcodeScanner, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<BarcodeScanner> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: BarcodeScanner) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&BarcodeScanner> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &BarcodeScanner) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&BarcodeScanner> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &BarcodeScanner) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for BarcodeScanner {}
unsafe impl ::core::marker::Sync for BarcodeScanner {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct BarcodeScannerCapabilities(::windows::core::IUnknown);
impl BarcodeScannerCapabilities {
    pub fn PowerReportingType(&self) -> ::windows::core::Result<UnifiedPosPowerReportingType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PowerReportingType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsStatisticsReportingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsStatisticsReportingSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsStatisticsUpdatingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsStatisticsUpdatingSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsImagePreviewSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsImagePreviewSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsSoftwareTriggerSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBarcodeScannerCapabilities1>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSoftwareTriggerSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsVideoPreviewSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBarcodeScannerCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsVideoPreviewSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerCapabilities {}
impl ::core::fmt::Debug for BarcodeScannerCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScannerCapabilities;{c60691e4-f2c8-4420-a307-b12ef6622857})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for BarcodeScannerCapabilities {
    type Vtable = IBarcodeScannerCapabilities_Vtbl;
}
unsafe impl ::windows::core::Interface for BarcodeScannerCapabilities {
    const IID: ::windows::core::GUID = <IBarcodeScannerCapabilities as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BarcodeScannerCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerCapabilities";
}
::windows::core::interface_hierarchy!(BarcodeScannerCapabilities, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BarcodeScannerCapabilities {}
unsafe impl ::core::marker::Sync for BarcodeScannerCapabilities {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct BarcodeScannerDataReceivedEventArgs(::windows::core::IUnknown);
impl BarcodeScannerDataReceivedEventArgs {
    pub fn Report(&self) -> ::windows::core::Result<BarcodeScannerReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Report)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerDataReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerDataReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerDataReceivedEventArgs {}
impl ::core::fmt::Debug for BarcodeScannerDataReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerDataReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerDataReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScannerDataReceivedEventArgs;{4234a7e2-ed97-467d-ad2b-01e44313a929})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for BarcodeScannerDataReceivedEventArgs {
    type Vtable = IBarcodeScannerDataReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for BarcodeScannerDataReceivedEventArgs {
    const IID: ::windows::core::GUID = <IBarcodeScannerDataReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BarcodeScannerDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerDataReceivedEventArgs";
}
::windows::core::interface_hierarchy!(BarcodeScannerDataReceivedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BarcodeScannerDataReceivedEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerDataReceivedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct BarcodeScannerErrorOccurredEventArgs(::windows::core::IUnknown);
impl BarcodeScannerErrorOccurredEventArgs {
    pub fn PartialInputData(&self) -> ::windows::core::Result<BarcodeScannerReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PartialInputData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsRetriable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsRetriable)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ErrorData(&self) -> ::windows::core::Result<UnifiedPosErrorData> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ErrorData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerErrorOccurredEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerErrorOccurredEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerErrorOccurredEventArgs {}
impl ::core::fmt::Debug for BarcodeScannerErrorOccurredEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerErrorOccurredEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerErrorOccurredEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScannerErrorOccurredEventArgs;{2cd2602f-cf3a-4002-a75a-c5ec468f0a20})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for BarcodeScannerErrorOccurredEventArgs {
    type Vtable = IBarcodeScannerErrorOccurredEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for BarcodeScannerErrorOccurredEventArgs {
    const IID: ::windows::core::GUID = <IBarcodeScannerErrorOccurredEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BarcodeScannerErrorOccurredEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerErrorOccurredEventArgs";
}
::windows::core::interface_hierarchy!(BarcodeScannerErrorOccurredEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BarcodeScannerErrorOccurredEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerErrorOccurredEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct BarcodeScannerImagePreviewReceivedEventArgs(::windows::core::IUnknown);
impl BarcodeScannerImagePreviewReceivedEventArgs {
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Preview(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Preview)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerImagePreviewReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerImagePreviewReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerImagePreviewReceivedEventArgs {}
impl ::core::fmt::Debug for BarcodeScannerImagePreviewReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerImagePreviewReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerImagePreviewReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScannerImagePreviewReceivedEventArgs;{f3b7de85-6e8b-434e-9f58-06ef26bc4baf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for BarcodeScannerImagePreviewReceivedEventArgs {
    type Vtable = IBarcodeScannerImagePreviewReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for BarcodeScannerImagePreviewReceivedEventArgs {
    const IID: ::windows::core::GUID = <IBarcodeScannerImagePreviewReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BarcodeScannerImagePreviewReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerImagePreviewReceivedEventArgs";
}
::windows::core::interface_hierarchy!(BarcodeScannerImagePreviewReceivedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BarcodeScannerImagePreviewReceivedEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerImagePreviewReceivedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct BarcodeScannerReport(::windows::core::IUnknown);
impl BarcodeScannerReport {
    pub fn ScanDataType(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ScanDataType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ScanData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ScanData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ScanDataLabel(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ScanDataLabel)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateInstance<P0, E0, P1, E1>(scandatatype: u32, scandata: P0, scandatalabel: P1) -> ::windows::core::Result<BarcodeScannerReport>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IBuffer>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IBarcodeScannerReportFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(::windows::core::Vtable::as_raw(this), scandatatype, scandata.try_into().map_err(|e| e.into())?.abi(), scandatalabel.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBarcodeScannerReportFactory<R, F: FnOnce(&IBarcodeScannerReportFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<BarcodeScannerReport, IBarcodeScannerReportFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for BarcodeScannerReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerReport {}
impl ::core::fmt::Debug for BarcodeScannerReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerReport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScannerReport;{5ce4d8b0-a489-4b96-86c4-f0bf8a37753d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for BarcodeScannerReport {
    type Vtable = IBarcodeScannerReport_Vtbl;
}
unsafe impl ::windows::core::Interface for BarcodeScannerReport {
    const IID: ::windows::core::GUID = <IBarcodeScannerReport as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BarcodeScannerReport {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerReport";
}
::windows::core::interface_hierarchy!(BarcodeScannerReport, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BarcodeScannerReport {}
unsafe impl ::core::marker::Sync for BarcodeScannerReport {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct BarcodeScannerStatusUpdatedEventArgs(::windows::core::IUnknown);
impl BarcodeScannerStatusUpdatedEventArgs {
    pub fn Status(&self) -> ::windows::core::Result<BarcodeScannerStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedStatus(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedStatus)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerStatusUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerStatusUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerStatusUpdatedEventArgs {}
impl ::core::fmt::Debug for BarcodeScannerStatusUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerStatusUpdatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerStatusUpdatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScannerStatusUpdatedEventArgs;{355d8586-9c43-462b-a91a-816dc97f452c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for BarcodeScannerStatusUpdatedEventArgs {
    type Vtable = IBarcodeScannerStatusUpdatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for BarcodeScannerStatusUpdatedEventArgs {
    const IID: ::windows::core::GUID = <IBarcodeScannerStatusUpdatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BarcodeScannerStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerStatusUpdatedEventArgs";
}
::windows::core::interface_hierarchy!(BarcodeScannerStatusUpdatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BarcodeScannerStatusUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerStatusUpdatedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
pub struct BarcodeSymbologies;
impl BarcodeSymbologies {
    pub fn Unknown() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Unknown)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Ean8() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Ean8)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Ean8Add2() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Ean8Add2)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Ean8Add5() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Ean8Add5)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Eanv() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Eanv)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn EanvAdd2() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EanvAdd2)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn EanvAdd5() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EanvAdd5)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Ean13() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Ean13)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Ean13Add2() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Ean13Add2)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Ean13Add5() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Ean13Add5)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Isbn() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Isbn)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn IsbnAdd5() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsbnAdd5)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Ismn() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Ismn)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn IsmnAdd2() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsmnAdd2)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn IsmnAdd5() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsmnAdd5)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Issn() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Issn)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn IssnAdd2() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IssnAdd2)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn IssnAdd5() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IssnAdd5)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Ean99() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Ean99)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Ean99Add2() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Ean99Add2)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Ean99Add5() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Ean99Add5)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Upca() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Upca)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn UpcaAdd2() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpcaAdd2)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn UpcaAdd5() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpcaAdd5)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Upce() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Upce)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn UpceAdd2() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpceAdd2)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn UpceAdd5() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpceAdd5)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn UpcCoupon() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpcCoupon)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TfStd() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TfStd)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TfDis() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TfDis)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TfInt() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TfInt)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TfInd() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TfInd)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TfMat() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TfMat)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TfIata() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TfIata)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Gs1DatabarType1() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Gs1DatabarType1)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Gs1DatabarType2() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Gs1DatabarType2)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Gs1DatabarType3() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Gs1DatabarType3)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Code39() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Code39)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Code39Ex() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Code39Ex)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Trioptic39() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Trioptic39)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Code32() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Code32)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Pzn() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Pzn)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Code93() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Code93)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Code93Ex() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Code93Ex)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Code128() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Code128)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Gs1128() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Gs1128)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Gs1128Coupon() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Gs1128Coupon)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn UccEan128() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UccEan128)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Sisac() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Sisac)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Isbt() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Isbt)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Codabar() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Codabar)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Code11() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Code11)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Msi() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Msi)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Plessey() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Plessey)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Telepen() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Telepen)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Code16k() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Code16k)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CodablockA() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CodablockA)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CodablockF() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CodablockF)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Codablock128() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Codablock128)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Code49() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Code49)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Aztec() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Aztec)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn DataCode() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DataCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn DataMatrix() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DataMatrix)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn HanXin() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HanXin)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Maxicode() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Maxicode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn MicroPdf417() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MicroPdf417)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn MicroQr() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MicroQr)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Pdf417() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Pdf417)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Qr() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Qr)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn MsTag() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MsTag)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Ccab() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Ccab)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Ccc() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Ccc)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Tlc39() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Tlc39)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn AusPost() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AusPost)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CanPost() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanPost)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn ChinaPost() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChinaPost)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn DutchKix() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DutchKix)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn InfoMail() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InfoMail)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn ItalianPost25() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ItalianPost25)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn ItalianPost39() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ItalianPost39)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn JapanPost() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).JapanPost)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn KoreanPost() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KoreanPost)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn SwedenPost() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SwedenPost)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn UkPost() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UkPost)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn UsIntelligent() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UsIntelligent)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn UsIntelligentPkg() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UsIntelligentPkg)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn UsPlanet() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UsPlanet)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn UsPostNet() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UsPostNet)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Us4StateFics() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Us4StateFics)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn OcrA() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OcrA)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn OcrB() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OcrB)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Micr() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Micr)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn ExtendedBase() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedBase)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetName(scandatatype: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetName)(::windows::core::Vtable::as_raw(this), scandatatype, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Gs1DWCode() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Gs1DWCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBarcodeSymbologiesStatics<R, F: FnOnce(&IBarcodeSymbologiesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<BarcodeSymbologies, IBarcodeSymbologiesStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IBarcodeSymbologiesStatics2<R, F: FnOnce(&IBarcodeSymbologiesStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<BarcodeSymbologies, IBarcodeSymbologiesStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for BarcodeSymbologies {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeSymbologies";
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct BarcodeSymbologyAttributes(::windows::core::IUnknown);
impl BarcodeSymbologyAttributes {
    pub fn IsCheckDigitValidationEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCheckDigitValidationEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsCheckDigitValidationEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsCheckDigitValidationEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsCheckDigitValidationSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCheckDigitValidationSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsCheckDigitTransmissionEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCheckDigitTransmissionEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsCheckDigitTransmissionEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsCheckDigitTransmissionEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsCheckDigitTransmissionSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCheckDigitTransmissionSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DecodeLength1(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DecodeLength1)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDecodeLength1(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDecodeLength1)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn DecodeLength2(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DecodeLength2)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDecodeLength2(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDecodeLength2)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn DecodeLengthKind(&self) -> ::windows::core::Result<BarcodeSymbologyDecodeLengthKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DecodeLengthKind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDecodeLengthKind(&self, value: BarcodeSymbologyDecodeLengthKind) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDecodeLengthKind)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsDecodeLengthSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDecodeLengthSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeSymbologyAttributes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeSymbologyAttributes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeSymbologyAttributes {}
impl ::core::fmt::Debug for BarcodeSymbologyAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeSymbologyAttributes").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BarcodeSymbologyAttributes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeSymbologyAttributes;{66413a78-ab7a-4ada-8ece-936014b2ead7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for BarcodeSymbologyAttributes {
    type Vtable = IBarcodeSymbologyAttributes_Vtbl;
}
unsafe impl ::windows::core::Interface for BarcodeSymbologyAttributes {
    const IID: ::windows::core::GUID = <IBarcodeSymbologyAttributes as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BarcodeSymbologyAttributes {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeSymbologyAttributes";
}
::windows::core::interface_hierarchy!(BarcodeSymbologyAttributes, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BarcodeSymbologyAttributes {}
unsafe impl ::core::marker::Sync for BarcodeSymbologyAttributes {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct CashDrawer(::windows::core::IUnknown);
impl CashDrawer {
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeviceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Capabilities(&self) -> ::windows::core::Result<CashDrawerCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Capabilities)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<CashDrawerStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsDrawerOpen(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDrawerOpen)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DrawerEventSource(&self) -> ::windows::core::Result<CashDrawerEventSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DrawerEventSource)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ClaimDrawerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedCashDrawer>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ClaimDrawerAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CheckHealthAsync)(::windows::core::Vtable::as_raw(this), level, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStatisticsAsync<P0, E0>(&self, statisticscategories: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStatisticsAsync)(::windows::core::Vtable::as_raw(this), statisticscategories.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StatusUpdated(&self, handler: &super::super::Foundation::TypedEventHandler<CashDrawer, CashDrawerStatusUpdatedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StatusUpdated)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusUpdated(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveStatusUpdated)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CashDrawer>> {
        Self::ICashDrawerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefaultAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CashDrawer>> {
        Self::ICashDrawerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FromIdAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(deviceid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICashDrawerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceSelector)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICashDrawerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceSelectorWithConnectionTypes)(::windows::core::Vtable::as_raw(this), connectiontypes, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc(hidden)]
    pub fn ICashDrawerStatics<R, F: FnOnce(&ICashDrawerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CashDrawer, ICashDrawerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICashDrawerStatics2<R, F: FnOnce(&ICashDrawerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CashDrawer, ICashDrawerStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CashDrawer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CashDrawer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CashDrawer {}
impl ::core::fmt::Debug for CashDrawer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CashDrawer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CashDrawer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawer;{9f88f5c8-de54-4aee-a890-920bcbfe30fc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CashDrawer {
    type Vtable = ICashDrawer_Vtbl;
}
unsafe impl ::windows::core::Interface for CashDrawer {
    const IID: ::windows::core::GUID = <ICashDrawer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CashDrawer {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawer";
}
::windows::core::interface_hierarchy!(CashDrawer, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CashDrawer> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CashDrawer) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CashDrawer> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CashDrawer) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CashDrawer> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CashDrawer) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CashDrawer {}
unsafe impl ::core::marker::Sync for CashDrawer {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct CashDrawerCapabilities(::windows::core::IUnknown);
impl CashDrawerCapabilities {
    pub fn PowerReportingType(&self) -> ::windows::core::Result<UnifiedPosPowerReportingType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PowerReportingType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsStatisticsReportingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsStatisticsReportingSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsStatisticsUpdatingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsStatisticsUpdatingSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsStatusReportingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsStatusReportingSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsStatusMultiDrawerDetectSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsStatusMultiDrawerDetectSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsDrawerOpenSensorAvailable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDrawerOpenSensorAvailable)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for CashDrawerCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CashDrawerCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CashDrawerCapabilities {}
impl ::core::fmt::Debug for CashDrawerCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CashDrawerCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CashDrawerCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerCapabilities;{0bc6de0b-e8e7-4b1f-b1d1-3e501ad08247})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CashDrawerCapabilities {
    type Vtable = ICashDrawerCapabilities_Vtbl;
}
unsafe impl ::windows::core::Interface for CashDrawerCapabilities {
    const IID: ::windows::core::GUID = <ICashDrawerCapabilities as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CashDrawerCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerCapabilities";
}
::windows::core::interface_hierarchy!(CashDrawerCapabilities, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CashDrawerCapabilities {}
unsafe impl ::core::marker::Sync for CashDrawerCapabilities {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct CashDrawerCloseAlarm(::windows::core::IUnknown);
impl CashDrawerCloseAlarm {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetAlarmTimeout(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAlarmTimeout)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AlarmTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AlarmTimeout)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetBeepFrequency(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBeepFrequency)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn BeepFrequency(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BeepFrequency)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBeepDuration(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBeepDuration)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BeepDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BeepDuration)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBeepDelay(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBeepDelay)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BeepDelay(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BeepDelay)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AlarmTimeoutExpired(&self, handler: &super::super::Foundation::TypedEventHandler<CashDrawerCloseAlarm, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AlarmTimeoutExpired)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAlarmTimeoutExpired(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAlarmTimeoutExpired)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for CashDrawerCloseAlarm {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CashDrawerCloseAlarm {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CashDrawerCloseAlarm {}
impl ::core::fmt::Debug for CashDrawerCloseAlarm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CashDrawerCloseAlarm").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CashDrawerCloseAlarm {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerCloseAlarm;{6bf88cc7-6f63-430e-ab3b-95d75ffbe87f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CashDrawerCloseAlarm {
    type Vtable = ICashDrawerCloseAlarm_Vtbl;
}
unsafe impl ::windows::core::Interface for CashDrawerCloseAlarm {
    const IID: ::windows::core::GUID = <ICashDrawerCloseAlarm as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CashDrawerCloseAlarm {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerCloseAlarm";
}
::windows::core::interface_hierarchy!(CashDrawerCloseAlarm, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CashDrawerCloseAlarm {}
unsafe impl ::core::marker::Sync for CashDrawerCloseAlarm {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct CashDrawerClosedEventArgs(::windows::core::IUnknown);
impl CashDrawerClosedEventArgs {
    pub fn CashDrawer(&self) -> ::windows::core::Result<CashDrawer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CashDrawer)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for CashDrawerClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CashDrawerClosedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CashDrawerClosedEventArgs {}
impl ::core::fmt::Debug for CashDrawerClosedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CashDrawerClosedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CashDrawerClosedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerClosedEventArgs;{69cb3bc1-147f-421c-9c23-090123bb786c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CashDrawerClosedEventArgs {
    type Vtable = ICashDrawerEventSourceEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for CashDrawerClosedEventArgs {
    const IID: ::windows::core::GUID = <ICashDrawerEventSourceEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CashDrawerClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerClosedEventArgs";
}
::windows::core::interface_hierarchy!(CashDrawerClosedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<CashDrawerClosedEventArgs> for ICashDrawerEventSourceEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: CashDrawerClosedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CashDrawerClosedEventArgs> for ICashDrawerEventSourceEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &CashDrawerClosedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&CashDrawerClosedEventArgs> for ::windows::core::InParam<ICashDrawerEventSourceEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CashDrawerClosedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CashDrawerClosedEventArgs {}
unsafe impl ::core::marker::Sync for CashDrawerClosedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct CashDrawerEventSource(::windows::core::IUnknown);
impl CashDrawerEventSource {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DrawerClosed(&self, handler: &super::super::Foundation::TypedEventHandler<CashDrawerEventSource, CashDrawerClosedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DrawerClosed)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDrawerClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveDrawerClosed)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DrawerOpened(&self, handler: &super::super::Foundation::TypedEventHandler<CashDrawerEventSource, CashDrawerOpenedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DrawerOpened)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDrawerOpened(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveDrawerOpened)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
}
impl ::core::clone::Clone for CashDrawerEventSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CashDrawerEventSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CashDrawerEventSource {}
impl ::core::fmt::Debug for CashDrawerEventSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CashDrawerEventSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CashDrawerEventSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerEventSource;{e006e46c-f2f9-442f-8dd6-06c10a4227ba})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CashDrawerEventSource {
    type Vtable = ICashDrawerEventSource_Vtbl;
}
unsafe impl ::windows::core::Interface for CashDrawerEventSource {
    const IID: ::windows::core::GUID = <ICashDrawerEventSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CashDrawerEventSource {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerEventSource";
}
::windows::core::interface_hierarchy!(CashDrawerEventSource, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CashDrawerEventSource {}
unsafe impl ::core::marker::Sync for CashDrawerEventSource {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct CashDrawerOpenedEventArgs(::windows::core::IUnknown);
impl CashDrawerOpenedEventArgs {
    pub fn CashDrawer(&self) -> ::windows::core::Result<CashDrawer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CashDrawer)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for CashDrawerOpenedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CashDrawerOpenedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CashDrawerOpenedEventArgs {}
impl ::core::fmt::Debug for CashDrawerOpenedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CashDrawerOpenedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CashDrawerOpenedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerOpenedEventArgs;{69cb3bc1-147f-421c-9c23-090123bb786c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CashDrawerOpenedEventArgs {
    type Vtable = ICashDrawerEventSourceEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for CashDrawerOpenedEventArgs {
    const IID: ::windows::core::GUID = <ICashDrawerEventSourceEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CashDrawerOpenedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerOpenedEventArgs";
}
::windows::core::interface_hierarchy!(CashDrawerOpenedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<CashDrawerOpenedEventArgs> for ICashDrawerEventSourceEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: CashDrawerOpenedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CashDrawerOpenedEventArgs> for ICashDrawerEventSourceEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &CashDrawerOpenedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&CashDrawerOpenedEventArgs> for ::windows::core::InParam<ICashDrawerEventSourceEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CashDrawerOpenedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CashDrawerOpenedEventArgs {}
unsafe impl ::core::marker::Sync for CashDrawerOpenedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct CashDrawerStatus(::windows::core::IUnknown);
impl CashDrawerStatus {
    pub fn StatusKind(&self) -> ::windows::core::Result<CashDrawerStatusKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StatusKind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedStatus(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedStatus)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for CashDrawerStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CashDrawerStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CashDrawerStatus {}
impl ::core::fmt::Debug for CashDrawerStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CashDrawerStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CashDrawerStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerStatus;{6bbd78bf-dca1-4e06-99eb-5af6a5aec108})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CashDrawerStatus {
    type Vtable = ICashDrawerStatus_Vtbl;
}
unsafe impl ::windows::core::Interface for CashDrawerStatus {
    const IID: ::windows::core::GUID = <ICashDrawerStatus as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CashDrawerStatus {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerStatus";
}
::windows::core::interface_hierarchy!(CashDrawerStatus, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CashDrawerStatus {}
unsafe impl ::core::marker::Sync for CashDrawerStatus {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct CashDrawerStatusUpdatedEventArgs(::windows::core::IUnknown);
impl CashDrawerStatusUpdatedEventArgs {
    pub fn Status(&self) -> ::windows::core::Result<CashDrawerStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for CashDrawerStatusUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CashDrawerStatusUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CashDrawerStatusUpdatedEventArgs {}
impl ::core::fmt::Debug for CashDrawerStatusUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CashDrawerStatusUpdatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CashDrawerStatusUpdatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerStatusUpdatedEventArgs;{30aae98a-0d70-459c-9553-87e124c52488})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CashDrawerStatusUpdatedEventArgs {
    type Vtable = ICashDrawerStatusUpdatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for CashDrawerStatusUpdatedEventArgs {
    const IID: ::windows::core::GUID = <ICashDrawerStatusUpdatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CashDrawerStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerStatusUpdatedEventArgs";
}
::windows::core::interface_hierarchy!(CashDrawerStatusUpdatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CashDrawerStatusUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for CashDrawerStatusUpdatedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ClaimedBarcodeScanner(::windows::core::IUnknown);
impl ClaimedBarcodeScanner {
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeviceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsDisabledOnDataReceived(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsDisabledOnDataReceived)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsDisabledOnDataReceived(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDisabledOnDataReceived)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsDecodeDataEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsDecodeDataEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsDecodeDataEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDecodeDataEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EnableAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DisableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisableAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RetainDevice(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RetainDevice)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetActiveSymbologiesAsync<P0, E0>(&self, symbologies: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<u32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetActiveSymbologiesAsync)(::windows::core::Vtable::as_raw(this), symbologies.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ResetStatisticsAsync<P0, E0>(&self, statisticscategories: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ResetStatisticsAsync)(::windows::core::Vtable::as_raw(this), statisticscategories.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdateStatisticsAsync<P0, E0>(&self, statistics: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpdateStatisticsAsync)(::windows::core::Vtable::as_raw(this), statistics.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetActiveProfileAsync(&self, profile: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetActiveProfileAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(profile), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DataReceived(&self, handler: &super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerDataReceivedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DataReceived)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDataReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveDataReceived)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TriggerPressed(&self, handler: &super::super::Foundation::EventHandler<ClaimedBarcodeScanner>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TriggerPressed)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTriggerPressed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveTriggerPressed)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TriggerReleased(&self, handler: &super::super::Foundation::EventHandler<ClaimedBarcodeScanner>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TriggerReleased)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTriggerReleased(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveTriggerReleased)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReleaseDeviceRequested(&self, handler: &super::super::Foundation::EventHandler<ClaimedBarcodeScanner>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReleaseDeviceRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveReleaseDeviceRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveReleaseDeviceRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ImagePreviewReceived(&self, handler: &super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerImagePreviewReceivedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImagePreviewReceived)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveImagePreviewReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveImagePreviewReceived)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ErrorOccurred(&self, handler: &super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerErrorOccurredEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ErrorOccurred)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveErrorOccurred(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveErrorOccurred)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartSoftwareTriggerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IClaimedBarcodeScanner1>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartSoftwareTriggerAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StopSoftwareTriggerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IClaimedBarcodeScanner1>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StopSoftwareTriggerAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetSymbologyAttributesAsync(&self, barcodesymbology: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BarcodeSymbologyAttributes>> {
        let this = &::windows::core::Interface::cast::<IClaimedBarcodeScanner2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSymbologyAttributesAsync)(::windows::core::Vtable::as_raw(this), barcodesymbology, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSymbologyAttributesAsync(&self, barcodesymbology: u32, attributes: &BarcodeSymbologyAttributes) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IClaimedBarcodeScanner2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetSymbologyAttributesAsync)(::windows::core::Vtable::as_raw(this), barcodesymbology, ::core::mem::transmute_copy(attributes), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowVideoPreviewAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IClaimedBarcodeScanner3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShowVideoPreviewAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn HideVideoPreview(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IClaimedBarcodeScanner3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).HideVideoPreview)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn SetIsVideoPreviewShownOnEnable(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IClaimedBarcodeScanner3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsVideoPreviewShownOnEnable)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsVideoPreviewShownOnEnable(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IClaimedBarcodeScanner3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsVideoPreviewShownOnEnable)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed(&self, handler: &super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, ClaimedBarcodeScannerClosedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IClaimedBarcodeScanner4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Closed)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IClaimedBarcodeScanner4>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveClosed)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for ClaimedBarcodeScanner {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedBarcodeScanner {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedBarcodeScanner {}
impl ::core::fmt::Debug for ClaimedBarcodeScanner {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedBarcodeScanner").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ClaimedBarcodeScanner {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedBarcodeScanner;{4a63b49c-8fa4-4332-bb26-945d11d81e0f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ClaimedBarcodeScanner {
    type Vtable = IClaimedBarcodeScanner_Vtbl;
}
unsafe impl ::windows::core::Interface for ClaimedBarcodeScanner {
    const IID: ::windows::core::GUID = <IClaimedBarcodeScanner as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ClaimedBarcodeScanner {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedBarcodeScanner";
}
::windows::core::interface_hierarchy!(ClaimedBarcodeScanner, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ClaimedBarcodeScanner> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ClaimedBarcodeScanner) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ClaimedBarcodeScanner> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ClaimedBarcodeScanner) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ClaimedBarcodeScanner> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ClaimedBarcodeScanner) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ClaimedBarcodeScanner {}
unsafe impl ::core::marker::Sync for ClaimedBarcodeScanner {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ClaimedBarcodeScannerClosedEventArgs(::windows::core::IUnknown);
impl ClaimedBarcodeScannerClosedEventArgs {}
impl ::core::clone::Clone for ClaimedBarcodeScannerClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedBarcodeScannerClosedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedBarcodeScannerClosedEventArgs {}
impl ::core::fmt::Debug for ClaimedBarcodeScannerClosedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedBarcodeScannerClosedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ClaimedBarcodeScannerClosedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedBarcodeScannerClosedEventArgs;{cf7d5489-a22c-4c65-a901-88d77d833954})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ClaimedBarcodeScannerClosedEventArgs {
    type Vtable = IClaimedBarcodeScannerClosedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ClaimedBarcodeScannerClosedEventArgs {
    const IID: ::windows::core::GUID = <IClaimedBarcodeScannerClosedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ClaimedBarcodeScannerClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedBarcodeScannerClosedEventArgs";
}
::windows::core::interface_hierarchy!(ClaimedBarcodeScannerClosedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ClaimedBarcodeScannerClosedEventArgs {}
unsafe impl ::core::marker::Sync for ClaimedBarcodeScannerClosedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ClaimedCashDrawer(::windows::core::IUnknown);
impl ClaimedCashDrawer {
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeviceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsDrawerOpen(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDrawerOpen)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CloseAlarm(&self) -> ::windows::core::Result<CashDrawerCloseAlarm> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CloseAlarm)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenDrawerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OpenDrawerAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EnableAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DisableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisableAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RetainDeviceAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RetainDeviceAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ResetStatisticsAsync<P0, E0>(&self, statisticscategories: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ResetStatisticsAsync)(::windows::core::Vtable::as_raw(this), statisticscategories.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdateStatisticsAsync<P0, E0>(&self, statistics: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpdateStatisticsAsync)(::windows::core::Vtable::as_raw(this), statistics.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReleaseDeviceRequested(&self, handler: &super::super::Foundation::TypedEventHandler<ClaimedCashDrawer, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReleaseDeviceRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveReleaseDeviceRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveReleaseDeviceRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed(&self, handler: &super::super::Foundation::TypedEventHandler<ClaimedCashDrawer, ClaimedCashDrawerClosedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IClaimedCashDrawer2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Closed)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IClaimedCashDrawer2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveClosed)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for ClaimedCashDrawer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedCashDrawer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedCashDrawer {}
impl ::core::fmt::Debug for ClaimedCashDrawer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedCashDrawer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ClaimedCashDrawer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedCashDrawer;{ca3f99af-abb8-42c1-8a84-5c66512f5a75})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ClaimedCashDrawer {
    type Vtable = IClaimedCashDrawer_Vtbl;
}
unsafe impl ::windows::core::Interface for ClaimedCashDrawer {
    const IID: ::windows::core::GUID = <IClaimedCashDrawer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ClaimedCashDrawer {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedCashDrawer";
}
::windows::core::interface_hierarchy!(ClaimedCashDrawer, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ClaimedCashDrawer> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ClaimedCashDrawer) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ClaimedCashDrawer> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ClaimedCashDrawer) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ClaimedCashDrawer> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ClaimedCashDrawer) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ClaimedCashDrawer {}
unsafe impl ::core::marker::Sync for ClaimedCashDrawer {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ClaimedCashDrawerClosedEventArgs(::windows::core::IUnknown);
impl ClaimedCashDrawerClosedEventArgs {}
impl ::core::clone::Clone for ClaimedCashDrawerClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedCashDrawerClosedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedCashDrawerClosedEventArgs {}
impl ::core::fmt::Debug for ClaimedCashDrawerClosedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedCashDrawerClosedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ClaimedCashDrawerClosedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedCashDrawerClosedEventArgs;{cc573f33-3f34-4c5c-baae-deadf16cd7fa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ClaimedCashDrawerClosedEventArgs {
    type Vtable = IClaimedCashDrawerClosedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ClaimedCashDrawerClosedEventArgs {
    const IID: ::windows::core::GUID = <IClaimedCashDrawerClosedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ClaimedCashDrawerClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedCashDrawerClosedEventArgs";
}
::windows::core::interface_hierarchy!(ClaimedCashDrawerClosedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ClaimedCashDrawerClosedEventArgs {}
unsafe impl ::core::marker::Sync for ClaimedCashDrawerClosedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ClaimedJournalPrinter(::windows::core::IUnknown);
impl ClaimedJournalPrinter {
    pub fn CreateJob(&self) -> ::windows::core::Result<JournalPrintJob> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateJob)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCharactersPerLine(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetCharactersPerLine)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn CharactersPerLine(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CharactersPerLine)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLineHeight(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetLineHeight)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn LineHeight(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LineHeight)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLineSpacing(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetLineSpacing)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn LineSpacing(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LineSpacing)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LineWidth(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LineWidth)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsLetterQuality(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsLetterQuality)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsLetterQuality(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsLetterQuality)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsPaperNearEnd(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPaperNearEnd)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetColorCartridge(&self, value: PosPrinterColorCartridge) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetColorCartridge)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ColorCartridge(&self) -> ::windows::core::Result<PosPrinterColorCartridge> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ColorCartridge)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsCoverOpen(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCoverOpen)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsCartridgeRemoved(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCartridgeRemoved)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsCartridgeEmpty(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCartridgeEmpty)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsHeadCleaning(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsHeadCleaning)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsPaperEmpty(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPaperEmpty)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsReadyToPrint(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsReadyToPrint)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ValidateData(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ValidateData)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(data), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ClaimedJournalPrinter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedJournalPrinter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedJournalPrinter {}
impl ::core::fmt::Debug for ClaimedJournalPrinter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedJournalPrinter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ClaimedJournalPrinter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedJournalPrinter;{67ea0630-517d-487f-9fdf-d2e0a0a264a5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ClaimedJournalPrinter {
    type Vtable = IClaimedJournalPrinter_Vtbl;
}
unsafe impl ::windows::core::Interface for ClaimedJournalPrinter {
    const IID: ::windows::core::GUID = <IClaimedJournalPrinter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ClaimedJournalPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedJournalPrinter";
}
::windows::core::interface_hierarchy!(ClaimedJournalPrinter, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<ClaimedJournalPrinter> for ICommonClaimedPosPrinterStation {
    type Error = ::windows::core::Error;
    fn try_from(value: ClaimedJournalPrinter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ClaimedJournalPrinter> for ICommonClaimedPosPrinterStation {
    type Error = ::windows::core::Error;
    fn try_from(value: &ClaimedJournalPrinter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&ClaimedJournalPrinter> for ::windows::core::InParam<ICommonClaimedPosPrinterStation> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ClaimedJournalPrinter) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ClaimedJournalPrinter {}
unsafe impl ::core::marker::Sync for ClaimedJournalPrinter {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ClaimedLineDisplay(::windows::core::IUnknown);
impl ClaimedLineDisplay {
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeviceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Capabilities(&self) -> ::windows::core::Result<LineDisplayCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Capabilities)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PhysicalDeviceName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PhysicalDeviceName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PhysicalDeviceDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PhysicalDeviceDescription)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DeviceControlDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeviceControlDescription)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DeviceControlVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeviceControlVersion)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DeviceServiceVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeviceServiceVersion)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DefaultWindow(&self) -> ::windows::core::Result<LineDisplayWindow> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DefaultWindow)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RetainDevice(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RetainDevice)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReleaseDeviceRequested(&self, handler: &super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReleaseDeviceRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveReleaseDeviceRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveReleaseDeviceRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStatisticsAsync<P0, E0>(&self, statisticscategories: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStatisticsAsync)(::windows::core::Vtable::as_raw(this), statisticscategories.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CheckHealthAsync)(::windows::core::Vtable::as_raw(this), level, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CheckPowerStatusAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayPowerStatus>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CheckPowerStatusAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StatusUpdated(&self, handler: &super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, LineDisplayStatusUpdatedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StatusUpdated)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusUpdated(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveStatusUpdated)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedScreenSizesInCharacters(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Size>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedScreenSizesInCharacters)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxBitmapSizeInPixels(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxBitmapSizeInPixels)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCharacterSets(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i32>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedCharacterSets)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CustomGlyphs(&self) -> ::windows::core::Result<LineDisplayCustomGlyphs> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CustomGlyphs)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetAttributes(&self) -> ::windows::core::Result<LineDisplayAttributes> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAttributes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryUpdateAttributesAsync(&self, attributes: &LineDisplayAttributes) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryUpdateAttributesAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(attributes), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrySetDescriptorAsync(&self, descriptor: u32, descriptorstate: LineDisplayDescriptorState) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrySetDescriptorAsync)(::windows::core::Vtable::as_raw(this), descriptor, descriptorstate, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryClearDescriptorsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryClearDescriptorsAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryCreateWindowAsync(&self, viewport: super::super::Foundation::Rect, windowsize: super::super::Foundation::Size) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayWindow>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryCreateWindowAsync)(::windows::core::Vtable::as_raw(this), viewport, windowsize, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn TryStoreStorageFileBitmapAsync(&self, bitmap: &super::super::Storage::StorageFile) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayStoredBitmap>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryStoreStorageFileBitmapAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(bitmap), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn TryStoreStorageFileBitmapWithAlignmentAsync(&self, bitmap: &super::super::Storage::StorageFile, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayStoredBitmap>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryStoreStorageFileBitmapWithAlignmentAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(bitmap), horizontalalignment, verticalalignment, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn TryStoreStorageFileBitmapWithAlignmentAndWidthAsync(&self, bitmap: &super::super::Storage::StorageFile, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, widthinpixels: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayStoredBitmap>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryStoreStorageFileBitmapWithAlignmentAndWidthAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(bitmap), horizontalalignment, verticalalignment, widthinpixels, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed(&self, handler: &super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, ClaimedLineDisplayClosedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Closed)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveClosed)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedLineDisplay>> {
        Self::IClaimedLineDisplayStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FromIdAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(deviceid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IClaimedLineDisplayStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceSelector)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IClaimedLineDisplayStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceSelectorWithConnectionTypes)(::windows::core::Vtable::as_raw(this), connectiontypes, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc(hidden)]
    pub fn IClaimedLineDisplayStatics<R, F: FnOnce(&IClaimedLineDisplayStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ClaimedLineDisplay, IClaimedLineDisplayStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for ClaimedLineDisplay {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedLineDisplay {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedLineDisplay {}
impl ::core::fmt::Debug for ClaimedLineDisplay {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedLineDisplay").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ClaimedLineDisplay {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedLineDisplay;{120ac970-9a75-4acf-aae7-09972bcf8794})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ClaimedLineDisplay {
    type Vtable = IClaimedLineDisplay_Vtbl;
}
unsafe impl ::windows::core::Interface for ClaimedLineDisplay {
    const IID: ::windows::core::GUID = <IClaimedLineDisplay as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ClaimedLineDisplay {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedLineDisplay";
}
::windows::core::interface_hierarchy!(ClaimedLineDisplay, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ClaimedLineDisplay> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ClaimedLineDisplay) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ClaimedLineDisplay> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ClaimedLineDisplay) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ClaimedLineDisplay> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ClaimedLineDisplay) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ClaimedLineDisplay {}
unsafe impl ::core::marker::Sync for ClaimedLineDisplay {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ClaimedLineDisplayClosedEventArgs(::windows::core::IUnknown);
impl ClaimedLineDisplayClosedEventArgs {}
impl ::core::clone::Clone for ClaimedLineDisplayClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedLineDisplayClosedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedLineDisplayClosedEventArgs {}
impl ::core::fmt::Debug for ClaimedLineDisplayClosedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedLineDisplayClosedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ClaimedLineDisplayClosedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedLineDisplayClosedEventArgs;{f915f364-d3d5-4f10-b511-90939edfacd8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ClaimedLineDisplayClosedEventArgs {
    type Vtable = IClaimedLineDisplayClosedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ClaimedLineDisplayClosedEventArgs {
    const IID: ::windows::core::GUID = <IClaimedLineDisplayClosedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ClaimedLineDisplayClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedLineDisplayClosedEventArgs";
}
::windows::core::interface_hierarchy!(ClaimedLineDisplayClosedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ClaimedLineDisplayClosedEventArgs {}
unsafe impl ::core::marker::Sync for ClaimedLineDisplayClosedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ClaimedMagneticStripeReader(::windows::core::IUnknown);
impl ClaimedMagneticStripeReader {
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeviceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsDisabledOnDataReceived(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsDisabledOnDataReceived)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsDisabledOnDataReceived(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDisabledOnDataReceived)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsDecodeDataEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsDecodeDataEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsDecodeDataEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDecodeDataEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsDeviceAuthenticated(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDeviceAuthenticated)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDataEncryptionAlgorithm(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDataEncryptionAlgorithm)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn DataEncryptionAlgorithm(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DataEncryptionAlgorithm)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetTracksToRead(&self, value: MagneticStripeReaderTrackIds) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTracksToRead)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn TracksToRead(&self) -> ::windows::core::Result<MagneticStripeReaderTrackIds> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TracksToRead)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsTransmitSentinelsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsTransmitSentinelsEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsTransmitSentinelsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsTransmitSentinelsEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EnableAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DisableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisableAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RetainDevice(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RetainDevice)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn SetErrorReportingType(&self, value: MagneticStripeReaderErrorReportingType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetErrorReportingType)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn RetrieveDeviceAuthenticationDataAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RetrieveDeviceAuthenticationDataAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AuthenticateDeviceAsync(&self, responsetoken: &[u8]) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AuthenticateDeviceAsync)(::windows::core::Vtable::as_raw(this), responsetoken.len() as u32, responsetoken.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeAuthenticateDeviceAsync(&self, responsetoken: &[u8]) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeAuthenticateDeviceAsync)(::windows::core::Vtable::as_raw(this), responsetoken.len() as u32, responsetoken.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateKeyAsync(&self, key: &::windows::core::HSTRING, keyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpdateKeyAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(key), ::core::mem::transmute_copy(keyname), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ResetStatisticsAsync<P0, E0>(&self, statisticscategories: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ResetStatisticsAsync)(::windows::core::Vtable::as_raw(this), statisticscategories.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdateStatisticsAsync<P0, E0>(&self, statistics: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpdateStatisticsAsync)(::windows::core::Vtable::as_raw(this), statistics.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BankCardDataReceived(&self, handler: &super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderBankCardDataReceivedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BankCardDataReceived)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBankCardDataReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveBankCardDataReceived)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AamvaCardDataReceived(&self, handler: &super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderAamvaCardDataReceivedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AamvaCardDataReceived)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAamvaCardDataReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAamvaCardDataReceived)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn VendorSpecificDataReceived(&self, handler: &super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VendorSpecificDataReceived)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveVendorSpecificDataReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveVendorSpecificDataReceived)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReleaseDeviceRequested(&self, handler: &super::super::Foundation::EventHandler<ClaimedMagneticStripeReader>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReleaseDeviceRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveReleaseDeviceRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveReleaseDeviceRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ErrorOccurred(&self, handler: &super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderErrorOccurredEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ErrorOccurred)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveErrorOccurred(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveErrorOccurred)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed(&self, handler: &super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, ClaimedMagneticStripeReaderClosedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IClaimedMagneticStripeReader2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Closed)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IClaimedMagneticStripeReader2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveClosed)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for ClaimedMagneticStripeReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedMagneticStripeReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedMagneticStripeReader {}
impl ::core::fmt::Debug for ClaimedMagneticStripeReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedMagneticStripeReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ClaimedMagneticStripeReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedMagneticStripeReader;{475ca8f3-9417-48bc-b9d7-4163a7844c02})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ClaimedMagneticStripeReader {
    type Vtable = IClaimedMagneticStripeReader_Vtbl;
}
unsafe impl ::windows::core::Interface for ClaimedMagneticStripeReader {
    const IID: ::windows::core::GUID = <IClaimedMagneticStripeReader as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ClaimedMagneticStripeReader {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedMagneticStripeReader";
}
::windows::core::interface_hierarchy!(ClaimedMagneticStripeReader, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ClaimedMagneticStripeReader> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ClaimedMagneticStripeReader) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ClaimedMagneticStripeReader> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ClaimedMagneticStripeReader) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ClaimedMagneticStripeReader> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ClaimedMagneticStripeReader) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ClaimedMagneticStripeReader {}
unsafe impl ::core::marker::Sync for ClaimedMagneticStripeReader {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ClaimedMagneticStripeReaderClosedEventArgs(::windows::core::IUnknown);
impl ClaimedMagneticStripeReaderClosedEventArgs {}
impl ::core::clone::Clone for ClaimedMagneticStripeReaderClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedMagneticStripeReaderClosedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedMagneticStripeReaderClosedEventArgs {}
impl ::core::fmt::Debug for ClaimedMagneticStripeReaderClosedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedMagneticStripeReaderClosedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ClaimedMagneticStripeReaderClosedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedMagneticStripeReaderClosedEventArgs;{14ada93a-adcd-4c80-acda-c3eaed2647e1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ClaimedMagneticStripeReaderClosedEventArgs {
    type Vtable = IClaimedMagneticStripeReaderClosedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ClaimedMagneticStripeReaderClosedEventArgs {
    const IID: ::windows::core::GUID = <IClaimedMagneticStripeReaderClosedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ClaimedMagneticStripeReaderClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedMagneticStripeReaderClosedEventArgs";
}
::windows::core::interface_hierarchy!(ClaimedMagneticStripeReaderClosedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ClaimedMagneticStripeReaderClosedEventArgs {}
unsafe impl ::core::marker::Sync for ClaimedMagneticStripeReaderClosedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ClaimedPosPrinter(::windows::core::IUnknown);
impl ClaimedPosPrinter {
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeviceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCharacterSet(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCharacterSet)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn CharacterSet(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CharacterSet)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsCoverOpen(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCoverOpen)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsCharacterSetMappingEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsCharacterSetMappingEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsCharacterSetMappingEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCharacterSetMappingEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetMapMode(&self, value: PosPrinterMapMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMapMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn MapMode(&self) -> ::windows::core::Result<PosPrinterMapMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MapMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Receipt(&self) -> ::windows::core::Result<ClaimedReceiptPrinter> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Receipt)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Slip(&self) -> ::windows::core::Result<ClaimedSlipPrinter> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Slip)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Journal(&self) -> ::windows::core::Result<ClaimedJournalPrinter> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Journal)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EnableAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DisableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisableAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RetainDeviceAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RetainDeviceAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ResetStatisticsAsync<P0, E0>(&self, statisticscategories: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ResetStatisticsAsync)(::windows::core::Vtable::as_raw(this), statisticscategories.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdateStatisticsAsync<P0, E0>(&self, statistics: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpdateStatisticsAsync)(::windows::core::Vtable::as_raw(this), statistics.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReleaseDeviceRequested(&self, handler: &super::super::Foundation::TypedEventHandler<ClaimedPosPrinter, PosPrinterReleaseDeviceRequestedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReleaseDeviceRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveReleaseDeviceRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveReleaseDeviceRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed(&self, handler: &super::super::Foundation::TypedEventHandler<ClaimedPosPrinter, ClaimedPosPrinterClosedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IClaimedPosPrinter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Closed)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IClaimedPosPrinter2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveClosed)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for ClaimedPosPrinter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedPosPrinter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedPosPrinter {}
impl ::core::fmt::Debug for ClaimedPosPrinter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedPosPrinter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ClaimedPosPrinter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedPosPrinter;{6d64ce0c-e03e-4b14-a38e-c28c34b86353})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ClaimedPosPrinter {
    type Vtable = IClaimedPosPrinter_Vtbl;
}
unsafe impl ::windows::core::Interface for ClaimedPosPrinter {
    const IID: ::windows::core::GUID = <IClaimedPosPrinter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ClaimedPosPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedPosPrinter";
}
::windows::core::interface_hierarchy!(ClaimedPosPrinter, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ClaimedPosPrinter> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ClaimedPosPrinter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ClaimedPosPrinter> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ClaimedPosPrinter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ClaimedPosPrinter> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ClaimedPosPrinter) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ClaimedPosPrinter {}
unsafe impl ::core::marker::Sync for ClaimedPosPrinter {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ClaimedPosPrinterClosedEventArgs(::windows::core::IUnknown);
impl ClaimedPosPrinterClosedEventArgs {}
impl ::core::clone::Clone for ClaimedPosPrinterClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedPosPrinterClosedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedPosPrinterClosedEventArgs {}
impl ::core::fmt::Debug for ClaimedPosPrinterClosedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedPosPrinterClosedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ClaimedPosPrinterClosedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedPosPrinterClosedEventArgs;{e2b7a27b-4d40-471d-92ed-63375b18c788})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ClaimedPosPrinterClosedEventArgs {
    type Vtable = IClaimedPosPrinterClosedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ClaimedPosPrinterClosedEventArgs {
    const IID: ::windows::core::GUID = <IClaimedPosPrinterClosedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ClaimedPosPrinterClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedPosPrinterClosedEventArgs";
}
::windows::core::interface_hierarchy!(ClaimedPosPrinterClosedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ClaimedPosPrinterClosedEventArgs {}
unsafe impl ::core::marker::Sync for ClaimedPosPrinterClosedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ClaimedReceiptPrinter(::windows::core::IUnknown);
impl ClaimedReceiptPrinter {
    pub fn SidewaysMaxLines(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SidewaysMaxLines)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SidewaysMaxChars(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SidewaysMaxChars)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LinesToPaperCut(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LinesToPaperCut)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PageSize(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PageSize)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PrintArea(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PrintArea)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateJob(&self) -> ::windows::core::Result<ReceiptPrintJob> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateJob)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCharactersPerLine(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetCharactersPerLine)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn CharactersPerLine(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CharactersPerLine)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLineHeight(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetLineHeight)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn LineHeight(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LineHeight)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLineSpacing(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetLineSpacing)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn LineSpacing(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LineSpacing)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LineWidth(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LineWidth)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsLetterQuality(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsLetterQuality)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsLetterQuality(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsLetterQuality)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsPaperNearEnd(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPaperNearEnd)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetColorCartridge(&self, value: PosPrinterColorCartridge) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetColorCartridge)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ColorCartridge(&self) -> ::windows::core::Result<PosPrinterColorCartridge> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ColorCartridge)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsCoverOpen(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCoverOpen)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsCartridgeRemoved(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCartridgeRemoved)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsCartridgeEmpty(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCartridgeEmpty)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsHeadCleaning(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsHeadCleaning)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsPaperEmpty(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPaperEmpty)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsReadyToPrint(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsReadyToPrint)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ValidateData(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ValidateData)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(data), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ClaimedReceiptPrinter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedReceiptPrinter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedReceiptPrinter {}
impl ::core::fmt::Debug for ClaimedReceiptPrinter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedReceiptPrinter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ClaimedReceiptPrinter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedReceiptPrinter;{9ad27a74-dd61-4ee2-9837-5b5d72d538b9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ClaimedReceiptPrinter {
    type Vtable = IClaimedReceiptPrinter_Vtbl;
}
unsafe impl ::windows::core::Interface for ClaimedReceiptPrinter {
    const IID: ::windows::core::GUID = <IClaimedReceiptPrinter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ClaimedReceiptPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedReceiptPrinter";
}
::windows::core::interface_hierarchy!(ClaimedReceiptPrinter, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<ClaimedReceiptPrinter> for ICommonClaimedPosPrinterStation {
    type Error = ::windows::core::Error;
    fn try_from(value: ClaimedReceiptPrinter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ClaimedReceiptPrinter> for ICommonClaimedPosPrinterStation {
    type Error = ::windows::core::Error;
    fn try_from(value: &ClaimedReceiptPrinter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&ClaimedReceiptPrinter> for ::windows::core::InParam<ICommonClaimedPosPrinterStation> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ClaimedReceiptPrinter) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ClaimedReceiptPrinter {}
unsafe impl ::core::marker::Sync for ClaimedReceiptPrinter {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ClaimedSlipPrinter(::windows::core::IUnknown);
impl ClaimedSlipPrinter {
    pub fn SidewaysMaxLines(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SidewaysMaxLines)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SidewaysMaxChars(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SidewaysMaxChars)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn MaxLines(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxLines)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LinesNearEndToEnd(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LinesNearEndToEnd)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PrintSide(&self) -> ::windows::core::Result<PosPrinterPrintSide> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PrintSide)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PageSize(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PageSize)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PrintArea(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PrintArea)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn OpenJaws(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).OpenJaws)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn CloseJaws(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).CloseJaws)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InsertSlipAsync(&self, timeout: super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InsertSlipAsync)(::windows::core::Vtable::as_raw(this), timeout, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSlipAsync(&self, timeout: super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveSlipAsync)(::windows::core::Vtable::as_raw(this), timeout, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ChangePrintSide(&self, printside: PosPrinterPrintSide) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ChangePrintSide)(::windows::core::Vtable::as_raw(this), printside).ok() }
    }
    pub fn CreateJob(&self) -> ::windows::core::Result<SlipPrintJob> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateJob)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCharactersPerLine(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetCharactersPerLine)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn CharactersPerLine(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CharactersPerLine)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLineHeight(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetLineHeight)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn LineHeight(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LineHeight)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLineSpacing(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetLineSpacing)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn LineSpacing(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LineSpacing)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LineWidth(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LineWidth)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsLetterQuality(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsLetterQuality)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsLetterQuality(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsLetterQuality)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsPaperNearEnd(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPaperNearEnd)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetColorCartridge(&self, value: PosPrinterColorCartridge) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetColorCartridge)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ColorCartridge(&self) -> ::windows::core::Result<PosPrinterColorCartridge> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ColorCartridge)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsCoverOpen(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCoverOpen)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsCartridgeRemoved(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCartridgeRemoved)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsCartridgeEmpty(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCartridgeEmpty)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsHeadCleaning(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsHeadCleaning)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsPaperEmpty(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPaperEmpty)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsReadyToPrint(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsReadyToPrint)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ValidateData(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ValidateData)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(data), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ClaimedSlipPrinter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedSlipPrinter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedSlipPrinter {}
impl ::core::fmt::Debug for ClaimedSlipPrinter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedSlipPrinter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ClaimedSlipPrinter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedSlipPrinter;{bd5deff2-af90-4e8a-b77b-e3ae9ca63a7f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ClaimedSlipPrinter {
    type Vtable = IClaimedSlipPrinter_Vtbl;
}
unsafe impl ::windows::core::Interface for ClaimedSlipPrinter {
    const IID: ::windows::core::GUID = <IClaimedSlipPrinter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ClaimedSlipPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedSlipPrinter";
}
::windows::core::interface_hierarchy!(ClaimedSlipPrinter, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<ClaimedSlipPrinter> for ICommonClaimedPosPrinterStation {
    type Error = ::windows::core::Error;
    fn try_from(value: ClaimedSlipPrinter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ClaimedSlipPrinter> for ICommonClaimedPosPrinterStation {
    type Error = ::windows::core::Error;
    fn try_from(value: &ClaimedSlipPrinter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&ClaimedSlipPrinter> for ::windows::core::InParam<ICommonClaimedPosPrinterStation> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ClaimedSlipPrinter) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ClaimedSlipPrinter {}
unsafe impl ::core::marker::Sync for ClaimedSlipPrinter {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct JournalPrintJob(::windows::core::IUnknown);
impl JournalPrintJob {
    pub fn Print(&self, data: &::windows::core::HSTRING, printoptions: &PosPrinterPrintOptions) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IJournalPrintJob>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Print)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(data), ::core::mem::transmute_copy(printoptions)).ok() }
    }
    pub fn FeedPaperByLine(&self, linecount: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IJournalPrintJob>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).FeedPaperByLine)(::windows::core::Vtable::as_raw(this), linecount).ok() }
    }
    pub fn FeedPaperByMapModeUnit(&self, distance: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IJournalPrintJob>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).FeedPaperByMapModeUnit)(::windows::core::Vtable::as_raw(this), distance).ok() }
    }
    pub fn Print2(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Print)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn PrintLine(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).PrintLine)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn PrintNewline(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).PrintNewline)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExecuteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExecuteAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for JournalPrintJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for JournalPrintJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for JournalPrintJob {}
impl ::core::fmt::Debug for JournalPrintJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JournalPrintJob").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for JournalPrintJob {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.JournalPrintJob;{9a94005c-0615-4591-a58f-30f87edfe2e4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for JournalPrintJob {
    type Vtable = IPosPrinterJob_Vtbl;
}
unsafe impl ::windows::core::Interface for JournalPrintJob {
    const IID: ::windows::core::GUID = <IPosPrinterJob as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for JournalPrintJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.JournalPrintJob";
}
::windows::core::interface_hierarchy!(JournalPrintJob, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<JournalPrintJob> for IPosPrinterJob {
    type Error = ::windows::core::Error;
    fn try_from(value: JournalPrintJob) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&JournalPrintJob> for IPosPrinterJob {
    type Error = ::windows::core::Error;
    fn try_from(value: &JournalPrintJob) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&JournalPrintJob> for ::windows::core::InParam<IPosPrinterJob> {
    type Error = ::windows::core::Error;
    fn try_from(value: &JournalPrintJob) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for JournalPrintJob {}
unsafe impl ::core::marker::Sync for JournalPrintJob {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct JournalPrinterCapabilities(::windows::core::IUnknown);
impl JournalPrinterCapabilities {
    pub fn IsPrinterPresent(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPrinterPresent)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsDualColorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDualColorSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ColorCartridgeCapabilities(&self) -> ::windows::core::Result<PosPrinterColorCapabilities> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ColorCartridgeCapabilities)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CartridgeSensors(&self) -> ::windows::core::Result<PosPrinterCartridgeSensors> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CartridgeSensors)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsBoldSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsBoldSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsItalicSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsItalicSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsUnderlineSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsUnderlineSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsDoubleHighPrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDoubleHighPrintSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDoubleWidePrintSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsDoubleHighDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDoubleHighDoubleWidePrintSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsPaperEmptySensorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPaperEmptySensorSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsPaperNearEndSensorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPaperNearEndSensorSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCharactersPerLine(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedCharactersPerLine)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsReverseVideoSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsReverseVideoSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsStrikethroughSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsStrikethroughSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsSuperscriptSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSuperscriptSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsSubscriptSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSubscriptSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsReversePaperFeedByLineSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsReversePaperFeedByLineSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsReversePaperFeedByMapModeUnitSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsReversePaperFeedByMapModeUnitSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for JournalPrinterCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for JournalPrinterCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for JournalPrinterCapabilities {}
impl ::core::fmt::Debug for JournalPrinterCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JournalPrinterCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for JournalPrinterCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.JournalPrinterCapabilities;{3b5ccc43-e047-4463-bb58-17b5ba1d8056})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for JournalPrinterCapabilities {
    type Vtable = IJournalPrinterCapabilities_Vtbl;
}
unsafe impl ::windows::core::Interface for JournalPrinterCapabilities {
    const IID: ::windows::core::GUID = <IJournalPrinterCapabilities as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for JournalPrinterCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.JournalPrinterCapabilities";
}
::windows::core::interface_hierarchy!(JournalPrinterCapabilities, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<JournalPrinterCapabilities> for ICommonPosPrintStationCapabilities {
    type Error = ::windows::core::Error;
    fn try_from(value: JournalPrinterCapabilities) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&JournalPrinterCapabilities> for ICommonPosPrintStationCapabilities {
    type Error = ::windows::core::Error;
    fn try_from(value: &JournalPrinterCapabilities) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&JournalPrinterCapabilities> for ::windows::core::InParam<ICommonPosPrintStationCapabilities> {
    type Error = ::windows::core::Error;
    fn try_from(value: &JournalPrinterCapabilities) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for JournalPrinterCapabilities {}
unsafe impl ::core::marker::Sync for JournalPrinterCapabilities {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct LineDisplay(::windows::core::IUnknown);
impl LineDisplay {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeviceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Capabilities(&self) -> ::windows::core::Result<LineDisplayCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Capabilities)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PhysicalDeviceName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PhysicalDeviceName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PhysicalDeviceDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PhysicalDeviceDescription)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DeviceControlDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeviceControlDescription)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DeviceControlVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeviceControlVersion)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DeviceServiceVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeviceServiceVersion)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ClaimAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedLineDisplay>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ClaimAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CheckPowerStatusAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayPowerStatus>> {
        let this = &::windows::core::Interface::cast::<ILineDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CheckPowerStatusAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplay>> {
        Self::ILineDisplayStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FromIdAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(deviceid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplay>> {
        Self::ILineDisplayStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefaultAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILineDisplayStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceSelector)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILineDisplayStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceSelectorWithConnectionTypes)(::windows::core::Vtable::as_raw(this), connectiontypes, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn StatisticsCategorySelector() -> ::windows::core::Result<LineDisplayStatisticsCategorySelector> {
        Self::ILineDisplayStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StatisticsCategorySelector)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILineDisplayStatics<R, F: FnOnce(&ILineDisplayStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<LineDisplay, ILineDisplayStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ILineDisplayStatics2<R, F: FnOnce(&ILineDisplayStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<LineDisplay, ILineDisplayStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for LineDisplay {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineDisplay {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineDisplay {}
impl ::core::fmt::Debug for LineDisplay {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplay").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplay {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplay;{24f5df4e-3c99-44e2-b73f-e51be3637a8c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for LineDisplay {
    type Vtable = ILineDisplay_Vtbl;
}
unsafe impl ::windows::core::Interface for LineDisplay {
    const IID: ::windows::core::GUID = <ILineDisplay as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LineDisplay {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplay";
}
::windows::core::interface_hierarchy!(LineDisplay, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<LineDisplay> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: LineDisplay) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&LineDisplay> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &LineDisplay) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&LineDisplay> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &LineDisplay) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for LineDisplay {}
unsafe impl ::core::marker::Sync for LineDisplay {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct LineDisplayAttributes(::windows::core::IUnknown);
impl LineDisplayAttributes {
    pub fn IsPowerNotifyEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPowerNotifyEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsPowerNotifyEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsPowerNotifyEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Brightness(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Brightness)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetBrightness(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBrightness)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BlinkRate(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BlinkRate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBlinkRate(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBlinkRate)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ScreenSizeInCharacters(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ScreenSizeInCharacters)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetScreenSizeInCharacters(&self, value: super::super::Foundation::Size) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetScreenSizeInCharacters)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn CharacterSet(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CharacterSet)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCharacterSet(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCharacterSet)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsCharacterSetMappingEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCharacterSetMappingEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsCharacterSetMappingEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsCharacterSetMappingEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn CurrentWindow(&self) -> ::windows::core::Result<LineDisplayWindow> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CurrentWindow)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCurrentWindow(&self, value: &LineDisplayWindow) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCurrentWindow)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::clone::Clone for LineDisplayAttributes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineDisplayAttributes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineDisplayAttributes {}
impl ::core::fmt::Debug for LineDisplayAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayAttributes").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayAttributes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayAttributes;{c17de99c-229a-4c14-a6f1-b4e4b1fead92})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for LineDisplayAttributes {
    type Vtable = ILineDisplayAttributes_Vtbl;
}
unsafe impl ::windows::core::Interface for LineDisplayAttributes {
    const IID: ::windows::core::GUID = <ILineDisplayAttributes as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LineDisplayAttributes {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayAttributes";
}
::windows::core::interface_hierarchy!(LineDisplayAttributes, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for LineDisplayAttributes {}
unsafe impl ::core::marker::Sync for LineDisplayAttributes {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct LineDisplayCapabilities(::windows::core::IUnknown);
impl LineDisplayCapabilities {
    pub fn IsStatisticsReportingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsStatisticsReportingSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsStatisticsUpdatingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsStatisticsUpdatingSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PowerReportingType(&self) -> ::windows::core::Result<UnifiedPosPowerReportingType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PowerReportingType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CanChangeScreenSize(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanChangeScreenSize)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CanDisplayBitmaps(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanDisplayBitmaps)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CanReadCharacterAtCursor(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanReadCharacterAtCursor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CanMapCharacterSets(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanMapCharacterSets)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CanDisplayCustomGlyphs(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanDisplayCustomGlyphs)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CanReverse(&self) -> ::windows::core::Result<LineDisplayTextAttributeGranularity> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanReverse)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CanBlink(&self) -> ::windows::core::Result<LineDisplayTextAttributeGranularity> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanBlink)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CanChangeBlinkRate(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanChangeBlinkRate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsBrightnessSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsBrightnessSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsCursorSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCursorSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsHorizontalMarqueeSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsHorizontalMarqueeSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsVerticalMarqueeSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsVerticalMarqueeSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsInterCharacterWaitSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsInterCharacterWaitSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SupportedDescriptors(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedDescriptors)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SupportedWindows(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedWindows)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for LineDisplayCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineDisplayCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineDisplayCapabilities {}
impl ::core::fmt::Debug for LineDisplayCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayCapabilities;{5a15b5d1-8dc5-4b9c-9172-303e47b70c55})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for LineDisplayCapabilities {
    type Vtable = ILineDisplayCapabilities_Vtbl;
}
unsafe impl ::windows::core::Interface for LineDisplayCapabilities {
    const IID: ::windows::core::GUID = <ILineDisplayCapabilities as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LineDisplayCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayCapabilities";
}
::windows::core::interface_hierarchy!(LineDisplayCapabilities, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for LineDisplayCapabilities {}
unsafe impl ::core::marker::Sync for LineDisplayCapabilities {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct LineDisplayCursor(::windows::core::IUnknown);
impl LineDisplayCursor {
    pub fn CanCustomize(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanCustomize)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsBlinkSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsBlinkSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsBlockSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsBlockSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsHalfBlockSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsHalfBlockSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsUnderlineSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsUnderlineSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsReverseSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsReverseSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsOtherSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsOtherSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetAttributes(&self) -> ::windows::core::Result<LineDisplayCursorAttributes> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAttributes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryUpdateAttributesAsync(&self, attributes: &LineDisplayCursorAttributes) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryUpdateAttributesAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(attributes), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for LineDisplayCursor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineDisplayCursor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineDisplayCursor {}
impl ::core::fmt::Debug for LineDisplayCursor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayCursor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayCursor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayCursor;{ecdffc45-754a-4e3b-ab2b-151181085605})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for LineDisplayCursor {
    type Vtable = ILineDisplayCursor_Vtbl;
}
unsafe impl ::windows::core::Interface for LineDisplayCursor {
    const IID: ::windows::core::GUID = <ILineDisplayCursor as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LineDisplayCursor {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayCursor";
}
::windows::core::interface_hierarchy!(LineDisplayCursor, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for LineDisplayCursor {}
unsafe impl ::core::marker::Sync for LineDisplayCursor {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct LineDisplayCursorAttributes(::windows::core::IUnknown);
impl LineDisplayCursorAttributes {
    pub fn IsBlinkEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsBlinkEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsBlinkEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsBlinkEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn CursorType(&self) -> ::windows::core::Result<LineDisplayCursorType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CursorType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCursorType(&self, value: LineDisplayCursorType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCursorType)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsAutoAdvanceEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAutoAdvanceEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsAutoAdvanceEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsAutoAdvanceEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Position)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPosition(&self, value: super::super::Foundation::Point) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPosition)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for LineDisplayCursorAttributes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineDisplayCursorAttributes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineDisplayCursorAttributes {}
impl ::core::fmt::Debug for LineDisplayCursorAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayCursorAttributes").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayCursorAttributes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayCursorAttributes;{4e2d54fe-4ffd-4190-aae1-ce285f20c896})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for LineDisplayCursorAttributes {
    type Vtable = ILineDisplayCursorAttributes_Vtbl;
}
unsafe impl ::windows::core::Interface for LineDisplayCursorAttributes {
    const IID: ::windows::core::GUID = <ILineDisplayCursorAttributes as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LineDisplayCursorAttributes {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayCursorAttributes";
}
::windows::core::interface_hierarchy!(LineDisplayCursorAttributes, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for LineDisplayCursorAttributes {}
unsafe impl ::core::marker::Sync for LineDisplayCursorAttributes {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct LineDisplayCustomGlyphs(::windows::core::IUnknown);
impl LineDisplayCustomGlyphs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SizeInPixels(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SizeInPixels)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedGlyphCodes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedGlyphCodes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn TryRedefineAsync<P0, E0>(&self, glyphcode: u32, glyphdata: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryRedefineAsync)(::windows::core::Vtable::as_raw(this), glyphcode, glyphdata.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for LineDisplayCustomGlyphs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineDisplayCustomGlyphs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineDisplayCustomGlyphs {}
impl ::core::fmt::Debug for LineDisplayCustomGlyphs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayCustomGlyphs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayCustomGlyphs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayCustomGlyphs;{2257f63c-f263-44f1-a1a0-e750a6a0ec54})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for LineDisplayCustomGlyphs {
    type Vtable = ILineDisplayCustomGlyphs_Vtbl;
}
unsafe impl ::windows::core::Interface for LineDisplayCustomGlyphs {
    const IID: ::windows::core::GUID = <ILineDisplayCustomGlyphs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LineDisplayCustomGlyphs {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayCustomGlyphs";
}
::windows::core::interface_hierarchy!(LineDisplayCustomGlyphs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for LineDisplayCustomGlyphs {}
unsafe impl ::core::marker::Sync for LineDisplayCustomGlyphs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct LineDisplayMarquee(::windows::core::IUnknown);
impl LineDisplayMarquee {
    pub fn Format(&self) -> ::windows::core::Result<LineDisplayMarqueeFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Format)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetFormat(&self, value: LineDisplayMarqueeFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetFormat)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RepeatWaitInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RepeatWaitInterval)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetRepeatWaitInterval(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetRepeatWaitInterval)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ScrollWaitInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ScrollWaitInterval)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetScrollWaitInterval(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetScrollWaitInterval)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryStartScrollingAsync(&self, direction: LineDisplayScrollDirection) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryStartScrollingAsync)(::windows::core::Vtable::as_raw(this), direction, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryStopScrollingAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryStopScrollingAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for LineDisplayMarquee {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineDisplayMarquee {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineDisplayMarquee {}
impl ::core::fmt::Debug for LineDisplayMarquee {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayMarquee").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayMarquee {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayMarquee;{a3d33e3e-f46a-4b7a-bc21-53eb3b57f8b4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for LineDisplayMarquee {
    type Vtable = ILineDisplayMarquee_Vtbl;
}
unsafe impl ::windows::core::Interface for LineDisplayMarquee {
    const IID: ::windows::core::GUID = <ILineDisplayMarquee as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LineDisplayMarquee {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayMarquee";
}
::windows::core::interface_hierarchy!(LineDisplayMarquee, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for LineDisplayMarquee {}
unsafe impl ::core::marker::Sync for LineDisplayMarquee {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct LineDisplayStatisticsCategorySelector(::windows::core::IUnknown);
impl LineDisplayStatisticsCategorySelector {
    pub fn AllStatistics(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllStatistics)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn UnifiedPosStatistics(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnifiedPosStatistics)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ManufacturerStatistics(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ManufacturerStatistics)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for LineDisplayStatisticsCategorySelector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineDisplayStatisticsCategorySelector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineDisplayStatisticsCategorySelector {}
impl ::core::fmt::Debug for LineDisplayStatisticsCategorySelector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayStatisticsCategorySelector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayStatisticsCategorySelector {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayStatisticsCategorySelector;{b521c46b-9274-4d24-94f3-b6017b832444})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for LineDisplayStatisticsCategorySelector {
    type Vtable = ILineDisplayStatisticsCategorySelector_Vtbl;
}
unsafe impl ::windows::core::Interface for LineDisplayStatisticsCategorySelector {
    const IID: ::windows::core::GUID = <ILineDisplayStatisticsCategorySelector as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LineDisplayStatisticsCategorySelector {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayStatisticsCategorySelector";
}
::windows::core::interface_hierarchy!(LineDisplayStatisticsCategorySelector, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for LineDisplayStatisticsCategorySelector {}
unsafe impl ::core::marker::Sync for LineDisplayStatisticsCategorySelector {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct LineDisplayStatusUpdatedEventArgs(::windows::core::IUnknown);
impl LineDisplayStatusUpdatedEventArgs {
    pub fn Status(&self) -> ::windows::core::Result<LineDisplayPowerStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for LineDisplayStatusUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineDisplayStatusUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineDisplayStatusUpdatedEventArgs {}
impl ::core::fmt::Debug for LineDisplayStatusUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayStatusUpdatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayStatusUpdatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayStatusUpdatedEventArgs;{ddd57c1a-86fb-4eba-93d1-6f5eda52b752})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for LineDisplayStatusUpdatedEventArgs {
    type Vtable = ILineDisplayStatusUpdatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for LineDisplayStatusUpdatedEventArgs {
    const IID: ::windows::core::GUID = <ILineDisplayStatusUpdatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LineDisplayStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayStatusUpdatedEventArgs";
}
::windows::core::interface_hierarchy!(LineDisplayStatusUpdatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for LineDisplayStatusUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for LineDisplayStatusUpdatedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct LineDisplayStoredBitmap(::windows::core::IUnknown);
impl LineDisplayStoredBitmap {
    pub fn EscapeSequence(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EscapeSequence)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryDeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryDeleteAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for LineDisplayStoredBitmap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineDisplayStoredBitmap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineDisplayStoredBitmap {}
impl ::core::fmt::Debug for LineDisplayStoredBitmap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayStoredBitmap").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayStoredBitmap {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayStoredBitmap;{f621515b-d81e-43ba-bf1b-bcfa3c785ba0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for LineDisplayStoredBitmap {
    type Vtable = ILineDisplayStoredBitmap_Vtbl;
}
unsafe impl ::windows::core::Interface for LineDisplayStoredBitmap {
    const IID: ::windows::core::GUID = <ILineDisplayStoredBitmap as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LineDisplayStoredBitmap {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayStoredBitmap";
}
::windows::core::interface_hierarchy!(LineDisplayStoredBitmap, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for LineDisplayStoredBitmap {}
unsafe impl ::core::marker::Sync for LineDisplayStoredBitmap {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct LineDisplayWindow(::windows::core::IUnknown);
impl LineDisplayWindow {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SizeInCharacters(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SizeInCharacters)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InterCharacterWaitInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InterCharacterWaitInterval)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetInterCharacterWaitInterval(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetInterCharacterWaitInterval)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryRefreshAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryRefreshAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryDisplayTextAsync(&self, text: &::windows::core::HSTRING, displayattribute: LineDisplayTextAttribute) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryDisplayTextAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(text), displayattribute, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryDisplayTextAtPositionAsync(&self, text: &::windows::core::HSTRING, displayattribute: LineDisplayTextAttribute, startposition: super::super::Foundation::Point) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryDisplayTextAtPositionAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(text), displayattribute, startposition, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryDisplayTextNormalAsync(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryDisplayTextNormalAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(text), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryScrollTextAsync(&self, direction: LineDisplayScrollDirection, numberofcolumnsorrows: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryScrollTextAsync)(::windows::core::Vtable::as_raw(this), direction, numberofcolumnsorrows, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryClearTextAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryClearTextAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Cursor(&self) -> ::windows::core::Result<LineDisplayCursor> {
        let this = &::windows::core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Cursor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Marquee(&self) -> ::windows::core::Result<LineDisplayMarquee> {
        let this = &::windows::core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Marquee)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadCharacterAtCursorAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = &::windows::core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadCharacterAtCursorAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryDisplayStoredBitmapAtCursorAsync(&self, bitmap: &LineDisplayStoredBitmap) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryDisplayStoredBitmapAtCursorAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(bitmap), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn TryDisplayStorageFileBitmapAtCursorAsync(&self, bitmap: &super::super::Storage::StorageFile) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryDisplayStorageFileBitmapAtCursorAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(bitmap), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn TryDisplayStorageFileBitmapAtCursorWithAlignmentAsync(&self, bitmap: &super::super::Storage::StorageFile, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryDisplayStorageFileBitmapAtCursorWithAlignmentAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(bitmap), horizontalalignment, verticalalignment, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn TryDisplayStorageFileBitmapAtCursorWithAlignmentAndWidthAsync(&self, bitmap: &super::super::Storage::StorageFile, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, widthinpixels: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryDisplayStorageFileBitmapAtCursorWithAlignmentAndWidthAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(bitmap), horizontalalignment, verticalalignment, widthinpixels, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn TryDisplayStorageFileBitmapAtPointAsync(&self, bitmap: &super::super::Storage::StorageFile, offsetinpixels: super::super::Foundation::Point) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryDisplayStorageFileBitmapAtPointAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(bitmap), offsetinpixels, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn TryDisplayStorageFileBitmapAtPointWithWidthAsync(&self, bitmap: &super::super::Storage::StorageFile, offsetinpixels: super::super::Foundation::Point, widthinpixels: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryDisplayStorageFileBitmapAtPointWithWidthAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(bitmap), offsetinpixels, widthinpixels, result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for LineDisplayWindow {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineDisplayWindow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineDisplayWindow {}
impl ::core::fmt::Debug for LineDisplayWindow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayWindow").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayWindow {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayWindow;{d21feef4-2364-4be5-bee1-851680af4964})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for LineDisplayWindow {
    type Vtable = ILineDisplayWindow_Vtbl;
}
unsafe impl ::windows::core::Interface for LineDisplayWindow {
    const IID: ::windows::core::GUID = <ILineDisplayWindow as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LineDisplayWindow {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayWindow";
}
::windows::core::interface_hierarchy!(LineDisplayWindow, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<LineDisplayWindow> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: LineDisplayWindow) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&LineDisplayWindow> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &LineDisplayWindow) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&LineDisplayWindow> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &LineDisplayWindow) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for LineDisplayWindow {}
unsafe impl ::core::marker::Sync for LineDisplayWindow {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct MagneticStripeReader(::windows::core::IUnknown);
impl MagneticStripeReader {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeviceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Capabilities(&self) -> ::windows::core::Result<MagneticStripeReaderCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Capabilities)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SupportedCardTypes(&self) -> ::windows::core::Result<::windows::core::Array<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedCardTypes)(::windows::core::Vtable::as_raw(this), ::windows::core::Array::<u32>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn DeviceAuthenticationProtocol(&self) -> ::windows::core::Result<MagneticStripeReaderAuthenticationProtocol> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeviceAuthenticationProtocol)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CheckHealthAsync)(::windows::core::Vtable::as_raw(this), level, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ClaimReaderAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedMagneticStripeReader>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ClaimReaderAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn RetrieveStatisticsAsync<P0, E0>(&self, statisticscategories: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RetrieveStatisticsAsync)(::windows::core::Vtable::as_raw(this), statisticscategories.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetErrorReportingType(&self) -> ::windows::core::Result<MagneticStripeReaderErrorReportingType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetErrorReportingType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StatusUpdated(&self, handler: &super::super::Foundation::TypedEventHandler<MagneticStripeReader, MagneticStripeReaderStatusUpdatedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StatusUpdated)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusUpdated(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveStatusUpdated)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MagneticStripeReader>> {
        Self::IMagneticStripeReaderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefaultAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MagneticStripeReader>> {
        Self::IMagneticStripeReaderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FromIdAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(deviceid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMagneticStripeReaderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceSelector)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMagneticStripeReaderStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceSelectorWithConnectionTypes)(::windows::core::Vtable::as_raw(this), connectiontypes, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMagneticStripeReaderStatics<R, F: FnOnce(&IMagneticStripeReaderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MagneticStripeReader, IMagneticStripeReaderStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMagneticStripeReaderStatics2<R, F: FnOnce(&IMagneticStripeReaderStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MagneticStripeReader, IMagneticStripeReaderStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for MagneticStripeReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MagneticStripeReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MagneticStripeReader {}
impl ::core::fmt::Debug for MagneticStripeReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReader;{1a92b015-47c3-468a-9333-0c6517574883})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MagneticStripeReader {
    type Vtable = IMagneticStripeReader_Vtbl;
}
unsafe impl ::windows::core::Interface for MagneticStripeReader {
    const IID: ::windows::core::GUID = <IMagneticStripeReader as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MagneticStripeReader {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReader";
}
::windows::core::interface_hierarchy!(MagneticStripeReader, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<MagneticStripeReader> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: MagneticStripeReader) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MagneticStripeReader> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &MagneticStripeReader) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MagneticStripeReader> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MagneticStripeReader) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for MagneticStripeReader {}
unsafe impl ::core::marker::Sync for MagneticStripeReader {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct MagneticStripeReaderAamvaCardDataReceivedEventArgs(::windows::core::IUnknown);
impl MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    pub fn Report(&self) -> ::windows::core::Result<MagneticStripeReaderReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Report)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LicenseNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LicenseNumber)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExpirationDate(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExpirationDate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Restrictions(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Restrictions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Class(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Class)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Endorsements(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Endorsements)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn BirthDate(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BirthDate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FirstName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FirstName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Surname(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Surname)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Suffix(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Suffix)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Gender(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Gender)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn HairColor(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HairColor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn EyeColor(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EyeColor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Height(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Height)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Weight(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Weight)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Address(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Address)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn City(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).City)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn State(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).State)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PostalCode(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PostalCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MagneticStripeReaderAamvaCardDataReceivedEventArgs {}
impl ::core::fmt::Debug for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderAamvaCardDataReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderAamvaCardDataReceivedEventArgs;{0a4bbd51-c316-4910-87f3-7a62ba862d31})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    type Vtable = IMagneticStripeReaderAamvaCardDataReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    const IID: ::windows::core::GUID = <IMagneticStripeReaderAamvaCardDataReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderAamvaCardDataReceivedEventArgs";
}
::windows::core::interface_hierarchy!(MagneticStripeReaderAamvaCardDataReceivedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MagneticStripeReaderAamvaCardDataReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MagneticStripeReaderAamvaCardDataReceivedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct MagneticStripeReaderBankCardDataReceivedEventArgs(::windows::core::IUnknown);
impl MagneticStripeReaderBankCardDataReceivedEventArgs {
    pub fn Report(&self) -> ::windows::core::Result<MagneticStripeReaderReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Report)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AccountNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccountNumber)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExpirationDate(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExpirationDate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ServiceCode(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ServiceCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Title)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FirstName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FirstName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn MiddleInitial(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MiddleInitial)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Surname(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Surname)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Suffix(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Suffix)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MagneticStripeReaderBankCardDataReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MagneticStripeReaderBankCardDataReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MagneticStripeReaderBankCardDataReceivedEventArgs {}
impl ::core::fmt::Debug for MagneticStripeReaderBankCardDataReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderBankCardDataReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderBankCardDataReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderBankCardDataReceivedEventArgs;{2e958823-a31a-4763-882c-23725e39b08e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MagneticStripeReaderBankCardDataReceivedEventArgs {
    type Vtable = IMagneticStripeReaderBankCardDataReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for MagneticStripeReaderBankCardDataReceivedEventArgs {
    const IID: ::windows::core::GUID = <IMagneticStripeReaderBankCardDataReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MagneticStripeReaderBankCardDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderBankCardDataReceivedEventArgs";
}
::windows::core::interface_hierarchy!(MagneticStripeReaderBankCardDataReceivedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MagneticStripeReaderBankCardDataReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MagneticStripeReaderBankCardDataReceivedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct MagneticStripeReaderCapabilities(::windows::core::IUnknown);
impl MagneticStripeReaderCapabilities {
    pub fn CardAuthentication(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CardAuthentication)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SupportedEncryptionAlgorithms(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedEncryptionAlgorithms)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AuthenticationLevel(&self) -> ::windows::core::Result<MagneticStripeReaderAuthenticationLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AuthenticationLevel)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsIsoSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsIsoSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsJisOneSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsJisOneSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsJisTwoSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsJisTwoSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PowerReportingType(&self) -> ::windows::core::Result<UnifiedPosPowerReportingType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PowerReportingType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsStatisticsReportingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsStatisticsReportingSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsStatisticsUpdatingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsStatisticsUpdatingSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsTrackDataMaskingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsTrackDataMaskingSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsTransmitSentinelsSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsTransmitSentinelsSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MagneticStripeReaderCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MagneticStripeReaderCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MagneticStripeReaderCapabilities {}
impl ::core::fmt::Debug for MagneticStripeReaderCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderCapabilities;{7128809c-c440-44a2-a467-469175d02896})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MagneticStripeReaderCapabilities {
    type Vtable = IMagneticStripeReaderCapabilities_Vtbl;
}
unsafe impl ::windows::core::Interface for MagneticStripeReaderCapabilities {
    const IID: ::windows::core::GUID = <IMagneticStripeReaderCapabilities as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MagneticStripeReaderCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderCapabilities";
}
::windows::core::interface_hierarchy!(MagneticStripeReaderCapabilities, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MagneticStripeReaderCapabilities {}
unsafe impl ::core::marker::Sync for MagneticStripeReaderCapabilities {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
pub struct MagneticStripeReaderCardTypes;
impl MagneticStripeReaderCardTypes {
    pub fn Unknown() -> ::windows::core::Result<u32> {
        Self::IMagneticStripeReaderCardTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Unknown)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Bank() -> ::windows::core::Result<u32> {
        Self::IMagneticStripeReaderCardTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Bank)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Aamva() -> ::windows::core::Result<u32> {
        Self::IMagneticStripeReaderCardTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Aamva)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn ExtendedBase() -> ::windows::core::Result<u32> {
        Self::IMagneticStripeReaderCardTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedBase)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMagneticStripeReaderCardTypesStatics<R, F: FnOnce(&IMagneticStripeReaderCardTypesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MagneticStripeReaderCardTypes, IMagneticStripeReaderCardTypesStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for MagneticStripeReaderCardTypes {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderCardTypes";
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
pub struct MagneticStripeReaderEncryptionAlgorithms;
impl MagneticStripeReaderEncryptionAlgorithms {
    pub fn None() -> ::windows::core::Result<u32> {
        Self::IMagneticStripeReaderEncryptionAlgorithmsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).None)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TripleDesDukpt() -> ::windows::core::Result<u32> {
        Self::IMagneticStripeReaderEncryptionAlgorithmsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TripleDesDukpt)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn ExtendedBase() -> ::windows::core::Result<u32> {
        Self::IMagneticStripeReaderEncryptionAlgorithmsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedBase)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMagneticStripeReaderEncryptionAlgorithmsStatics<R, F: FnOnce(&IMagneticStripeReaderEncryptionAlgorithmsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MagneticStripeReaderEncryptionAlgorithms, IMagneticStripeReaderEncryptionAlgorithmsStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for MagneticStripeReaderEncryptionAlgorithms {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderEncryptionAlgorithms";
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct MagneticStripeReaderErrorOccurredEventArgs(::windows::core::IUnknown);
impl MagneticStripeReaderErrorOccurredEventArgs {
    pub fn Track1Status(&self) -> ::windows::core::Result<MagneticStripeReaderTrackErrorType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Track1Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Track2Status(&self) -> ::windows::core::Result<MagneticStripeReaderTrackErrorType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Track2Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Track3Status(&self) -> ::windows::core::Result<MagneticStripeReaderTrackErrorType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Track3Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Track4Status(&self) -> ::windows::core::Result<MagneticStripeReaderTrackErrorType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Track4Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ErrorData(&self) -> ::windows::core::Result<UnifiedPosErrorData> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ErrorData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PartialInputData(&self) -> ::windows::core::Result<MagneticStripeReaderReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PartialInputData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MagneticStripeReaderErrorOccurredEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MagneticStripeReaderErrorOccurredEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MagneticStripeReaderErrorOccurredEventArgs {}
impl ::core::fmt::Debug for MagneticStripeReaderErrorOccurredEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderErrorOccurredEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderErrorOccurredEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderErrorOccurredEventArgs;{1fedf95d-2c84-41ad-b778-f2356a789ab1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MagneticStripeReaderErrorOccurredEventArgs {
    type Vtable = IMagneticStripeReaderErrorOccurredEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for MagneticStripeReaderErrorOccurredEventArgs {
    const IID: ::windows::core::GUID = <IMagneticStripeReaderErrorOccurredEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MagneticStripeReaderErrorOccurredEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderErrorOccurredEventArgs";
}
::windows::core::interface_hierarchy!(MagneticStripeReaderErrorOccurredEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MagneticStripeReaderErrorOccurredEventArgs {}
unsafe impl ::core::marker::Sync for MagneticStripeReaderErrorOccurredEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct MagneticStripeReaderReport(::windows::core::IUnknown);
impl MagneticStripeReaderReport {
    pub fn CardType(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CardType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Track1(&self) -> ::windows::core::Result<MagneticStripeReaderTrackData> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Track1)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Track2(&self) -> ::windows::core::Result<MagneticStripeReaderTrackData> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Track2)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Track3(&self) -> ::windows::core::Result<MagneticStripeReaderTrackData> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Track3)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Track4(&self) -> ::windows::core::Result<MagneticStripeReaderTrackData> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Track4)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CardAuthenticationData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CardAuthenticationData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CardAuthenticationDataLength(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CardAuthenticationDataLength)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn AdditionalSecurityInformation(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AdditionalSecurityInformation)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MagneticStripeReaderReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MagneticStripeReaderReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MagneticStripeReaderReport {}
impl ::core::fmt::Debug for MagneticStripeReaderReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderReport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderReport;{6a5b6047-99b0-4188-bef1-eddf79f78fe6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MagneticStripeReaderReport {
    type Vtable = IMagneticStripeReaderReport_Vtbl;
}
unsafe impl ::windows::core::Interface for MagneticStripeReaderReport {
    const IID: ::windows::core::GUID = <IMagneticStripeReaderReport as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MagneticStripeReaderReport {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderReport";
}
::windows::core::interface_hierarchy!(MagneticStripeReaderReport, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MagneticStripeReaderReport {}
unsafe impl ::core::marker::Sync for MagneticStripeReaderReport {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct MagneticStripeReaderStatusUpdatedEventArgs(::windows::core::IUnknown);
impl MagneticStripeReaderStatusUpdatedEventArgs {
    pub fn Status(&self) -> ::windows::core::Result<MagneticStripeReaderStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedStatus(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedStatus)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MagneticStripeReaderStatusUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MagneticStripeReaderStatusUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MagneticStripeReaderStatusUpdatedEventArgs {}
impl ::core::fmt::Debug for MagneticStripeReaderStatusUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderStatusUpdatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderStatusUpdatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderStatusUpdatedEventArgs;{09cc6bb0-3262-401d-9e8a-e80d6358906b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MagneticStripeReaderStatusUpdatedEventArgs {
    type Vtable = IMagneticStripeReaderStatusUpdatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for MagneticStripeReaderStatusUpdatedEventArgs {
    const IID: ::windows::core::GUID = <IMagneticStripeReaderStatusUpdatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MagneticStripeReaderStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderStatusUpdatedEventArgs";
}
::windows::core::interface_hierarchy!(MagneticStripeReaderStatusUpdatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MagneticStripeReaderStatusUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for MagneticStripeReaderStatusUpdatedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct MagneticStripeReaderTrackData(::windows::core::IUnknown);
impl MagneticStripeReaderTrackData {
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Data)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn DiscretionaryData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DiscretionaryData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn EncryptedData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EncryptedData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MagneticStripeReaderTrackData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MagneticStripeReaderTrackData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MagneticStripeReaderTrackData {}
impl ::core::fmt::Debug for MagneticStripeReaderTrackData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderTrackData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderTrackData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderTrackData;{104cf671-4a9d-446e-abc5-20402307ba36})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MagneticStripeReaderTrackData {
    type Vtable = IMagneticStripeReaderTrackData_Vtbl;
}
unsafe impl ::windows::core::Interface for MagneticStripeReaderTrackData {
    const IID: ::windows::core::GUID = <IMagneticStripeReaderTrackData as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MagneticStripeReaderTrackData {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderTrackData";
}
::windows::core::interface_hierarchy!(MagneticStripeReaderTrackData, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MagneticStripeReaderTrackData {}
unsafe impl ::core::marker::Sync for MagneticStripeReaderTrackData {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs(::windows::core::IUnknown);
impl MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    pub fn Report(&self) -> ::windows::core::Result<MagneticStripeReaderReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Report)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {}
impl ::core::fmt::Debug for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs;{af0a5514-59cc-4a60-99e8-99a53dace5aa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    type Vtable = IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    const IID: ::windows::core::GUID = <IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs";
}
::windows::core::interface_hierarchy!(MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct PosPrinter(::windows::core::IUnknown);
impl PosPrinter {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeviceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Capabilities(&self) -> ::windows::core::Result<PosPrinterCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Capabilities)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCharacterSets(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedCharacterSets)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedTypeFaces(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedTypeFaces)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<PosPrinterStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ClaimPrinterAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedPosPrinter>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ClaimPrinterAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CheckHealthAsync)(::windows::core::Vtable::as_raw(this), level, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStatisticsAsync<P0, E0>(&self, statisticscategories: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStatisticsAsync)(::windows::core::Vtable::as_raw(this), statisticscategories.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StatusUpdated(&self, handler: &super::super::Foundation::TypedEventHandler<PosPrinter, PosPrinterStatusUpdatedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StatusUpdated)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusUpdated(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveStatusUpdated)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedBarcodeSymbologies(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::core::Interface::cast::<IPosPrinter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedBarcodeSymbologies)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetFontProperty(&self, typeface: &::windows::core::HSTRING) -> ::windows::core::Result<PosPrinterFontProperty> {
        let this = &::windows::core::Interface::cast::<IPosPrinter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetFontProperty)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(typeface), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PosPrinter>> {
        Self::IPosPrinterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefaultAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PosPrinter>> {
        Self::IPosPrinterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FromIdAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(deviceid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IPosPrinterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceSelector)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IPosPrinterStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceSelectorWithConnectionTypes)(::windows::core::Vtable::as_raw(this), connectiontypes, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPosPrinterStatics<R, F: FnOnce(&IPosPrinterStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PosPrinter, IPosPrinterStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPosPrinterStatics2<R, F: FnOnce(&IPosPrinterStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PosPrinter, IPosPrinterStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PosPrinter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PosPrinter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PosPrinter {}
impl ::core::fmt::Debug for PosPrinter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinter;{2a03c10e-9a19-4a01-994f-12dfad6adcbf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PosPrinter {
    type Vtable = IPosPrinter_Vtbl;
}
unsafe impl ::windows::core::Interface for PosPrinter {
    const IID: ::windows::core::GUID = <IPosPrinter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PosPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinter";
}
::windows::core::interface_hierarchy!(PosPrinter, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<PosPrinter> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: PosPrinter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&PosPrinter> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &PosPrinter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&PosPrinter> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PosPrinter) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for PosPrinter {}
unsafe impl ::core::marker::Sync for PosPrinter {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct PosPrinterCapabilities(::windows::core::IUnknown);
impl PosPrinterCapabilities {
    pub fn PowerReportingType(&self) -> ::windows::core::Result<UnifiedPosPowerReportingType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PowerReportingType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsStatisticsReportingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsStatisticsReportingSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsStatisticsUpdatingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsStatisticsUpdatingSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DefaultCharacterSet(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DefaultCharacterSet)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn HasCoverSensor(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasCoverSensor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CanMapCharacterSet(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanMapCharacterSet)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsTransactionSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsTransactionSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Receipt(&self) -> ::windows::core::Result<ReceiptPrinterCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Receipt)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Slip(&self) -> ::windows::core::Result<SlipPrinterCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Slip)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Journal(&self) -> ::windows::core::Result<JournalPrinterCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Journal)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for PosPrinterCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PosPrinterCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PosPrinterCapabilities {}
impl ::core::fmt::Debug for PosPrinterCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinterCapabilities;{cde95721-4380-4985-adc5-39db30cd93bc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PosPrinterCapabilities {
    type Vtable = IPosPrinterCapabilities_Vtbl;
}
unsafe impl ::windows::core::Interface for PosPrinterCapabilities {
    const IID: ::windows::core::GUID = <IPosPrinterCapabilities as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PosPrinterCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterCapabilities";
}
::windows::core::interface_hierarchy!(PosPrinterCapabilities, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PosPrinterCapabilities {}
unsafe impl ::core::marker::Sync for PosPrinterCapabilities {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
pub struct PosPrinterCharacterSetIds;
impl PosPrinterCharacterSetIds {
    pub fn Utf16LE() -> ::windows::core::Result<u32> {
        Self::IPosPrinterCharacterSetIdsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Utf16LE)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Ascii() -> ::windows::core::Result<u32> {
        Self::IPosPrinterCharacterSetIdsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Ascii)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Ansi() -> ::windows::core::Result<u32> {
        Self::IPosPrinterCharacterSetIdsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Ansi)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPosPrinterCharacterSetIdsStatics<R, F: FnOnce(&IPosPrinterCharacterSetIdsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PosPrinterCharacterSetIds, IPosPrinterCharacterSetIdsStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for PosPrinterCharacterSetIds {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterCharacterSetIds";
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct PosPrinterFontProperty(::windows::core::IUnknown);
impl PosPrinterFontProperty {
    pub fn TypeFace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TypeFace)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsScalableToAnySize(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsScalableToAnySize)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CharacterSizes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SizeUInt32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CharacterSizes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for PosPrinterFontProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PosPrinterFontProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PosPrinterFontProperty {}
impl ::core::fmt::Debug for PosPrinterFontProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterFontProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterFontProperty {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinterFontProperty;{a7f4e93a-f8ac-5f04-84d2-29b16d8a633c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PosPrinterFontProperty {
    type Vtable = IPosPrinterFontProperty_Vtbl;
}
unsafe impl ::windows::core::Interface for PosPrinterFontProperty {
    const IID: ::windows::core::GUID = <IPosPrinterFontProperty as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PosPrinterFontProperty {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterFontProperty";
}
::windows::core::interface_hierarchy!(PosPrinterFontProperty, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PosPrinterFontProperty {}
unsafe impl ::core::marker::Sync for PosPrinterFontProperty {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct PosPrinterPrintOptions(::windows::core::IUnknown);
impl PosPrinterPrintOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PosPrinterPrintOptions, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn TypeFace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TypeFace)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetTypeFace(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTypeFace)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn CharacterHeight(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CharacterHeight)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCharacterHeight(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCharacterHeight)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Bold(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Bold)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetBold(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBold)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Italic(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Italic)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetItalic(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetItalic)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Underline(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Underline)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetUnderline(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetUnderline)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ReverseVideo(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReverseVideo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetReverseVideo(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetReverseVideo)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Strikethrough(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Strikethrough)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetStrikethrough(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetStrikethrough)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Superscript(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Superscript)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSuperscript(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSuperscript)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Subscript(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Subscript)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSubscript(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSubscript)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn DoubleWide(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DoubleWide)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDoubleWide(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDoubleWide)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn DoubleHigh(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DoubleHigh)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDoubleHigh(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDoubleHigh)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Alignment(&self) -> ::windows::core::Result<PosPrinterAlignment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Alignment)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAlignment(&self, value: PosPrinterAlignment) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAlignment)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn CharacterSet(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CharacterSet)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCharacterSet(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCharacterSet)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for PosPrinterPrintOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PosPrinterPrintOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PosPrinterPrintOptions {}
impl ::core::fmt::Debug for PosPrinterPrintOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterPrintOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterPrintOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinterPrintOptions;{0a2e16fd-1d02-5a58-9d59-bfcde76fde86})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PosPrinterPrintOptions {
    type Vtable = IPosPrinterPrintOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for PosPrinterPrintOptions {
    const IID: ::windows::core::GUID = <IPosPrinterPrintOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PosPrinterPrintOptions {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterPrintOptions";
}
::windows::core::interface_hierarchy!(PosPrinterPrintOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PosPrinterPrintOptions {}
unsafe impl ::core::marker::Sync for PosPrinterPrintOptions {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct PosPrinterReleaseDeviceRequestedEventArgs(::windows::core::IUnknown);
impl PosPrinterReleaseDeviceRequestedEventArgs {}
impl ::core::clone::Clone for PosPrinterReleaseDeviceRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PosPrinterReleaseDeviceRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PosPrinterReleaseDeviceRequestedEventArgs {}
impl ::core::fmt::Debug for PosPrinterReleaseDeviceRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterReleaseDeviceRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterReleaseDeviceRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinterReleaseDeviceRequestedEventArgs;{2bcba359-1cef-40b2-9ecb-f927f856ae3c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PosPrinterReleaseDeviceRequestedEventArgs {
    type Vtable = IPosPrinterReleaseDeviceRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for PosPrinterReleaseDeviceRequestedEventArgs {
    const IID: ::windows::core::GUID = <IPosPrinterReleaseDeviceRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PosPrinterReleaseDeviceRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterReleaseDeviceRequestedEventArgs";
}
::windows::core::interface_hierarchy!(PosPrinterReleaseDeviceRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PosPrinterReleaseDeviceRequestedEventArgs {}
unsafe impl ::core::marker::Sync for PosPrinterReleaseDeviceRequestedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct PosPrinterStatus(::windows::core::IUnknown);
impl PosPrinterStatus {
    pub fn StatusKind(&self) -> ::windows::core::Result<PosPrinterStatusKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StatusKind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedStatus(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedStatus)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for PosPrinterStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PosPrinterStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PosPrinterStatus {}
impl ::core::fmt::Debug for PosPrinterStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinterStatus;{d1f0c730-da40-4328-bf76-5156fa33b747})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PosPrinterStatus {
    type Vtable = IPosPrinterStatus_Vtbl;
}
unsafe impl ::windows::core::Interface for PosPrinterStatus {
    const IID: ::windows::core::GUID = <IPosPrinterStatus as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PosPrinterStatus {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterStatus";
}
::windows::core::interface_hierarchy!(PosPrinterStatus, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PosPrinterStatus {}
unsafe impl ::core::marker::Sync for PosPrinterStatus {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct PosPrinterStatusUpdatedEventArgs(::windows::core::IUnknown);
impl PosPrinterStatusUpdatedEventArgs {
    pub fn Status(&self) -> ::windows::core::Result<PosPrinterStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for PosPrinterStatusUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PosPrinterStatusUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PosPrinterStatusUpdatedEventArgs {}
impl ::core::fmt::Debug for PosPrinterStatusUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterStatusUpdatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterStatusUpdatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinterStatusUpdatedEventArgs;{2edb87df-13a6-428d-ba81-b0e7c3e5a3cd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PosPrinterStatusUpdatedEventArgs {
    type Vtable = IPosPrinterStatusUpdatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for PosPrinterStatusUpdatedEventArgs {
    const IID: ::windows::core::GUID = <IPosPrinterStatusUpdatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PosPrinterStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterStatusUpdatedEventArgs";
}
::windows::core::interface_hierarchy!(PosPrinterStatusUpdatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PosPrinterStatusUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for PosPrinterStatusUpdatedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ReceiptPrintJob(::windows::core::IUnknown);
impl ReceiptPrintJob {
    pub fn Print(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Print)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn PrintLine(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).PrintLine)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn PrintNewline(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).PrintNewline)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExecuteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExecuteAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetBarcodeRotation(&self, value: PosPrinterRotation) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetBarcodeRotation)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn SetPrintRotation(&self, value: PosPrinterRotation, includebitmaps: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetPrintRotation)(::windows::core::Vtable::as_raw(this), value, includebitmaps).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPrintArea(&self, value: super::super::Foundation::Rect) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetPrintArea)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetBitmap(&self, bitmapnumber: u32, bitmap: &super::super::Graphics::Imaging::BitmapFrame, alignment: PosPrinterAlignment) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetBitmap)(::windows::core::Vtable::as_raw(this), bitmapnumber, ::core::mem::transmute_copy(bitmap), alignment).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetBitmapCustomWidthStandardAlign(&self, bitmapnumber: u32, bitmap: &super::super::Graphics::Imaging::BitmapFrame, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetBitmapCustomWidthStandardAlign)(::windows::core::Vtable::as_raw(this), bitmapnumber, ::core::mem::transmute_copy(bitmap), alignment, width).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetCustomAlignedBitmap(&self, bitmapnumber: u32, bitmap: &super::super::Graphics::Imaging::BitmapFrame, alignmentdistance: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetCustomAlignedBitmap)(::windows::core::Vtable::as_raw(this), bitmapnumber, ::core::mem::transmute_copy(bitmap), alignmentdistance).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetBitmapCustomWidthCustomAlign(&self, bitmapnumber: u32, bitmap: &super::super::Graphics::Imaging::BitmapFrame, alignmentdistance: u32, width: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetBitmapCustomWidthCustomAlign)(::windows::core::Vtable::as_raw(this), bitmapnumber, ::core::mem::transmute_copy(bitmap), alignmentdistance, width).ok() }
    }
    pub fn PrintSavedBitmap(&self, bitmapnumber: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).PrintSavedBitmap)(::windows::core::Vtable::as_raw(this), bitmapnumber).ok() }
    }
    pub fn DrawRuledLine(&self, positionlist: &::windows::core::HSTRING, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).DrawRuledLine)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(positionlist), linedirection, linewidth, linestyle, linecolor).ok() }
    }
    pub fn PrintBarcode(&self, data: &::windows::core::HSTRING, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).PrintBarcode)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(data), symbology, height, width, textposition, alignment).ok() }
    }
    pub fn PrintBarcodeCustomAlign(&self, data: &::windows::core::HSTRING, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).PrintBarcodeCustomAlign)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(data), symbology, height, width, textposition, alignmentdistance).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintBitmap(&self, bitmap: &super::super::Graphics::Imaging::BitmapFrame, alignment: PosPrinterAlignment) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).PrintBitmap)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(bitmap), alignment).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintBitmapCustomWidthStandardAlign(&self, bitmap: &super::super::Graphics::Imaging::BitmapFrame, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).PrintBitmapCustomWidthStandardAlign)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(bitmap), alignment, width).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintCustomAlignedBitmap(&self, bitmap: &super::super::Graphics::Imaging::BitmapFrame, alignmentdistance: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).PrintCustomAlignedBitmap)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(bitmap), alignmentdistance).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintBitmapCustomWidthCustomAlign(&self, bitmap: &super::super::Graphics::Imaging::BitmapFrame, alignmentdistance: u32, width: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).PrintBitmapCustomWidthCustomAlign)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(bitmap), alignmentdistance, width).ok() }
    }
    pub fn MarkFeed(&self, kind: PosPrinterMarkFeedKind) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).MarkFeed)(::windows::core::Vtable::as_raw(this), kind).ok() }
    }
    pub fn CutPaper(&self, percentage: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).CutPaper)(::windows::core::Vtable::as_raw(this), percentage).ok() }
    }
    pub fn CutPaperDefault(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).CutPaperDefault)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn StampPaper(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptPrintJob2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StampPaper)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Print2(&self, data: &::windows::core::HSTRING, printoptions: &PosPrinterPrintOptions) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptPrintJob2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Print)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(data), ::core::mem::transmute_copy(printoptions)).ok() }
    }
    pub fn FeedPaperByLine(&self, linecount: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptPrintJob2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).FeedPaperByLine)(::windows::core::Vtable::as_raw(this), linecount).ok() }
    }
    pub fn FeedPaperByMapModeUnit(&self, distance: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptPrintJob2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).FeedPaperByMapModeUnit)(::windows::core::Vtable::as_raw(this), distance).ok() }
    }
}
impl ::core::clone::Clone for ReceiptPrintJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ReceiptPrintJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ReceiptPrintJob {}
impl ::core::fmt::Debug for ReceiptPrintJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ReceiptPrintJob").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ReceiptPrintJob {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ReceiptPrintJob;{aa96066e-acad-4b79-9d0f-c0cfc08dc77b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ReceiptPrintJob {
    type Vtable = IReceiptPrintJob_Vtbl;
}
unsafe impl ::windows::core::Interface for ReceiptPrintJob {
    const IID: ::windows::core::GUID = <IReceiptPrintJob as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ReceiptPrintJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.ReceiptPrintJob";
}
::windows::core::interface_hierarchy!(ReceiptPrintJob, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<ReceiptPrintJob> for IPosPrinterJob {
    type Error = ::windows::core::Error;
    fn try_from(value: ReceiptPrintJob) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ReceiptPrintJob> for IPosPrinterJob {
    type Error = ::windows::core::Error;
    fn try_from(value: &ReceiptPrintJob) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&ReceiptPrintJob> for ::windows::core::InParam<IPosPrinterJob> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ReceiptPrintJob) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ReceiptPrintJob> for IReceiptOrSlipJob {
    type Error = ::windows::core::Error;
    fn try_from(value: ReceiptPrintJob) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ReceiptPrintJob> for IReceiptOrSlipJob {
    type Error = ::windows::core::Error;
    fn try_from(value: &ReceiptPrintJob) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&ReceiptPrintJob> for ::windows::core::InParam<IReceiptOrSlipJob> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ReceiptPrintJob) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ReceiptPrintJob {}
unsafe impl ::core::marker::Sync for ReceiptPrintJob {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ReceiptPrinterCapabilities(::windows::core::IUnknown);
impl ReceiptPrinterCapabilities {
    pub fn IsPrinterPresent(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPrinterPresent)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsDualColorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDualColorSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ColorCartridgeCapabilities(&self) -> ::windows::core::Result<PosPrinterColorCapabilities> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ColorCartridgeCapabilities)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CartridgeSensors(&self) -> ::windows::core::Result<PosPrinterCartridgeSensors> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CartridgeSensors)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsBoldSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsBoldSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsItalicSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsItalicSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsUnderlineSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsUnderlineSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsDoubleHighPrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDoubleHighPrintSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDoubleWidePrintSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsDoubleHighDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDoubleHighDoubleWidePrintSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsPaperEmptySensorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPaperEmptySensorSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsPaperNearEndSensorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPaperNearEndSensorSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCharactersPerLine(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedCharactersPerLine)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsBarcodeSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsBarcodeSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsBitmapSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsBitmapSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsLeft90RotationSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsLeft90RotationSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsRight90RotationSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsRight90RotationSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Is180RotationSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Is180RotationSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsPrintAreaSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPrintAreaSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RuledLineCapabilities(&self) -> ::windows::core::Result<PosPrinterRuledLineCapabilities> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RuledLineCapabilities)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedBarcodeRotations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedBarcodeRotations)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedBitmapRotations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedBitmapRotations)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CanCutPaper(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanCutPaper)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsStampSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsStampSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn MarkFeedCapabilities(&self) -> ::windows::core::Result<PosPrinterMarkFeedCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MarkFeedCapabilities)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsReverseVideoSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsReverseVideoSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsStrikethroughSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsStrikethroughSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsSuperscriptSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSuperscriptSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsSubscriptSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSubscriptSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsReversePaperFeedByLineSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsReversePaperFeedByLineSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsReversePaperFeedByMapModeUnitSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsReversePaperFeedByMapModeUnitSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ReceiptPrinterCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ReceiptPrinterCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ReceiptPrinterCapabilities {}
impl ::core::fmt::Debug for ReceiptPrinterCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ReceiptPrinterCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ReceiptPrinterCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ReceiptPrinterCapabilities;{b8f0b58f-51a8-43fc-9bd5-8de272a6415b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ReceiptPrinterCapabilities {
    type Vtable = IReceiptPrinterCapabilities_Vtbl;
}
unsafe impl ::windows::core::Interface for ReceiptPrinterCapabilities {
    const IID: ::windows::core::GUID = <IReceiptPrinterCapabilities as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ReceiptPrinterCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.ReceiptPrinterCapabilities";
}
::windows::core::interface_hierarchy!(ReceiptPrinterCapabilities, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<ReceiptPrinterCapabilities> for ICommonPosPrintStationCapabilities {
    type Error = ::windows::core::Error;
    fn try_from(value: ReceiptPrinterCapabilities) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ReceiptPrinterCapabilities> for ICommonPosPrintStationCapabilities {
    type Error = ::windows::core::Error;
    fn try_from(value: &ReceiptPrinterCapabilities) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&ReceiptPrinterCapabilities> for ::windows::core::InParam<ICommonPosPrintStationCapabilities> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ReceiptPrinterCapabilities) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ReceiptPrinterCapabilities> for ICommonReceiptSlipCapabilities {
    type Error = ::windows::core::Error;
    fn try_from(value: ReceiptPrinterCapabilities) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ReceiptPrinterCapabilities> for ICommonReceiptSlipCapabilities {
    type Error = ::windows::core::Error;
    fn try_from(value: &ReceiptPrinterCapabilities) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&ReceiptPrinterCapabilities> for ::windows::core::InParam<ICommonReceiptSlipCapabilities> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ReceiptPrinterCapabilities) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ReceiptPrinterCapabilities {}
unsafe impl ::core::marker::Sync for ReceiptPrinterCapabilities {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct SlipPrintJob(::windows::core::IUnknown);
impl SlipPrintJob {
    pub fn Print(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Print)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn PrintLine(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).PrintLine)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(data)).ok() }
    }
    pub fn PrintNewline(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).PrintNewline)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExecuteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExecuteAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetBarcodeRotation(&self, value: PosPrinterRotation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBarcodeRotation)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn SetPrintRotation(&self, value: PosPrinterRotation, includebitmaps: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPrintRotation)(::windows::core::Vtable::as_raw(this), value, includebitmaps).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPrintArea(&self, value: super::super::Foundation::Rect) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPrintArea)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetBitmap(&self, bitmapnumber: u32, bitmap: &super::super::Graphics::Imaging::BitmapFrame, alignment: PosPrinterAlignment) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBitmap)(::windows::core::Vtable::as_raw(this), bitmapnumber, ::core::mem::transmute_copy(bitmap), alignment).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetBitmapCustomWidthStandardAlign(&self, bitmapnumber: u32, bitmap: &super::super::Graphics::Imaging::BitmapFrame, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBitmapCustomWidthStandardAlign)(::windows::core::Vtable::as_raw(this), bitmapnumber, ::core::mem::transmute_copy(bitmap), alignment, width).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetCustomAlignedBitmap(&self, bitmapnumber: u32, bitmap: &super::super::Graphics::Imaging::BitmapFrame, alignmentdistance: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCustomAlignedBitmap)(::windows::core::Vtable::as_raw(this), bitmapnumber, ::core::mem::transmute_copy(bitmap), alignmentdistance).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetBitmapCustomWidthCustomAlign(&self, bitmapnumber: u32, bitmap: &super::super::Graphics::Imaging::BitmapFrame, alignmentdistance: u32, width: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBitmapCustomWidthCustomAlign)(::windows::core::Vtable::as_raw(this), bitmapnumber, ::core::mem::transmute_copy(bitmap), alignmentdistance, width).ok() }
    }
    pub fn PrintSavedBitmap(&self, bitmapnumber: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).PrintSavedBitmap)(::windows::core::Vtable::as_raw(this), bitmapnumber).ok() }
    }
    pub fn DrawRuledLine(&self, positionlist: &::windows::core::HSTRING, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).DrawRuledLine)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(positionlist), linedirection, linewidth, linestyle, linecolor).ok() }
    }
    pub fn PrintBarcode(&self, data: &::windows::core::HSTRING, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).PrintBarcode)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(data), symbology, height, width, textposition, alignment).ok() }
    }
    pub fn PrintBarcodeCustomAlign(&self, data: &::windows::core::HSTRING, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).PrintBarcodeCustomAlign)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(data), symbology, height, width, textposition, alignmentdistance).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintBitmap(&self, bitmap: &super::super::Graphics::Imaging::BitmapFrame, alignment: PosPrinterAlignment) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).PrintBitmap)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(bitmap), alignment).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintBitmapCustomWidthStandardAlign(&self, bitmap: &super::super::Graphics::Imaging::BitmapFrame, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).PrintBitmapCustomWidthStandardAlign)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(bitmap), alignment, width).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintCustomAlignedBitmap(&self, bitmap: &super::super::Graphics::Imaging::BitmapFrame, alignmentdistance: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).PrintCustomAlignedBitmap)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(bitmap), alignmentdistance).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintBitmapCustomWidthCustomAlign(&self, bitmap: &super::super::Graphics::Imaging::BitmapFrame, alignmentdistance: u32, width: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).PrintBitmapCustomWidthCustomAlign)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(bitmap), alignmentdistance, width).ok() }
    }
    pub fn Print2(&self, data: &::windows::core::HSTRING, printoptions: &PosPrinterPrintOptions) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISlipPrintJob>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Print)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(data), ::core::mem::transmute_copy(printoptions)).ok() }
    }
    pub fn FeedPaperByLine(&self, linecount: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISlipPrintJob>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).FeedPaperByLine)(::windows::core::Vtable::as_raw(this), linecount).ok() }
    }
    pub fn FeedPaperByMapModeUnit(&self, distance: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISlipPrintJob>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).FeedPaperByMapModeUnit)(::windows::core::Vtable::as_raw(this), distance).ok() }
    }
}
impl ::core::clone::Clone for SlipPrintJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SlipPrintJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SlipPrintJob {}
impl ::core::fmt::Debug for SlipPrintJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SlipPrintJob").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SlipPrintJob {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.SlipPrintJob;{532199be-c8c3-4dc2-89e9-5c4a37b34ddc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SlipPrintJob {
    type Vtable = IReceiptOrSlipJob_Vtbl;
}
unsafe impl ::windows::core::Interface for SlipPrintJob {
    const IID: ::windows::core::GUID = <IReceiptOrSlipJob as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SlipPrintJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.SlipPrintJob";
}
::windows::core::interface_hierarchy!(SlipPrintJob, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<SlipPrintJob> for IPosPrinterJob {
    type Error = ::windows::core::Error;
    fn try_from(value: SlipPrintJob) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SlipPrintJob> for IPosPrinterJob {
    type Error = ::windows::core::Error;
    fn try_from(value: &SlipPrintJob) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&SlipPrintJob> for ::windows::core::InParam<IPosPrinterJob> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SlipPrintJob) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<SlipPrintJob> for IReceiptOrSlipJob {
    type Error = ::windows::core::Error;
    fn try_from(value: SlipPrintJob) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SlipPrintJob> for IReceiptOrSlipJob {
    type Error = ::windows::core::Error;
    fn try_from(value: &SlipPrintJob) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&SlipPrintJob> for ::windows::core::InParam<IReceiptOrSlipJob> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SlipPrintJob) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for SlipPrintJob {}
unsafe impl ::core::marker::Sync for SlipPrintJob {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct SlipPrinterCapabilities(::windows::core::IUnknown);
impl SlipPrinterCapabilities {
    pub fn IsPrinterPresent(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPrinterPresent)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsDualColorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDualColorSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ColorCartridgeCapabilities(&self) -> ::windows::core::Result<PosPrinterColorCapabilities> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ColorCartridgeCapabilities)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CartridgeSensors(&self) -> ::windows::core::Result<PosPrinterCartridgeSensors> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CartridgeSensors)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsBoldSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsBoldSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsItalicSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsItalicSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsUnderlineSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsUnderlineSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsDoubleHighPrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDoubleHighPrintSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDoubleWidePrintSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsDoubleHighDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDoubleHighDoubleWidePrintSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsPaperEmptySensorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPaperEmptySensorSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsPaperNearEndSensorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPaperNearEndSensorSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCharactersPerLine(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedCharactersPerLine)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsBarcodeSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsBarcodeSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsBitmapSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsBitmapSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsLeft90RotationSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsLeft90RotationSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsRight90RotationSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsRight90RotationSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Is180RotationSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Is180RotationSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsPrintAreaSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPrintAreaSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RuledLineCapabilities(&self) -> ::windows::core::Result<PosPrinterRuledLineCapabilities> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RuledLineCapabilities)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedBarcodeRotations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedBarcodeRotations)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedBitmapRotations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedBitmapRotations)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsFullLengthSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsFullLengthSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsBothSidesPrintingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsBothSidesPrintingSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsReverseVideoSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsReverseVideoSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsStrikethroughSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsStrikethroughSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsSuperscriptSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSuperscriptSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsSubscriptSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSubscriptSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsReversePaperFeedByLineSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsReversePaperFeedByLineSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsReversePaperFeedByMapModeUnitSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsReversePaperFeedByMapModeUnitSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for SlipPrinterCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SlipPrinterCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SlipPrinterCapabilities {}
impl ::core::fmt::Debug for SlipPrinterCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SlipPrinterCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SlipPrinterCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.SlipPrinterCapabilities;{99b16399-488c-4157-8ac2-9f57f708d3db})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SlipPrinterCapabilities {
    type Vtable = ISlipPrinterCapabilities_Vtbl;
}
unsafe impl ::windows::core::Interface for SlipPrinterCapabilities {
    const IID: ::windows::core::GUID = <ISlipPrinterCapabilities as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SlipPrinterCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.SlipPrinterCapabilities";
}
::windows::core::interface_hierarchy!(SlipPrinterCapabilities, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<SlipPrinterCapabilities> for ICommonPosPrintStationCapabilities {
    type Error = ::windows::core::Error;
    fn try_from(value: SlipPrinterCapabilities) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SlipPrinterCapabilities> for ICommonPosPrintStationCapabilities {
    type Error = ::windows::core::Error;
    fn try_from(value: &SlipPrinterCapabilities) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&SlipPrinterCapabilities> for ::windows::core::InParam<ICommonPosPrintStationCapabilities> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SlipPrinterCapabilities) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<SlipPrinterCapabilities> for ICommonReceiptSlipCapabilities {
    type Error = ::windows::core::Error;
    fn try_from(value: SlipPrinterCapabilities) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SlipPrinterCapabilities> for ICommonReceiptSlipCapabilities {
    type Error = ::windows::core::Error;
    fn try_from(value: &SlipPrinterCapabilities) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&SlipPrinterCapabilities> for ::windows::core::InParam<ICommonReceiptSlipCapabilities> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SlipPrinterCapabilities) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for SlipPrinterCapabilities {}
unsafe impl ::core::marker::Sync for SlipPrinterCapabilities {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct UnifiedPosErrorData(::windows::core::IUnknown);
impl UnifiedPosErrorData {
    pub fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Message)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Severity(&self) -> ::windows::core::Result<UnifiedPosErrorSeverity> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Severity)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Reason(&self) -> ::windows::core::Result<UnifiedPosErrorReason> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Reason)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedReason(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedReason)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateInstance(message: &::windows::core::HSTRING, severity: UnifiedPosErrorSeverity, reason: UnifiedPosErrorReason, extendedreason: u32) -> ::windows::core::Result<UnifiedPosErrorData> {
        Self::IUnifiedPosErrorDataFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(message), severity, reason, extendedreason, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUnifiedPosErrorDataFactory<R, F: FnOnce(&IUnifiedPosErrorDataFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<UnifiedPosErrorData, IUnifiedPosErrorDataFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for UnifiedPosErrorData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UnifiedPosErrorData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UnifiedPosErrorData {}
impl ::core::fmt::Debug for UnifiedPosErrorData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnifiedPosErrorData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UnifiedPosErrorData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.UnifiedPosErrorData;{2b998c3a-555c-4889-8ed8-c599bb3a712a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UnifiedPosErrorData {
    type Vtable = IUnifiedPosErrorData_Vtbl;
}
unsafe impl ::windows::core::Interface for UnifiedPosErrorData {
    const IID: ::windows::core::GUID = <IUnifiedPosErrorData as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UnifiedPosErrorData {
    const NAME: &'static str = "Windows.Devices.PointOfService.UnifiedPosErrorData";
}
::windows::core::interface_hierarchy!(UnifiedPosErrorData, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UnifiedPosErrorData {}
unsafe impl ::core::marker::Sync for UnifiedPosErrorData {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BarcodeScannerStatus(pub i32);
impl BarcodeScannerStatus {
    pub const Online: Self = Self(0i32);
    pub const Off: Self = Self(1i32);
    pub const Offline: Self = Self(2i32);
    pub const OffOrOffline: Self = Self(3i32);
    pub const Extended: Self = Self(4i32);
}
impl ::core::marker::Copy for BarcodeScannerStatus {}
impl ::core::clone::Clone for BarcodeScannerStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BarcodeScannerStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BarcodeScannerStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for BarcodeScannerStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.BarcodeScannerStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BarcodeSymbologyDecodeLengthKind(pub i32);
impl BarcodeSymbologyDecodeLengthKind {
    pub const AnyLength: Self = Self(0i32);
    pub const Discrete: Self = Self(1i32);
    pub const Range: Self = Self(2i32);
}
impl ::core::marker::Copy for BarcodeSymbologyDecodeLengthKind {}
impl ::core::clone::Clone for BarcodeSymbologyDecodeLengthKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BarcodeSymbologyDecodeLengthKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BarcodeSymbologyDecodeLengthKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for BarcodeSymbologyDecodeLengthKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeSymbologyDecodeLengthKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BarcodeSymbologyDecodeLengthKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.BarcodeSymbologyDecodeLengthKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CashDrawerStatusKind(pub i32);
impl CashDrawerStatusKind {
    pub const Online: Self = Self(0i32);
    pub const Off: Self = Self(1i32);
    pub const Offline: Self = Self(2i32);
    pub const OffOrOffline: Self = Self(3i32);
    pub const Extended: Self = Self(4i32);
}
impl ::core::marker::Copy for CashDrawerStatusKind {}
impl ::core::clone::Clone for CashDrawerStatusKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CashDrawerStatusKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CashDrawerStatusKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for CashDrawerStatusKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CashDrawerStatusKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CashDrawerStatusKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.CashDrawerStatusKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LineDisplayCursorType(pub i32);
impl LineDisplayCursorType {
    pub const None: Self = Self(0i32);
    pub const Block: Self = Self(1i32);
    pub const HalfBlock: Self = Self(2i32);
    pub const Underline: Self = Self(3i32);
    pub const Reverse: Self = Self(4i32);
    pub const Other: Self = Self(5i32);
}
impl ::core::marker::Copy for LineDisplayCursorType {}
impl ::core::clone::Clone for LineDisplayCursorType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LineDisplayCursorType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LineDisplayCursorType {
    type Abi = Self;
}
impl ::core::fmt::Debug for LineDisplayCursorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayCursorType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayCursorType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayCursorType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LineDisplayDescriptorState(pub i32);
impl LineDisplayDescriptorState {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Blink: Self = Self(2i32);
}
impl ::core::marker::Copy for LineDisplayDescriptorState {}
impl ::core::clone::Clone for LineDisplayDescriptorState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LineDisplayDescriptorState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LineDisplayDescriptorState {
    type Abi = Self;
}
impl ::core::fmt::Debug for LineDisplayDescriptorState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayDescriptorState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayDescriptorState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayDescriptorState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LineDisplayHorizontalAlignment(pub i32);
impl LineDisplayHorizontalAlignment {
    pub const Left: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
}
impl ::core::marker::Copy for LineDisplayHorizontalAlignment {}
impl ::core::clone::Clone for LineDisplayHorizontalAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LineDisplayHorizontalAlignment {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LineDisplayHorizontalAlignment {
    type Abi = Self;
}
impl ::core::fmt::Debug for LineDisplayHorizontalAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayHorizontalAlignment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayHorizontalAlignment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayHorizontalAlignment;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LineDisplayMarqueeFormat(pub i32);
impl LineDisplayMarqueeFormat {
    pub const None: Self = Self(0i32);
    pub const Walk: Self = Self(1i32);
    pub const Place: Self = Self(2i32);
}
impl ::core::marker::Copy for LineDisplayMarqueeFormat {}
impl ::core::clone::Clone for LineDisplayMarqueeFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LineDisplayMarqueeFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LineDisplayMarqueeFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for LineDisplayMarqueeFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayMarqueeFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayMarqueeFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayMarqueeFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LineDisplayPowerStatus(pub i32);
impl LineDisplayPowerStatus {
    pub const Unknown: Self = Self(0i32);
    pub const Online: Self = Self(1i32);
    pub const Off: Self = Self(2i32);
    pub const Offline: Self = Self(3i32);
    pub const OffOrOffline: Self = Self(4i32);
}
impl ::core::marker::Copy for LineDisplayPowerStatus {}
impl ::core::clone::Clone for LineDisplayPowerStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LineDisplayPowerStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LineDisplayPowerStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for LineDisplayPowerStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayPowerStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayPowerStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayPowerStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LineDisplayScrollDirection(pub i32);
impl LineDisplayScrollDirection {
    pub const Up: Self = Self(0i32);
    pub const Down: Self = Self(1i32);
    pub const Left: Self = Self(2i32);
    pub const Right: Self = Self(3i32);
}
impl ::core::marker::Copy for LineDisplayScrollDirection {}
impl ::core::clone::Clone for LineDisplayScrollDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LineDisplayScrollDirection {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LineDisplayScrollDirection {
    type Abi = Self;
}
impl ::core::fmt::Debug for LineDisplayScrollDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayScrollDirection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayScrollDirection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayScrollDirection;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LineDisplayTextAttribute(pub i32);
impl LineDisplayTextAttribute {
    pub const Normal: Self = Self(0i32);
    pub const Blink: Self = Self(1i32);
    pub const Reverse: Self = Self(2i32);
    pub const ReverseBlink: Self = Self(3i32);
}
impl ::core::marker::Copy for LineDisplayTextAttribute {}
impl ::core::clone::Clone for LineDisplayTextAttribute {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LineDisplayTextAttribute {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LineDisplayTextAttribute {
    type Abi = Self;
}
impl ::core::fmt::Debug for LineDisplayTextAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayTextAttribute").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayTextAttribute {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayTextAttribute;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LineDisplayTextAttributeGranularity(pub i32);
impl LineDisplayTextAttributeGranularity {
    pub const NotSupported: Self = Self(0i32);
    pub const EntireDisplay: Self = Self(1i32);
    pub const PerCharacter: Self = Self(2i32);
}
impl ::core::marker::Copy for LineDisplayTextAttributeGranularity {}
impl ::core::clone::Clone for LineDisplayTextAttributeGranularity {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LineDisplayTextAttributeGranularity {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LineDisplayTextAttributeGranularity {
    type Abi = Self;
}
impl ::core::fmt::Debug for LineDisplayTextAttributeGranularity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayTextAttributeGranularity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayTextAttributeGranularity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayTextAttributeGranularity;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LineDisplayVerticalAlignment(pub i32);
impl LineDisplayVerticalAlignment {
    pub const Top: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Bottom: Self = Self(2i32);
}
impl ::core::marker::Copy for LineDisplayVerticalAlignment {}
impl ::core::clone::Clone for LineDisplayVerticalAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LineDisplayVerticalAlignment {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LineDisplayVerticalAlignment {
    type Abi = Self;
}
impl ::core::fmt::Debug for LineDisplayVerticalAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayVerticalAlignment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayVerticalAlignment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayVerticalAlignment;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MagneticStripeReaderAuthenticationLevel(pub i32);
impl MagneticStripeReaderAuthenticationLevel {
    pub const NotSupported: Self = Self(0i32);
    pub const Optional: Self = Self(1i32);
    pub const Required: Self = Self(2i32);
}
impl ::core::marker::Copy for MagneticStripeReaderAuthenticationLevel {}
impl ::core::clone::Clone for MagneticStripeReaderAuthenticationLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MagneticStripeReaderAuthenticationLevel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MagneticStripeReaderAuthenticationLevel {
    type Abi = Self;
}
impl ::core::fmt::Debug for MagneticStripeReaderAuthenticationLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderAuthenticationLevel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderAuthenticationLevel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderAuthenticationLevel;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MagneticStripeReaderAuthenticationProtocol(pub i32);
impl MagneticStripeReaderAuthenticationProtocol {
    pub const None: Self = Self(0i32);
    pub const ChallengeResponse: Self = Self(1i32);
}
impl ::core::marker::Copy for MagneticStripeReaderAuthenticationProtocol {}
impl ::core::clone::Clone for MagneticStripeReaderAuthenticationProtocol {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MagneticStripeReaderAuthenticationProtocol {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MagneticStripeReaderAuthenticationProtocol {
    type Abi = Self;
}
impl ::core::fmt::Debug for MagneticStripeReaderAuthenticationProtocol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderAuthenticationProtocol").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderAuthenticationProtocol {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderAuthenticationProtocol;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MagneticStripeReaderErrorReportingType(pub i32);
impl MagneticStripeReaderErrorReportingType {
    pub const CardLevel: Self = Self(0i32);
    pub const TrackLevel: Self = Self(1i32);
}
impl ::core::marker::Copy for MagneticStripeReaderErrorReportingType {}
impl ::core::clone::Clone for MagneticStripeReaderErrorReportingType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MagneticStripeReaderErrorReportingType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MagneticStripeReaderErrorReportingType {
    type Abi = Self;
}
impl ::core::fmt::Debug for MagneticStripeReaderErrorReportingType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderErrorReportingType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderErrorReportingType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderErrorReportingType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MagneticStripeReaderStatus(pub i32);
impl MagneticStripeReaderStatus {
    pub const Unauthenticated: Self = Self(0i32);
    pub const Authenticated: Self = Self(1i32);
    pub const Extended: Self = Self(2i32);
}
impl ::core::marker::Copy for MagneticStripeReaderStatus {}
impl ::core::clone::Clone for MagneticStripeReaderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MagneticStripeReaderStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MagneticStripeReaderStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for MagneticStripeReaderStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MagneticStripeReaderTrackErrorType(pub i32);
impl MagneticStripeReaderTrackErrorType {
    pub const None: Self = Self(0i32);
    pub const StartSentinelError: Self = Self(1i32);
    pub const EndSentinelError: Self = Self(2i32);
    pub const ParityError: Self = Self(3i32);
    pub const LrcError: Self = Self(4i32);
    pub const Unknown: Self = Self(-1i32);
}
impl ::core::marker::Copy for MagneticStripeReaderTrackErrorType {}
impl ::core::clone::Clone for MagneticStripeReaderTrackErrorType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MagneticStripeReaderTrackErrorType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MagneticStripeReaderTrackErrorType {
    type Abi = Self;
}
impl ::core::fmt::Debug for MagneticStripeReaderTrackErrorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderTrackErrorType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderTrackErrorType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderTrackErrorType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MagneticStripeReaderTrackIds(pub i32);
impl MagneticStripeReaderTrackIds {
    pub const None: Self = Self(0i32);
    pub const Track1: Self = Self(1i32);
    pub const Track2: Self = Self(2i32);
    pub const Track3: Self = Self(4i32);
    pub const Track4: Self = Self(8i32);
}
impl ::core::marker::Copy for MagneticStripeReaderTrackIds {}
impl ::core::clone::Clone for MagneticStripeReaderTrackIds {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MagneticStripeReaderTrackIds {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MagneticStripeReaderTrackIds {
    type Abi = Self;
}
impl ::core::fmt::Debug for MagneticStripeReaderTrackIds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderTrackIds").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderTrackIds {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderTrackIds;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PosConnectionTypes(pub u32);
impl PosConnectionTypes {
    pub const Local: Self = Self(1u32);
    pub const IP: Self = Self(2u32);
    pub const Bluetooth: Self = Self(4u32);
    pub const All: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for PosConnectionTypes {}
impl ::core::clone::Clone for PosConnectionTypes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosConnectionTypes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PosConnectionTypes {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosConnectionTypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosConnectionTypes").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PosConnectionTypes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PosConnectionTypes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PosConnectionTypes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PosConnectionTypes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PosConnectionTypes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for PosConnectionTypes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosConnectionTypes;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PosPrinterAlignment(pub i32);
impl PosPrinterAlignment {
    pub const Left: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
}
impl ::core::marker::Copy for PosPrinterAlignment {}
impl ::core::clone::Clone for PosPrinterAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterAlignment {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterAlignment {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterAlignment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterAlignment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterAlignment;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PosPrinterBarcodeTextPosition(pub i32);
impl PosPrinterBarcodeTextPosition {
    pub const None: Self = Self(0i32);
    pub const Above: Self = Self(1i32);
    pub const Below: Self = Self(2i32);
}
impl ::core::marker::Copy for PosPrinterBarcodeTextPosition {}
impl ::core::clone::Clone for PosPrinterBarcodeTextPosition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterBarcodeTextPosition {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterBarcodeTextPosition {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterBarcodeTextPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterBarcodeTextPosition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterBarcodeTextPosition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterBarcodeTextPosition;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PosPrinterCartridgeSensors(pub u32);
impl PosPrinterCartridgeSensors {
    pub const None: Self = Self(0u32);
    pub const Removed: Self = Self(1u32);
    pub const Empty: Self = Self(2u32);
    pub const HeadCleaning: Self = Self(4u32);
    pub const NearEnd: Self = Self(8u32);
}
impl ::core::marker::Copy for PosPrinterCartridgeSensors {}
impl ::core::clone::Clone for PosPrinterCartridgeSensors {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterCartridgeSensors {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterCartridgeSensors {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterCartridgeSensors {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterCartridgeSensors").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PosPrinterCartridgeSensors {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PosPrinterCartridgeSensors {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PosPrinterCartridgeSensors {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PosPrinterCartridgeSensors {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PosPrinterCartridgeSensors {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterCartridgeSensors {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterCartridgeSensors;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PosPrinterColorCapabilities(pub u32);
impl PosPrinterColorCapabilities {
    pub const None: Self = Self(0u32);
    pub const Primary: Self = Self(1u32);
    pub const Custom1: Self = Self(2u32);
    pub const Custom2: Self = Self(4u32);
    pub const Custom3: Self = Self(8u32);
    pub const Custom4: Self = Self(16u32);
    pub const Custom5: Self = Self(32u32);
    pub const Custom6: Self = Self(64u32);
    pub const Cyan: Self = Self(128u32);
    pub const Magenta: Self = Self(256u32);
    pub const Yellow: Self = Self(512u32);
    pub const Full: Self = Self(1024u32);
}
impl ::core::marker::Copy for PosPrinterColorCapabilities {}
impl ::core::clone::Clone for PosPrinterColorCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterColorCapabilities {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterColorCapabilities {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterColorCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterColorCapabilities").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PosPrinterColorCapabilities {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PosPrinterColorCapabilities {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PosPrinterColorCapabilities {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PosPrinterColorCapabilities {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PosPrinterColorCapabilities {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterColorCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterColorCapabilities;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PosPrinterColorCartridge(pub i32);
impl PosPrinterColorCartridge {
    pub const Unknown: Self = Self(0i32);
    pub const Primary: Self = Self(1i32);
    pub const Custom1: Self = Self(2i32);
    pub const Custom2: Self = Self(3i32);
    pub const Custom3: Self = Self(4i32);
    pub const Custom4: Self = Self(5i32);
    pub const Custom5: Self = Self(6i32);
    pub const Custom6: Self = Self(7i32);
    pub const Cyan: Self = Self(8i32);
    pub const Magenta: Self = Self(9i32);
    pub const Yellow: Self = Self(10i32);
}
impl ::core::marker::Copy for PosPrinterColorCartridge {}
impl ::core::clone::Clone for PosPrinterColorCartridge {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterColorCartridge {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterColorCartridge {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterColorCartridge {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterColorCartridge").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterColorCartridge {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterColorCartridge;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PosPrinterLineDirection(pub i32);
impl PosPrinterLineDirection {
    pub const Horizontal: Self = Self(0i32);
    pub const Vertical: Self = Self(1i32);
}
impl ::core::marker::Copy for PosPrinterLineDirection {}
impl ::core::clone::Clone for PosPrinterLineDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterLineDirection {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterLineDirection {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterLineDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterLineDirection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterLineDirection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterLineDirection;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PosPrinterLineStyle(pub i32);
impl PosPrinterLineStyle {
    pub const SingleSolid: Self = Self(0i32);
    pub const DoubleSolid: Self = Self(1i32);
    pub const Broken: Self = Self(2i32);
    pub const Chain: Self = Self(3i32);
}
impl ::core::marker::Copy for PosPrinterLineStyle {}
impl ::core::clone::Clone for PosPrinterLineStyle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterLineStyle {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterLineStyle {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterLineStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterLineStyle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterLineStyle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterLineStyle;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PosPrinterMapMode(pub i32);
impl PosPrinterMapMode {
    pub const Dots: Self = Self(0i32);
    pub const Twips: Self = Self(1i32);
    pub const English: Self = Self(2i32);
    pub const Metric: Self = Self(3i32);
}
impl ::core::marker::Copy for PosPrinterMapMode {}
impl ::core::clone::Clone for PosPrinterMapMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterMapMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterMapMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterMapMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterMapMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterMapMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterMapMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PosPrinterMarkFeedCapabilities(pub u32);
impl PosPrinterMarkFeedCapabilities {
    pub const None: Self = Self(0u32);
    pub const ToTakeUp: Self = Self(1u32);
    pub const ToCutter: Self = Self(2u32);
    pub const ToCurrentTopOfForm: Self = Self(4u32);
    pub const ToNextTopOfForm: Self = Self(8u32);
}
impl ::core::marker::Copy for PosPrinterMarkFeedCapabilities {}
impl ::core::clone::Clone for PosPrinterMarkFeedCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterMarkFeedCapabilities {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterMarkFeedCapabilities {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterMarkFeedCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterMarkFeedCapabilities").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PosPrinterMarkFeedCapabilities {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PosPrinterMarkFeedCapabilities {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PosPrinterMarkFeedCapabilities {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PosPrinterMarkFeedCapabilities {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PosPrinterMarkFeedCapabilities {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterMarkFeedCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterMarkFeedCapabilities;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PosPrinterMarkFeedKind(pub i32);
impl PosPrinterMarkFeedKind {
    pub const ToTakeUp: Self = Self(0i32);
    pub const ToCutter: Self = Self(1i32);
    pub const ToCurrentTopOfForm: Self = Self(2i32);
    pub const ToNextTopOfForm: Self = Self(3i32);
}
impl ::core::marker::Copy for PosPrinterMarkFeedKind {}
impl ::core::clone::Clone for PosPrinterMarkFeedKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterMarkFeedKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterMarkFeedKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterMarkFeedKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterMarkFeedKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterMarkFeedKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterMarkFeedKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PosPrinterPrintSide(pub i32);
impl PosPrinterPrintSide {
    pub const Unknown: Self = Self(0i32);
    pub const Side1: Self = Self(1i32);
    pub const Side2: Self = Self(2i32);
}
impl ::core::marker::Copy for PosPrinterPrintSide {}
impl ::core::clone::Clone for PosPrinterPrintSide {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterPrintSide {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterPrintSide {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterPrintSide {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterPrintSide").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterPrintSide {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterPrintSide;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PosPrinterRotation(pub i32);
impl PosPrinterRotation {
    pub const Normal: Self = Self(0i32);
    pub const Right90: Self = Self(1i32);
    pub const Left90: Self = Self(2i32);
    pub const Rotate180: Self = Self(3i32);
}
impl ::core::marker::Copy for PosPrinterRotation {}
impl ::core::clone::Clone for PosPrinterRotation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterRotation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterRotation {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterRotation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterRotation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterRotation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterRotation;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PosPrinterRuledLineCapabilities(pub u32);
impl PosPrinterRuledLineCapabilities {
    pub const None: Self = Self(0u32);
    pub const Horizontal: Self = Self(1u32);
    pub const Vertical: Self = Self(2u32);
}
impl ::core::marker::Copy for PosPrinterRuledLineCapabilities {}
impl ::core::clone::Clone for PosPrinterRuledLineCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterRuledLineCapabilities {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterRuledLineCapabilities {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterRuledLineCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterRuledLineCapabilities").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PosPrinterRuledLineCapabilities {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PosPrinterRuledLineCapabilities {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PosPrinterRuledLineCapabilities {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PosPrinterRuledLineCapabilities {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PosPrinterRuledLineCapabilities {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterRuledLineCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterRuledLineCapabilities;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PosPrinterStatusKind(pub i32);
impl PosPrinterStatusKind {
    pub const Online: Self = Self(0i32);
    pub const Off: Self = Self(1i32);
    pub const Offline: Self = Self(2i32);
    pub const OffOrOffline: Self = Self(3i32);
    pub const Extended: Self = Self(4i32);
}
impl ::core::marker::Copy for PosPrinterStatusKind {}
impl ::core::clone::Clone for PosPrinterStatusKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterStatusKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterStatusKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterStatusKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterStatusKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterStatusKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterStatusKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UnifiedPosErrorReason(pub i32);
impl UnifiedPosErrorReason {
    pub const UnknownErrorReason: Self = Self(0i32);
    pub const NoService: Self = Self(1i32);
    pub const Disabled: Self = Self(2i32);
    pub const Illegal: Self = Self(3i32);
    pub const NoHardware: Self = Self(4i32);
    pub const Closed: Self = Self(5i32);
    pub const Offline: Self = Self(6i32);
    pub const Failure: Self = Self(7i32);
    pub const Timeout: Self = Self(8i32);
    pub const Busy: Self = Self(9i32);
    pub const Extended: Self = Self(10i32);
}
impl ::core::marker::Copy for UnifiedPosErrorReason {}
impl ::core::clone::Clone for UnifiedPosErrorReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UnifiedPosErrorReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UnifiedPosErrorReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for UnifiedPosErrorReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnifiedPosErrorReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UnifiedPosErrorReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.UnifiedPosErrorReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UnifiedPosErrorSeverity(pub i32);
impl UnifiedPosErrorSeverity {
    pub const UnknownErrorSeverity: Self = Self(0i32);
    pub const Warning: Self = Self(1i32);
    pub const Recoverable: Self = Self(2i32);
    pub const Unrecoverable: Self = Self(3i32);
    pub const AssistanceRequired: Self = Self(4i32);
    pub const Fatal: Self = Self(5i32);
}
impl ::core::marker::Copy for UnifiedPosErrorSeverity {}
impl ::core::clone::Clone for UnifiedPosErrorSeverity {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UnifiedPosErrorSeverity {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UnifiedPosErrorSeverity {
    type Abi = Self;
}
impl ::core::fmt::Debug for UnifiedPosErrorSeverity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnifiedPosErrorSeverity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UnifiedPosErrorSeverity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.UnifiedPosErrorSeverity;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UnifiedPosHealthCheckLevel(pub i32);
impl UnifiedPosHealthCheckLevel {
    pub const UnknownHealthCheckLevel: Self = Self(0i32);
    pub const POSInternal: Self = Self(1i32);
    pub const External: Self = Self(2i32);
    pub const Interactive: Self = Self(3i32);
}
impl ::core::marker::Copy for UnifiedPosHealthCheckLevel {}
impl ::core::clone::Clone for UnifiedPosHealthCheckLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UnifiedPosHealthCheckLevel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UnifiedPosHealthCheckLevel {
    type Abi = Self;
}
impl ::core::fmt::Debug for UnifiedPosHealthCheckLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnifiedPosHealthCheckLevel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UnifiedPosHealthCheckLevel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.UnifiedPosHealthCheckLevel;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UnifiedPosPowerReportingType(pub i32);
impl UnifiedPosPowerReportingType {
    pub const UnknownPowerReportingType: Self = Self(0i32);
    pub const Standard: Self = Self(1i32);
    pub const Advanced: Self = Self(2i32);
}
impl ::core::marker::Copy for UnifiedPosPowerReportingType {}
impl ::core::clone::Clone for UnifiedPosPowerReportingType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UnifiedPosPowerReportingType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UnifiedPosPowerReportingType {
    type Abi = Self;
}
impl ::core::fmt::Debug for UnifiedPosPowerReportingType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnifiedPosPowerReportingType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UnifiedPosPowerReportingType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.UnifiedPosPowerReportingType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
pub struct SizeUInt32 {
    pub Width: u32,
    pub Height: u32,
}
impl ::core::marker::Copy for SizeUInt32 {}
impl ::core::clone::Clone for SizeUInt32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SizeUInt32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SizeUInt32").field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
unsafe impl ::windows::core::Abi for SizeUInt32 {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SizeUInt32 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Devices.PointOfService.SizeUInt32;u4;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for SizeUInt32 {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height
    }
}
impl ::core::cmp::Eq for SizeUInt32 {}
impl ::core::default::Default for SizeUInt32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
