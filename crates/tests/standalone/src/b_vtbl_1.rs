#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub type ADVANCED_FEATURE_FLAGS = u16;
#[repr(C)]
pub struct ARRAYDESC {
    pub tdescElem: TYPEDESC,
    pub cDims: u16,
    pub rgbounds: [SAFEARRAYBOUND; 1],
}
impl Copy for ARRAYDESC {}
impl Clone for ARRAYDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union BINDPTR {
    pub lpfuncdesc: *mut FUNCDESC,
    pub lpvardesc: *mut VARDESC,
    pub lptcomp: *mut core::ffi::c_void,
}
impl Copy for BINDPTR {}
impl Clone for BINDPTR {
    fn clone(&self) -> Self {
        *self
    }
}
pub type BOOL = i32;
pub type BSTR = *const u16;
pub type CALLCONV = i32;
#[repr(C)]
pub union CY {
    pub Anonymous: CY_0,
    pub int64: i64,
}
impl Copy for CY {}
impl Clone for CY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CY_0 {
    pub Lo: u32,
    pub Hi: i32,
}
impl Copy for CY_0 {}
impl Clone for CY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DECIMAL {
    pub wReserved: u16,
    pub Anonymous1: DECIMAL_0,
    pub Hi32: u32,
    pub Anonymous2: DECIMAL_1,
}
impl Copy for DECIMAL {}
impl Clone for DECIMAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union DECIMAL_0 {
    pub Anonymous: DECIMAL_0_0,
    pub signscale: u16,
}
impl Copy for DECIMAL_0 {}
impl Clone for DECIMAL_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DECIMAL_0_0 {
    pub scale: u8,
    pub sign: u8,
}
impl Copy for DECIMAL_0_0 {}
impl Clone for DECIMAL_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union DECIMAL_1 {
    pub Anonymous: DECIMAL_1_0,
    pub Lo64: u64,
}
impl Copy for DECIMAL_1 {}
impl Clone for DECIMAL_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DECIMAL_1_0 {
    pub Lo32: u32,
    pub Mid32: u32,
}
impl Copy for DECIMAL_1_0 {}
impl Clone for DECIMAL_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DESCKIND = i32;
pub type DISPATCH_FLAGS = u16;
#[repr(C)]
pub struct DISPPARAMS {
    pub rgvarg: *mut VARIANT,
    pub rgdispidNamedArgs: *mut i32,
    pub cArgs: u32,
    pub cNamedArgs: u32,
}
impl Copy for DISPPARAMS {}
impl Clone for DISPPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ELEMDESC {
    pub tdesc: TYPEDESC,
    pub Anonymous: ELEMDESC_0,
}
impl Copy for ELEMDESC {}
impl Clone for ELEMDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union ELEMDESC_0 {
    pub idldesc: IDLDESC,
    pub paramdesc: PARAMDESC,
}
impl Copy for ELEMDESC_0 {}
impl Clone for ELEMDESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct EXCEPINFO {
    pub wCode: u16,
    pub wReserved: u16,
    pub bstrSource: BSTR,
    pub bstrDescription: BSTR,
    pub bstrHelpFile: BSTR,
    pub dwHelpContext: u32,
    pub pvReserved: *mut core::ffi::c_void,
    pub pfnDeferredFillIn: LPEXCEPFINO_DEFERRED_FILLIN,
    pub scode: i32,
}
impl Copy for EXCEPINFO {}
impl Clone for EXCEPINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct FUNCDESC {
    pub memid: i32,
    pub lprgscode: *mut i32,
    pub lprgelemdescParam: *mut ELEMDESC,
    pub funckind: FUNCKIND,
    pub invkind: INVOKEKIND,
    pub callconv: CALLCONV,
    pub cParams: i16,
    pub cParamsOpt: i16,
    pub oVft: i16,
    pub cScodes: i16,
    pub elemdescFunc: ELEMDESC,
    pub wFuncFlags: FUNCFLAGS,
}
impl Copy for FUNCDESC {}
impl Clone for FUNCDESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub type FUNCFLAGS = u16;
pub type FUNCKIND = i32;
#[repr(C)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}
impl Copy for GUID {}
impl Clone for GUID {
    fn clone(&self) -> Self {
        *self
    }
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
#[repr(C)]
pub struct IDLDESC {
    pub dwReserved: usize,
    pub wIDLFlags: IDLFLAGS,
}
impl Copy for IDLDESC {}
impl Clone for IDLDESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub type IDLFLAGS = u16;
pub const IID_IDispatch: GUID = GUID::from_u128(0x00020400_0000_0000_c000_000000000046);
#[repr(C)]
pub struct IDispatch_Vtbl {
    pub base__: IUnknown_Vtbl,
    pub GetTypeInfoCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> HRESULT,
    pub GetTypeInfo: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> HRESULT,
    pub GetIDsOfNames: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const GUID,
        *const PCWSTR,
        u32,
        u32,
        *mut i32,
    ) -> HRESULT,
    pub Invoke: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        *const GUID,
        u32,
        DISPATCH_FLAGS,
        *const DISPPARAMS,
        *mut VARIANT,
        *mut EXCEPINFO,
        *mut u32,
    ) -> HRESULT,
}
pub type IMPLTYPEFLAGS = i32;
pub type INVOKEKIND = i32;
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
    pub GetTypeInfo:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> HRESULT,
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
pub const IID_ITypeComp: GUID = GUID::from_u128(0x00020403_0000_0000_c000_000000000046);
#[repr(C)]
pub struct ITypeComp_Vtbl {
    pub base__: IUnknown_Vtbl,
    pub Bind: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        PCWSTR,
        u32,
        u16,
        *mut *mut core::ffi::c_void,
        *mut DESCKIND,
        *mut BINDPTR,
    ) -> HRESULT,
    pub BindType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        PCWSTR,
        u32,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> HRESULT,
}
pub const IID_ITypeInfo: GUID = GUID::from_u128(0x00020401_0000_0000_c000_000000000046);
#[repr(C)]
pub struct ITypeInfo_Vtbl {
    pub base__: IUnknown_Vtbl,
    pub GetTypeAttr:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut TYPEATTR) -> HRESULT,
    pub GetTypeComp:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> HRESULT,
    pub GetFuncDesc:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut FUNCDESC) -> HRESULT,
    pub GetVarDesc:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut VARDESC) -> HRESULT,
    pub GetNames:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut BSTR, u32, *mut u32) -> HRESULT,
    pub GetRefTypeOfImplType:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> HRESULT,
    pub GetImplTypeFlags:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut IMPLTYPEFLAGS) -> HRESULT,
    pub GetIDsOfNames:
        unsafe extern "system" fn(*mut core::ffi::c_void, *const PCWSTR, u32, *mut i32) -> HRESULT,
    pub Invoke: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        i32,
        DISPATCH_FLAGS,
        *mut DISPPARAMS,
        *mut VARIANT,
        *mut EXCEPINFO,
        *mut u32,
    ) -> HRESULT,
    pub GetDocumentation: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        *mut BSTR,
        *mut BSTR,
        *mut u32,
        *mut BSTR,
    ) -> HRESULT,
    pub GetDllEntry: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        INVOKEKIND,
        *mut BSTR,
        *mut BSTR,
        *mut u16,
    ) -> HRESULT,
    pub GetRefTypeInfo: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> HRESULT,
    pub AddressOfMember: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        INVOKEKIND,
        *mut *mut core::ffi::c_void,
    ) -> HRESULT,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const GUID,
        *mut *mut core::ffi::c_void,
    ) -> HRESULT,
    pub GetMops: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut BSTR) -> HRESULT,
    pub GetContainingTypeLib: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut u32,
    ) -> HRESULT,
    pub ReleaseTypeAttr: unsafe extern "system" fn(*mut core::ffi::c_void, *const TYPEATTR),
    pub ReleaseFuncDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *const FUNCDESC),
    pub ReleaseVarDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *const VARDESC),
}
pub const IID_ITypeLib: GUID = GUID::from_u128(0x00020402_0000_0000_c000_000000000046);
#[repr(C)]
pub struct ITypeLib_Vtbl {
    pub base__: IUnknown_Vtbl,
    pub GetTypeInfoCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetTypeInfo: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> HRESULT,
    pub GetTypeInfoType:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut TYPEKIND) -> HRESULT,
    pub GetTypeInfoOfGuid: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const GUID,
        *mut *mut core::ffi::c_void,
    ) -> HRESULT,
    pub GetLibAttr:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut TLIBATTR) -> HRESULT,
    pub GetTypeComp:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> HRESULT,
    pub GetDocumentation: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        *mut BSTR,
        *mut BSTR,
        *mut u32,
        *mut BSTR,
    ) -> HRESULT,
    pub IsName: unsafe extern "system" fn(*mut core::ffi::c_void, PWSTR, u32, *mut BOOL) -> HRESULT,
    pub FindName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        PWSTR,
        u32,
        *mut *mut core::ffi::c_void,
        *mut i32,
        *mut u16,
    ) -> HRESULT,
    pub ReleaseTLibAttr: unsafe extern "system" fn(*mut core::ffi::c_void, *const TLIBATTR),
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
pub type LPEXCEPFINO_DEFERRED_FILLIN =
    Option<unsafe extern "system" fn(pexcepinfo: *mut EXCEPINFO) -> HRESULT>;
#[repr(C)]
pub struct PARAMDESC {
    pub pparamdescex: *mut PARAMDESCEX,
    pub wParamFlags: PARAMFLAGS,
}
impl Copy for PARAMDESC {}
impl Clone for PARAMDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PARAMDESCEX {
    pub cBytes: u32,
    pub varDefaultValue: VARIANT,
}
impl Copy for PARAMDESCEX {}
impl Clone for PARAMDESCEX {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PARAMFLAGS = u16;
pub type PCWSTR = *const u16;
pub type PSTR = *mut u8;
pub type PWSTR = *mut u16;
#[repr(C)]
pub struct SAFEARRAY {
    pub cDims: u16,
    pub fFeatures: ADVANCED_FEATURE_FLAGS,
    pub cbElements: u32,
    pub cLocks: u32,
    pub pvData: *mut core::ffi::c_void,
    pub rgsabound: [SAFEARRAYBOUND; 1],
}
impl Copy for SAFEARRAY {}
impl Clone for SAFEARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SAFEARRAYBOUND {
    pub cElements: u32,
    pub lLbound: i32,
}
impl Copy for SAFEARRAYBOUND {}
impl Clone for SAFEARRAYBOUND {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SYSKIND = i32;
#[repr(C)]
pub struct TLIBATTR {
    pub guid: GUID,
    pub lcid: u32,
    pub syskind: SYSKIND,
    pub wMajorVerNum: u16,
    pub wMinorVerNum: u16,
    pub wLibFlags: u16,
}
impl Copy for TLIBATTR {}
impl Clone for TLIBATTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TYPEATTR {
    pub guid: GUID,
    pub lcid: u32,
    pub dwReserved: u32,
    pub memidConstructor: i32,
    pub memidDestructor: i32,
    pub lpstrSchema: PWSTR,
    pub cbSizeInstance: u32,
    pub typekind: TYPEKIND,
    pub cFuncs: u16,
    pub cVars: u16,
    pub cImplTypes: u16,
    pub cbSizeVft: u16,
    pub cbAlignment: u16,
    pub wTypeFlags: u16,
    pub wMajorVerNum: u16,
    pub wMinorVerNum: u16,
    pub tdescAlias: TYPEDESC,
    pub idldescType: IDLDESC,
}
impl Copy for TYPEATTR {}
impl Clone for TYPEATTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TYPEDESC {
    pub Anonymous: TYPEDESC_0,
    pub vt: VARENUM,
}
impl Copy for TYPEDESC {}
impl Clone for TYPEDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union TYPEDESC_0 {
    pub lptdesc: *mut TYPEDESC,
    pub lpadesc: *mut ARRAYDESC,
    pub hreftype: u32,
}
impl Copy for TYPEDESC_0 {}
impl Clone for TYPEDESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TYPEKIND = i32;
#[repr(C)]
pub struct VARDESC {
    pub memid: i32,
    pub lpstrSchema: PWSTR,
    pub Anonymous: VARDESC_0,
    pub elemdescVar: ELEMDESC,
    pub wVarFlags: VARFLAGS,
    pub varkind: VARKIND,
}
impl Copy for VARDESC {}
impl Clone for VARDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union VARDESC_0 {
    pub oInst: u32,
    pub lpvarValue: *mut VARIANT,
}
impl Copy for VARDESC_0 {}
impl Clone for VARDESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type VARENUM = u16;
pub type VARFLAGS = u16;
#[repr(C)]
pub struct VARIANT {
    pub Anonymous: VARIANT_0,
}
impl Copy for VARIANT {}
impl Clone for VARIANT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union VARIANT_0 {
    pub Anonymous: VARIANT_0_0,
    pub decVal: DECIMAL,
}
impl Copy for VARIANT_0 {}
impl Clone for VARIANT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VARIANT_0_0 {
    pub vt: VARENUM,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: VARIANT_0_0_0,
}
impl Copy for VARIANT_0_0 {}
impl Clone for VARIANT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
impl Copy for VARIANT_0_0_0 {}
impl Clone for VARIANT_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VARIANT_0_0_0_0 {
    pub pvRecord: *mut core::ffi::c_void,
    pub pRecInfo: *mut core::ffi::c_void,
}
impl Copy for VARIANT_0_0_0_0 {}
impl Clone for VARIANT_0_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type VARIANT_BOOL = i16;
pub type VARKIND = i32;
