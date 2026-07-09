#[repr(C)]
#[cfg(feature = "Win32_wtypes")]
#[derive(Clone, Copy)]
pub struct ARRAYDESC {
    pub tdescElem: TYPEDESC,
    pub cDims: u16,
    pub rgbounds: [SAFEARRAYBOUND; 1],
}
#[cfg(feature = "Win32_wtypes")]
impl Default for ARRAYDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub union BINDPTR {
    pub lpfuncdesc: *mut FUNCDESC,
    pub lpvardesc: *mut VARDESC,
    pub lptcomp: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for BINDPTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CALLCONV = i32;
pub const CC_CDECL: CALLCONV = 1;
pub const CC_FASTCALL: CALLCONV = 0;
pub const CC_FPFASTCALL: CALLCONV = 5;
pub const CC_MACPASCAL: CALLCONV = 3;
pub const CC_MAX: CALLCONV = 9;
pub const CC_MPWCDECL: CALLCONV = 7;
pub const CC_MPWPASCAL: CALLCONV = 8;
pub const CC_MSCPASCAL: CALLCONV = 2;
pub const CC_PASCAL: CALLCONV = 2;
pub const CC_STDCALL: CALLCONV = 4;
pub const CC_SYSCALL: CALLCONV = 6;
pub type CHANGEKIND = i32;
pub const CHANGEKIND_ADDMEMBER: CHANGEKIND = 0;
pub const CHANGEKIND_CHANGEFAILED: CHANGEKIND = 6;
pub const CHANGEKIND_DELETEMEMBER: CHANGEKIND = 1;
pub const CHANGEKIND_GENERAL: CHANGEKIND = 4;
pub const CHANGEKIND_INVALIDATE: CHANGEKIND = 5;
pub const CHANGEKIND_MAX: CHANGEKIND = 7;
pub const CHANGEKIND_SETDOCUMENTATION: CHANGEKIND = 3;
pub const CHANGEKIND_SETNAMES: CHANGEKIND = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLEANLOCALSTORAGE {
    pub pInterface: *mut core::ffi::c_void,
    pub pStorage: *mut core::ffi::c_void,
    pub flags: u32,
}
impl Default for CLEANLOCALSTORAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_wtypes")]
pub type CURRENCY = super::wtypes::CY;
#[repr(C)]
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub struct CUSTDATA {
    pub cCustData: u32,
    pub prgCustData: LPCUSTDATAITEM,
}
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for CUSTDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub struct CUSTDATAITEM {
    pub guid: windows_sys::core::GUID,
    pub varValue: VARIANTARG,
}
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for CUSTDATAITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DESCKIND = i32;
pub const DESCKIND_FUNCDESC: DESCKIND = 1;
pub const DESCKIND_IMPLICITAPPOBJ: DESCKIND = 4;
pub const DESCKIND_MAX: DESCKIND = 5;
pub const DESCKIND_NONE: DESCKIND = 0;
pub const DESCKIND_TYPECOMP: DESCKIND = 3;
pub const DESCKIND_VARDESC: DESCKIND = 2;
pub type DISPID = i32;
pub const DISPID_COLLECT: i32 = -8;
pub const DISPID_CONSTRUCTOR: i32 = -6;
pub const DISPID_DESTRUCTOR: i32 = -7;
pub const DISPID_EVALUATE: i32 = -5;
pub const DISPID_NEWENUM: i32 = -4;
pub const DISPID_PROPERTYPUT: i32 = -3;
pub const DISPID_UNKNOWN: i32 = -1;
pub const DISPID_VALUE: u32 = 0;
#[repr(C)]
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub struct DISPPARAMS {
    pub rgvarg: *mut VARIANTARG,
    pub rgdispidNamedArgs: *mut DISPID,
    pub cArgs: u32,
    pub cNamedArgs: u32,
}
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for DISPPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub struct ELEMDESC {
    pub tdesc: TYPEDESC,
    pub Anonymous: ELEMDESC_0,
}
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for ELEMDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub union ELEMDESC_0 {
    pub idldesc: IDLDESC,
    pub paramdesc: PARAMDESC,
}
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for ELEMDESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wtypesbase")]
#[derive(Clone, Copy)]
pub struct EXCEPINFO {
    pub wCode: u16,
    pub wReserved: u16,
    pub bstrSource: windows_sys::core::BSTR,
    pub bstrDescription: windows_sys::core::BSTR,
    pub bstrHelpFile: windows_sys::core::BSTR,
    pub dwHelpContext: u32,
    pub pvReserved: *mut core::ffi::c_void,
    pub pfnDeferredFillIn: *mut u8,
    pub scode: super::wtypesbase::SCODE,
}
#[cfg(feature = "Win32_wtypesbase")]
impl Default for EXCEPINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FADF_AUTO: u32 = 1;
pub const FADF_BSTR: u32 = 256;
pub const FADF_DISPATCH: u32 = 1024;
pub const FADF_EMBEDDED: u32 = 4;
pub const FADF_FIXEDSIZE: u32 = 16;
pub const FADF_HAVEIID: u32 = 64;
pub const FADF_HAVEVARTYPE: u32 = 128;
pub const FADF_RECORD: u32 = 32;
pub const FADF_RESERVED: u32 = 61448;
pub const FADF_STATIC: u32 = 2;
pub const FADF_UNKNOWN: u32 = 512;
pub const FADF_VARIANT: u32 = 2048;
#[repr(C)]
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub struct FUNCDESC {
    pub memid: MEMBERID,
    pub lprgscode: *mut super::wtypesbase::SCODE,
    pub lprgelemdescParam: *mut ELEMDESC,
    pub funckind: FUNCKIND,
    pub invkind: INVOKEKIND,
    pub callconv: CALLCONV,
    pub cParams: i16,
    pub cParamsOpt: i16,
    pub oVft: i16,
    pub cScodes: i16,
    pub elemdescFunc: ELEMDESC,
    pub wFuncFlags: u16,
}
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for FUNCDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FUNCFLAGS = i32;
pub const FUNCFLAG_FBINDABLE: FUNCFLAGS = 4;
pub const FUNCFLAG_FDEFAULTBIND: FUNCFLAGS = 32;
pub const FUNCFLAG_FDEFAULTCOLLELEM: FUNCFLAGS = 256;
pub const FUNCFLAG_FDISPLAYBIND: FUNCFLAGS = 16;
pub const FUNCFLAG_FHIDDEN: FUNCFLAGS = 64;
pub const FUNCFLAG_FIMMEDIATEBIND: FUNCFLAGS = 4096;
pub const FUNCFLAG_FNONBROWSABLE: FUNCFLAGS = 1024;
pub const FUNCFLAG_FREPLACEABLE: FUNCFLAGS = 2048;
pub const FUNCFLAG_FREQUESTEDIT: FUNCFLAGS = 8;
pub const FUNCFLAG_FRESTRICTED: FUNCFLAGS = 1;
pub const FUNCFLAG_FSOURCE: FUNCFLAGS = 2;
pub const FUNCFLAG_FUIDEFAULT: FUNCFLAGS = 512;
pub const FUNCFLAG_FUSESGETLASTERROR: FUNCFLAGS = 128;
pub type FUNCKIND = i32;
pub const FUNC_DISPATCH: FUNCKIND = 4;
pub const FUNC_NONVIRTUAL: FUNCKIND = 2;
pub const FUNC_PUREVIRTUAL: FUNCKIND = 1;
pub const FUNC_STATIC: FUNCKIND = 3;
pub const FUNC_VIRTUAL: FUNCKIND = 0;
pub type HREFTYPE = u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IDLDESC {
    pub dwReserved: usize,
    pub wIDLFlags: u16,
}
pub const IDLFLAG_FIN: u32 = 1;
pub const IDLFLAG_FLCID: u32 = 4;
pub const IDLFLAG_FOUT: u32 = 2;
pub const IDLFLAG_FRETVAL: u32 = 8;
pub const IDLFLAG_NONE: u32 = 0;
pub const IMPLTYPEFLAG_FDEFAULT: u32 = 1;
pub const IMPLTYPEFLAG_FDEFAULTVTABLE: u32 = 8;
pub const IMPLTYPEFLAG_FRESTRICTED: u32 = 4;
pub const IMPLTYPEFLAG_FSOURCE: u32 = 2;
pub type INVOKEKIND = i32;
pub const INVOKE_FUNC: INVOKEKIND = 1;
pub const INVOKE_PROPERTYGET: INVOKEKIND = 2;
pub const INVOKE_PROPERTYPUT: INVOKEKIND = 4;
pub const INVOKE_PROPERTYPUTREF: INVOKEKIND = 8;
pub type LIBFLAGS = i32;
pub const LIBFLAG_FCONTROL: LIBFLAGS = 2;
pub const LIBFLAG_FHASDISKIMAGE: LIBFLAGS = 8;
pub const LIBFLAG_FHIDDEN: LIBFLAGS = 4;
pub const LIBFLAG_FRESTRICTED: LIBFLAGS = 1;
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub type LPBINDPTR = *mut BINDPTR;
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub type LPCUSTDATA = *mut CUSTDATA;
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub type LPCUSTDATAITEM = *mut CUSTDATAITEM;
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub type LPELEMDESC = *mut ELEMDESC;
#[cfg(feature = "Win32_wtypesbase")]
pub type LPEXCEPINFO = *mut EXCEPINFO;
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub type LPFUNCDESC = *mut FUNCDESC;
pub type LPIDLDESC = *mut IDLDESC;
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub type LPPARAMDESC = *mut PARAMDESC;
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub type LPPARAMDESCEX = *mut PARAMDESCEX;
pub type LPSAFEARRAY = *mut SAFEARRAY;
pub type LPSAFEARRAYBOUND = *mut SAFEARRAYBOUND;
#[cfg(feature = "Win32_winnt")]
pub type LPTLIBATTR = *mut TLIBATTR;
#[cfg(all(feature = "Win32_winnt", feature = "Win32_wtypes"))]
pub type LPTYPEATTR = *mut TYPEATTR;
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub type LPVARDESC = *mut VARDESC;
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub type LPVARIANT = *mut VARIANT;
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub type LPVARIANTARG = *mut VARIANT;
pub type MEMBERID = DISPID;
#[repr(C)]
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub struct PARAMDESC {
    pub pparamdescex: LPPARAMDESCEX,
    pub wParamFlags: u16,
}
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for PARAMDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub struct PARAMDESCEX {
    pub cBytes: u32,
    pub varDefaultValue: VARIANTARG,
}
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for PARAMDESCEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PARAMFLAG_FHASCUSTDATA: u32 = 64;
pub const PARAMFLAG_FHASDEFAULT: u32 = 32;
pub const PARAMFLAG_FIN: u32 = 1;
pub const PARAMFLAG_FLCID: u32 = 4;
pub const PARAMFLAG_FOPT: u32 = 16;
pub const PARAMFLAG_FOUT: u32 = 2;
pub const PARAMFLAG_FRETVAL: u32 = 8;
pub const PARAMFLAG_NONE: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SAFEARRAY {
    pub cDims: u16,
    pub fFeatures: u16,
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
#[repr(C)]
#[cfg(all(feature = "Win32_rpcndr", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub struct SAFEARRAYUNION {
    pub sfType: u32,
    pub u: SAFEARRAYUNION_0,
}
#[cfg(all(feature = "Win32_rpcndr", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for SAFEARRAYUNION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_rpcndr", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub union SAFEARRAYUNION_0 {
    pub BstrStr: SAFEARR_BSTR,
    pub UnknownStr: SAFEARR_UNKNOWN,
    pub DispatchStr: SAFEARR_DISPATCH,
    pub VariantStr: SAFEARR_VARIANT,
    pub RecordStr: SAFEARR_BRECORD,
    pub HaveIidStr: SAFEARR_HAVEIID,
    pub ByteStr: super::wtypesbase::BYTE_SIZEDARR,
    pub WordStr: super::wtypesbase::WORD_SIZEDARR,
    pub LongStr: super::wtypesbase::DWORD_SIZEDARR,
    pub HyperStr: super::wtypesbase::HYPER_SIZEDARR,
}
#[cfg(all(feature = "Win32_rpcndr", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for SAFEARRAYUNION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_rpcndr")]
#[derive(Clone, Copy)]
pub struct SAFEARR_BRECORD {
    pub Size: u32,
    pub aRecord: *mut wireBRECORD,
}
#[cfg(feature = "Win32_rpcndr")]
impl Default for SAFEARR_BRECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub struct SAFEARR_BSTR {
    pub Size: u32,
    pub aBstr: *mut super::wtypes::wireBSTR,
}
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for SAFEARR_BSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SAFEARR_DISPATCH {
    pub Size: u32,
    pub apDispatch: *mut *mut core::ffi::c_void,
}
impl Default for SAFEARR_DISPATCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SAFEARR_HAVEIID {
    pub Size: u32,
    pub apUnknown: *mut *mut core::ffi::c_void,
    pub iid: windows_sys::core::GUID,
}
impl Default for SAFEARR_HAVEIID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SAFEARR_UNKNOWN {
    pub Size: u32,
    pub apUnknown: *mut *mut core::ffi::c_void,
}
impl Default for SAFEARR_UNKNOWN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_rpcndr", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub struct SAFEARR_VARIANT {
    pub Size: u32,
    pub aVariant: *mut wireVARIANT,
}
#[cfg(all(feature = "Win32_rpcndr", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for SAFEARR_VARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SF_BSTR: SF_TYPE = 8;
pub const SF_DISPATCH: SF_TYPE = 9;
pub const SF_ERROR: SF_TYPE = 10;
pub const SF_HAVEIID: SF_TYPE = 32781;
pub const SF_I1: SF_TYPE = 16;
pub const SF_I2: SF_TYPE = 2;
pub const SF_I4: SF_TYPE = 3;
pub const SF_I8: SF_TYPE = 20;
pub const SF_RECORD: SF_TYPE = 36;
pub type SF_TYPE = i32;
pub const SF_UNKNOWN: SF_TYPE = 13;
pub const SF_VARIANT: SF_TYPE = 12;
pub type SYSKIND = i32;
pub const SYS_MAC: SYSKIND = 2;
pub const SYS_WIN16: SYSKIND = 0;
pub const SYS_WIN32: SYSKIND = 1;
pub const SYS_WIN64: SYSKIND = 3;
pub const TKIND_ALIAS: TYPEKIND = 6;
pub const TKIND_COCLASS: TYPEKIND = 5;
pub const TKIND_DISPATCH: TYPEKIND = 4;
pub const TKIND_ENUM: TYPEKIND = 0;
pub const TKIND_INTERFACE: TYPEKIND = 3;
pub const TKIND_MAX: TYPEKIND = 8;
pub const TKIND_MODULE: TYPEKIND = 2;
pub const TKIND_RECORD: TYPEKIND = 1;
pub const TKIND_UNION: TYPEKIND = 7;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct TLIBATTR {
    pub guid: windows_sys::core::GUID,
    pub lcid: super::winnt::LCID,
    pub syskind: SYSKIND,
    pub wMajorVerNum: u16,
    pub wMinorVerNum: u16,
    pub wLibFlags: u16,
}
#[repr(C)]
#[cfg(all(feature = "Win32_winnt", feature = "Win32_wtypes"))]
#[derive(Clone, Copy)]
pub struct TYPEATTR {
    pub guid: windows_sys::core::GUID,
    pub lcid: super::winnt::LCID,
    pub dwReserved: u32,
    pub memidConstructor: MEMBERID,
    pub memidDestructor: MEMBERID,
    pub lpstrSchema: windows_sys::core::PWSTR,
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
#[cfg(all(feature = "Win32_winnt", feature = "Win32_wtypes"))]
impl Default for TYPEATTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wtypes")]
#[derive(Clone, Copy)]
pub struct TYPEDESC {
    pub Anonymous: TYPEDESC_0,
    pub vt: super::wtypes::VARTYPE,
}
#[cfg(feature = "Win32_wtypes")]
impl Default for TYPEDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wtypes")]
#[derive(Clone, Copy)]
pub union TYPEDESC_0 {
    pub lptdesc: *mut TYPEDESC,
    pub lpadesc: *mut ARRAYDESC,
    pub hreftype: HREFTYPE,
}
#[cfg(feature = "Win32_wtypes")]
impl Default for TYPEDESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type TYPEFLAGS = i32;
pub const TYPEFLAG_FAGGREGATABLE: TYPEFLAGS = 1024;
pub const TYPEFLAG_FAPPOBJECT: TYPEFLAGS = 1;
pub const TYPEFLAG_FCANCREATE: TYPEFLAGS = 2;
pub const TYPEFLAG_FCONTROL: TYPEFLAGS = 32;
pub const TYPEFLAG_FDISPATCHABLE: TYPEFLAGS = 4096;
pub const TYPEFLAG_FDUAL: TYPEFLAGS = 64;
pub const TYPEFLAG_FHIDDEN: TYPEFLAGS = 16;
pub const TYPEFLAG_FLICENSED: TYPEFLAGS = 4;
pub const TYPEFLAG_FNONEXTENSIBLE: TYPEFLAGS = 128;
pub const TYPEFLAG_FOLEAUTOMATION: TYPEFLAGS = 256;
pub const TYPEFLAG_FPREDECLID: TYPEFLAGS = 8;
pub const TYPEFLAG_FPROXY: TYPEFLAGS = 16384;
pub const TYPEFLAG_FREPLACEABLE: TYPEFLAGS = 2048;
pub const TYPEFLAG_FRESTRICTED: TYPEFLAGS = 512;
pub const TYPEFLAG_FREVERSEBIND: TYPEFLAGS = 8192;
pub type TYPEKIND = i32;
#[repr(C)]
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub struct VARDESC {
    pub memid: MEMBERID,
    pub lpstrSchema: windows_sys::core::PWSTR,
    pub Anonymous: VARDESC_0,
    pub elemdescVar: ELEMDESC,
    pub wVarFlags: u16,
    pub varkind: VARKIND,
}
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for VARDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub union VARDESC_0 {
    pub oInst: u32,
    pub lpvarValue: *mut VARIANT,
}
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for VARDESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type VARFLAGS = i32;
pub const VARFLAG_FBINDABLE: VARFLAGS = 4;
pub const VARFLAG_FDEFAULTBIND: VARFLAGS = 32;
pub const VARFLAG_FDEFAULTCOLLELEM: VARFLAGS = 256;
pub const VARFLAG_FDISPLAYBIND: VARFLAGS = 16;
pub const VARFLAG_FHIDDEN: VARFLAGS = 64;
pub const VARFLAG_FIMMEDIATEBIND: VARFLAGS = 4096;
pub const VARFLAG_FNONBROWSABLE: VARFLAGS = 1024;
pub const VARFLAG_FREADONLY: VARFLAGS = 1;
pub const VARFLAG_FREPLACEABLE: VARFLAGS = 2048;
pub const VARFLAG_FREQUESTEDIT: VARFLAGS = 8;
pub const VARFLAG_FRESTRICTED: VARFLAGS = 128;
pub const VARFLAG_FSOURCE: VARFLAGS = 2;
pub const VARFLAG_FUIDEFAULT: VARFLAGS = 512;
#[repr(C)]
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub struct VARIANT {
    pub Anonymous: VARIANT_0,
}
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for VARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub union VARIANT_0 {
    pub Anonymous: VARIANT_0_0,
    pub decVal: super::wtypes::DECIMAL,
}
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for VARIANT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub struct VARIANT_0_0 {
    pub vt: super::wtypes::VARTYPE,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: VARIANT_0_0_0,
}
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for VARIANT_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub union VARIANT_0_0_0 {
    pub llVal: i64,
    pub lVal: i32,
    pub bVal: u8,
    pub iVal: i16,
    pub fltVal: f32,
    pub dblVal: f64,
    pub boolVal: super::wtypes::VARIANT_BOOL,
    pub __OBSOLETE__VARIANT_BOOL: super::wtypes::VARIANT_BOOL,
    pub scode: super::wtypesbase::SCODE,
    pub cyVal: super::wtypes::CY,
    pub date: f64,
    pub bstrVal: windows_sys::core::BSTR,
    pub punkVal: *mut core::ffi::c_void,
    pub pdispVal: *mut core::ffi::c_void,
    pub parray: *mut SAFEARRAY,
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub plVal: *mut i32,
    pub pllVal: *mut i64,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut super::wtypes::VARIANT_BOOL,
    pub __OBSOLETE__VARIANT_PBOOL: *mut super::wtypes::VARIANT_BOOL,
    pub pscode: *mut super::wtypesbase::SCODE,
    pub pcyVal: *mut super::wtypes::CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut windows_sys::core::BSTR,
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
    pub pdecVal: *mut super::wtypes::DECIMAL,
    pub pcVal: *mut i8,
    pub puiVal: *mut u16,
    pub pulVal: *mut u32,
    pub pullVal: *mut u64,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
    pub Anonymous: VARIANT_0_0_0_0,
}
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for VARIANT_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub struct VARIANT_0_0_0_0 {
    pub pvRecord: *mut core::ffi::c_void,
    pub pRecInfo: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for VARIANT_0_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub type VARIANTARG = VARIANT;
pub type VARKIND = i32;
pub const VAR_CONST: VARKIND = 2;
pub const VAR_DISPATCH: VARKIND = 3;
pub const VAR_PERINSTANCE: VARKIND = 0;
pub const VAR_STATIC: VARKIND = 1;
#[repr(C)]
#[cfg(feature = "Win32_rpcndr")]
#[derive(Clone, Copy)]
pub struct _wireBRECORD {
    pub fFlags: u32,
    pub clSize: u32,
    pub pRecInfo: *mut core::ffi::c_void,
    pub pRecord: *mut super::rpcndr::byte,
}
#[cfg(feature = "Win32_rpcndr")]
impl Default for _wireBRECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_rpcndr", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub struct _wireSAFEARRAY {
    pub cDims: u16,
    pub fFeatures: u16,
    pub cbElements: u32,
    pub cLocks: u32,
    pub uArrayStructs: SAFEARRAYUNION,
    pub rgsabound: [SAFEARRAYBOUND; 1],
}
#[cfg(all(feature = "Win32_rpcndr", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for _wireSAFEARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_rpcndr", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub struct _wireVARIANT {
    pub clSize: u32,
    pub rpcReserved: u32,
    pub vt: u16,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: _wireVARIANT_0,
}
#[cfg(all(feature = "Win32_rpcndr", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for _wireVARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_rpcndr", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub union _wireVARIANT_0 {
    pub llVal: i64,
    pub lVal: i32,
    pub bVal: u8,
    pub iVal: i16,
    pub fltVal: f32,
    pub dblVal: f64,
    pub boolVal: super::wtypes::VARIANT_BOOL,
    pub scode: super::wtypesbase::SCODE,
    pub cyVal: super::wtypes::CY,
    pub date: f64,
    pub bstrVal: super::wtypes::wireBSTR,
    pub punkVal: *mut core::ffi::c_void,
    pub pdispVal: *mut core::ffi::c_void,
    pub parray: wirePSAFEARRAY,
    pub brecVal: wireBRECORD,
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub plVal: *mut i32,
    pub pllVal: *mut i64,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut super::wtypes::VARIANT_BOOL,
    pub pscode: *mut super::wtypesbase::SCODE,
    pub pcyVal: *mut super::wtypes::CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut super::wtypes::wireBSTR,
    pub ppunkVal: *mut *mut core::ffi::c_void,
    pub ppdispVal: *mut *mut core::ffi::c_void,
    pub pparray: *mut wirePSAFEARRAY,
    pub pvarVal: *mut wireVARIANT,
    pub cVal: i8,
    pub uiVal: u16,
    pub ulVal: u32,
    pub ullVal: u64,
    pub intVal: i32,
    pub uintVal: u32,
    pub decVal: super::wtypes::DECIMAL,
    pub pdecVal: *mut super::wtypes::DECIMAL,
    pub pcVal: *mut i8,
    pub puiVal: *mut u16,
    pub pulVal: *mut u32,
    pub pullVal: *mut u64,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
}
#[cfg(all(feature = "Win32_rpcndr", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for _wireVARIANT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_rpcndr")]
pub type wireBRECORD = *mut _wireBRECORD;
#[cfg(all(feature = "Win32_rpcndr", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub type wirePSAFEARRAY = *mut wireSAFEARRAY;
#[cfg(all(feature = "Win32_rpcndr", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub type wireSAFEARRAY = *mut _wireSAFEARRAY;
#[cfg(all(feature = "Win32_rpcndr", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub type wireVARIANT = *mut _wireVARIANT;
