#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct AccessCacheOptions(i32);
pub struct AccessListEntry(i32);
#[repr(transparent)]
pub struct AccessListEntryView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemRemovedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageApplicationPermissionsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageApplicationPermissionsStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageItemAccessList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageItemMostRecentlyUsedList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageItemMostRecentlyUsedList2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ItemRemovedEventArgs(pub *mut ::core::ffi::c_void);
pub struct RecentStorageItemVisibility(i32);
#[repr(transparent)]
pub struct StorageApplicationPermissions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageItemAccessList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageItemMostRecentlyUsedList(pub *mut ::core::ffi::c_void);
