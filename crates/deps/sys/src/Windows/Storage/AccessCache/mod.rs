#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AccessCacheOptions(pub u32);
impl AccessCacheOptions {
    pub const None: AccessCacheOptions = AccessCacheOptions(0u32);
    pub const DisallowUserInput: AccessCacheOptions = AccessCacheOptions(1u32);
    pub const FastLocationsOnly: AccessCacheOptions = AccessCacheOptions(2u32);
    pub const UseReadOnlyCachedCopy: AccessCacheOptions = AccessCacheOptions(4u32);
    pub const SuppressAccessTimeUpdate: AccessCacheOptions = AccessCacheOptions(8u32);
}
#[repr(C)]
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
#[repr(transparent)]
pub struct RecentStorageItemVisibility(pub i32);
impl RecentStorageItemVisibility {
    pub const AppOnly: RecentStorageItemVisibility = RecentStorageItemVisibility(0i32);
    pub const AppAndSystem: RecentStorageItemVisibility = RecentStorageItemVisibility(1i32);
}
#[repr(transparent)]
pub struct StorageItemAccessList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageItemMostRecentlyUsedList(pub *mut ::core::ffi::c_void);
