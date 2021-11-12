#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ISystemUpdateItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemUpdateLastErrorInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemUpdateManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SystemUpdateAttentionRequiredReason(pub i32);
impl SystemUpdateAttentionRequiredReason {
    pub const None: SystemUpdateAttentionRequiredReason = SystemUpdateAttentionRequiredReason(0i32);
    pub const NetworkRequired: SystemUpdateAttentionRequiredReason = SystemUpdateAttentionRequiredReason(1i32);
    pub const InsufficientDiskSpace: SystemUpdateAttentionRequiredReason = SystemUpdateAttentionRequiredReason(2i32);
    pub const InsufficientBattery: SystemUpdateAttentionRequiredReason = SystemUpdateAttentionRequiredReason(3i32);
    pub const UpdateBlocked: SystemUpdateAttentionRequiredReason = SystemUpdateAttentionRequiredReason(4i32);
}
#[repr(transparent)]
pub struct SystemUpdateItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SystemUpdateItemState(pub i32);
impl SystemUpdateItemState {
    pub const NotStarted: SystemUpdateItemState = SystemUpdateItemState(0i32);
    pub const Initializing: SystemUpdateItemState = SystemUpdateItemState(1i32);
    pub const Preparing: SystemUpdateItemState = SystemUpdateItemState(2i32);
    pub const Calculating: SystemUpdateItemState = SystemUpdateItemState(3i32);
    pub const Downloading: SystemUpdateItemState = SystemUpdateItemState(4i32);
    pub const Installing: SystemUpdateItemState = SystemUpdateItemState(5i32);
    pub const Completed: SystemUpdateItemState = SystemUpdateItemState(6i32);
    pub const RebootRequired: SystemUpdateItemState = SystemUpdateItemState(7i32);
    pub const Error: SystemUpdateItemState = SystemUpdateItemState(8i32);
}
#[repr(transparent)]
pub struct SystemUpdateLastErrorInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SystemUpdateManagerState(pub i32);
impl SystemUpdateManagerState {
    pub const Idle: SystemUpdateManagerState = SystemUpdateManagerState(0i32);
    pub const Detecting: SystemUpdateManagerState = SystemUpdateManagerState(1i32);
    pub const ReadyToDownload: SystemUpdateManagerState = SystemUpdateManagerState(2i32);
    pub const Downloading: SystemUpdateManagerState = SystemUpdateManagerState(3i32);
    pub const ReadyToInstall: SystemUpdateManagerState = SystemUpdateManagerState(4i32);
    pub const Installing: SystemUpdateManagerState = SystemUpdateManagerState(5i32);
    pub const RebootRequired: SystemUpdateManagerState = SystemUpdateManagerState(6i32);
    pub const ReadyToFinalize: SystemUpdateManagerState = SystemUpdateManagerState(7i32);
    pub const Finalizing: SystemUpdateManagerState = SystemUpdateManagerState(8i32);
    pub const Completed: SystemUpdateManagerState = SystemUpdateManagerState(9i32);
    pub const AttentionRequired: SystemUpdateManagerState = SystemUpdateManagerState(10i32);
    pub const Error: SystemUpdateManagerState = SystemUpdateManagerState(11i32);
}
#[repr(transparent)]
pub struct SystemUpdateStartInstallAction(pub i32);
impl SystemUpdateStartInstallAction {
    pub const UpToReboot: SystemUpdateStartInstallAction = SystemUpdateStartInstallAction(0i32);
    pub const AllowReboot: SystemUpdateStartInstallAction = SystemUpdateStartInstallAction(1i32);
}
