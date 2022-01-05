#[cfg(feature = "implement_exclusive")]
pub trait IItemRemovedEventArgsImpl: Sized {
    fn RemovedEntry(&self) -> ::windows::core::Result<AccessListEntry>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageApplicationPermissionsStaticsImpl: Sized {
    fn FutureAccessList(&self) -> ::windows::core::Result<StorageItemAccessList>;
    fn MostRecentlyUsedList(&self) -> ::windows::core::Result<StorageItemMostRecentlyUsedList>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageApplicationPermissionsStatics2Impl: Sized {
    fn GetFutureAccessListForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<StorageItemAccessList>;
    fn GetMostRecentlyUsedListForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<StorageItemMostRecentlyUsedList>;
}
pub trait IStorageItemAccessListImpl: Sized {
    fn AddOverloadDefaultMetadata(&self, file: &::core::option::Option<super::IStorageItem>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Add(&self, file: &::core::option::Option<super::IStorageItem>, metadata: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AddOrReplaceOverloadDefaultMetadata(&self, token: &::windows::core::HSTRING, file: &::core::option::Option<super::IStorageItem>) -> ::windows::core::Result<()>;
    fn AddOrReplace(&self, token: &::windows::core::HSTRING, file: &::core::option::Option<super::IStorageItem>, metadata: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetItemAsync(&self, token: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>>;
    fn GetFileAsync(&self, token: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>>;
    fn GetFolderAsync(&self, token: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>>;
    fn GetItemWithOptionsAsync(&self, token: &::windows::core::HSTRING, options: AccessCacheOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>>;
    fn GetFileWithOptionsAsync(&self, token: &::windows::core::HSTRING, options: AccessCacheOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>>;
    fn GetFolderWithOptionsAsync(&self, token: &::windows::core::HSTRING, options: AccessCacheOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>>;
    fn Remove(&self, token: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ContainsItem(&self, token: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn CheckAccess(&self, file: &::core::option::Option<super::IStorageItem>) -> ::windows::core::Result<bool>;
    fn Entries(&self) -> ::windows::core::Result<AccessListEntryView>;
    fn MaximumItemsAllowed(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageItemMostRecentlyUsedListImpl: Sized + IStorageItemAccessListImpl {
    fn ItemRemoved(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<StorageItemMostRecentlyUsedList, ItemRemovedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveItemRemoved(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageItemMostRecentlyUsedList2Impl: Sized + IStorageItemAccessListImpl + IStorageItemMostRecentlyUsedListImpl {
    fn AddWithMetadataAndVisibility(&self, file: &::core::option::Option<super::IStorageItem>, metadata: &::windows::core::HSTRING, visibility: RecentStorageItemVisibility) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AddOrReplaceWithMetadataAndVisibility(&self, token: &::windows::core::HSTRING, file: &::core::option::Option<super::IStorageItem>, metadata: &::windows::core::HSTRING, visibility: RecentStorageItemVisibility) -> ::windows::core::Result<()>;
}
