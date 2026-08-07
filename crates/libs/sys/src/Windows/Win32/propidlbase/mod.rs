#[repr(C)]
#[cfg(feature = "wtypes")]
#[derive(Clone, Copy, Default)]
pub struct CABOOL {
    pub cElems: u32,
    pub pElems: *mut super::VARIANT_BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CABSTR {
    pub cElems: u32,
    pub pElems: *mut windows_sys::core::BSTR,
}
#[repr(C)]
#[cfg(feature = "wtypes")]
#[derive(Clone, Copy, Default)]
pub struct CABSTRBLOB {
    pub cElems: u32,
    pub pElems: *mut super::BSTRBLOB,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CAC {
    pub cElems: u32,
    pub pElems: *mut i8,
}
#[repr(C)]
#[cfg(feature = "wtypes")]
#[derive(Clone, Copy, Default)]
pub struct CACLIPDATA {
    pub cElems: u32,
    pub pElems: *mut super::CLIPDATA,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CACLSID {
    pub cElems: u32,
    pub pElems: *mut windows_sys::core::GUID,
}
#[repr(C)]
#[cfg(feature = "wtypes")]
#[derive(Clone, Copy, Default)]
pub struct CACY {
    pub cElems: u32,
    pub pElems: *mut super::CY,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CADATE {
    pub cElems: u32,
    pub pElems: *mut f64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CADBL {
    pub cElems: u32,
    pub pElems: *mut f64,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct CAFILETIME {
    pub cElems: u32,
    pub pElems: *mut super::FILETIME,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CAFLT {
    pub cElems: u32,
    pub pElems: *mut f32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CAH {
    pub cElems: u32,
    pub pElems: *mut i64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CAI {
    pub cElems: u32,
    pub pElems: *mut i16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CAL {
    pub cElems: u32,
    pub pElems: *mut i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CALPSTR {
    pub cElems: u32,
    pub pElems: *mut windows_sys::core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CALPWSTR {
    pub cElems: u32,
    pub pElems: *mut windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct CAPROPVARIANT {
    pub cElems: u32,
    pub pElems: *mut PROPVARIANT,
}
#[repr(C)]
#[cfg(feature = "wtypesbase")]
#[derive(Clone, Copy, Default)]
pub struct CASCODE {
    pub cElems: u32,
    pub pElems: *mut super::SCODE,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CAUB {
    pub cElems: u32,
    pub pElems: *mut u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CAUH {
    pub cElems: u32,
    pub pElems: *mut u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CAUI {
    pub cElems: u32,
    pub pElems: *mut u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CAUL {
    pub cElems: u32,
    pub pElems: *mut u32,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub type LPPROPVARIANT = *mut PROPVARIANT;
#[cfg(feature = "objidlbase")]
pub type LPVERSIONEDSTREAM = *mut VERSIONEDSTREAM;
pub const PID_BEHAVIOR: u32 = 2147483651;
pub const PID_CODEPAGE: i32 = 1;
pub const PID_DICTIONARY: i32 = 0;
pub const PID_FIRST_NAME_DEFAULT: i32 = 4095;
pub const PID_FIRST_USABLE: i32 = 2;
pub const PID_ILLEGAL: u32 = 4294967295;
pub const PID_LOCALE: u32 = 2147483648;
pub const PID_MAX_READONLY: u32 = 3221225471;
pub const PID_MIN_READONLY: u32 = 2147483648;
pub const PID_MODIFY_TIME: u32 = 2147483649;
pub const PID_SECURITY: u32 = 2147483650;
pub const PROPSETFLAG_ANSI: i32 = 2;
pub const PROPSETFLAG_CASE_SENSITIVE: i32 = 8;
pub const PROPSETFLAG_DEFAULT: i32 = 0;
pub const PROPSETFLAG_NONSIMPLE: i32 = 1;
pub const PROPSETFLAG_UNBUFFERED: i32 = 4;
pub const PROPSETHDR_OSVERSION_UNKNOWN: u32 = 4294967295;
pub const PROPSET_BEHAVIOR_CASE_SENSITIVE: i32 = 1;
#[repr(C)]
#[cfg(feature = "wtypes")]
#[derive(Clone, Copy)]
pub struct PROPSPEC {
    pub ulKind: u32,
    pub Anonymous: PROPSPEC_0,
}
#[cfg(feature = "wtypes")]
impl Default for PROPSPEC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wtypes")]
#[derive(Clone, Copy)]
pub union PROPSPEC_0 {
    pub propid: super::PROPID,
    pub lpwstr: windows_sys::core::PWSTR,
}
#[cfg(feature = "wtypes")]
impl Default for PROPSPEC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct PROPVARIANT {
    pub Anonymous: PROPVARIANT_0,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl Default for PROPVARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub union PROPVARIANT_0 {
    pub Anonymous: PROPVARIANT_0_0,
    pub decVal: super::DECIMAL,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl Default for PROPVARIANT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct PROPVARIANT_0_0 {
    pub vt: super::VARTYPE,
    pub wReserved1: PROPVAR_PAD1,
    pub wReserved2: PROPVAR_PAD2,
    pub wReserved3: PROPVAR_PAD3,
    pub Anonymous: PROPVARIANT_0_0_0,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl Default for PROPVARIANT_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub union PROPVARIANT_0_0_0 {
    pub cVal: i8,
    pub bVal: u8,
    pub iVal: i16,
    pub uiVal: u16,
    pub lVal: i32,
    pub ulVal: u32,
    pub intVal: i32,
    pub uintVal: u32,
    pub hVal: i64,
    pub uhVal: u64,
    pub fltVal: f32,
    pub dblVal: f64,
    pub boolVal: super::VARIANT_BOOL,
    pub __OBSOLETE__VARIANT_BOOL: super::VARIANT_BOOL,
    pub scode: super::SCODE,
    pub cyVal: super::CY,
    pub date: f64,
    pub filetime: super::FILETIME,
    pub puuid: *mut windows_sys::core::GUID,
    pub pclipdata: *mut super::CLIPDATA,
    pub bstrVal: windows_sys::core::BSTR,
    pub bstrblobVal: super::BSTRBLOB,
    pub blob: super::BLOB,
    pub pszVal: windows_sys::core::PSTR,
    pub pwszVal: windows_sys::core::PWSTR,
    pub punkVal: *mut core::ffi::c_void,
    pub pdispVal: *mut core::ffi::c_void,
    pub pStream: *mut core::ffi::c_void,
    pub pStorage: *mut core::ffi::c_void,
    pub pVersionedStream: LPVERSIONEDSTREAM,
    pub parray: super::LPSAFEARRAY,
    pub cac: CAC,
    pub caub: CAUB,
    pub cai: CAI,
    pub caui: CAUI,
    pub cal: CAL,
    pub caul: CAUL,
    pub cah: CAH,
    pub cauh: CAUH,
    pub caflt: CAFLT,
    pub cadbl: CADBL,
    pub cabool: CABOOL,
    pub cascode: CASCODE,
    pub cacy: CACY,
    pub cadate: CADATE,
    pub cafiletime: CAFILETIME,
    pub cauuid: CACLSID,
    pub caclipdata: CACLIPDATA,
    pub cabstr: CABSTR,
    pub cabstrblob: CABSTRBLOB,
    pub calpstr: CALPSTR,
    pub calpwstr: CALPWSTR,
    pub capropvar: CAPROPVARIANT,
    pub pcVal: *mut i8,
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub puiVal: *mut u16,
    pub plVal: *mut i32,
    pub pulVal: *mut u32,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut super::VARIANT_BOOL,
    pub pdecVal: *mut super::DECIMAL,
    pub pscode: *mut super::SCODE,
    pub pcyVal: *mut super::CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut windows_sys::core::BSTR,
    pub ppunkVal: *mut *mut core::ffi::c_void,
    pub ppdispVal: *mut *mut core::ffi::c_void,
    pub pparray: *mut super::LPSAFEARRAY,
    pub pvarVal: *mut PROPVARIANT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl Default for PROPVARIANT_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PROPVAR_PAD1 = u16;
pub type PROPVAR_PAD2 = u16;
pub type PROPVAR_PAD3 = u16;
pub const PRSPEC_INVALID: u32 = 4294967295;
pub const PRSPEC_LPWSTR: i32 = 0;
pub const PRSPEC_PROPID: i32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct STATPROPSETSTG {
    pub fmtid: windows_sys::core::GUID,
    pub clsid: windows_sys::core::GUID,
    pub grfFlags: u32,
    pub mtime: super::FILETIME,
    pub ctime: super::FILETIME,
    pub atime: super::FILETIME,
    pub dwOSVersion: u32,
}
#[repr(C)]
#[cfg(feature = "wtypes")]
#[derive(Clone, Copy, Default)]
pub struct STATPROPSTG {
    pub lpwstrName: windows_sys::core::PWSTR,
    pub propid: super::PROPID,
    pub vt: super::VARTYPE,
}
#[repr(C)]
#[cfg(feature = "objidlbase")]
#[derive(Clone, Copy, Default)]
pub struct VERSIONEDSTREAM {
    pub guidVersion: windows_sys::core::GUID,
    pub pStream: *mut core::ffi::c_void,
}
