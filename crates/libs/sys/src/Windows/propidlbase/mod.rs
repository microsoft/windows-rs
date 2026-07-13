#[repr(C)]
#[cfg(feature = "wtypes")]
#[derive(Clone, Copy)]
pub struct CABOOL {
    pub cElems: u32,
    pub pElems: *mut super::wtypes::VARIANT_BOOL,
}
#[cfg(feature = "wtypes")]
impl Default for CABOOL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CABSTR {
    pub cElems: u32,
    pub pElems: *mut windows_sys::core::BSTR,
}
impl Default for CABSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wtypes")]
#[derive(Clone, Copy)]
pub struct CABSTRBLOB {
    pub cElems: u32,
    pub pElems: *mut super::wtypes::BSTRBLOB,
}
#[cfg(feature = "wtypes")]
impl Default for CABSTRBLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CAC {
    pub cElems: u32,
    pub pElems: *mut i8,
}
impl Default for CAC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wtypes")]
#[derive(Clone, Copy)]
pub struct CACLIPDATA {
    pub cElems: u32,
    pub pElems: *mut super::wtypes::CLIPDATA,
}
#[cfg(feature = "wtypes")]
impl Default for CACLIPDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CACLSID {
    pub cElems: u32,
    pub pElems: *mut windows_sys::core::GUID,
}
impl Default for CACLSID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wtypes")]
#[derive(Clone, Copy)]
pub struct CACY {
    pub cElems: u32,
    pub pElems: *mut super::wtypes::CY,
}
#[cfg(feature = "wtypes")]
impl Default for CACY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CADATE {
    pub cElems: u32,
    pub pElems: *mut f64,
}
impl Default for CADATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CADBL {
    pub cElems: u32,
    pub pElems: *mut f64,
}
impl Default for CADBL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CAFILETIME {
    pub cElems: u32,
    pub pElems: *mut super::minwindef::FILETIME,
}
#[cfg(feature = "minwindef")]
impl Default for CAFILETIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CAFLT {
    pub cElems: u32,
    pub pElems: *mut f32,
}
impl Default for CAFLT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CAH {
    pub cElems: u32,
    pub pElems: *mut i64,
}
impl Default for CAH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CAI {
    pub cElems: u32,
    pub pElems: *mut i16,
}
impl Default for CAI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CAL {
    pub cElems: u32,
    pub pElems: *mut i32,
}
impl Default for CAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CALPSTR {
    pub cElems: u32,
    pub pElems: *mut windows_sys::core::PSTR,
}
impl Default for CALPSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CALPWSTR {
    pub cElems: u32,
    pub pElems: *mut windows_sys::core::PWSTR,
}
impl Default for CALPWSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct CAPROPVARIANT {
    pub cElems: u32,
    pub pElems: *mut PROPVARIANT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl Default for CAPROPVARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wtypesbase")]
#[derive(Clone, Copy)]
pub struct CASCODE {
    pub cElems: u32,
    pub pElems: *mut super::wtypesbase::SCODE,
}
#[cfg(feature = "wtypesbase")]
impl Default for CASCODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CAUB {
    pub cElems: u32,
    pub pElems: *mut u8,
}
impl Default for CAUB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CAUH {
    pub cElems: u32,
    pub pElems: *mut u64,
}
impl Default for CAUH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CAUI {
    pub cElems: u32,
    pub pElems: *mut u16,
}
impl Default for CAUI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CAUL {
    pub cElems: u32,
    pub pElems: *mut u32,
}
impl Default for CAUL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub type LPPROPVARIANT = *mut PROPVARIANT;
#[cfg(feature = "objidlbase")]
pub type LPVERSIONEDSTREAM = *mut VERSIONEDSTREAM;
pub const PID_BEHAVIOR: u32 = 2147483651;
pub const PID_CODEPAGE: u32 = 1;
pub const PID_DICTIONARY: u32 = 0;
pub const PID_FIRST_NAME_DEFAULT: u32 = 4095;
pub const PID_FIRST_USABLE: u32 = 2;
pub const PID_ILLEGAL: u32 = 4294967295;
pub const PID_LOCALE: u32 = 2147483648;
pub const PID_MAX_READONLY: u32 = 3221225471;
pub const PID_MIN_READONLY: u32 = 2147483648;
pub const PID_MODIFY_TIME: u32 = 2147483649;
pub const PID_SECURITY: u32 = 2147483650;
pub const PROPSETFLAG_ANSI: u32 = 2;
pub const PROPSETFLAG_CASE_SENSITIVE: u32 = 8;
pub const PROPSETFLAG_DEFAULT: u32 = 0;
pub const PROPSETFLAG_NONSIMPLE: u32 = 1;
pub const PROPSETFLAG_UNBUFFERED: u32 = 4;
pub const PROPSETHDR_OSVERSION_UNKNOWN: u32 = 4294967295;
pub const PROPSET_BEHAVIOR_CASE_SENSITIVE: u32 = 1;
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
    pub propid: super::wtypes::PROPID,
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
    pub decVal: super::wtypes::DECIMAL,
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
    pub vt: super::wtypes::VARTYPE,
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
    pub boolVal: super::wtypes::VARIANT_BOOL,
    pub __OBSOLETE__VARIANT_BOOL: super::wtypes::VARIANT_BOOL,
    pub scode: super::wtypesbase::SCODE,
    pub cyVal: super::wtypes::CY,
    pub date: f64,
    pub filetime: super::minwindef::FILETIME,
    pub puuid: *mut windows_sys::core::GUID,
    pub pclipdata: *mut super::wtypes::CLIPDATA,
    pub bstrVal: windows_sys::core::BSTR,
    pub bstrblobVal: super::wtypes::BSTRBLOB,
    pub blob: super::wtypesbase::BLOB,
    pub pszVal: windows_sys::core::PSTR,
    pub pwszVal: windows_sys::core::PWSTR,
    pub punkVal: *mut core::ffi::c_void,
    pub pdispVal: *mut core::ffi::c_void,
    pub pStream: *mut core::ffi::c_void,
    pub pStorage: *mut core::ffi::c_void,
    pub pVersionedStream: LPVERSIONEDSTREAM,
    pub parray: super::oaidl::LPSAFEARRAY,
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
    pub pboolVal: *mut super::wtypes::VARIANT_BOOL,
    pub pdecVal: *mut super::wtypes::DECIMAL,
    pub pscode: *mut super::wtypesbase::SCODE,
    pub pcyVal: *mut super::wtypes::CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut windows_sys::core::BSTR,
    pub ppunkVal: *mut *mut core::ffi::c_void,
    pub ppdispVal: *mut *mut core::ffi::c_void,
    pub pparray: *mut super::oaidl::LPSAFEARRAY,
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
pub const PRSPEC_LPWSTR: u32 = 0;
pub const PRSPEC_PROPID: u32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct STATPROPSETSTG {
    pub fmtid: windows_sys::core::GUID,
    pub clsid: windows_sys::core::GUID,
    pub grfFlags: u32,
    pub mtime: super::minwindef::FILETIME,
    pub ctime: super::minwindef::FILETIME,
    pub atime: super::minwindef::FILETIME,
    pub dwOSVersion: u32,
}
#[repr(C)]
#[cfg(feature = "wtypes")]
#[derive(Clone, Copy)]
pub struct STATPROPSTG {
    pub lpwstrName: windows_sys::core::PWSTR,
    pub propid: super::wtypes::PROPID,
    pub vt: super::wtypes::VARTYPE,
}
#[cfg(feature = "wtypes")]
impl Default for STATPROPSTG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "objidlbase")]
#[derive(Clone, Copy)]
pub struct VERSIONEDSTREAM {
    pub guidVersion: windows_sys::core::GUID,
    pub pStream: *mut core::ffi::c_void,
}
#[cfg(feature = "objidlbase")]
impl Default for VERSIONEDSTREAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
