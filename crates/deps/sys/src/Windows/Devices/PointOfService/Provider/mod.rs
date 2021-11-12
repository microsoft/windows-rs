#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct BarcodeScannerDisableScannerRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BarcodeScannerDisableScannerRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BarcodeScannerEnableScannerRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BarcodeScannerEnableScannerRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BarcodeScannerFrameReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BarcodeScannerFrameReaderFrameArrivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BarcodeScannerGetSymbologyAttributesRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BarcodeScannerGetSymbologyAttributesRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BarcodeScannerHideVideoPreviewRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BarcodeScannerHideVideoPreviewRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BarcodeScannerProviderConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BarcodeScannerProviderTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BarcodeScannerSetActiveSymbologiesRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BarcodeScannerSetActiveSymbologiesRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BarcodeScannerSetSymbologyAttributesRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BarcodeScannerSetSymbologyAttributesRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BarcodeScannerStartSoftwareTriggerRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BarcodeScannerStartSoftwareTriggerRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BarcodeScannerStopSoftwareTriggerRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BarcodeScannerStopSoftwareTriggerRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BarcodeScannerTriggerState(pub i32);
impl BarcodeScannerTriggerState {
    pub const Released: Self = Self(0i32);
    pub const Pressed: Self = Self(1i32);
}
#[repr(transparent)]
pub struct BarcodeScannerVideoFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BarcodeSymbologyAttributesBuilder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerDisableScannerRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerDisableScannerRequest2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerDisableScannerRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerEnableScannerRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerEnableScannerRequest2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerEnableScannerRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerFrameReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerFrameReaderFrameArrivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerGetSymbologyAttributesRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerGetSymbologyAttributesRequest2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerGetSymbologyAttributesRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerHideVideoPreviewRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerHideVideoPreviewRequest2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerHideVideoPreviewRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerProviderConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerProviderConnection2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerProviderTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerSetActiveSymbologiesRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerSetActiveSymbologiesRequest2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerSetActiveSymbologiesRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerSetSymbologyAttributesRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerSetSymbologyAttributesRequest2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerSetSymbologyAttributesRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerStartSoftwareTriggerRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerStartSoftwareTriggerRequest2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerStartSoftwareTriggerRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerStopSoftwareTriggerRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerStopSoftwareTriggerRequest2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerStopSoftwareTriggerRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerVideoFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeSymbologyAttributesBuilder(pub *mut ::core::ffi::c_void);
