#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct BasicProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BasicProperties {}
impl ::core::clone::Clone for BasicProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DocumentProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DocumentProperties {}
impl ::core::clone::Clone for DocumentProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBasicProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBasicProperties {}
impl ::core::clone::Clone for IBasicProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDocumentProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDocumentProperties {}
impl ::core::clone::Clone for IDocumentProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeotagHelperStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeotagHelperStatics {}
impl ::core::clone::Clone for IGeotagHelperStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IImageProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IImageProperties {}
impl ::core::clone::Clone for IImageProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMusicProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMusicProperties {}
impl ::core::clone::Clone for IMusicProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageItemContentProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageItemContentProperties {}
impl ::core::clone::Clone for IStorageItemContentProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageItemExtraProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageItemExtraProperties {}
impl ::core::clone::Clone for IStorageItemExtraProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IThumbnailProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IThumbnailProperties {}
impl ::core::clone::Clone for IThumbnailProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoProperties {}
impl ::core::clone::Clone for IVideoProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ImageProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ImageProperties {}
impl ::core::clone::Clone for ImageProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MusicProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MusicProperties {}
impl ::core::clone::Clone for MusicProperties {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for PhotoOrientation {}
impl ::core::clone::Clone for PhotoOrientation {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for PropertyPrefetchOptions {}
impl ::core::clone::Clone for PropertyPrefetchOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageItemContentProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StorageItemContentProperties {}
impl ::core::clone::Clone for StorageItemContentProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageItemThumbnail(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StorageItemThumbnail {}
impl ::core::clone::Clone for StorageItemThumbnail {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for ThumbnailMode {}
impl ::core::clone::Clone for ThumbnailMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ThumbnailOptions(pub u32);
impl ThumbnailOptions {
    pub const None: Self = Self(0u32);
    pub const ReturnOnlyIfCached: Self = Self(1u32);
    pub const ResizeThumbnail: Self = Self(2u32);
    pub const UseCurrentScale: Self = Self(4u32);
}
impl ::core::marker::Copy for ThumbnailOptions {}
impl ::core::clone::Clone for ThumbnailOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ThumbnailType(pub i32);
impl ThumbnailType {
    pub const Image: Self = Self(0i32);
    pub const Icon: Self = Self(1i32);
}
impl ::core::marker::Copy for ThumbnailType {}
impl ::core::clone::Clone for ThumbnailType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VideoOrientation(pub i32);
impl VideoOrientation {
    pub const Normal: Self = Self(0i32);
    pub const Rotate90: Self = Self(90i32);
    pub const Rotate180: Self = Self(180i32);
    pub const Rotate270: Self = Self(270i32);
}
impl ::core::marker::Copy for VideoOrientation {}
impl ::core::clone::Clone for VideoOrientation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VideoProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VideoProperties {}
impl ::core::clone::Clone for VideoProperties {
    fn clone(&self) -> Self {
        *self
    }
}
