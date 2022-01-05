#[cfg(feature = "implement_exclusive")]
pub trait IPlaylistImpl: Sized {
    fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Storage::StorageFile>>;
    fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SaveAsAsync(&self, savelocation: &::core::option::Option<super::super::Storage::IStorageFolder>, desiredname: &::windows::core::HSTRING, option: super::super::Storage::NameCollisionOption) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFile>>;
    fn SaveAsWithFormatAsync(&self, savelocation: &::core::option::Option<super::super::Storage::IStorageFolder>, desiredname: &::windows::core::HSTRING, option: super::super::Storage::NameCollisionOption, playlistformat: PlaylistFormat) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFile>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlaylistStaticsImpl: Sized {
    fn LoadAsync(&self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Playlist>>;
}
