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
    pub const Unspecified: Self = Self(0i32);
    pub const Normal: Self = Self(1i32);
    pub const FlipHorizontal: Self = Self(2i32);
    pub const Rotate180: Self = Self(3i32);
    pub const FlipVertical: Self = Self(4i32);
    pub const Transpose: Self = Self(5i32);
    pub const Rotate270: Self = Self(6i32);
    pub const Transverse: Self = Self(7i32);
    pub const Rotate90: Self = Self(8i32);
}
#[repr(transparent)]
pub struct PropertyPrefetchOptions(pub u32);
impl PropertyPrefetchOptions {
    pub const None: Self = Self(0u32);
    pub const MusicProperties: Self = Self(1u32);
    pub const VideoProperties: Self = Self(2u32);
    pub const ImageProperties: Self = Self(4u32);
    pub const DocumentProperties: Self = Self(8u32);
    pub const BasicProperties: Self = Self(16u32);
}
#[repr(transparent)]
pub struct StorageItemContentProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageItemThumbnail(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ThumbnailMode(pub i32);
impl ThumbnailMode {
    pub const PicturesView: Self = Self(0i32);
    pub const VideosView: Self = Self(1i32);
    pub const MusicView: Self = Self(2i32);
    pub const DocumentsView: Self = Self(3i32);
    pub const ListView: Self = Self(4i32);
    pub const SingleItem: Self = Self(5i32);
}
#[repr(transparent)]
pub struct ThumbnailOptions(pub u32);
impl ThumbnailOptions {
    pub const None: Self = Self(0u32);
    pub const ReturnOnlyIfCached: Self = Self(1u32);
    pub const ResizeThumbnail: Self = Self(2u32);
    pub const UseCurrentScale: Self = Self(4u32);
}
#[repr(transparent)]
pub struct ThumbnailType(pub i32);
impl ThumbnailType {
    pub const Image: Self = Self(0i32);
    pub const Icon: Self = Self(1i32);
}
#[repr(transparent)]
pub struct VideoOrientation(pub i32);
impl VideoOrientation {
    pub const Normal: Self = Self(0i32);
    pub const Rotate90: Self = Self(90i32);
    pub const Rotate180: Self = Self(180i32);
    pub const Rotate270: Self = Self(270i32);
}
#[repr(transparent)]
pub struct VideoProperties(pub *mut ::core::ffi::c_void);
