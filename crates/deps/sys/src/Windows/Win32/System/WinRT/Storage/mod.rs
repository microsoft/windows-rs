#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
pub const HAO_NONE: u32 = 0u32;
pub const HAO_READ_ATTRIBUTES: u32 = 128u32;
pub const HAO_READ: u32 = 1179785u32;
pub const HAO_WRITE: u32 = 1179926u32;
pub const HAO_DELETE: u32 = 65536u32;
pub const HCO_CREATE_NEW: i32 = 1i32;
pub const HCO_CREATE_ALWAYS: i32 = 2i32;
pub const HCO_OPEN_EXISTING: i32 = 3i32;
pub const HCO_OPEN_ALWAYS: i32 = 4i32;
pub const HCO_TRUNCATE_EXISTING: i32 = 5i32;
pub const HO_NONE: u32 = 0u32;
pub const HO_OPEN_REQUIRING_OPLOCK: u32 = 262144u32;
pub const HO_DELETE_ON_CLOSE: u32 = 67108864u32;
pub const HO_SEQUENTIAL_SCAN: u32 = 134217728u32;
pub const HO_RANDOM_ACCESS: u32 = 268435456u32;
pub const HO_NO_BUFFERING: u32 = 536870912u32;
pub const HO_OVERLAPPED: u32 = 1073741824u32;
pub const HO_WRITE_THROUGH: u32 = 2147483648u32;
pub const HSO_SHARE_NONE: u32 = 0u32;
pub const HSO_SHARE_READ: u32 = 1u32;
pub const HSO_SHARE_WRITE: u32 = 2u32;
pub const HSO_SHARE_DELETE: u32 = 4u32;
#[repr(transparent)]
pub struct IOplockBreakingHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOplockBreakingHandler {}
impl ::core::clone::Clone for IOplockBreakingHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRandomAccessStreamFileAccessMode(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRandomAccessStreamFileAccessMode {}
impl ::core::clone::Clone for IRandomAccessStreamFileAccessMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageFolderHandleAccess(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageFolderHandleAccess {}
impl ::core::clone::Clone for IStorageFolderHandleAccess {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageItemHandleAccess(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageItemHandleAccess {}
impl ::core::clone::Clone for IStorageItemHandleAccess {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUnbufferedFileHandleOplockCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUnbufferedFileHandleOplockCallback {}
impl ::core::clone::Clone for IUnbufferedFileHandleOplockCallback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUnbufferedFileHandleProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUnbufferedFileHandleProvider {}
impl ::core::clone::Clone for IUnbufferedFileHandleProvider {
    fn clone(&self) -> Self {
        *self
    }
}
