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
    pub const Unknown: PnpObjectType = PnpObjectType(0i32);
    pub const DeviceInterface: PnpObjectType = PnpObjectType(1i32);
    pub const DeviceContainer: PnpObjectType = PnpObjectType(2i32);
    pub const Device: PnpObjectType = PnpObjectType(3i32);
    pub const DeviceInterfaceClass: PnpObjectType = PnpObjectType(4i32);
    pub const AssociationEndpoint: PnpObjectType = PnpObjectType(5i32);
    pub const AssociationEndpointContainer: PnpObjectType = PnpObjectType(6i32);
    pub const AssociationEndpointService: PnpObjectType = PnpObjectType(7i32);
    pub const DevicePanel: PnpObjectType = PnpObjectType(8i32);
}
#[repr(transparent)]
pub struct PnpObjectUpdate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PnpObjectWatcher(pub *mut ::core::ffi::c_void);
