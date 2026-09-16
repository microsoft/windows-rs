windows_link::link!("oleaut32.dll" "system" fn BSTR_UserFree(param0 : *mut u32, param1 : *mut windows_sys::core::BSTR));
windows_link::link!("oleaut32.dll" "system" fn BSTR_UserFree64(param0 : *mut u32, param1 : *mut windows_sys::core::BSTR));
windows_link::link!("oleaut32.dll" "system" fn BSTR_UserMarshal(param0 : *mut u32, param1 : *mut u8, param2 : *mut windows_sys::core::BSTR) -> *mut u8);
windows_link::link!("oleaut32.dll" "system" fn BSTR_UserMarshal64(param0 : *mut u32, param1 : *mut u8, param2 : *mut windows_sys::core::BSTR) -> *mut u8);
windows_link::link!("oleaut32.dll" "system" fn BSTR_UserSize(param0 : *mut u32, param1 : u32, param2 : *mut windows_sys::core::BSTR) -> u32);
windows_link::link!("oleaut32.dll" "system" fn BSTR_UserSize64(param0 : *mut u32, param1 : u32, param2 : *mut windows_sys::core::BSTR) -> u32);
windows_link::link!("oleaut32.dll" "system" fn BSTR_UserUnmarshal(param0 : *mut u32, param1 : *mut u8, param2 : *mut windows_sys::core::BSTR) -> *mut u8);
windows_link::link!("oleaut32.dll" "system" fn BSTR_UserUnmarshal64(param0 : *mut u32, param1 : *mut u8, param2 : *mut windows_sys::core::BSTR) -> *mut u8);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VARIANT_UserFree(param0 : *mut u32, param1 : *mut super::VARIANT));
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VARIANT_UserFree64(param0 : *mut u32, param1 : *mut super::VARIANT));
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VARIANT_UserMarshal(param0 : *mut u32, param1 : *mut u8, param2 : *mut super::VARIANT) -> *mut u8);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VARIANT_UserMarshal64(param0 : *mut u32, param1 : *mut u8, param2 : *mut super::VARIANT) -> *mut u8);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VARIANT_UserSize(param0 : *mut u32, param1 : u32, param2 : *mut super::VARIANT) -> u32);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VARIANT_UserSize64(param0 : *mut u32, param1 : u32, param2 : *mut super::VARIANT) -> u32);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VARIANT_UserUnmarshal(param0 : *mut u32, param1 : *mut u8, param2 : *mut super::VARIANT) -> *mut u8);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VARIANT_UserUnmarshal64(param0 : *mut u32, param1 : *mut u8, param2 : *mut super::VARIANT) -> *mut u8);
pub const DBGPROP_ATTRIB_ACCESS_FINAL: DBGPROP_ATTRIB_FLAGS = 32768;
pub const DBGPROP_ATTRIB_ACCESS_PRIVATE: DBGPROP_ATTRIB_FLAGS = 8192;
pub const DBGPROP_ATTRIB_ACCESS_PROTECTED: DBGPROP_ATTRIB_FLAGS = 16384;
pub const DBGPROP_ATTRIB_ACCESS_PUBLIC: DBGPROP_ATTRIB_FLAGS = 4096;
pub type DBGPROP_ATTRIB_FLAGS = u32;
pub const DBGPROP_ATTRIB_FRAME_INCATCHBLOCK: DBGPROP_ATTRIB_FLAGS = 33554432;
pub const DBGPROP_ATTRIB_FRAME_INFINALLYBLOCK: DBGPROP_ATTRIB_FLAGS = 67108864;
pub const DBGPROP_ATTRIB_FRAME_INTRYBLOCK: DBGPROP_ATTRIB_FLAGS = 16777216;
pub const DBGPROP_ATTRIB_HAS_EXTENDED_ATTRIBS: DBGPROP_ATTRIB_FLAGS = 8388608;
pub const DBGPROP_ATTRIB_NO_ATTRIB: DBGPROP_ATTRIB_FLAGS = 0;
pub const DBGPROP_ATTRIB_STORAGE_FIELD: DBGPROP_ATTRIB_FLAGS = 262144;
pub const DBGPROP_ATTRIB_STORAGE_GLOBAL: DBGPROP_ATTRIB_FLAGS = 65536;
pub const DBGPROP_ATTRIB_STORAGE_STATIC: DBGPROP_ATTRIB_FLAGS = 131072;
pub const DBGPROP_ATTRIB_STORAGE_VIRTUAL: DBGPROP_ATTRIB_FLAGS = 524288;
pub const DBGPROP_ATTRIB_TYPE_IS_CONSTANT: DBGPROP_ATTRIB_FLAGS = 1048576;
pub const DBGPROP_ATTRIB_TYPE_IS_SYNCHRONIZED: DBGPROP_ATTRIB_FLAGS = 2097152;
pub const DBGPROP_ATTRIB_TYPE_IS_VOLATILE: DBGPROP_ATTRIB_FLAGS = 4194304;
pub const DBGPROP_ATTRIB_VALUE_IS_EVENT: DBGPROP_ATTRIB_FLAGS = 512;
pub const DBGPROP_ATTRIB_VALUE_IS_EXPANDABLE: DBGPROP_ATTRIB_FLAGS = 16;
pub const DBGPROP_ATTRIB_VALUE_IS_FAKE: DBGPROP_ATTRIB_FLAGS = 32;
pub const DBGPROP_ATTRIB_VALUE_IS_INVALID: DBGPROP_ATTRIB_FLAGS = 8;
pub const DBGPROP_ATTRIB_VALUE_IS_METHOD: DBGPROP_ATTRIB_FLAGS = 256;
pub const DBGPROP_ATTRIB_VALUE_IS_RAW_STRING: DBGPROP_ATTRIB_FLAGS = 1024;
pub const DBGPROP_ATTRIB_VALUE_IS_RETURN_VALUE: DBGPROP_ATTRIB_FLAGS = 134217728;
pub const DBGPROP_ATTRIB_VALUE_PENDING_MUTATION: DBGPROP_ATTRIB_FLAGS = 268435456;
pub const DBGPROP_ATTRIB_VALUE_READONLY: DBGPROP_ATTRIB_FLAGS = 2048;
pub const DBGPROP_INFO_ALL: i32 = 63;
pub const DBGPROP_INFO_ATTRIBUTES: DBGPROP_INFO_FLAGS = 8;
pub const DBGPROP_INFO_AUTOEXPAND: DBGPROP_INFO_FLAGS = 134217728;
pub const DBGPROP_INFO_BEAUTIFY: DBGPROP_INFO_FLAGS = 33554432;
pub const DBGPROP_INFO_CALLTOSTRING: DBGPROP_INFO_FLAGS = 67108864;
pub const DBGPROP_INFO_DEBUGPROP: DBGPROP_INFO_FLAGS = 16;
pub type DBGPROP_INFO_FLAGS = u32;
pub const DBGPROP_INFO_FULLNAME: DBGPROP_INFO_FLAGS = 32;
pub const DBGPROP_INFO_NAME: DBGPROP_INFO_FLAGS = 1;
pub const DBGPROP_INFO_STANDARD: i32 = 15;
pub const DBGPROP_INFO_TYPE: DBGPROP_INFO_FLAGS = 2;
pub const DBGPROP_INFO_VALUE: DBGPROP_INFO_FLAGS = 4;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DebugPropertyInfo {
    pub m_dwValidFields: u32,
    pub m_bstrName: windows_sys::core::BSTR,
    pub m_bstrType: windows_sys::core::BSTR,
    pub m_bstrValue: windows_sys::core::BSTR,
    pub m_bstrFullName: windows_sys::core::BSTR,
    pub m_dwAttrib: u32,
    pub m_pDebugProp: *mut core::ffi::c_void,
}
pub const EX_PROP_INFO_DEBUGEXTPROP: EX_PROP_INFO_FLAGS = 4096;
pub type EX_PROP_INFO_FLAGS = i32;
pub const EX_PROP_INFO_ID: EX_PROP_INFO_FLAGS = 256;
pub const EX_PROP_INFO_LOCKBYTES: EX_PROP_INFO_FLAGS = 2048;
pub const EX_PROP_INFO_NTYPE: EX_PROP_INFO_FLAGS = 512;
pub const EX_PROP_INFO_NVALUE: EX_PROP_INFO_FLAGS = 1024;
#[repr(C)]
#[cfg(all(feature = "oaidl", feature = "objidl", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct ExtendedDebugPropertyInfo {
    pub dwValidFields: u32,
    pub pszName: super::LPOLESTR,
    pub pszType: super::LPOLESTR,
    pub pszValue: super::LPOLESTR,
    pub pszFullName: super::LPOLESTR,
    pub dwAttrib: u32,
    pub pDebugProp: *mut core::ffi::c_void,
    pub nDISPID: u32,
    pub nType: u32,
    pub varValue: super::VARIANT,
    pub plbValue: *mut core::ffi::c_void,
    pub pDebugExtProp: *mut core::ffi::c_void,
}
#[cfg(all(feature = "oaidl", feature = "objidl", feature = "wtypes", feature = "wtypesbase"))]
impl Default for ExtendedDebugPropertyInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OBJECT_ATTRIB_ACCESS_FINAL: OBJECT_ATTRIB_FLAGS = 32768;
pub const OBJECT_ATTRIB_ACCESS_PRIVATE: OBJECT_ATTRIB_FLAGS = 8192;
pub const OBJECT_ATTRIB_ACCESS_PROTECTED: OBJECT_ATTRIB_FLAGS = 16384;
pub const OBJECT_ATTRIB_ACCESS_PUBLIC: OBJECT_ATTRIB_FLAGS = 4096;
pub type OBJECT_ATTRIB_FLAGS = i32;
pub const OBJECT_ATTRIB_HAS_EXTENDED_ATTRIBS: OBJECT_ATTRIB_FLAGS = 8388608;
pub const OBJECT_ATTRIB_IS_CLASS: OBJECT_ATTRIB_FLAGS = 16777216;
pub const OBJECT_ATTRIB_IS_FUNCTION: OBJECT_ATTRIB_FLAGS = 33554432;
pub const OBJECT_ATTRIB_IS_INHERITED: OBJECT_ATTRIB_FLAGS = 1073741824;
pub const OBJECT_ATTRIB_IS_INTERFACE: OBJECT_ATTRIB_FLAGS = -2147483648;
pub const OBJECT_ATTRIB_IS_MACRO: OBJECT_ATTRIB_FLAGS = 268435456;
pub const OBJECT_ATTRIB_IS_PROPERTY: OBJECT_ATTRIB_FLAGS = 134217728;
pub const OBJECT_ATTRIB_IS_TYPE: OBJECT_ATTRIB_FLAGS = 536870912;
pub const OBJECT_ATTRIB_IS_VARIABLE: OBJECT_ATTRIB_FLAGS = 67108864;
pub const OBJECT_ATTRIB_NO_ATTRIB: OBJECT_ATTRIB_FLAGS = 0;
pub const OBJECT_ATTRIB_NO_NAME: OBJECT_ATTRIB_FLAGS = 1;
pub const OBJECT_ATTRIB_NO_TYPE: OBJECT_ATTRIB_FLAGS = 2;
pub const OBJECT_ATTRIB_NO_VALUE: OBJECT_ATTRIB_FLAGS = 4;
pub const OBJECT_ATTRIB_OBJECT_IS_EXPANDABLE: OBJECT_ATTRIB_FLAGS = 112;
pub const OBJECT_ATTRIB_SLOT_IS_CATEGORY: OBJECT_ATTRIB_FLAGS = 1024;
pub const OBJECT_ATTRIB_STORAGE_FIELD: OBJECT_ATTRIB_FLAGS = 262144;
pub const OBJECT_ATTRIB_STORAGE_GLOBAL: OBJECT_ATTRIB_FLAGS = 65536;
pub const OBJECT_ATTRIB_STORAGE_STATIC: OBJECT_ATTRIB_FLAGS = 131072;
pub const OBJECT_ATTRIB_STORAGE_VIRTUAL: OBJECT_ATTRIB_FLAGS = 524288;
pub const OBJECT_ATTRIB_TYPE_HAS_CODE: OBJECT_ATTRIB_FLAGS = 512;
pub const OBJECT_ATTRIB_TYPE_IS_CONSTANT: OBJECT_ATTRIB_FLAGS = 1048576;
pub const OBJECT_ATTRIB_TYPE_IS_EXPANDABLE: OBJECT_ATTRIB_FLAGS = 256;
pub const OBJECT_ATTRIB_TYPE_IS_OBJECT: OBJECT_ATTRIB_FLAGS = 256;
pub const OBJECT_ATTRIB_TYPE_IS_SYNCHRONIZED: OBJECT_ATTRIB_FLAGS = 2097152;
pub const OBJECT_ATTRIB_TYPE_IS_VOLATILE: OBJECT_ATTRIB_FLAGS = 4194304;
pub const OBJECT_ATTRIB_VALUE_HAS_CODE: OBJECT_ATTRIB_FLAGS = 128;
pub const OBJECT_ATTRIB_VALUE_IS_CUSTOM: OBJECT_ATTRIB_FLAGS = 64;
pub const OBJECT_ATTRIB_VALUE_IS_ENUM: OBJECT_ATTRIB_FLAGS = 32;
pub const OBJECT_ATTRIB_VALUE_IS_INVALID: OBJECT_ATTRIB_FLAGS = 8;
pub const OBJECT_ATTRIB_VALUE_IS_OBJECT: OBJECT_ATTRIB_FLAGS = 16;
pub const OBJECT_ATTRIB_VALUE_READONLY: OBJECT_ATTRIB_FLAGS = 2048;
pub const PROP_INFO_ALL: i32 = 63;
pub const PROP_INFO_ATTRIBUTES: PROP_INFO_FLAGS = 8;
pub const PROP_INFO_AUTOEXPAND: PROP_INFO_FLAGS = 134217728;
pub const PROP_INFO_DEBUGPROP: PROP_INFO_FLAGS = 16;
pub type PROP_INFO_FLAGS = i32;
pub const PROP_INFO_FULLNAME: PROP_INFO_FLAGS = 32;
pub const PROP_INFO_NAME: PROP_INFO_FLAGS = 1;
pub const PROP_INFO_STANDARD: i32 = 15;
pub const PROP_INFO_TYPE: PROP_INFO_FLAGS = 2;
pub const PROP_INFO_VALUE: PROP_INFO_FLAGS = 4;
