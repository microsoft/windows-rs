#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
impl ::core::marker::Copy for AccessCacheOptions {}
impl ::core::clone::Clone for AccessCacheOptions {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for AccessListEntryView {}
impl ::core::clone::Clone for AccessListEntryView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItemRemovedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemRemovedEventArgs {}
impl ::core::clone::Clone for IItemRemovedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageApplicationPermissionsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageApplicationPermissionsStatics {}
impl ::core::clone::Clone for IStorageApplicationPermissionsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageApplicationPermissionsStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageApplicationPermissionsStatics2 {}
impl ::core::clone::Clone for IStorageApplicationPermissionsStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageItemAccessList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageItemAccessList {}
impl ::core::clone::Clone for IStorageItemAccessList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageItemMostRecentlyUsedList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageItemMostRecentlyUsedList {}
impl ::core::clone::Clone for IStorageItemMostRecentlyUsedList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageItemMostRecentlyUsedList2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageItemMostRecentlyUsedList2 {}
impl ::core::clone::Clone for IStorageItemMostRecentlyUsedList2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ItemRemovedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ItemRemovedEventArgs {}
impl ::core::clone::Clone for ItemRemovedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RecentStorageItemVisibility(pub i32);
impl RecentStorageItemVisibility {
    pub const AppOnly: Self = Self(0i32);
    pub const AppAndSystem: Self = Self(1i32);
}
impl ::core::marker::Copy for RecentStorageItemVisibility {}
impl ::core::clone::Clone for RecentStorageItemVisibility {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageItemAccessList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StorageItemAccessList {}
impl ::core::clone::Clone for StorageItemAccessList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageItemMostRecentlyUsedList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StorageItemMostRecentlyUsedList {}
impl ::core::clone::Clone for StorageItemMostRecentlyUsedList {
    fn clone(&self) -> Self {
        *self
    }
}
