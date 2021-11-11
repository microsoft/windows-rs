#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn AccessCacheOptions();
    fn AccessListEntry();
    fn AccessListEntryView();
    fn IItemRemovedEventArgs();
    fn IStorageApplicationPermissionsStatics();
    fn IStorageApplicationPermissionsStatics2();
    fn IStorageItemAccessList();
    fn IStorageItemMostRecentlyUsedList();
    fn IStorageItemMostRecentlyUsedList2();
    fn ItemRemovedEventArgs();
    fn RecentStorageItemVisibility();
    fn StorageApplicationPermissions();
    fn StorageItemAccessList();
    fn StorageItemMostRecentlyUsedList();
}
