#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct BasicProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DocumentProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBasicProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDocumentProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeotagHelperStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImageProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMusicProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageItemContentProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageItemExtraProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IThumbnailProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ImageProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MusicProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhotoOrientation(pub i32);
impl PhotoOrientation {
    pub const Unspecified: PhotoOrientation = PhotoOrientation(0i32);
    pub const Normal: PhotoOrientation = PhotoOrientation(1i32);
    pub const FlipHorizontal: PhotoOrientation = PhotoOrientation(2i32);
    pub const Rotate180: PhotoOrientation = PhotoOrientation(3i32);
    pub const FlipVertical: PhotoOrientation = PhotoOrientation(4i32);
    pub const Transpose: PhotoOrientation = PhotoOrientation(5i32);
    pub const Rotate270: PhotoOrientation = PhotoOrientation(6i32);
    pub const Transverse: PhotoOrientation = PhotoOrientation(7i32);
    pub const Rotate90: PhotoOrientation = PhotoOrientation(8i32);
}
#[repr(transparent)]
pub struct PropertyPrefetchOptions(pub u32);
impl PropertyPrefetchOptions {
    pub const None: PropertyPrefetchOptions = PropertyPrefetchOptions(0u32);
    pub const MusicProperties: PropertyPrefetchOptions = PropertyPrefetchOptions(1u32);
    pub const VideoProperties: PropertyPrefetchOptions = PropertyPrefetchOptions(2u32);
    pub const ImageProperties: PropertyPrefetchOptions = PropertyPrefetchOptions(4u32);
    pub const DocumentProperties: PropertyPrefetchOptions = PropertyPrefetchOptions(8u32);
    pub const BasicProperties: PropertyPrefetchOptions = PropertyPrefetchOptions(16u32);
}
#[repr(transparent)]
pub struct StorageItemContentProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageItemThumbnail(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ThumbnailMode(pub i32);
impl ThumbnailMode {
    pub const PicturesView: ThumbnailMode = ThumbnailMode(0i32);
    pub const VideosView: ThumbnailMode = ThumbnailMode(1i32);
    pub const MusicView: ThumbnailMode = ThumbnailMode(2i32);
    pub const DocumentsView: ThumbnailMode = ThumbnailMode(3i32);
    pub const ListView: ThumbnailMode = ThumbnailMode(4i32);
    pub const SingleItem: ThumbnailMode = ThumbnailMode(5i32);
}
#[repr(transparent)]
pub struct ThumbnailOptions(pub u32);
impl ThumbnailOptions {
    pub const None: ThumbnailOptions = ThumbnailOptions(0u32);
    pub const ReturnOnlyIfCached: ThumbnailOptions = ThumbnailOptions(1u32);
    pub const ResizeThumbnail: ThumbnailOptions = ThumbnailOptions(2u32);
    pub const UseCurrentScale: ThumbnailOptions = ThumbnailOptions(4u32);
}
#[repr(transparent)]
pub struct ThumbnailType(pub i32);
impl ThumbnailType {
    pub const Image: ThumbnailType = ThumbnailType(0i32);
    pub const Icon: ThumbnailType = ThumbnailType(1i32);
}
#[repr(transparent)]
pub struct VideoOrientation(pub i32);
impl VideoOrientation {
    pub const Normal: VideoOrientation = VideoOrientation(0i32);
    pub const Rotate90: VideoOrientation = VideoOrientation(90i32);
    pub const Rotate180: VideoOrientation = VideoOrientation(180i32);
    pub const Rotate270: VideoOrientation = VideoOrientation(270i32);
}
#[repr(transparent)]
pub struct VideoProperties(pub *mut ::core::ffi::c_void);
