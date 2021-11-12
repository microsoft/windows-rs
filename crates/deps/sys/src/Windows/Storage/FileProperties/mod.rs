#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct BasicProperties(i32);
pub struct DocumentProperties(i32);
pub struct GeotagHelper(i32);
pub struct IBasicProperties(pub *mut ::core::ffi::c_void);
pub struct IDocumentProperties(pub *mut ::core::ffi::c_void);
pub struct IGeotagHelperStatics(pub *mut ::core::ffi::c_void);
pub struct IImageProperties(pub *mut ::core::ffi::c_void);
pub struct IMusicProperties(pub *mut ::core::ffi::c_void);
pub struct IStorageItemContentProperties(pub *mut ::core::ffi::c_void);
pub struct IStorageItemExtraProperties(pub *mut ::core::ffi::c_void);
pub struct IThumbnailProperties(pub *mut ::core::ffi::c_void);
pub struct IVideoProperties(pub *mut ::core::ffi::c_void);
pub struct ImageProperties(i32);
pub struct MusicProperties(i32);
pub struct PhotoOrientation(i32);
pub struct PropertyPrefetchOptions(i32);
pub struct StorageItemContentProperties(i32);
pub struct StorageItemThumbnail(i32);
pub struct ThumbnailMode(i32);
pub struct ThumbnailOptions(i32);
pub struct ThumbnailType(i32);
pub struct VideoOrientation(i32);
pub struct VideoProperties(i32);
