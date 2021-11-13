#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Perception_Automation")]
pub mod Automation;
#[cfg(feature = "Perception_People")]
pub mod People;
#[cfg(feature = "Perception_Spatial")]
pub mod Spatial;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPerceptionTimestamp(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionTimestamp {}
impl ::core::clone::Clone for IPerceptionTimestamp {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionTimestamp2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionTimestamp2 {}
impl ::core::clone::Clone for IPerceptionTimestamp2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionTimestampHelperStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionTimestampHelperStatics {}
impl ::core::clone::Clone for IPerceptionTimestampHelperStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionTimestampHelperStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionTimestampHelperStatics2 {}
impl ::core::clone::Clone for IPerceptionTimestampHelperStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PerceptionTimestamp(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PerceptionTimestamp {}
impl ::core::clone::Clone for PerceptionTimestamp {
    fn clone(&self) -> Self {
        *self
    }
}
