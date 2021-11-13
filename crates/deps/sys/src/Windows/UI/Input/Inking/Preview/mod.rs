#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPalmRejectionDelayZonePreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPalmRejectionDelayZonePreview {}
impl ::core::clone::Clone for IPalmRejectionDelayZonePreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPalmRejectionDelayZonePreviewStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPalmRejectionDelayZonePreviewStatics {}
impl ::core::clone::Clone for IPalmRejectionDelayZonePreviewStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PalmRejectionDelayZonePreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PalmRejectionDelayZonePreview {}
impl ::core::clone::Clone for PalmRejectionDelayZonePreview {
    fn clone(&self) -> Self {
        *self
    }
}
