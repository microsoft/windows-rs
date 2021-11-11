#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn BasicProperties();
    fn DocumentProperties();
    fn GeotagHelper();
    fn IBasicProperties();
    fn IDocumentProperties();
    fn IGeotagHelperStatics();
    fn IImageProperties();
    fn IMusicProperties();
    fn IStorageItemContentProperties();
    fn IStorageItemExtraProperties();
    fn IThumbnailProperties();
    fn IVideoProperties();
    fn ImageProperties();
    fn MusicProperties();
    fn PhotoOrientation();
    fn PropertyPrefetchOptions();
    fn StorageItemContentProperties();
    fn StorageItemThumbnail();
    fn ThumbnailMode();
    fn ThumbnailOptions();
    fn ThumbnailType();
    fn VideoOrientation();
    fn VideoProperties();
}
