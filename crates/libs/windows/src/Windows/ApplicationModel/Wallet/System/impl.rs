#[cfg(feature = "implement_exclusive")]
pub trait IWalletItemSystemStoreImpl: Sized {
    fn GetItemsAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<super::WalletItem>>>;
    fn DeleteAsync(&self, item: &::core::option::Option<super::WalletItem>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ImportItemAsync(&self, stream: &::core::option::Option<super::super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::WalletItem>>;
    fn GetAppStatusForItem(&self, item: &::core::option::Option<super::WalletItem>) -> ::windows::core::Result<WalletItemAppAssociation>;
    fn LaunchAppForItemAsync(&self, item: &::core::option::Option<super::WalletItem>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWalletItemSystemStore2Impl: Sized {
    fn ItemsChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WalletItemSystemStore, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveItemsChanged(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWalletManagerSystemStaticsImpl: Sized {
    fn RequestStoreAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WalletItemSystemStore>>;
}
