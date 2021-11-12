#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IWsbApplicationAsync(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWsbApplicationBackupSupport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWsbApplicationRestoreSupport(pub *mut ::core::ffi::c_void);
pub const WSBAPP_ASYNC_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(7995396i32 as _);
pub const WSB_MAX_OB_STATUS_ENTRY: u32 = 5u32;
pub const WSB_MAX_OB_STATUS_VALUE_TYPE_PAIR: u32 = 5u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSB_OB_REGISTRATION_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSB_OB_STATUS_ENTRY(i32);
#[repr(transparent)]
pub struct WSB_OB_STATUS_ENTRY_PAIR_TYPE(pub i32);
pub const WSB_OB_ET_UNDEFINED: WSB_OB_STATUS_ENTRY_PAIR_TYPE = WSB_OB_STATUS_ENTRY_PAIR_TYPE(0i32);
pub const WSB_OB_ET_STRING: WSB_OB_STATUS_ENTRY_PAIR_TYPE = WSB_OB_STATUS_ENTRY_PAIR_TYPE(1i32);
pub const WSB_OB_ET_NUMBER: WSB_OB_STATUS_ENTRY_PAIR_TYPE = WSB_OB_STATUS_ENTRY_PAIR_TYPE(2i32);
pub const WSB_OB_ET_DATETIME: WSB_OB_STATUS_ENTRY_PAIR_TYPE = WSB_OB_STATUS_ENTRY_PAIR_TYPE(3i32);
pub const WSB_OB_ET_TIME: WSB_OB_STATUS_ENTRY_PAIR_TYPE = WSB_OB_STATUS_ENTRY_PAIR_TYPE(4i32);
pub const WSB_OB_ET_SIZE: WSB_OB_STATUS_ENTRY_PAIR_TYPE = WSB_OB_STATUS_ENTRY_PAIR_TYPE(5i32);
pub const WSB_OB_ET_MAX: WSB_OB_STATUS_ENTRY_PAIR_TYPE = WSB_OB_STATUS_ENTRY_PAIR_TYPE(6i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSB_OB_STATUS_INFO(i32);
