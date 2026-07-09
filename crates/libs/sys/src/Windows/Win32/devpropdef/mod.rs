#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEVPROPCOMPKEY {
    pub Key: DEVPROPKEY,
    pub Store: DEVPROPSTORE,
    pub LocaleName: windows_sys::core::PCWSTR,
}
impl Default for DEVPROPCOMPKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEVPROPERTY {
    pub CompKey: DEVPROPCOMPKEY,
    pub Type: DEVPROPTYPE,
    pub BufferSize: u32,
    pub Buffer: *mut core::ffi::c_void,
}
impl Default for DEVPROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DEVPROPGUID = windows_sys::core::GUID;
pub type DEVPROPID = u32;
pub const DEVPROPID_FIRST_USABLE: u32 = 2;
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
pub const DEVPROP_MASK_TYPE: u32 = 4095;
pub const DEVPROP_MASK_TYPEMOD: u32 = 61440;
pub const DEVPROP_STORE_SYSTEM: DEVPROPSTORE = 0;
pub const DEVPROP_STORE_USER: DEVPROPSTORE = 1;
pub const DEVPROP_TRUE: DEVPROP_BOOLEAN = -1;
pub const DEVPROP_TYPEMOD_ARRAY: u32 = 4096;
pub const DEVPROP_TYPEMOD_LIST: u32 = 8192;
pub const DEVPROP_TYPE_BINARY: u32 = 4099;
pub const DEVPROP_TYPE_BOOLEAN: u32 = 17;
pub const DEVPROP_TYPE_BYTE: u32 = 3;
pub const DEVPROP_TYPE_CURRENCY: u32 = 14;
pub const DEVPROP_TYPE_DATE: u32 = 15;
pub const DEVPROP_TYPE_DECIMAL: u32 = 12;
pub const DEVPROP_TYPE_DEVPROPKEY: u32 = 21;
pub const DEVPROP_TYPE_DEVPROPTYPE: u32 = 22;
pub const DEVPROP_TYPE_DOUBLE: u32 = 11;
pub const DEVPROP_TYPE_EMPTY: u32 = 0;
pub const DEVPROP_TYPE_ERROR: u32 = 23;
pub const DEVPROP_TYPE_FILETIME: u32 = 16;
pub const DEVPROP_TYPE_FLOAT: u32 = 10;
pub const DEVPROP_TYPE_GUID: u32 = 13;
pub const DEVPROP_TYPE_INT16: u32 = 4;
pub const DEVPROP_TYPE_INT32: u32 = 6;
pub const DEVPROP_TYPE_INT64: u32 = 8;
pub const DEVPROP_TYPE_NTSTATUS: u32 = 24;
pub const DEVPROP_TYPE_NULL: u32 = 1;
pub const DEVPROP_TYPE_SBYTE: u32 = 2;
pub const DEVPROP_TYPE_SECURITY_DESCRIPTOR: u32 = 19;
pub const DEVPROP_TYPE_SECURITY_DESCRIPTOR_STRING: u32 = 20;
pub const DEVPROP_TYPE_STRING: u32 = 18;
pub const DEVPROP_TYPE_STRING_INDIRECT: u32 = 25;
pub const DEVPROP_TYPE_STRING_LIST: u32 = 8210;
pub const DEVPROP_TYPE_UINT16: u32 = 5;
pub const DEVPROP_TYPE_UINT32: u32 = 7;
pub const DEVPROP_TYPE_UINT64: u32 = 9;
pub const MAX_DEVPROP_TYPE: u32 = 25;
pub const MAX_DEVPROP_TYPEMOD: u32 = 8192;
pub type PDEVPROPCOMPKEY = *mut DEVPROPCOMPKEY;
pub type PDEVPROPERTY = *mut DEVPROPERTY;
pub type PDEVPROPGUID = *mut windows_sys::core::GUID;
pub type PDEVPROPID = *mut u32;
pub type PDEVPROPKEY = *mut DEVPROPKEY;
pub type PDEVPROPSTORE = *mut DEVPROPSTORE;
pub type PDEVPROPTYPE = *mut u32;
pub type PDEVPROP_BOOLEAN = *mut i8;
