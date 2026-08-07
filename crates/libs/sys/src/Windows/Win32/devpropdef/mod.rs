#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEVPROPCOMPKEY {
    pub Key: DEVPROPKEY,
    pub Store: DEVPROPSTORE,
    pub LocaleName: windows_sys::core::PCWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEVPROPERTY {
    pub CompKey: DEVPROPCOMPKEY,
    pub Type: DEVPROPTYPE,
    pub BufferSize: u32,
    pub Buffer: *mut core::ffi::c_void,
}
pub type DEVPROPGUID = windows_sys::core::GUID;
pub type DEVPROPID = u32;
pub const DEVPROPID_FIRST_USABLE: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEVPROPKEY {
    pub fmtid: DEVPROPGUID,
    pub pid: DEVPROPID,
}
pub type DEVPROPSTORE = i32;
pub type DEVPROPTYPE = u32;
pub type DEVPROP_BOOLEAN = i8;
pub const DEVPROP_FALSE: DEVPROP_BOOLEAN = 0;
pub const DEVPROP_MASK_TYPE: i32 = 4095;
pub const DEVPROP_MASK_TYPEMOD: i32 = 61440;
pub const DEVPROP_STORE_SYSTEM: DEVPROPSTORE = 0;
pub const DEVPROP_STORE_USER: DEVPROPSTORE = 1;
pub const DEVPROP_TRUE: DEVPROP_BOOLEAN = -1;
pub const DEVPROP_TYPEMOD_ARRAY: i32 = 4096;
pub const DEVPROP_TYPEMOD_LIST: i32 = 8192;
pub const DEVPROP_TYPE_BINARY: i32 = 4099;
pub const DEVPROP_TYPE_BOOLEAN: i32 = 17;
pub const DEVPROP_TYPE_BYTE: i32 = 3;
pub const DEVPROP_TYPE_CURRENCY: i32 = 14;
pub const DEVPROP_TYPE_DATE: i32 = 15;
pub const DEVPROP_TYPE_DECIMAL: i32 = 12;
pub const DEVPROP_TYPE_DEVPROPKEY: i32 = 21;
pub const DEVPROP_TYPE_DEVPROPTYPE: i32 = 22;
pub const DEVPROP_TYPE_DOUBLE: i32 = 11;
pub const DEVPROP_TYPE_EMPTY: i32 = 0;
pub const DEVPROP_TYPE_ERROR: i32 = 23;
pub const DEVPROP_TYPE_FILETIME: i32 = 16;
pub const DEVPROP_TYPE_FLOAT: i32 = 10;
pub const DEVPROP_TYPE_GUID: i32 = 13;
pub const DEVPROP_TYPE_INT16: i32 = 4;
pub const DEVPROP_TYPE_INT32: i32 = 6;
pub const DEVPROP_TYPE_INT64: i32 = 8;
pub const DEVPROP_TYPE_NTSTATUS: i32 = 24;
pub const DEVPROP_TYPE_NULL: i32 = 1;
pub const DEVPROP_TYPE_SBYTE: i32 = 2;
pub const DEVPROP_TYPE_SECURITY_DESCRIPTOR: i32 = 19;
pub const DEVPROP_TYPE_SECURITY_DESCRIPTOR_STRING: i32 = 20;
pub const DEVPROP_TYPE_STRING: i32 = 18;
pub const DEVPROP_TYPE_STRING_INDIRECT: i32 = 25;
pub const DEVPROP_TYPE_STRING_LIST: i32 = 8210;
pub const DEVPROP_TYPE_UINT16: i32 = 5;
pub const DEVPROP_TYPE_UINT32: i32 = 7;
pub const DEVPROP_TYPE_UINT64: i32 = 9;
pub const MAX_DEVPROP_TYPE: i32 = 25;
pub const MAX_DEVPROP_TYPEMOD: i32 = 8192;
pub type PDEVPROPCOMPKEY = *mut DEVPROPCOMPKEY;
pub type PDEVPROPERTY = *mut DEVPROPERTY;
pub type PDEVPROPGUID = *mut windows_sys::core::GUID;
pub type PDEVPROPID = *mut u32;
pub type PDEVPROPKEY = *mut DEVPROPKEY;
pub type PDEVPROPSTORE = *mut DEVPROPSTORE;
pub type PDEVPROPTYPE = *mut u32;
pub type PDEVPROP_BOOLEAN = *mut i8;
