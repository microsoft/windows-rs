#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct AccessCacheOptions(i32);
pub struct AccessListEntry(i32);
pub struct AccessListEntryView(i32);
pub struct IItemRemovedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IStorageApplicationPermissionsStatics(pub *mut ::core::ffi::c_void);
pub struct IStorageApplicationPermissionsStatics2(pub *mut ::core::ffi::c_void);
pub struct IStorageItemAccessList(pub *mut ::core::ffi::c_void);
pub struct IStorageItemMostRecentlyUsedList(pub *mut ::core::ffi::c_void);
pub struct IStorageItemMostRecentlyUsedList2(pub *mut ::core::ffi::c_void);
pub struct ItemRemovedEventArgs(i32);
pub struct RecentStorageItemVisibility(i32);
pub struct StorageApplicationPermissions(i32);
pub struct StorageItemAccessList(i32);
pub struct StorageItemMostRecentlyUsedList(i32);
