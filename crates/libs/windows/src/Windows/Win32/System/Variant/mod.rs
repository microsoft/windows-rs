pub const DPF_ERROR: DRAWPROGRESSFLAGS = 4i32;
pub const DPF_MARQUEE: DRAWPROGRESSFLAGS = 1i32;
pub const DPF_MARQUEE_COMPLETE: DRAWPROGRESSFLAGS = 2i32;
pub const DPF_NONE: DRAWPROGRESSFLAGS = 0i32;
pub const DPF_STOPPED: DRAWPROGRESSFLAGS = 16i32;
pub const DPF_WARNING: DRAWPROGRESSFLAGS = 8i32;
pub const PSTF_LOCAL: PSTIME_FLAGS = 1i32;
pub const PSTF_UTC: PSTIME_FLAGS = 0i32;
pub const VARIANT_ALPHABOOL: VAR_CHANGE_FLAGS = 2u16;
pub const VARIANT_CALENDAR_GREGORIAN: VAR_CHANGE_FLAGS = 64u16;
pub const VARIANT_CALENDAR_HIJRI: VAR_CHANGE_FLAGS = 8u16;
pub const VARIANT_CALENDAR_THAI: VAR_CHANGE_FLAGS = 32u16;
pub const VARIANT_LOCALBOOL: VAR_CHANGE_FLAGS = 16u16;
pub const VARIANT_NOUSEROVERRIDE: VAR_CHANGE_FLAGS = 4u16;
pub const VARIANT_NOVALUEPROP: VAR_CHANGE_FLAGS = 1u16;
pub const VARIANT_USE_NLS: VAR_CHANGE_FLAGS = 128u16;
pub const VT_ARRAY: VARENUM = 8192u16;
pub const VT_BLOB: VARENUM = 65u16;
pub const VT_BLOB_OBJECT: VARENUM = 70u16;
pub const VT_BOOL: VARENUM = 11u16;
pub const VT_BSTR: VARENUM = 8u16;
pub const VT_BSTR_BLOB: VARENUM = 4095u16;
pub const VT_BYREF: VARENUM = 16384u16;
pub const VT_CARRAY: VARENUM = 28u16;
pub const VT_CF: VARENUM = 71u16;
pub const VT_CLSID: VARENUM = 72u16;
pub const VT_CY: VARENUM = 6u16;
pub const VT_DATE: VARENUM = 7u16;
pub const VT_DECIMAL: VARENUM = 14u16;
pub const VT_DISPATCH: VARENUM = 9u16;
pub const VT_EMPTY: VARENUM = 0u16;
pub const VT_ERROR: VARENUM = 10u16;
pub const VT_FILETIME: VARENUM = 64u16;
pub const VT_HRESULT: VARENUM = 25u16;
pub const VT_I1: VARENUM = 16u16;
pub const VT_I2: VARENUM = 2u16;
pub const VT_I4: VARENUM = 3u16;
pub const VT_I8: VARENUM = 20u16;
pub const VT_ILLEGAL: VARENUM = 65535u16;
pub const VT_ILLEGALMASKED: VARENUM = 4095u16;
pub const VT_INT: VARENUM = 22u16;
pub const VT_INT_PTR: VARENUM = 37u16;
pub const VT_LPSTR: VARENUM = 30u16;
pub const VT_LPWSTR: VARENUM = 31u16;
pub const VT_NULL: VARENUM = 1u16;
pub const VT_PTR: VARENUM = 26u16;
pub const VT_R4: VARENUM = 4u16;
pub const VT_R8: VARENUM = 5u16;
pub const VT_RECORD: VARENUM = 36u16;
pub const VT_RESERVED: VARENUM = 32768u16;
pub const VT_SAFEARRAY: VARENUM = 27u16;
pub const VT_STORAGE: VARENUM = 67u16;
pub const VT_STORED_OBJECT: VARENUM = 69u16;
pub const VT_STREAM: VARENUM = 66u16;
pub const VT_STREAMED_OBJECT: VARENUM = 68u16;
pub const VT_TYPEMASK: VARENUM = 4095u16;
pub const VT_UI1: VARENUM = 17u16;
pub const VT_UI2: VARENUM = 18u16;
pub const VT_UI4: VARENUM = 19u16;
pub const VT_UI8: VARENUM = 21u16;
pub const VT_UINT: VARENUM = 23u16;
pub const VT_UINT_PTR: VARENUM = 38u16;
pub const VT_UNKNOWN: VARENUM = 13u16;
pub const VT_USERDEFINED: VARENUM = 29u16;
pub const VT_VARIANT: VARENUM = 12u16;
pub const VT_VECTOR: VARENUM = 4096u16;
pub const VT_VERSIONED_STREAM: VARENUM = 73u16;
pub const VT_VOID: VARENUM = 24u16;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DRAWPROGRESSFLAGS(pub i32);
impl windows_core::TypeKind for DRAWPROGRESSFLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PSTIME_FLAGS(pub i32);
impl windows_core::TypeKind for PSTIME_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VARENUM(pub u16);
impl windows_core::TypeKind for VARENUM {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VAR_CHANGE_FLAGS(pub u16);
impl windows_core::TypeKind for VAR_CHANGE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VARIANT {
    pub Anonymous: VARIANT_0,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl Default for VARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl windows_core::TypeKind for VARIANT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union VARIANT_0 {
    pub Anonymous: VARIANT_0_0,
    pub decVal: super::super::Foundation::DECIMAL,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl Default for VARIANT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl windows_core::TypeKind for VARIANT_0 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VARIANT_0_0 {
    pub vt: VARENUM,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: VARIANT_0_0_0,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl Default for VARIANT_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl windows_core::TypeKind for VARIANT_0_0 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union VARIANT_0_0_0 {
    pub llVal: i64,
    pub lVal: i32,
    pub bVal: u8,
    pub iVal: i16,
    pub fltVal: f32,
    pub dblVal: f64,
    pub boolVal: super::super::Foundation::VARIANT_BOOL,
    pub __OBSOLETE__VARIANT_BOOL: super::super::Foundation::VARIANT_BOOL,
    pub scode: i32,
    pub cyVal: super::Com::CY,
    pub date: f64,
    pub bstrVal: windows_core::BSTR,
    pub punkVal: Option<windows_core::IUnknown>,
    pub pdispVal: Option<super::Com::IDispatch>,
    pub parray: *mut super::Com::SAFEARRAY,
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub plVal: *mut i32,
    pub pllVal: *mut i64,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut super::super::Foundation::VARIANT_BOOL,
    pub __OBSOLETE__VARIANT_PBOOL: *mut super::super::Foundation::VARIANT_BOOL,
    pub pscode: *mut i32,
    pub pcyVal: *mut super::Com::CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut windows_core::BSTR,
    pub ppunkVal: *mut Option<windows_core::IUnknown>,
    pub ppdispVal: *mut Option<super::Com::IDispatch>,
    pub pparray: *mut *mut super::Com::SAFEARRAY,
    pub pvarVal: *mut VARIANT,
    pub byref: *mut core::ffi::c_void,
    pub cVal: i8,
    pub uiVal: u16,
    pub ulVal: u32,
    pub ullVal: u64,
    pub intVal: i32,
    pub uintVal: u32,
    pub pdecVal: *mut super::super::Foundation::DECIMAL,
    pub pcVal: windows_core::PSTR,
    pub puiVal: *mut u16,
    pub pulVal: *mut u32,
    pub pullVal: *mut u64,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
    pub Anonymous: VARIANT_0_0_0_0,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl Default for VARIANT_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl windows_core::TypeKind for VARIANT_0_0_0 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VARIANT_0_0_0_0 {
    pub pvRecord: *mut core::ffi::c_void,
    pub pRecInfo: Option<super::Ole::IRecordInfo>,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl Default for VARIANT_0_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl windows_core::TypeKind for VARIANT_0_0_0_0 {
    type TypeKind = windows_core::CloneType;
}
