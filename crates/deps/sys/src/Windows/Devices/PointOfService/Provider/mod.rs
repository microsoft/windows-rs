#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct BarcodeScannerDisableScannerRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BarcodeScannerDisableScannerRequest {}
impl ::core::clone::Clone for BarcodeScannerDisableScannerRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BarcodeScannerDisableScannerRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BarcodeScannerDisableScannerRequestEventArgs {}
impl ::core::clone::Clone for BarcodeScannerDisableScannerRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BarcodeScannerEnableScannerRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BarcodeScannerEnableScannerRequest {}
impl ::core::clone::Clone for BarcodeScannerEnableScannerRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BarcodeScannerEnableScannerRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BarcodeScannerEnableScannerRequestEventArgs {}
impl ::core::clone::Clone for BarcodeScannerEnableScannerRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BarcodeScannerFrameReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BarcodeScannerFrameReader {}
impl ::core::clone::Clone for BarcodeScannerFrameReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BarcodeScannerFrameReaderFrameArrivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BarcodeScannerFrameReaderFrameArrivedEventArgs {}
impl ::core::clone::Clone for BarcodeScannerFrameReaderFrameArrivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BarcodeScannerGetSymbologyAttributesRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BarcodeScannerGetSymbologyAttributesRequest {}
impl ::core::clone::Clone for BarcodeScannerGetSymbologyAttributesRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BarcodeScannerGetSymbologyAttributesRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BarcodeScannerGetSymbologyAttributesRequestEventArgs {}
impl ::core::clone::Clone for BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BarcodeScannerHideVideoPreviewRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BarcodeScannerHideVideoPreviewRequest {}
impl ::core::clone::Clone for BarcodeScannerHideVideoPreviewRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BarcodeScannerHideVideoPreviewRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BarcodeScannerHideVideoPreviewRequestEventArgs {}
impl ::core::clone::Clone for BarcodeScannerHideVideoPreviewRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BarcodeScannerProviderConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BarcodeScannerProviderConnection {}
impl ::core::clone::Clone for BarcodeScannerProviderConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BarcodeScannerProviderTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BarcodeScannerProviderTriggerDetails {}
impl ::core::clone::Clone for BarcodeScannerProviderTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BarcodeScannerSetActiveSymbologiesRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BarcodeScannerSetActiveSymbologiesRequest {}
impl ::core::clone::Clone for BarcodeScannerSetActiveSymbologiesRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BarcodeScannerSetActiveSymbologiesRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BarcodeScannerSetActiveSymbologiesRequestEventArgs {}
impl ::core::clone::Clone for BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BarcodeScannerSetSymbologyAttributesRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BarcodeScannerSetSymbologyAttributesRequest {}
impl ::core::clone::Clone for BarcodeScannerSetSymbologyAttributesRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BarcodeScannerSetSymbologyAttributesRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BarcodeScannerSetSymbologyAttributesRequestEventArgs {}
impl ::core::clone::Clone for BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BarcodeScannerStartSoftwareTriggerRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BarcodeScannerStartSoftwareTriggerRequest {}
impl ::core::clone::Clone for BarcodeScannerStartSoftwareTriggerRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BarcodeScannerStartSoftwareTriggerRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BarcodeScannerStartSoftwareTriggerRequestEventArgs {}
impl ::core::clone::Clone for BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BarcodeScannerStopSoftwareTriggerRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BarcodeScannerStopSoftwareTriggerRequest {}
impl ::core::clone::Clone for BarcodeScannerStopSoftwareTriggerRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BarcodeScannerStopSoftwareTriggerRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BarcodeScannerStopSoftwareTriggerRequestEventArgs {}
impl ::core::clone::Clone for BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BarcodeScannerTriggerState(pub i32);
impl BarcodeScannerTriggerState {
    pub const Released: Self = Self(0i32);
    pub const Pressed: Self = Self(1i32);
}
impl ::core::marker::Copy for BarcodeScannerTriggerState {}
impl ::core::clone::Clone for BarcodeScannerTriggerState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BarcodeScannerVideoFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BarcodeScannerVideoFrame {}
impl ::core::clone::Clone for BarcodeScannerVideoFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BarcodeSymbologyAttributesBuilder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BarcodeSymbologyAttributesBuilder {}
impl ::core::clone::Clone for BarcodeSymbologyAttributesBuilder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerDisableScannerRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerDisableScannerRequest {}
impl ::core::clone::Clone for IBarcodeScannerDisableScannerRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerDisableScannerRequest2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerDisableScannerRequest2 {}
impl ::core::clone::Clone for IBarcodeScannerDisableScannerRequest2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerDisableScannerRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerDisableScannerRequestEventArgs {}
impl ::core::clone::Clone for IBarcodeScannerDisableScannerRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerEnableScannerRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerEnableScannerRequest {}
impl ::core::clone::Clone for IBarcodeScannerEnableScannerRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerEnableScannerRequest2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerEnableScannerRequest2 {}
impl ::core::clone::Clone for IBarcodeScannerEnableScannerRequest2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerEnableScannerRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerEnableScannerRequestEventArgs {}
impl ::core::clone::Clone for IBarcodeScannerEnableScannerRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerFrameReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerFrameReader {}
impl ::core::clone::Clone for IBarcodeScannerFrameReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerFrameReaderFrameArrivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerFrameReaderFrameArrivedEventArgs {}
impl ::core::clone::Clone for IBarcodeScannerFrameReaderFrameArrivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerGetSymbologyAttributesRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerGetSymbologyAttributesRequest {}
impl ::core::clone::Clone for IBarcodeScannerGetSymbologyAttributesRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerGetSymbologyAttributesRequest2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerGetSymbologyAttributesRequest2 {}
impl ::core::clone::Clone for IBarcodeScannerGetSymbologyAttributesRequest2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerGetSymbologyAttributesRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerGetSymbologyAttributesRequestEventArgs {}
impl ::core::clone::Clone for IBarcodeScannerGetSymbologyAttributesRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerHideVideoPreviewRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerHideVideoPreviewRequest {}
impl ::core::clone::Clone for IBarcodeScannerHideVideoPreviewRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerHideVideoPreviewRequest2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerHideVideoPreviewRequest2 {}
impl ::core::clone::Clone for IBarcodeScannerHideVideoPreviewRequest2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerHideVideoPreviewRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerHideVideoPreviewRequestEventArgs {}
impl ::core::clone::Clone for IBarcodeScannerHideVideoPreviewRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerProviderConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerProviderConnection {}
impl ::core::clone::Clone for IBarcodeScannerProviderConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerProviderConnection2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerProviderConnection2 {}
impl ::core::clone::Clone for IBarcodeScannerProviderConnection2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerProviderTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerProviderTriggerDetails {}
impl ::core::clone::Clone for IBarcodeScannerProviderTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerSetActiveSymbologiesRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerSetActiveSymbologiesRequest {}
impl ::core::clone::Clone for IBarcodeScannerSetActiveSymbologiesRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerSetActiveSymbologiesRequest2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerSetActiveSymbologiesRequest2 {}
impl ::core::clone::Clone for IBarcodeScannerSetActiveSymbologiesRequest2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerSetActiveSymbologiesRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerSetActiveSymbologiesRequestEventArgs {}
impl ::core::clone::Clone for IBarcodeScannerSetActiveSymbologiesRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerSetSymbologyAttributesRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerSetSymbologyAttributesRequest {}
impl ::core::clone::Clone for IBarcodeScannerSetSymbologyAttributesRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerSetSymbologyAttributesRequest2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerSetSymbologyAttributesRequest2 {}
impl ::core::clone::Clone for IBarcodeScannerSetSymbologyAttributesRequest2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerSetSymbologyAttributesRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerSetSymbologyAttributesRequestEventArgs {}
impl ::core::clone::Clone for IBarcodeScannerSetSymbologyAttributesRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerStartSoftwareTriggerRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerStartSoftwareTriggerRequest {}
impl ::core::clone::Clone for IBarcodeScannerStartSoftwareTriggerRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerStartSoftwareTriggerRequest2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerStartSoftwareTriggerRequest2 {}
impl ::core::clone::Clone for IBarcodeScannerStartSoftwareTriggerRequest2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerStartSoftwareTriggerRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerStartSoftwareTriggerRequestEventArgs {}
impl ::core::clone::Clone for IBarcodeScannerStartSoftwareTriggerRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerStopSoftwareTriggerRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerStopSoftwareTriggerRequest {}
impl ::core::clone::Clone for IBarcodeScannerStopSoftwareTriggerRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerStopSoftwareTriggerRequest2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerStopSoftwareTriggerRequest2 {}
impl ::core::clone::Clone for IBarcodeScannerStopSoftwareTriggerRequest2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerStopSoftwareTriggerRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerStopSoftwareTriggerRequestEventArgs {}
impl ::core::clone::Clone for IBarcodeScannerStopSoftwareTriggerRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerVideoFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerVideoFrame {}
impl ::core::clone::Clone for IBarcodeScannerVideoFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeSymbologyAttributesBuilder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeSymbologyAttributesBuilder {}
impl ::core::clone::Clone for IBarcodeSymbologyAttributesBuilder {
    fn clone(&self) -> Self {
        *self
    }
}
