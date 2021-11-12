#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct HANDLE_ACCESS_OPTIONS(i32);
pub struct HANDLE_CREATION_OPTIONS(i32);
pub struct HANDLE_OPTIONS(i32);
pub struct HANDLE_SHARING_OPTIONS(i32);
pub struct IOplockBreakingHandler(pub *mut ::core::ffi::c_void);
pub struct IRandomAccessStreamFileAccessMode(pub *mut ::core::ffi::c_void);
pub struct IStorageFolderHandleAccess(pub *mut ::core::ffi::c_void);
pub struct IStorageItemHandleAccess(pub *mut ::core::ffi::c_void);
pub struct IUnbufferedFileHandleOplockCallback(pub *mut ::core::ffi::c_void);
pub struct IUnbufferedFileHandleProvider(pub *mut ::core::ffi::c_void);
