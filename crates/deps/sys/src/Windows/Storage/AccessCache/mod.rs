#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AccessCacheOptions(pub u32);
impl AccessCacheOptions {
    pub const None: Self = Self(0u32);
    pub const DisallowUserInput: Self = Self(1u32);
    pub const FastLocationsOnly: Self = Self(2u32);
    pub const UseReadOnlyCachedCopy: Self = Self(4u32);
    pub const SuppressAccessTimeUpdate: Self = Self(8u32);
}
#[repr(C)]
pub struct AccessListEntry {
    pub Token: ::windows_sys::core::HSTRING,
    pub Metadata: ::windows_sys::core::HSTRING,
}
impl ::core::marker::Copy for AccessListEntry {}
impl ::core::clone::Clone for AccessListEntry {
    fn clone(&self) -> Self {
        *self
    }
}
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
    pub const AppOnly: Self = Self(0i32);
    pub const AppAndSystem: Self = Self(1i32);
}
#[repr(transparent)]
pub struct StorageItemAccessList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageItemMostRecentlyUsedList(pub *mut ::core::ffi::c_void);
