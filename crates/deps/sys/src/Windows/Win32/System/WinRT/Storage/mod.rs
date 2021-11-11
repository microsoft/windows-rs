#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn HANDLE_ACCESS_OPTIONS();
    fn HANDLE_CREATION_OPTIONS();
    fn HANDLE_OPTIONS();
    fn HANDLE_SHARING_OPTIONS();
    fn IOplockBreakingHandler();
    fn IRandomAccessStreamFileAccessMode();
    fn IStorageFolderHandleAccess();
    fn IStorageItemHandleAccess();
    fn IUnbufferedFileHandleOplockCallback();
    fn IUnbufferedFileHandleProvider();
}
