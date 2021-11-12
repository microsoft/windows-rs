#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct HANDLE_ACCESS_OPTIONS(i32);
#[repr(C)]
pub struct HANDLE_CREATION_OPTIONS(i32);
#[repr(C)]
pub struct HANDLE_OPTIONS(i32);
#[repr(C)]
pub struct HANDLE_SHARING_OPTIONS(i32);
#[repr(transparent)]
pub struct IOplockBreakingHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRandomAccessStreamFileAccessMode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageFolderHandleAccess(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageItemHandleAccess(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUnbufferedFileHandleOplockCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUnbufferedFileHandleProvider(pub *mut ::core::ffi::c_void);
