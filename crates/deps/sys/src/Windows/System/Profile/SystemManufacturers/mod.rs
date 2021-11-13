#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IOemSupportInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOemSupportInfo {}
impl ::core::clone::Clone for IOemSupportInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmbiosInformationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmbiosInformationStatics {}
impl ::core::clone::Clone for ISmbiosInformationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemSupportDeviceInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemSupportDeviceInfo {}
impl ::core::clone::Clone for ISystemSupportDeviceInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemSupportInfoStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemSupportInfoStatics {}
impl ::core::clone::Clone for ISystemSupportInfoStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemSupportInfoStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemSupportInfoStatics2 {}
impl ::core::clone::Clone for ISystemSupportInfoStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct OemSupportInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for OemSupportInfo {}
impl ::core::clone::Clone for OemSupportInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemSupportDeviceInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SystemSupportDeviceInfo {}
impl ::core::clone::Clone for SystemSupportDeviceInfo {
    fn clone(&self) -> Self {
        *self
    }
}
