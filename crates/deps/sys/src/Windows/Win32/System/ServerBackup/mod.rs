#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IWsbApplicationAsync(i32);
pub struct IWsbApplicationBackupSupport(i32);
pub struct IWsbApplicationRestoreSupport(i32);
#[doc = "*Required features: `Win32_System_ServerBackup`*"]
pub const WSBAPP_ASYNC_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(7995396i32 as _);
#[doc = "*Required features: `Win32_System_ServerBackup`*"]
pub const WSB_MAX_OB_STATUS_ENTRY: u32 = 5u32;
#[doc = "*Required features: `Win32_System_ServerBackup`*"]
pub const WSB_MAX_OB_STATUS_VALUE_TYPE_PAIR: u32 = 5u32;
pub struct WSB_OB_REGISTRATION_INFO(i32);
pub struct WSB_OB_STATUS_ENTRY(i32);
pub struct WSB_OB_STATUS_ENTRY_PAIR_TYPE(i32);
pub struct WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR(i32);
pub struct WSB_OB_STATUS_INFO(i32);
