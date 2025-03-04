#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

pub type ADVANCED_FEATURE_FLAGS = u16;
pub type BOOL = i32;
pub type BSTR = *const u16;
#[repr(C)]
#[derive(Clone, Copy)]
pub union CY {
    pub Anonymous: CY_0,
    pub int64: i64,
}
impl Default for CY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CY_0 {
    pub Lo: u32,
    pub Hi: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DECIMAL {
    pub wReserved: u16,
    pub Anonymous1: DECIMAL_0,
    pub Hi32: u32,
    pub Anonymous2: DECIMAL_1,
}
impl Default for DECIMAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DECIMAL_0 {
    pub Anonymous: DECIMAL_0_0,
    pub signscale: u16,
}
impl Default for DECIMAL_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DECIMAL_0_0 {
    pub scale: u8,
    pub sign: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DECIMAL_1 {
    pub Anonymous: DECIMAL_1_0,
    pub Lo64: u64,
}
impl Default for DECIMAL_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DECIMAL_1_0 {
    pub Lo32: u32,
    pub Mid32: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}
impl GUID {
    pub const fn from_u128(uuid: u128) -> Self {
        Self {
            data1: (uuid >> 96) as u32,
            data2: (uuid >> 80 & 0xffff) as u16,
            data3: (uuid >> 64 & 0xffff) as u16,
            data4: (uuid as u64).to_be_bytes(),
        }
    }
}
pub type HRESULT = i32;
pub const IID_IDispatch: GUID = GUID::from_u128(0x00020400_0000_0000_c000_000000000046);
#[repr(C)]
pub struct IDispatch_Vtbl {
    pub base__: IUnknown_Vtbl,
    pub GetTypeInfoCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> HRESULT,
    GetTypeInfo: usize,
    pub GetIDsOfNames: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const GUID,
        *const PCWSTR,
        u32,
        u32,
        *mut i32,
    ) -> HRESULT,
    Invoke: usize,
}
pub const IID_IRecordInfo: GUID = GUID::from_u128(0x0000002f_0000_0000_c000_000000000046);
#[repr(C)]
pub struct IRecordInfo_Vtbl {
    pub base__: IUnknown_Vtbl,
    pub RecordInit:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> HRESULT,
    pub RecordClear:
        unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> HRESULT,
    pub RecordCopy: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> HRESULT,
    pub GetGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GUID) -> HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BSTR) -> HRESULT,
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> HRESULT,
    GetTypeInfo: usize,
    pub GetField: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        PCWSTR,
        *mut VARIANT,
    ) -> HRESULT,
    pub GetFieldNoCopy: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        PCWSTR,
        *mut VARIANT,
        *mut *mut core::ffi::c_void,
    ) -> HRESULT,
    pub PutField: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut core::ffi::c_void,
        PCWSTR,
        *const VARIANT,
    ) -> HRESULT,
    pub PutFieldNoCopy: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut core::ffi::c_void,
        PCWSTR,
        *const VARIANT,
    ) -> HRESULT,
    pub GetFieldNames:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut BSTR) -> HRESULT,
    pub IsMatchingType:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> BOOL,
    pub RecordCreate: unsafe extern "system" fn(*mut core::ffi::c_void) -> *mut core::ffi::c_void,
    pub RecordCreateCopy: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> HRESULT,
    pub RecordDestroy:
        unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> HRESULT,
}
pub const IID_IUnknown: GUID = GUID::from_u128(0x00000000_0000_0000_c000_000000000046);
#[repr(C)]
pub struct IUnknown_Vtbl {
    pub QueryInterface: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        iid: *const GUID,
        interface: *mut *mut core::ffi::c_void,
    ) -> HRESULT,
    pub AddRef: unsafe extern "system" fn(this: *mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(this: *mut core::ffi::c_void) -> u32,
}
pub type PCWSTR = *const u16;
pub type PSTR = *mut u8;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SAFEARRAY {
    pub cDims: u16,
    pub fFeatures: ADVANCED_FEATURE_FLAGS,
    pub cbElements: u32,
    pub cLocks: u32,
    pub pvData: *mut core::ffi::c_void,
    pub rgsabound: [SAFEARRAYBOUND; 1],
}
impl Default for SAFEARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SAFEARRAYBOUND {
    pub cElements: u32,
    pub lLbound: i32,
}
pub type VARENUM = u16;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VARIANT {
    pub Anonymous: VARIANT_0,
}
impl Default for VARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VARIANT_0 {
    pub Anonymous: VARIANT_0_0,
    pub decVal: DECIMAL,
}
impl Default for VARIANT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VARIANT_0_0 {
    pub vt: VARENUM,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: VARIANT_0_0_0,
}
impl Default for VARIANT_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VARIANT_0_0_0 {
    pub llVal: i64,
    pub lVal: i32,
    pub bVal: u8,
    pub iVal: i16,
    pub fltVal: f32,
    pub dblVal: f64,
    pub boolVal: VARIANT_BOOL,
    pub __OBSOLETE__VARIANT_BOOL: VARIANT_BOOL,
    pub scode: i32,
    pub cyVal: CY,
    pub date: f64,
    pub bstrVal: BSTR,
    pub punkVal: *mut core::ffi::c_void,
    pub pdispVal: *mut core::ffi::c_void,
    pub parray: *mut SAFEARRAY,
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub plVal: *mut i32,
    pub pllVal: *mut i64,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut VARIANT_BOOL,
    pub __OBSOLETE__VARIANT_PBOOL: *mut VARIANT_BOOL,
    pub pscode: *mut i32,
    pub pcyVal: *mut CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut BSTR,
    pub ppunkVal: *mut *mut core::ffi::c_void,
    pub ppdispVal: *mut *mut core::ffi::c_void,
    pub pparray: *mut *mut SAFEARRAY,
    pub pvarVal: *mut VARIANT,
    pub byref: *mut core::ffi::c_void,
    pub cVal: i8,
    pub uiVal: u16,
    pub ulVal: u32,
    pub ullVal: u64,
    pub intVal: i32,
    pub uintVal: u32,
    pub pdecVal: *mut DECIMAL,
    pub pcVal: PSTR,
    pub puiVal: *mut u16,
    pub pulVal: *mut u32,
    pub pullVal: *mut u64,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
    pub Anonymous: VARIANT_0_0_0_0,
}
impl Default for VARIANT_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VARIANT_0_0_0_0 {
    pub pvRecord: *mut core::ffi::c_void,
    pub pRecInfo: *mut core::ffi::c_void,
}
impl Default for VARIANT_0_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type VARIANT_BOOL = i16;
