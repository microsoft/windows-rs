#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DeviceAccountAuthenticationType(pub i32);
impl DeviceAccountAuthenticationType {
    pub const Basic: DeviceAccountAuthenticationType = DeviceAccountAuthenticationType(0i32);
    pub const OAuth: DeviceAccountAuthenticationType = DeviceAccountAuthenticationType(1i32);
    pub const SingleSignOn: DeviceAccountAuthenticationType = DeviceAccountAuthenticationType(2i32);
}
#[repr(transparent)]
pub struct DeviceAccountConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeviceAccountIconId(pub i32);
impl DeviceAccountIconId {
    pub const Exchange: DeviceAccountIconId = DeviceAccountIconId(0i32);
    pub const Msa: DeviceAccountIconId = DeviceAccountIconId(1i32);
    pub const Outlook: DeviceAccountIconId = DeviceAccountIconId(2i32);
    pub const Generic: DeviceAccountIconId = DeviceAccountIconId(3i32);
}
#[repr(transparent)]
pub struct DeviceAccountMailAgeFilter(pub i32);
impl DeviceAccountMailAgeFilter {
    pub const All: DeviceAccountMailAgeFilter = DeviceAccountMailAgeFilter(0i32);
    pub const Last1Day: DeviceAccountMailAgeFilter = DeviceAccountMailAgeFilter(1i32);
    pub const Last3Days: DeviceAccountMailAgeFilter = DeviceAccountMailAgeFilter(2i32);
    pub const Last7Days: DeviceAccountMailAgeFilter = DeviceAccountMailAgeFilter(3i32);
    pub const Last14Days: DeviceAccountMailAgeFilter = DeviceAccountMailAgeFilter(4i32);
    pub const Last30Days: DeviceAccountMailAgeFilter = DeviceAccountMailAgeFilter(5i32);
    pub const Last90Days: DeviceAccountMailAgeFilter = DeviceAccountMailAgeFilter(6i32);
}
#[repr(transparent)]
pub struct DeviceAccountServerType(pub i32);
impl DeviceAccountServerType {
    pub const Exchange: DeviceAccountServerType = DeviceAccountServerType(0i32);
    pub const Pop: DeviceAccountServerType = DeviceAccountServerType(1i32);
    pub const Imap: DeviceAccountServerType = DeviceAccountServerType(2i32);
}
#[repr(transparent)]
pub struct DeviceAccountSyncScheduleKind(pub i32);
impl DeviceAccountSyncScheduleKind {
    pub const Manual: DeviceAccountSyncScheduleKind = DeviceAccountSyncScheduleKind(0i32);
    pub const Every15Minutes: DeviceAccountSyncScheduleKind = DeviceAccountSyncScheduleKind(1i32);
    pub const Every30Minutes: DeviceAccountSyncScheduleKind = DeviceAccountSyncScheduleKind(2i32);
    pub const Every60Minutes: DeviceAccountSyncScheduleKind = DeviceAccountSyncScheduleKind(3i32);
    pub const Every2Hours: DeviceAccountSyncScheduleKind = DeviceAccountSyncScheduleKind(4i32);
    pub const Daily: DeviceAccountSyncScheduleKind = DeviceAccountSyncScheduleKind(5i32);
    pub const AsItemsArrive: DeviceAccountSyncScheduleKind = DeviceAccountSyncScheduleKind(6i32);
}
#[repr(transparent)]
pub struct IDeviceAccountConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceAccountConfiguration2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataAccountSystemAccessManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataAccountSystemAccessManagerStatics2(pub *mut ::core::ffi::c_void);
