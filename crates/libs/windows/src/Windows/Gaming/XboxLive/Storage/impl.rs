#[cfg(feature = "implement_exclusive")]
pub trait IGameSaveBlobGetResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<GameSaveErrorStatus>;
    fn Value(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::super::Storage::Streams::IBuffer>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameSaveBlobInfoImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Size(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameSaveBlobInfoGetResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<GameSaveErrorStatus>;
    fn Value(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GameSaveBlobInfo>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameSaveBlobInfoQueryImpl: Sized {
    fn GetBlobInfoAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveBlobInfoGetResult>>;
    fn GetBlobInfoWithIndexAndMaxAsync(&self, startindex: u32, maxnumberofitems: u32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveBlobInfoGetResult>>;
    fn GetItemCountAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<u32>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameSaveContainerImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Provider(&self) -> ::windows::core::Result<GameSaveProvider>;
    fn SubmitUpdatesAsync(&self, blobstowrite: &::core::option::Option<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::super::Storage::Streams::IBuffer>>, blobstodelete: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>>;
    fn ReadAsync(&self, blobstoread: &::core::option::Option<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::super::Storage::Streams::IBuffer>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>>;
    fn GetAsync(&self, blobstoread: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveBlobGetResult>>;
    fn SubmitPropertySetUpdatesAsync(&self, blobstowrite: &::core::option::Option<super::super::super::Foundation::Collections::IPropertySet>, blobstodelete: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>>;
    fn CreateBlobInfoQuery(&self, blobnameprefix: &::windows::core::HSTRING) -> ::windows::core::Result<GameSaveBlobInfoQuery>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameSaveContainerInfoImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TotalSize(&self) -> ::windows::core::Result<u64>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LastModifiedTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn NeedsSync(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameSaveContainerInfoGetResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<GameSaveErrorStatus>;
    fn Value(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GameSaveContainerInfo>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameSaveContainerInfoQueryImpl: Sized {
    fn GetContainerInfoAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveContainerInfoGetResult>>;
    fn GetContainerInfoWithIndexAndMaxAsync(&self, startindex: u32, maxnumberofitems: u32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveContainerInfoGetResult>>;
    fn GetItemCountAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<u32>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameSaveOperationResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<GameSaveErrorStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameSaveProviderImpl: Sized {
    fn User(&self) -> ::windows::core::Result<super::super::super::System::User>;
    fn CreateContainer(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<GameSaveContainer>;
    fn DeleteContainerAsync(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>>;
    fn CreateContainerInfoQuery(&self) -> ::windows::core::Result<GameSaveContainerInfoQuery>;
    fn CreateContainerInfoQueryWithName(&self, containernameprefix: &::windows::core::HSTRING) -> ::windows::core::Result<GameSaveContainerInfoQuery>;
    fn GetRemainingBytesInQuotaAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<i64>>;
    fn ContainersChangedSinceLastSync(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameSaveProviderGetResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<GameSaveErrorStatus>;
    fn Value(&self) -> ::windows::core::Result<GameSaveProvider>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameSaveProviderStaticsImpl: Sized {
    fn GetForUserAsync(&self, user: &::core::option::Option<super::super::super::System::User>, serviceconfigid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveProviderGetResult>>;
    fn GetSyncOnDemandForUserAsync(&self, user: &::core::option::Option<super::super::super::System::User>, serviceconfigid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveProviderGetResult>>;
}
