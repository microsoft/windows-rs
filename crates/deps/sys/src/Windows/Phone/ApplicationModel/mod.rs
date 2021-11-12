#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ApplicationProfileModes(pub u32);
impl ApplicationProfileModes {
    pub const Default: ApplicationProfileModes = ApplicationProfileModes(0u32);
    pub const Alternate: ApplicationProfileModes = ApplicationProfileModes(1u32);
}
#[repr(transparent)]
pub struct IApplicationProfileStatics(pub *mut ::core::ffi::c_void);
