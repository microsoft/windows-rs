#[cfg(feature = "implement_exclusive")]
pub trait IBasicPropertiesImpl: Sized {
    fn Size(&self) -> ::windows::core::Result<u64>;
    fn DateModified(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn ItemDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDocumentPropertiesImpl: Sized + IStorageItemExtraPropertiesImpl {
    fn Author(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Keywords(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeotagHelperStaticsImpl: Sized {
    fn GetGeotagAsync(&self, file: &::core::option::Option<super::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Devices::Geolocation::Geopoint>>;
    fn SetGeotagFromGeolocatorAsync(&self, file: &::core::option::Option<super::IStorageFile>, geolocator: &::core::option::Option<super::super::Devices::Geolocation::Geolocator>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetGeotagAsync(&self, file: &::core::option::Option<super::IStorageFile>, geopoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IImagePropertiesImpl: Sized + IStorageItemExtraPropertiesImpl {
    fn Rating(&self) -> ::windows::core::Result<u32>;
    fn SetRating(&self, value: u32) -> ::windows::core::Result<()>;
    fn Keywords(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn DateTaken(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetDateTaken(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn Width(&self) -> ::windows::core::Result<u32>;
    fn Height(&self) -> ::windows::core::Result<u32>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Latitude(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn Longitude(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn CameraManufacturer(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCameraManufacturer(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CameraModel(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCameraModel(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Orientation(&self) -> ::windows::core::Result<PhotoOrientation>;
    fn PeopleNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMusicPropertiesImpl: Sized + IStorageItemExtraPropertiesImpl {
    fn Album(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAlbum(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Artist(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetArtist(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Genre(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn TrackNumber(&self) -> ::windows::core::Result<u32>;
    fn SetTrackNumber(&self, value: u32) -> ::windows::core::Result<()>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Rating(&self) -> ::windows::core::Result<u32>;
    fn SetRating(&self, value: u32) -> ::windows::core::Result<()>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Bitrate(&self) -> ::windows::core::Result<u32>;
    fn AlbumArtist(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAlbumArtist(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Composers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn Conductors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn Subtitle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSubtitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Producers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn Publisher(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPublisher(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Writers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn Year(&self) -> ::windows::core::Result<u32>;
    fn SetYear(&self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageItemContentPropertiesImpl: Sized + IStorageItemExtraPropertiesImpl {
    fn GetMusicPropertiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MusicProperties>>;
    fn GetVideoPropertiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VideoProperties>>;
    fn GetImagePropertiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ImageProperties>>;
    fn GetDocumentPropertiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DocumentProperties>>;
}
pub trait IStorageItemExtraPropertiesImpl: Sized {
    fn RetrievePropertiesAsync(&self, propertiestoretrieve: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>>;
    fn SavePropertiesAsync(&self, propertiestosave: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SavePropertiesAsyncOverloadDefault(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IThumbnailPropertiesImpl: Sized {
    fn OriginalWidth(&self) -> ::windows::core::Result<u32>;
    fn OriginalHeight(&self) -> ::windows::core::Result<u32>;
    fn ReturnedSmallerCachedSize(&self) -> ::windows::core::Result<bool>;
    fn Type(&self) -> ::windows::core::Result<ThumbnailType>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoPropertiesImpl: Sized + IStorageItemExtraPropertiesImpl {
    fn Rating(&self) -> ::windows::core::Result<u32>;
    fn SetRating(&self, value: u32) -> ::windows::core::Result<()>;
    fn Keywords(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn Width(&self) -> ::windows::core::Result<u32>;
    fn Height(&self) -> ::windows::core::Result<u32>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Latitude(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn Longitude(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Subtitle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSubtitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Producers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn Publisher(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPublisher(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Writers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn Year(&self) -> ::windows::core::Result<u32>;
    fn SetYear(&self, value: u32) -> ::windows::core::Result<()>;
    fn Bitrate(&self) -> ::windows::core::Result<u32>;
    fn Directors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn Orientation(&self) -> ::windows::core::Result<VideoOrientation>;
}
