windows_link::link!("oleaut32.dll" "system" fn BSTR_UserFree(param0 : *mut u32, param1 : *mut windows_sys::core::BSTR));
windows_link::link!("oleaut32.dll" "system" fn BSTR_UserFree64(param0 : *mut u32, param1 : *mut windows_sys::core::BSTR));
windows_link::link!("oleaut32.dll" "system" fn BSTR_UserMarshal(param0 : *mut u32, param1 : *mut u8, param2 : *mut windows_sys::core::BSTR) -> *mut u8);
windows_link::link!("oleaut32.dll" "system" fn BSTR_UserMarshal64(param0 : *mut u32, param1 : *mut u8, param2 : *mut windows_sys::core::BSTR) -> *mut u8);
windows_link::link!("oleaut32.dll" "system" fn BSTR_UserSize(param0 : *mut u32, param1 : u32, param2 : *mut windows_sys::core::BSTR) -> u32);
windows_link::link!("oleaut32.dll" "system" fn BSTR_UserSize64(param0 : *mut u32, param1 : u32, param2 : *mut windows_sys::core::BSTR) -> u32);
windows_link::link!("oleaut32.dll" "system" fn BSTR_UserUnmarshal(param0 : *mut u32, param1 : *mut u8, param2 : *mut windows_sys::core::BSTR) -> *mut u8);
windows_link::link!("oleaut32.dll" "system" fn BSTR_UserUnmarshal64(param0 : *mut u32, param1 : *mut u8, param2 : *mut windows_sys::core::BSTR) -> *mut u8);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VARIANT_UserFree(param0 : *mut u32, param1 : *mut super::VARIANT));
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VARIANT_UserFree64(param0 : *mut u32, param1 : *mut super::VARIANT));
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VARIANT_UserMarshal(param0 : *mut u32, param1 : *mut u8, param2 : *mut super::VARIANT) -> *mut u8);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VARIANT_UserMarshal64(param0 : *mut u32, param1 : *mut u8, param2 : *mut super::VARIANT) -> *mut u8);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VARIANT_UserSize(param0 : *mut u32, param1 : u32, param2 : *mut super::VARIANT) -> u32);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VARIANT_UserSize64(param0 : *mut u32, param1 : u32, param2 : *mut super::VARIANT) -> u32);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VARIANT_UserUnmarshal(param0 : *mut u32, param1 : *mut u8, param2 : *mut super::VARIANT) -> *mut u8);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VARIANT_UserUnmarshal64(param0 : *mut u32, param1 : *mut u8, param2 : *mut super::VARIANT) -> *mut u8);
pub const DBGPROP_ATTRIB_ACCESS_FINAL: __MIDL___MIDL_itf_dbgprop_0000_0000_0001 = 32768;
pub const DBGPROP_ATTRIB_ACCESS_PRIVATE: __MIDL___MIDL_itf_dbgprop_0000_0000_0001 = 8192;
pub const DBGPROP_ATTRIB_ACCESS_PROTECTED: __MIDL___MIDL_itf_dbgprop_0000_0000_0001 = 16384;
pub const DBGPROP_ATTRIB_ACCESS_PUBLIC: __MIDL___MIDL_itf_dbgprop_0000_0000_0001 = 4096;
pub type DBGPROP_ATTRIB_FLAGS = u32;
pub const DBGPROP_ATTRIB_FRAME_INCATCHBLOCK: __MIDL___MIDL_itf_dbgprop_0000_0000_0001 = 33554432;
pub const DBGPROP_ATTRIB_FRAME_INFINALLYBLOCK: __MIDL___MIDL_itf_dbgprop_0000_0000_0001 = 67108864;
pub const DBGPROP_ATTRIB_FRAME_INTRYBLOCK: __MIDL___MIDL_itf_dbgprop_0000_0000_0001 = 16777216;
pub const DBGPROP_ATTRIB_HAS_EXTENDED_ATTRIBS: __MIDL___MIDL_itf_dbgprop_0000_0000_0001 = 8388608;
pub const DBGPROP_ATTRIB_NO_ATTRIB: __MIDL___MIDL_itf_dbgprop_0000_0000_0001 = 0;
pub const DBGPROP_ATTRIB_STORAGE_FIELD: __MIDL___MIDL_itf_dbgprop_0000_0000_0001 = 262144;
pub const DBGPROP_ATTRIB_STORAGE_GLOBAL: __MIDL___MIDL_itf_dbgprop_0000_0000_0001 = 65536;
pub const DBGPROP_ATTRIB_STORAGE_STATIC: __MIDL___MIDL_itf_dbgprop_0000_0000_0001 = 131072;
pub const DBGPROP_ATTRIB_STORAGE_VIRTUAL: __MIDL___MIDL_itf_dbgprop_0000_0000_0001 = 524288;
pub const DBGPROP_ATTRIB_TYPE_IS_CONSTANT: __MIDL___MIDL_itf_dbgprop_0000_0000_0001 = 1048576;
pub const DBGPROP_ATTRIB_TYPE_IS_SYNCHRONIZED: __MIDL___MIDL_itf_dbgprop_0000_0000_0001 = 2097152;
pub const DBGPROP_ATTRIB_TYPE_IS_VOLATILE: __MIDL___MIDL_itf_dbgprop_0000_0000_0001 = 4194304;
pub const DBGPROP_ATTRIB_VALUE_IS_EVENT: __MIDL___MIDL_itf_dbgprop_0000_0000_0001 = 512;
pub const DBGPROP_ATTRIB_VALUE_IS_EXPANDABLE: __MIDL___MIDL_itf_dbgprop_0000_0000_0001 = 16;
pub const DBGPROP_ATTRIB_VALUE_IS_FAKE: __MIDL___MIDL_itf_dbgprop_0000_0000_0001 = 32;
pub const DBGPROP_ATTRIB_VALUE_IS_INVALID: __MIDL___MIDL_itf_dbgprop_0000_0000_0001 = 8;
pub const DBGPROP_ATTRIB_VALUE_IS_METHOD: __MIDL___MIDL_itf_dbgprop_0000_0000_0001 = 256;
pub const DBGPROP_ATTRIB_VALUE_IS_RAW_STRING: __MIDL___MIDL_itf_dbgprop_0000_0000_0001 = 1024;
pub const DBGPROP_ATTRIB_VALUE_IS_RETURN_VALUE: __MIDL___MIDL_itf_dbgprop_0000_0000_0001 = 134217728;
pub const DBGPROP_ATTRIB_VALUE_PENDING_MUTATION: __MIDL___MIDL_itf_dbgprop_0000_0000_0001 = 268435456;
pub const DBGPROP_ATTRIB_VALUE_READONLY: __MIDL___MIDL_itf_dbgprop_0000_0000_0001 = 2048;
pub const DBGPROP_INFO_ALL: i32 = 63;
pub const DBGPROP_INFO_ATTRIBUTES: __MIDL___MIDL_itf_dbgprop_0000_0000_0002 = 8;
pub const DBGPROP_INFO_AUTOEXPAND: __MIDL___MIDL_itf_dbgprop_0000_0000_0002 = 134217728;
pub const DBGPROP_INFO_BEAUTIFY: __MIDL___MIDL_itf_dbgprop_0000_0000_0002 = 33554432;
pub const DBGPROP_INFO_CALLTOSTRING: __MIDL___MIDL_itf_dbgprop_0000_0000_0002 = 67108864;
pub const DBGPROP_INFO_DEBUGPROP: __MIDL___MIDL_itf_dbgprop_0000_0000_0002 = 16;
pub type DBGPROP_INFO_FLAGS = u32;
pub const DBGPROP_INFO_FULLNAME: __MIDL___MIDL_itf_dbgprop_0000_0000_0002 = 32;
pub const DBGPROP_INFO_NAME: __MIDL___MIDL_itf_dbgprop_0000_0000_0002 = 1;
pub const DBGPROP_INFO_STANDARD: i32 = 15;
pub const DBGPROP_INFO_TYPE: __MIDL___MIDL_itf_dbgprop_0000_0000_0002 = 2;
pub const DBGPROP_INFO_VALUE: __MIDL___MIDL_itf_dbgprop_0000_0000_0002 = 4;
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
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "wtypes", feature = "wtypesbase"))]
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
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "wtypes", feature = "wtypesbase"))]
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
pub type __MIDL___MIDL_itf_dbgprop_0000_0000_0001 = i32;
pub type __MIDL___MIDL_itf_dbgprop_0000_0000_0002 = i32;
