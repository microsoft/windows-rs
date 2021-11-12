#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPnpObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPnpObjectStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPnpObjectUpdate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPnpObjectWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PnpObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PnpObjectCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PnpObjectType(pub i32);
impl PnpObjectType {
    pub const Unknown: Self = Self(0i32);
    pub const DeviceInterface: Self = Self(1i32);
    pub const DeviceContainer: Self = Self(2i32);
    pub const Device: Self = Self(3i32);
    pub const DeviceInterfaceClass: Self = Self(4i32);
    pub const AssociationEndpoint: Self = Self(5i32);
    pub const AssociationEndpointContainer: Self = Self(6i32);
    pub const AssociationEndpointService: Self = Self(7i32);
    pub const DevicePanel: Self = Self(8i32);
}
#[repr(transparent)]
pub struct PnpObjectUpdate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PnpObjectWatcher(pub *mut ::core::ffi::c_void);
