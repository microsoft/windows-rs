#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DeviceAccountAuthenticationType(pub i32);
impl DeviceAccountAuthenticationType {
    pub const Basic: Self = Self(0i32);
    pub const OAuth: Self = Self(1i32);
    pub const SingleSignOn: Self = Self(2i32);
}
#[repr(transparent)]
pub struct DeviceAccountConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeviceAccountIconId(pub i32);
impl DeviceAccountIconId {
    pub const Exchange: Self = Self(0i32);
    pub const Msa: Self = Self(1i32);
    pub const Outlook: Self = Self(2i32);
    pub const Generic: Self = Self(3i32);
}
#[repr(transparent)]
pub struct DeviceAccountMailAgeFilter(pub i32);
impl DeviceAccountMailAgeFilter {
    pub const All: Self = Self(0i32);
    pub const Last1Day: Self = Self(1i32);
    pub const Last3Days: Self = Self(2i32);
    pub const Last7Days: Self = Self(3i32);
    pub const Last14Days: Self = Self(4i32);
    pub const Last30Days: Self = Self(5i32);
    pub const Last90Days: Self = Self(6i32);
}
#[repr(transparent)]
pub struct DeviceAccountServerType(pub i32);
impl DeviceAccountServerType {
    pub const Exchange: Self = Self(0i32);
    pub const Pop: Self = Self(1i32);
    pub const Imap: Self = Self(2i32);
}
#[repr(transparent)]
pub struct DeviceAccountSyncScheduleKind(pub i32);
impl DeviceAccountSyncScheduleKind {
    pub const Manual: Self = Self(0i32);
    pub const Every15Minutes: Self = Self(1i32);
    pub const Every30Minutes: Self = Self(2i32);
    pub const Every60Minutes: Self = Self(3i32);
    pub const Every2Hours: Self = Self(4i32);
    pub const Daily: Self = Self(5i32);
    pub const AsItemsArrive: Self = Self(6i32);
}
#[repr(transparent)]
pub struct IDeviceAccountConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceAccountConfiguration2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataAccountSystemAccessManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataAccountSystemAccessManagerStatics2(pub *mut ::core::ffi::c_void);
