#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct HANDLE_ACCESS_OPTIONS(i32);
pub struct HANDLE_CREATION_OPTIONS(i32);
pub struct HANDLE_OPTIONS(i32);
pub struct HANDLE_SHARING_OPTIONS(i32);
pub struct IOplockBreakingHandler(i32);
pub struct IRandomAccessStreamFileAccessMode(i32);
pub struct IStorageFolderHandleAccess(i32);
pub struct IStorageItemHandleAccess(i32);
pub struct IUnbufferedFileHandleOplockCallback(i32);
pub struct IUnbufferedFileHandleProvider(i32);
