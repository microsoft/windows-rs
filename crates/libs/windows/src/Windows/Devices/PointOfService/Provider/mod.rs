#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBarcodeScannerDisableScannerRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerDisableScannerRequest {
    type Vtable = IBarcodeScannerDisableScannerRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBarcodeScannerDisableScannerRequest {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x88ecf7c0_37b9_4275_8e77_c8e52ae5a9c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerDisableScannerRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBarcodeScannerDisableScannerRequest2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerDisableScannerRequest2 {
    type Vtable = IBarcodeScannerDisableScannerRequest2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBarcodeScannerDisableScannerRequest2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xccdfe625_65c3_4ccc_b457_f39c7a9ea60d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerDisableScannerRequest2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ReportFailedWithFailedReasonAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportFailedWithFailedReasonAndDescriptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, failedreasondescription: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBarcodeScannerDisableScannerRequestEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerDisableScannerRequestEventArgs {
    type Vtable = IBarcodeScannerDisableScannerRequestEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBarcodeScannerDisableScannerRequestEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7006e142_e802_46f5_b604_352a15ce9232);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerDisableScannerRequestEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBarcodeScannerEnableScannerRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerEnableScannerRequest {
    type Vtable = IBarcodeScannerEnableScannerRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBarcodeScannerEnableScannerRequest {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc0b3e9ba_816a_452b_bd77_b7e453ec446d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerEnableScannerRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBarcodeScannerEnableScannerRequest2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerEnableScannerRequest2 {
    type Vtable = IBarcodeScannerEnableScannerRequest2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBarcodeScannerEnableScannerRequest2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x71a4f2a8_9905_41ac_9121_b645916a84a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerEnableScannerRequest2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ReportFailedWithFailedReasonAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportFailedWithFailedReasonAndDescriptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, failedreasondescription: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBarcodeScannerEnableScannerRequestEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerEnableScannerRequestEventArgs {
    type Vtable = IBarcodeScannerEnableScannerRequestEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBarcodeScannerEnableScannerRequestEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x956c9419_7b4e_4451_8c41_8e10cfbc5b41);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerEnableScannerRequestEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBarcodeScannerFrameReader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerFrameReader {
    type Vtable = IBarcodeScannerFrameReader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBarcodeScannerFrameReader {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdbc72b07_64c3_482b_93c8_65fb33c22208);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerFrameReader_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub StopAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TryAcquireLatestFrameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Connection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveFrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBarcodeScannerFrameReaderFrameArrivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerFrameReaderFrameArrivedEventArgs {
    type Vtable = IBarcodeScannerFrameReaderFrameArrivedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBarcodeScannerFrameReaderFrameArrivedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb0bbd604_54fd_436d_8629_712e787223dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerFrameReaderFrameArrivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBarcodeScannerGetSymbologyAttributesRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerGetSymbologyAttributesRequest {
    type Vtable = IBarcodeScannerGetSymbologyAttributesRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBarcodeScannerGetSymbologyAttributesRequest {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9774c46a_58e4_4c5f_b8e9_e41467632700);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerGetSymbologyAttributesRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Symbology: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributes: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBarcodeScannerGetSymbologyAttributesRequest2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerGetSymbologyAttributesRequest2 {
    type Vtable = IBarcodeScannerGetSymbologyAttributesRequest2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBarcodeScannerGetSymbologyAttributesRequest2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6a6a2b13_75a8_49fb_b852_bfb93d760af7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerGetSymbologyAttributesRequest2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ReportFailedWithFailedReasonAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportFailedWithFailedReasonAndDescriptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, failedreasondescription: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBarcodeScannerGetSymbologyAttributesRequestEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerGetSymbologyAttributesRequestEventArgs {
    type Vtable = IBarcodeScannerGetSymbologyAttributesRequestEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBarcodeScannerGetSymbologyAttributesRequestEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7f89de3e_fb5d_493c_b402_356b24d574a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerGetSymbologyAttributesRequestEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBarcodeScannerHideVideoPreviewRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerHideVideoPreviewRequest {
    type Vtable = IBarcodeScannerHideVideoPreviewRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBarcodeScannerHideVideoPreviewRequest {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfa4ebe7f_6670_40e1_b90b_bb10d8d425fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerHideVideoPreviewRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBarcodeScannerHideVideoPreviewRequest2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerHideVideoPreviewRequest2 {
    type Vtable = IBarcodeScannerHideVideoPreviewRequest2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBarcodeScannerHideVideoPreviewRequest2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7e28435d_9814_431d_a2f2_d6248c5ad4b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerHideVideoPreviewRequest2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ReportFailedWithFailedReasonAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportFailedWithFailedReasonAndDescriptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, failedreasondescription: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBarcodeScannerHideVideoPreviewRequestEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerHideVideoPreviewRequestEventArgs {
    type Vtable = IBarcodeScannerHideVideoPreviewRequestEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBarcodeScannerHideVideoPreviewRequestEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x16a281fc_d6be_4bc7_9df1_33741f3eadea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerHideVideoPreviewRequestEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBarcodeScannerProviderConnection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerProviderConnection {
    type Vtable = IBarcodeScannerProviderConnection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBarcodeScannerProviderConnection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb44acbed_0b3a_4fa3_86c5_491ea30780eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerProviderConnection_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedSymbologies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedSymbologies: usize,
    pub CompanyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCompanyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportScannedDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, report: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportTriggerStateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: BarcodeScannerTriggerState, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportErrorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errordata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportErrorAsyncWithScanReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errordata: *mut ::core::ffi::c_void, isretriable: bool, scanreport: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EnableScannerRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveEnableScannerRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub DisableScannerRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDisableScannerRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub SetActiveSymbologiesRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSetActiveSymbologiesRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub StartSoftwareTriggerRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveStartSoftwareTriggerRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub StopSoftwareTriggerRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveStopSoftwareTriggerRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub GetBarcodeSymbologyAttributesRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveGetBarcodeSymbologyAttributesRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub SetBarcodeSymbologyAttributesRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSetBarcodeSymbologyAttributesRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub HideVideoPreviewRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveHideVideoPreviewRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBarcodeScannerProviderConnection2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerProviderConnection2 {
    type Vtable = IBarcodeScannerProviderConnection2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBarcodeScannerProviderConnection2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbe9b53cd_1134_418c_a06b_04423a73f3d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerProviderConnection2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateFrameReaderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")]
    pub CreateFrameReaderWithFormatAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preferredformat: super::super::super::Graphics::Imaging::BitmapPixelFormat, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    CreateFrameReaderWithFormatAsync: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub CreateFrameReaderWithFormatAndSizeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preferredformat: super::super::super::Graphics::Imaging::BitmapPixelFormat, preferredsize: super::super::super::Graphics::Imaging::BitmapSize, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    CreateFrameReaderWithFormatAndSizeAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBarcodeScannerProviderTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerProviderTriggerDetails {
    type Vtable = IBarcodeScannerProviderTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBarcodeScannerProviderTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x50856d82_24e3_48ce_99c7_70aac1cbc9f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerProviderTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Connection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBarcodeScannerSetActiveSymbologiesRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerSetActiveSymbologiesRequest {
    type Vtable = IBarcodeScannerSetActiveSymbologiesRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBarcodeScannerSetActiveSymbologiesRequest {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdb3f32b9_f7da_41a1_9f79_07bcd95f0bdf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerSetActiveSymbologiesRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Symbologies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Symbologies: usize,
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBarcodeScannerSetActiveSymbologiesRequest2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerSetActiveSymbologiesRequest2 {
    type Vtable = IBarcodeScannerSetActiveSymbologiesRequest2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBarcodeScannerSetActiveSymbologiesRequest2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf5ff6edf_fa9a_4749_b11b_e8fccb75bc6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerSetActiveSymbologiesRequest2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ReportFailedWithFailedReasonAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportFailedWithFailedReasonAndDescriptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, failedreasondescription: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBarcodeScannerSetActiveSymbologiesRequestEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerSetActiveSymbologiesRequestEventArgs {
    type Vtable = IBarcodeScannerSetActiveSymbologiesRequestEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBarcodeScannerSetActiveSymbologiesRequestEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x06305afa_7bf6_4d52_801a_330272f60ae1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerSetActiveSymbologiesRequestEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBarcodeScannerSetSymbologyAttributesRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerSetSymbologyAttributesRequest {
    type Vtable = IBarcodeScannerSetSymbologyAttributesRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBarcodeScannerSetSymbologyAttributesRequest {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x32fb814f_a37f_48b0_acea_dce1480f12ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerSetSymbologyAttributesRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Symbology: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Attributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBarcodeScannerSetSymbologyAttributesRequest2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerSetSymbologyAttributesRequest2 {
    type Vtable = IBarcodeScannerSetSymbologyAttributesRequest2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBarcodeScannerSetSymbologyAttributesRequest2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdffbbfc1_dba8_4b77_be1e_b56cd72f65b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerSetSymbologyAttributesRequest2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ReportFailedWithFailedReasonAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportFailedWithFailedReasonAndDescriptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, failedreasondescription: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBarcodeScannerSetSymbologyAttributesRequestEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerSetSymbologyAttributesRequestEventArgs {
    type Vtable = IBarcodeScannerSetSymbologyAttributesRequestEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBarcodeScannerSetSymbologyAttributesRequestEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb2b89809_9824_47d4_85bd_d0077baa7bd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerSetSymbologyAttributesRequestEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBarcodeScannerStartSoftwareTriggerRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerStartSoftwareTriggerRequest {
    type Vtable = IBarcodeScannerStartSoftwareTriggerRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBarcodeScannerStartSoftwareTriggerRequest {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe3fa7b27_ff62_4454_af4a_cb6144a3e3f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStartSoftwareTriggerRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBarcodeScannerStartSoftwareTriggerRequest2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerStartSoftwareTriggerRequest2 {
    type Vtable = IBarcodeScannerStartSoftwareTriggerRequest2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBarcodeScannerStartSoftwareTriggerRequest2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeb72a25c_6658_4765_a68e_327482653deb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStartSoftwareTriggerRequest2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ReportFailedWithFailedReasonAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportFailedWithFailedReasonAndDescriptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, failedreasondescription: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBarcodeScannerStartSoftwareTriggerRequestEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerStartSoftwareTriggerRequestEventArgs {
    type Vtable = IBarcodeScannerStartSoftwareTriggerRequestEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBarcodeScannerStartSoftwareTriggerRequestEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2305d843_c88f_4f3b_8c3b_d3df071051ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStartSoftwareTriggerRequestEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBarcodeScannerStopSoftwareTriggerRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerStopSoftwareTriggerRequest {
    type Vtable = IBarcodeScannerStopSoftwareTriggerRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBarcodeScannerStopSoftwareTriggerRequest {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f9faf35_e287_4ca8_b70d_5a91d694f668);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStopSoftwareTriggerRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBarcodeScannerStopSoftwareTriggerRequest2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerStopSoftwareTriggerRequest2 {
    type Vtable = IBarcodeScannerStopSoftwareTriggerRequest2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBarcodeScannerStopSoftwareTriggerRequest2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcb57c5dd_fe50_49f8_a0b4_bdc230814da2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStopSoftwareTriggerRequest2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ReportFailedWithFailedReasonAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportFailedWithFailedReasonAndDescriptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, failedreasondescription: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBarcodeScannerStopSoftwareTriggerRequestEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerStopSoftwareTriggerRequestEventArgs {
    type Vtable = IBarcodeScannerStopSoftwareTriggerRequestEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBarcodeScannerStopSoftwareTriggerRequestEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeac34450_4eb7_481a_9273_147a273b99b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStopSoftwareTriggerRequestEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBarcodeScannerVideoFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerVideoFrame {
    type Vtable = IBarcodeScannerVideoFrame_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBarcodeScannerVideoFrame {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7e585248_9df7_4121_a175_801d8000112e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerVideoFrame_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    Format: usize,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub PixelData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PixelData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBarcodeSymbologyAttributesBuilder(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeSymbologyAttributesBuilder {
    type Vtable = IBarcodeSymbologyAttributesBuilder_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBarcodeSymbologyAttributesBuilder {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc57b0cbf_e4f5_40b9_84cf_e63fbaea42b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeSymbologyAttributesBuilder_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsCheckDigitValidationSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsCheckDigitValidationSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsCheckDigitTransmissionSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsCheckDigitTransmissionSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsDecodeLengthSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsDecodeLengthSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CreateAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BarcodeScannerDisableScannerRequest(::windows_core::IUnknown);
impl BarcodeScannerDisableScannerRequest {
    pub fn ReportCompletedAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportCompletedAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportFailedAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IBarcodeScannerDisableScannerRequest2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAsync)(::windows_core::Interface::as_raw(this), reason, &mut result__).from_abi(result__)
        }
    }
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync(&self, reason: i32, failedreasondescription: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IBarcodeScannerDisableScannerRequest2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAndDescriptionAsync)(::windows_core::Interface::as_raw(this), reason, ::core::mem::transmute_copy(failedreasondescription), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for BarcodeScannerDisableScannerRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for BarcodeScannerDisableScannerRequest {
    type Vtable = IBarcodeScannerDisableScannerRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BarcodeScannerDisableScannerRequest {
    const IID: ::windows_core::GUID = <IBarcodeScannerDisableScannerRequest as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerDisableScannerRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerDisableScannerRequest";
}
::windows_core::imp::interface_hierarchy!(BarcodeScannerDisableScannerRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for BarcodeScannerDisableScannerRequest {}
unsafe impl ::core::marker::Sync for BarcodeScannerDisableScannerRequest {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BarcodeScannerDisableScannerRequestEventArgs(::windows_core::IUnknown);
impl BarcodeScannerDisableScannerRequestEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<BarcodeScannerDisableScannerRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for BarcodeScannerDisableScannerRequestEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for BarcodeScannerDisableScannerRequestEventArgs {
    type Vtable = IBarcodeScannerDisableScannerRequestEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BarcodeScannerDisableScannerRequestEventArgs {
    const IID: ::windows_core::GUID = <IBarcodeScannerDisableScannerRequestEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerDisableScannerRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerDisableScannerRequestEventArgs";
}
::windows_core::imp::interface_hierarchy!(BarcodeScannerDisableScannerRequestEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for BarcodeScannerDisableScannerRequestEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerDisableScannerRequestEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BarcodeScannerEnableScannerRequest(::windows_core::IUnknown);
impl BarcodeScannerEnableScannerRequest {
    pub fn ReportCompletedAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportCompletedAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportFailedAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IBarcodeScannerEnableScannerRequest2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAsync)(::windows_core::Interface::as_raw(this), reason, &mut result__).from_abi(result__)
        }
    }
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync(&self, reason: i32, failedreasondescription: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IBarcodeScannerEnableScannerRequest2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAndDescriptionAsync)(::windows_core::Interface::as_raw(this), reason, ::core::mem::transmute_copy(failedreasondescription), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for BarcodeScannerEnableScannerRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for BarcodeScannerEnableScannerRequest {
    type Vtable = IBarcodeScannerEnableScannerRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BarcodeScannerEnableScannerRequest {
    const IID: ::windows_core::GUID = <IBarcodeScannerEnableScannerRequest as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerEnableScannerRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerEnableScannerRequest";
}
::windows_core::imp::interface_hierarchy!(BarcodeScannerEnableScannerRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for BarcodeScannerEnableScannerRequest {}
unsafe impl ::core::marker::Sync for BarcodeScannerEnableScannerRequest {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BarcodeScannerEnableScannerRequestEventArgs(::windows_core::IUnknown);
impl BarcodeScannerEnableScannerRequestEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<BarcodeScannerEnableScannerRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for BarcodeScannerEnableScannerRequestEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for BarcodeScannerEnableScannerRequestEventArgs {
    type Vtable = IBarcodeScannerEnableScannerRequestEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BarcodeScannerEnableScannerRequestEventArgs {
    const IID: ::windows_core::GUID = <IBarcodeScannerEnableScannerRequestEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerEnableScannerRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerEnableScannerRequestEventArgs";
}
::windows_core::imp::interface_hierarchy!(BarcodeScannerEnableScannerRequestEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for BarcodeScannerEnableScannerRequestEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerEnableScannerRequestEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BarcodeScannerFrameReader(::windows_core::IUnknown);
impl BarcodeScannerFrameReader {
    pub fn StartAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StopAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StopAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryAcquireLatestFrameAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<BarcodeScannerVideoFrame>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryAcquireLatestFrameAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Connection(&self) -> ::windows_core::Result<BarcodeScannerProviderConnection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Connection)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FrameArrived<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<BarcodeScannerFrameReader, BarcodeScannerFrameReaderFrameArrivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameArrived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveFrameArrived(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFrameArrived)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::windows_core::RuntimeType for BarcodeScannerFrameReader {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for BarcodeScannerFrameReader {
    type Vtable = IBarcodeScannerFrameReader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BarcodeScannerFrameReader {
    const IID: ::windows_core::GUID = <IBarcodeScannerFrameReader as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerFrameReader {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerFrameReader";
}
::windows_core::imp::interface_hierarchy!(BarcodeScannerFrameReader, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for BarcodeScannerFrameReader {}
unsafe impl ::core::marker::Send for BarcodeScannerFrameReader {}
unsafe impl ::core::marker::Sync for BarcodeScannerFrameReader {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BarcodeScannerFrameReaderFrameArrivedEventArgs(::windows_core::IUnknown);
impl BarcodeScannerFrameReaderFrameArrivedEventArgs {
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for BarcodeScannerFrameReaderFrameArrivedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for BarcodeScannerFrameReaderFrameArrivedEventArgs {
    type Vtable = IBarcodeScannerFrameReaderFrameArrivedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BarcodeScannerFrameReaderFrameArrivedEventArgs {
    const IID: ::windows_core::GUID = <IBarcodeScannerFrameReaderFrameArrivedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerFrameReaderFrameArrivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerFrameReaderFrameArrivedEventArgs";
}
::windows_core::imp::interface_hierarchy!(BarcodeScannerFrameReaderFrameArrivedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for BarcodeScannerFrameReaderFrameArrivedEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerFrameReaderFrameArrivedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BarcodeScannerGetSymbologyAttributesRequest(::windows_core::IUnknown);
impl BarcodeScannerGetSymbologyAttributesRequest {
    pub fn Symbology(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Symbology)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportCompletedAsync<P0>(&self, attributes: P0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::BarcodeSymbologyAttributes>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportCompletedAsync)(::windows_core::Interface::as_raw(this), attributes.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportFailedAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IBarcodeScannerGetSymbologyAttributesRequest2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAsync)(::windows_core::Interface::as_raw(this), reason, &mut result__).from_abi(result__)
        }
    }
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync(&self, reason: i32, failedreasondescription: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IBarcodeScannerGetSymbologyAttributesRequest2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAndDescriptionAsync)(::windows_core::Interface::as_raw(this), reason, ::core::mem::transmute_copy(failedreasondescription), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for BarcodeScannerGetSymbologyAttributesRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for BarcodeScannerGetSymbologyAttributesRequest {
    type Vtable = IBarcodeScannerGetSymbologyAttributesRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BarcodeScannerGetSymbologyAttributesRequest {
    const IID: ::windows_core::GUID = <IBarcodeScannerGetSymbologyAttributesRequest as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerGetSymbologyAttributesRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerGetSymbologyAttributesRequest";
}
::windows_core::imp::interface_hierarchy!(BarcodeScannerGetSymbologyAttributesRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for BarcodeScannerGetSymbologyAttributesRequest {}
unsafe impl ::core::marker::Sync for BarcodeScannerGetSymbologyAttributesRequest {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BarcodeScannerGetSymbologyAttributesRequestEventArgs(::windows_core::IUnknown);
impl BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<BarcodeScannerGetSymbologyAttributesRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    type Vtable = IBarcodeScannerGetSymbologyAttributesRequestEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    const IID: ::windows_core::GUID = <IBarcodeScannerGetSymbologyAttributesRequestEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerGetSymbologyAttributesRequestEventArgs";
}
::windows_core::imp::interface_hierarchy!(BarcodeScannerGetSymbologyAttributesRequestEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for BarcodeScannerGetSymbologyAttributesRequestEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerGetSymbologyAttributesRequestEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BarcodeScannerHideVideoPreviewRequest(::windows_core::IUnknown);
impl BarcodeScannerHideVideoPreviewRequest {
    pub fn ReportCompletedAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportCompletedAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportFailedAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IBarcodeScannerHideVideoPreviewRequest2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAsync)(::windows_core::Interface::as_raw(this), reason, &mut result__).from_abi(result__)
        }
    }
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync(&self, reason: i32, failedreasondescription: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IBarcodeScannerHideVideoPreviewRequest2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAndDescriptionAsync)(::windows_core::Interface::as_raw(this), reason, ::core::mem::transmute_copy(failedreasondescription), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for BarcodeScannerHideVideoPreviewRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for BarcodeScannerHideVideoPreviewRequest {
    type Vtable = IBarcodeScannerHideVideoPreviewRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BarcodeScannerHideVideoPreviewRequest {
    const IID: ::windows_core::GUID = <IBarcodeScannerHideVideoPreviewRequest as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerHideVideoPreviewRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerHideVideoPreviewRequest";
}
::windows_core::imp::interface_hierarchy!(BarcodeScannerHideVideoPreviewRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for BarcodeScannerHideVideoPreviewRequest {}
unsafe impl ::core::marker::Sync for BarcodeScannerHideVideoPreviewRequest {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BarcodeScannerHideVideoPreviewRequestEventArgs(::windows_core::IUnknown);
impl BarcodeScannerHideVideoPreviewRequestEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<BarcodeScannerHideVideoPreviewRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for BarcodeScannerHideVideoPreviewRequestEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for BarcodeScannerHideVideoPreviewRequestEventArgs {
    type Vtable = IBarcodeScannerHideVideoPreviewRequestEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BarcodeScannerHideVideoPreviewRequestEventArgs {
    const IID: ::windows_core::GUID = <IBarcodeScannerHideVideoPreviewRequestEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerHideVideoPreviewRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerHideVideoPreviewRequestEventArgs";
}
::windows_core::imp::interface_hierarchy!(BarcodeScannerHideVideoPreviewRequestEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for BarcodeScannerHideVideoPreviewRequestEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerHideVideoPreviewRequestEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BarcodeScannerProviderConnection(::windows_core::IUnknown);
impl BarcodeScannerProviderConnection {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn VideoDeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoDeviceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedSymbologies(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVector<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedSymbologies)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CompanyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CompanyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCompanyName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCompanyName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Version(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Version)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetVersion(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVersion)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ReportScannedDataAsync<P0>(&self, report: P0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::BarcodeScannerReport>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportScannedDataAsync)(::windows_core::Interface::as_raw(this), report.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportTriggerStateAsync(&self, state: BarcodeScannerTriggerState) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportTriggerStateAsync)(::windows_core::Interface::as_raw(this), state, &mut result__).from_abi(result__)
        }
    }
    pub fn ReportErrorAsync<P0>(&self, errordata: P0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::UnifiedPosErrorData>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportErrorAsync)(::windows_core::Interface::as_raw(this), errordata.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportErrorAsyncWithScanReport<P0, P1>(&self, errordata: P0, isretriable: bool, scanreport: P1) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::UnifiedPosErrorData>,
        P1: ::windows_core::IntoParam<super::BarcodeScannerReport>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportErrorAsyncWithScanReport)(::windows_core::Interface::as_raw(this), errordata.into_param().abi(), isretriable, scanreport.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn EnableScannerRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerEnableScannerRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnableScannerRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveEnableScannerRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveEnableScannerRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn DisableScannerRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerDisableScannerRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisableScannerRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveDisableScannerRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDisableScannerRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SetActiveSymbologiesRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerSetActiveSymbologiesRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetActiveSymbologiesRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveSetActiveSymbologiesRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSetActiveSymbologiesRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn StartSoftwareTriggerRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerStartSoftwareTriggerRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartSoftwareTriggerRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveStartSoftwareTriggerRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStartSoftwareTriggerRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn StopSoftwareTriggerRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerStopSoftwareTriggerRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StopSoftwareTriggerRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveStopSoftwareTriggerRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStopSoftwareTriggerRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetBarcodeSymbologyAttributesRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerGetSymbologyAttributesRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetBarcodeSymbologyAttributesRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveGetBarcodeSymbologyAttributesRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveGetBarcodeSymbologyAttributesRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SetBarcodeSymbologyAttributesRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerSetSymbologyAttributesRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetBarcodeSymbologyAttributesRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveSetBarcodeSymbologyAttributesRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSetBarcodeSymbologyAttributesRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn HideVideoPreviewRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerHideVideoPreviewRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HideVideoPreviewRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveHideVideoPreviewRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveHideVideoPreviewRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CreateFrameReaderAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<BarcodeScannerFrameReader>> {
        let this = &::windows_core::ComInterface::cast::<IBarcodeScannerProviderConnection2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFrameReaderAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Graphics_Imaging\"`"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn CreateFrameReaderWithFormatAsync(&self, preferredformat: super::super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<BarcodeScannerFrameReader>> {
        let this = &::windows_core::ComInterface::cast::<IBarcodeScannerProviderConnection2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFrameReaderWithFormatAsync)(::windows_core::Interface::as_raw(this), preferredformat, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Graphics_Imaging\"`"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn CreateFrameReaderWithFormatAndSizeAsync(&self, preferredformat: super::super::super::Graphics::Imaging::BitmapPixelFormat, preferredsize: super::super::super::Graphics::Imaging::BitmapSize) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<BarcodeScannerFrameReader>> {
        let this = &::windows_core::ComInterface::cast::<IBarcodeScannerProviderConnection2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFrameReaderWithFormatAndSizeAsync)(::windows_core::Interface::as_raw(this), preferredformat, preferredsize, &mut result__).from_abi(result__)
        }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::windows_core::RuntimeType for BarcodeScannerProviderConnection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for BarcodeScannerProviderConnection {
    type Vtable = IBarcodeScannerProviderConnection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BarcodeScannerProviderConnection {
    const IID: ::windows_core::GUID = <IBarcodeScannerProviderConnection as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerProviderConnection {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerProviderConnection";
}
::windows_core::imp::interface_hierarchy!(BarcodeScannerProviderConnection, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for BarcodeScannerProviderConnection {}
unsafe impl ::core::marker::Send for BarcodeScannerProviderConnection {}
unsafe impl ::core::marker::Sync for BarcodeScannerProviderConnection {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BarcodeScannerProviderTriggerDetails(::windows_core::IUnknown);
impl BarcodeScannerProviderTriggerDetails {
    pub fn Connection(&self) -> ::windows_core::Result<BarcodeScannerProviderConnection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Connection)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for BarcodeScannerProviderTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for BarcodeScannerProviderTriggerDetails {
    type Vtable = IBarcodeScannerProviderTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BarcodeScannerProviderTriggerDetails {
    const IID: ::windows_core::GUID = <IBarcodeScannerProviderTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerProviderTriggerDetails {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerProviderTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(BarcodeScannerProviderTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for BarcodeScannerProviderTriggerDetails {}
unsafe impl ::core::marker::Sync for BarcodeScannerProviderTriggerDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BarcodeScannerSetActiveSymbologiesRequest(::windows_core::IUnknown);
impl BarcodeScannerSetActiveSymbologiesRequest {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Symbologies(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Symbologies)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportCompletedAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportCompletedAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportFailedAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IBarcodeScannerSetActiveSymbologiesRequest2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAsync)(::windows_core::Interface::as_raw(this), reason, &mut result__).from_abi(result__)
        }
    }
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync(&self, reason: i32, failedreasondescription: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IBarcodeScannerSetActiveSymbologiesRequest2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAndDescriptionAsync)(::windows_core::Interface::as_raw(this), reason, ::core::mem::transmute_copy(failedreasondescription), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for BarcodeScannerSetActiveSymbologiesRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for BarcodeScannerSetActiveSymbologiesRequest {
    type Vtable = IBarcodeScannerSetActiveSymbologiesRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BarcodeScannerSetActiveSymbologiesRequest {
    const IID: ::windows_core::GUID = <IBarcodeScannerSetActiveSymbologiesRequest as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerSetActiveSymbologiesRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerSetActiveSymbologiesRequest";
}
::windows_core::imp::interface_hierarchy!(BarcodeScannerSetActiveSymbologiesRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for BarcodeScannerSetActiveSymbologiesRequest {}
unsafe impl ::core::marker::Sync for BarcodeScannerSetActiveSymbologiesRequest {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BarcodeScannerSetActiveSymbologiesRequestEventArgs(::windows_core::IUnknown);
impl BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<BarcodeScannerSetActiveSymbologiesRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    type Vtable = IBarcodeScannerSetActiveSymbologiesRequestEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    const IID: ::windows_core::GUID = <IBarcodeScannerSetActiveSymbologiesRequestEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerSetActiveSymbologiesRequestEventArgs";
}
::windows_core::imp::interface_hierarchy!(BarcodeScannerSetActiveSymbologiesRequestEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for BarcodeScannerSetActiveSymbologiesRequestEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerSetActiveSymbologiesRequestEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BarcodeScannerSetSymbologyAttributesRequest(::windows_core::IUnknown);
impl BarcodeScannerSetSymbologyAttributesRequest {
    pub fn Symbology(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Symbology)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows_core::Result<super::BarcodeSymbologyAttributes> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportCompletedAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportCompletedAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportFailedAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IBarcodeScannerSetSymbologyAttributesRequest2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAsync)(::windows_core::Interface::as_raw(this), reason, &mut result__).from_abi(result__)
        }
    }
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync(&self, reason: i32, failedreasondescription: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IBarcodeScannerSetSymbologyAttributesRequest2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAndDescriptionAsync)(::windows_core::Interface::as_raw(this), reason, ::core::mem::transmute_copy(failedreasondescription), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for BarcodeScannerSetSymbologyAttributesRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for BarcodeScannerSetSymbologyAttributesRequest {
    type Vtable = IBarcodeScannerSetSymbologyAttributesRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BarcodeScannerSetSymbologyAttributesRequest {
    const IID: ::windows_core::GUID = <IBarcodeScannerSetSymbologyAttributesRequest as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerSetSymbologyAttributesRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerSetSymbologyAttributesRequest";
}
::windows_core::imp::interface_hierarchy!(BarcodeScannerSetSymbologyAttributesRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for BarcodeScannerSetSymbologyAttributesRequest {}
unsafe impl ::core::marker::Sync for BarcodeScannerSetSymbologyAttributesRequest {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BarcodeScannerSetSymbologyAttributesRequestEventArgs(::windows_core::IUnknown);
impl BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<BarcodeScannerSetSymbologyAttributesRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    type Vtable = IBarcodeScannerSetSymbologyAttributesRequestEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    const IID: ::windows_core::GUID = <IBarcodeScannerSetSymbologyAttributesRequestEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerSetSymbologyAttributesRequestEventArgs";
}
::windows_core::imp::interface_hierarchy!(BarcodeScannerSetSymbologyAttributesRequestEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for BarcodeScannerSetSymbologyAttributesRequestEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerSetSymbologyAttributesRequestEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BarcodeScannerStartSoftwareTriggerRequest(::windows_core::IUnknown);
impl BarcodeScannerStartSoftwareTriggerRequest {
    pub fn ReportCompletedAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportCompletedAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportFailedAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IBarcodeScannerStartSoftwareTriggerRequest2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAsync)(::windows_core::Interface::as_raw(this), reason, &mut result__).from_abi(result__)
        }
    }
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync(&self, reason: i32, failedreasondescription: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IBarcodeScannerStartSoftwareTriggerRequest2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAndDescriptionAsync)(::windows_core::Interface::as_raw(this), reason, ::core::mem::transmute_copy(failedreasondescription), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for BarcodeScannerStartSoftwareTriggerRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for BarcodeScannerStartSoftwareTriggerRequest {
    type Vtable = IBarcodeScannerStartSoftwareTriggerRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BarcodeScannerStartSoftwareTriggerRequest {
    const IID: ::windows_core::GUID = <IBarcodeScannerStartSoftwareTriggerRequest as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerStartSoftwareTriggerRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerStartSoftwareTriggerRequest";
}
::windows_core::imp::interface_hierarchy!(BarcodeScannerStartSoftwareTriggerRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for BarcodeScannerStartSoftwareTriggerRequest {}
unsafe impl ::core::marker::Sync for BarcodeScannerStartSoftwareTriggerRequest {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BarcodeScannerStartSoftwareTriggerRequestEventArgs(::windows_core::IUnknown);
impl BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<BarcodeScannerStartSoftwareTriggerRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    type Vtable = IBarcodeScannerStartSoftwareTriggerRequestEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    const IID: ::windows_core::GUID = <IBarcodeScannerStartSoftwareTriggerRequestEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerStartSoftwareTriggerRequestEventArgs";
}
::windows_core::imp::interface_hierarchy!(BarcodeScannerStartSoftwareTriggerRequestEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for BarcodeScannerStartSoftwareTriggerRequestEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerStartSoftwareTriggerRequestEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BarcodeScannerStopSoftwareTriggerRequest(::windows_core::IUnknown);
impl BarcodeScannerStopSoftwareTriggerRequest {
    pub fn ReportCompletedAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportCompletedAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportFailedAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IBarcodeScannerStopSoftwareTriggerRequest2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAsync)(::windows_core::Interface::as_raw(this), reason, &mut result__).from_abi(result__)
        }
    }
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync(&self, reason: i32, failedreasondescription: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IBarcodeScannerStopSoftwareTriggerRequest2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAndDescriptionAsync)(::windows_core::Interface::as_raw(this), reason, ::core::mem::transmute_copy(failedreasondescription), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for BarcodeScannerStopSoftwareTriggerRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for BarcodeScannerStopSoftwareTriggerRequest {
    type Vtable = IBarcodeScannerStopSoftwareTriggerRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BarcodeScannerStopSoftwareTriggerRequest {
    const IID: ::windows_core::GUID = <IBarcodeScannerStopSoftwareTriggerRequest as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerStopSoftwareTriggerRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerStopSoftwareTriggerRequest";
}
::windows_core::imp::interface_hierarchy!(BarcodeScannerStopSoftwareTriggerRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for BarcodeScannerStopSoftwareTriggerRequest {}
unsafe impl ::core::marker::Sync for BarcodeScannerStopSoftwareTriggerRequest {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BarcodeScannerStopSoftwareTriggerRequestEventArgs(::windows_core::IUnknown);
impl BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<BarcodeScannerStopSoftwareTriggerRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    type Vtable = IBarcodeScannerStopSoftwareTriggerRequestEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    const IID: ::windows_core::GUID = <IBarcodeScannerStopSoftwareTriggerRequestEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerStopSoftwareTriggerRequestEventArgs";
}
::windows_core::imp::interface_hierarchy!(BarcodeScannerStopSoftwareTriggerRequestEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for BarcodeScannerStopSoftwareTriggerRequestEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerStopSoftwareTriggerRequestEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BarcodeScannerVideoFrame(::windows_core::IUnknown);
impl BarcodeScannerVideoFrame {
    #[doc = "Required features: `\"Graphics_Imaging\"`"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn Format(&self) -> ::windows_core::Result<super::super::super::Graphics::Imaging::BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Format)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Width(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Width)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Height(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Height)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn PixelData(&self) -> ::windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PixelData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::windows_core::RuntimeType for BarcodeScannerVideoFrame {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for BarcodeScannerVideoFrame {
    type Vtable = IBarcodeScannerVideoFrame_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BarcodeScannerVideoFrame {
    const IID: ::windows_core::GUID = <IBarcodeScannerVideoFrame as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerVideoFrame {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerVideoFrame";
}
::windows_core::imp::interface_hierarchy!(BarcodeScannerVideoFrame, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for BarcodeScannerVideoFrame {}
unsafe impl ::core::marker::Send for BarcodeScannerVideoFrame {}
unsafe impl ::core::marker::Sync for BarcodeScannerVideoFrame {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BarcodeSymbologyAttributesBuilder(::windows_core::IUnknown);
impl BarcodeSymbologyAttributesBuilder {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<BarcodeSymbologyAttributesBuilder, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn IsCheckDigitValidationSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsCheckDigitValidationSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsCheckDigitValidationSupported(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsCheckDigitValidationSupported)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsCheckDigitTransmissionSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsCheckDigitTransmissionSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsCheckDigitTransmissionSupported(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsCheckDigitTransmissionSupported)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDecodeLengthSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDecodeLengthSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsDecodeLengthSupported(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDecodeLengthSupported)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateAttributes(&self) -> ::windows_core::Result<super::BarcodeSymbologyAttributes> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateAttributes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for BarcodeSymbologyAttributesBuilder {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for BarcodeSymbologyAttributesBuilder {
    type Vtable = IBarcodeSymbologyAttributesBuilder_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BarcodeSymbologyAttributesBuilder {
    const IID: ::windows_core::GUID = <IBarcodeSymbologyAttributesBuilder as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeSymbologyAttributesBuilder {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeSymbologyAttributesBuilder";
}
::windows_core::imp::interface_hierarchy!(BarcodeSymbologyAttributesBuilder, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for BarcodeSymbologyAttributesBuilder {}
unsafe impl ::core::marker::Sync for BarcodeSymbologyAttributesBuilder {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct BarcodeScannerTriggerState(pub i32);
impl BarcodeScannerTriggerState {
    pub const Released: Self = Self(0i32);
    pub const Pressed: Self = Self(1i32);
}
impl ::windows_core::TypeKind for BarcodeScannerTriggerState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for BarcodeScannerTriggerState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerTriggerState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for BarcodeScannerTriggerState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.Provider.BarcodeScannerTriggerState;i4)");
}
