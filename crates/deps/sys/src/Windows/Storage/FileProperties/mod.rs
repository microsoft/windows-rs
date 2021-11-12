#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct PhotoOrientation(i32);
#[repr(C)]
pub struct PropertyPrefetchOptions(i32);
#[repr(transparent)]
pub struct StorageItemContentProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageItemThumbnail(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ThumbnailMode(i32);
#[repr(C)]
pub struct ThumbnailOptions(i32);
#[repr(C)]
pub struct ThumbnailType(i32);
#[repr(C)]
pub struct VideoOrientation(i32);
#[repr(transparent)]
pub struct VideoProperties(pub *mut ::core::ffi::c_void);
