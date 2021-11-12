#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct ISystemUpdateItem(i32);
pub struct ISystemUpdateLastErrorInfo(i32);
pub struct ISystemUpdateManagerStatics(i32);
pub struct SystemUpdateAttentionRequiredReason(i32);
pub struct SystemUpdateItem(i32);
pub struct SystemUpdateItemState(i32);
pub struct SystemUpdateLastErrorInfo(i32);
pub struct SystemUpdateManager(i32);
pub struct SystemUpdateManagerState(i32);
pub struct SystemUpdateStartInstallAction(i32);
