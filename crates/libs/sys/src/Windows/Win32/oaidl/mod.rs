#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[derive(Clone, Copy)]
pub struct ARRAYDESC {
    pub tdescElem: TYPEDESC,
    pub cDims: super::USHORT,
    pub rgbounds: [SAFEARRAYBOUND; 1],
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
impl Default for ARRAYDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub union BINDPTR {
    pub lpfuncdesc: *mut FUNCDESC,
    pub lpvardesc: *mut VARDESC,
    pub lptcomp: *mut core::ffi::c_void,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct CLEANLOCALSTORAGE {
    pub pInterface: *mut core::ffi::c_void,
    pub pStorage: super::PVOID,
    pub flags: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub type CURRENCY = super::CY;
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct CUSTDATA {
    pub cCustData: super::DWORD,
    pub prgCustData: LPCUSTDATAITEM,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct CUSTDATAITEM {
    pub guid: windows_sys::core::GUID,
    pub varValue: VARIANTARG,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
pub const DISPID_COLLECT: i32 = -8;
pub const DISPID_CONSTRUCTOR: i32 = -6;
pub const DISPID_DESTRUCTOR: i32 = -7;
pub const DISPID_EVALUATE: i32 = -5;
pub const DISPID_NEWENUM: i32 = -4;
pub const DISPID_PROPERTYPUT: i32 = -3;
pub const DISPID_UNKNOWN: i32 = -1;
pub const DISPID_VALUE: i32 = 0;
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct DISPPARAMS {
    pub rgvarg: *mut VARIANTARG,
    pub rgdispidNamedArgs: *mut super::DISPID,
    pub cArgs: super::UINT,
    pub cNamedArgs: super::UINT,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct ELEMDESC {
    pub tdesc: TYPEDESC,
    pub Anonymous: ELEMDESC_0,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Default for ELEMDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub union ELEMDESC_0 {
    pub idldesc: IDLDESC,
    pub paramdesc: PARAMDESC,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Default for ELEMDESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt", feature = "wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct EXCEPINFO {
    pub wCode: super::WORD,
    pub wReserved: super::WORD,
    pub bstrSource: windows_sys::core::BSTR,
    pub bstrDescription: windows_sys::core::BSTR,
    pub bstrHelpFile: windows_sys::core::BSTR,
    pub dwHelpContext: super::DWORD,
    pub pvReserved: super::PVOID,
    pub pfnDeferredFillIn: Option<unsafe extern "system" fn(*mut Self) -> windows_sys::core::HRESULT>,
    pub scode: super::SCODE,
}
pub const FADF_AUTO: i32 = 1;
pub const FADF_BSTR: i32 = 256;
pub const FADF_DISPATCH: i32 = 1024;
pub const FADF_EMBEDDED: i32 = 4;
pub const FADF_FIXEDSIZE: i32 = 16;
pub const FADF_HAVEIID: i32 = 64;
pub const FADF_HAVEVARTYPE: i32 = 128;
pub const FADF_RECORD: i32 = 32;
pub const FADF_RESERVED: i32 = 61448;
pub const FADF_STATIC: i32 = 2;
pub const FADF_UNKNOWN: i32 = 512;
pub const FADF_VARIANT: i32 = 2048;
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct FUNCDESC {
    pub memid: super::MEMBERID,
    pub lprgscode: *mut super::SCODE,
    pub lprgelemdescParam: *mut ELEMDESC,
    pub funckind: FUNCKIND,
    pub invkind: INVOKEKIND,
    pub callconv: CALLCONV,
    pub cParams: super::SHORT,
    pub cParamsOpt: super::SHORT,
    pub oVft: super::SHORT,
    pub cScodes: super::SHORT,
    pub elemdescFunc: ELEMDESC,
    pub wFuncFlags: super::WORD,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct IDLDESC {
    pub dwReserved: super::ULONG_PTR,
    pub wIDLFlags: super::USHORT,
}
pub const IDLFLAG_FIN: i32 = 1;
pub const IDLFLAG_FLCID: i32 = 4;
pub const IDLFLAG_FOUT: i32 = 2;
pub const IDLFLAG_FRETVAL: i32 = 8;
pub const IDLFLAG_NONE: i32 = 0;
pub const IMPLTYPEFLAG_FDEFAULT: i32 = 1;
pub const IMPLTYPEFLAG_FDEFAULTVTABLE: i32 = 8;
pub const IMPLTYPEFLAG_FRESTRICTED: i32 = 4;
pub const IMPLTYPEFLAG_FSOURCE: i32 = 2;
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
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub type LPBINDPTR = *mut BINDPTR;
pub type LPCREATEERRORINFO = *mut core::ffi::c_void;
pub type LPCREATETYPEINFO = *mut core::ffi::c_void;
pub type LPCREATETYPEINFO2 = *mut core::ffi::c_void;
pub type LPCREATETYPELIB = *mut core::ffi::c_void;
pub type LPCREATETYPELIB2 = *mut core::ffi::c_void;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub type LPCUSTDATA = *mut CUSTDATA;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub type LPCUSTDATAITEM = *mut CUSTDATAITEM;
pub type LPDISPATCH = *mut core::ffi::c_void;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub type LPELEMDESC = *mut ELEMDESC;
pub type LPENUMVARIANT = *mut core::ffi::c_void;
pub type LPERRORINFO = *mut core::ffi::c_void;
pub type LPERRORLOG = *mut core::ffi::c_void;
#[cfg(all(feature = "shared", feature = "winnt", feature = "wtypesbase"))]
pub type LPEXCEPINFO = *mut EXCEPINFO;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub type LPFUNCDESC = *mut FUNCDESC;
#[cfg(feature = "shared")]
pub type LPIDLDESC = *mut IDLDESC;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub type LPPARAMDESC = *mut PARAMDESC;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub type LPPARAMDESCEX = *mut PARAMDESCEX;
pub type LPPROPERTYBAG = *mut core::ffi::c_void;
pub type LPRECORDINFO = *mut core::ffi::c_void;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPSAFEARRAY = *mut SAFEARRAY;
#[cfg(feature = "shared")]
pub type LPSAFEARRAYBOUND = *mut SAFEARRAYBOUND;
pub type LPSUPPORTERRORINFO = *mut core::ffi::c_void;
#[cfg(feature = "shared")]
pub type LPTLIBATTR = *mut TLIBATTR;
#[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
pub type LPTYPEATTR = *mut TYPEATTR;
pub type LPTYPECHANGEEVENTS = *mut core::ffi::c_void;
pub type LPTYPECOMP = *mut core::ffi::c_void;
pub type LPTYPEINFO = *mut core::ffi::c_void;
pub type LPTYPEINFO2 = *mut core::ffi::c_void;
pub type LPTYPELIB = *mut core::ffi::c_void;
pub type LPTYPELIB2 = *mut core::ffi::c_void;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub type LPVARDESC = *mut VARDESC;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub type LPVARIANT = *mut tagVARIANT;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub type LPVARIANTARG = *mut tagVARIANT;
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct PARAMDESC {
    pub pparamdescex: LPPARAMDESCEX,
    pub wParamFlags: super::USHORT,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct PARAMDESCEX {
    pub cBytes: super::ULONG,
    pub varDefaultValue: VARIANTARG,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Default for PARAMDESCEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PARAMFLAG_FHASCUSTDATA: i32 = 64;
pub const PARAMFLAG_FHASDEFAULT: i32 = 32;
pub const PARAMFLAG_FIN: i32 = 1;
pub const PARAMFLAG_FLCID: i32 = 4;
pub const PARAMFLAG_FOPT: i32 = 16;
pub const PARAMFLAG_FOUT: i32 = 2;
pub const PARAMFLAG_FRETVAL: i32 = 8;
pub const PARAMFLAG_NONE: i32 = 0;
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct SAFEARRAY {
    pub cDims: super::USHORT,
    pub fFeatures: super::USHORT,
    pub cbElements: super::ULONG,
    pub cLocks: super::ULONG,
    pub pvData: super::PVOID,
    pub rgsabound: [SAFEARRAYBOUND; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SAFEARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SAFEARRAYBOUND {
    pub cElements: super::ULONG,
    pub lLbound: super::LONG,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "rpc", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct SAFEARRAYUNION {
    pub sfType: super::ULONG,
    pub u: SAFEARRAYUNION___MIDL_IOleAutomationTypes_0001,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "rpc", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
impl Default for SAFEARRAYUNION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "rpc", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub union SAFEARRAYUNION___MIDL_IOleAutomationTypes_0001 {
    pub BstrStr: SAFEARR_BSTR,
    pub UnknownStr: SAFEARR_UNKNOWN,
    pub DispatchStr: SAFEARR_DISPATCH,
    pub VariantStr: SAFEARR_VARIANT,
    pub RecordStr: SAFEARR_BRECORD,
    pub HaveIidStr: SAFEARR_HAVEIID,
    pub ByteStr: super::BYTE_SIZEDARR,
    pub WordStr: super::WORD_SIZEDARR,
    pub LongStr: super::DWORD_SIZEDARR,
    pub HyperStr: super::HYPER_SIZEDARR,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "rpc", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
impl Default for SAFEARRAYUNION___MIDL_IOleAutomationTypes_0001 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "rpc", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct SAFEARR_BRECORD {
    pub Size: super::ULONG,
    pub aRecord: *mut wireBRECORD,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct SAFEARR_BSTR {
    pub Size: super::ULONG,
    pub aBstr: *mut super::wireBSTR,
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SAFEARR_DISPATCH {
    pub Size: super::ULONG,
    pub apDispatch: *mut *mut core::ffi::c_void,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "guiddef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct SAFEARR_HAVEIID {
    pub Size: super::ULONG,
    pub apUnknown: *mut *mut core::ffi::c_void,
    pub iid: super::IID,
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SAFEARR_UNKNOWN {
    pub Size: super::ULONG,
    pub apUnknown: *mut *mut core::ffi::c_void,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "rpc", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct SAFEARR_VARIANT {
    pub Size: super::ULONG,
    pub aVariant: *mut wireVARIANT,
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
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TLIBATTR {
    pub guid: windows_sys::core::GUID,
    pub lcid: super::LCID,
    pub syskind: SYSKIND,
    pub wMajorVerNum: super::WORD,
    pub wMinorVerNum: super::WORD,
    pub wLibFlags: super::WORD,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct TYPEATTR {
    pub guid: windows_sys::core::GUID,
    pub lcid: super::LCID,
    pub dwReserved: super::DWORD,
    pub memidConstructor: super::MEMBERID,
    pub memidDestructor: super::MEMBERID,
    pub lpstrSchema: super::LPOLESTR,
    pub cbSizeInstance: super::ULONG,
    pub typekind: TYPEKIND,
    pub cFuncs: super::WORD,
    pub cVars: super::WORD,
    pub cImplTypes: super::WORD,
    pub cbSizeVft: super::WORD,
    pub cbAlignment: super::WORD,
    pub wTypeFlags: super::WORD,
    pub wMajorVerNum: super::WORD,
    pub wMinorVerNum: super::WORD,
    pub tdescAlias: TYPEDESC,
    pub idldescType: IDLDESC,
}
#[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
impl Default for TYPEATTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[derive(Clone, Copy)]
pub struct TYPEDESC {
    pub Anonymous: TYPEDESC_0,
    pub vt: super::VARTYPE,
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
impl Default for TYPEDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[derive(Clone, Copy)]
pub union TYPEDESC_0 {
    pub lptdesc: *mut TYPEDESC,
    pub lpadesc: *mut ARRAYDESC,
    pub hreftype: HREFTYPE,
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
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
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct VARDESC {
    pub memid: super::MEMBERID,
    pub lpstrSchema: super::LPOLESTR,
    pub Anonymous: VARDESC_0,
    pub elemdescVar: ELEMDESC,
    pub wVarFlags: super::WORD,
    pub varkind: VARKIND,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Default for VARDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub union VARDESC_0 {
    pub oInst: super::ULONG,
    pub lpvarValue: *mut VARIANT,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub type VARIANT = tagVARIANT;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub type VARIANTARG = tagVARIANT;
pub type VARKIND = i32;
pub const VAR_CONST: VARKIND = 2;
pub const VAR_DISPATCH: VARKIND = 3;
pub const VAR_PERINSTANCE: VARKIND = 0;
pub const VAR_STATIC: VARKIND = 1;
#[repr(C, packed(8))]
#[cfg(all(feature = "rpc", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct _wireBRECORD {
    pub fFlags: super::ULONG,
    pub clSize: super::ULONG,
    pub pRecInfo: *mut core::ffi::c_void,
    pub pRecord: *mut super::byte,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "rpc", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct _wireSAFEARRAY {
    pub cDims: super::USHORT,
    pub fFeatures: super::USHORT,
    pub cbElements: super::ULONG,
    pub cLocks: super::ULONG,
    pub uArrayStructs: SAFEARRAYUNION,
    pub rgsabound: [SAFEARRAYBOUND; 1],
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "rpc", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
impl Default for _wireSAFEARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "rpc", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct _wireVARIANT {
    pub clSize: super::DWORD,
    pub rpcReserved: super::DWORD,
    pub vt: super::USHORT,
    pub wReserved1: super::USHORT,
    pub wReserved2: super::USHORT,
    pub wReserved3: super::USHORT,
    pub Anonymous: _wireVARIANT_0,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "rpc", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
impl Default for _wireVARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "rpc", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub union _wireVARIANT_0 {
    pub llVal: super::LONGLONG,
    pub lVal: super::LONG,
    pub bVal: super::BYTE,
    pub iVal: super::SHORT,
    pub fltVal: super::FLOAT,
    pub dblVal: super::DOUBLE,
    pub boolVal: super::VARIANT_BOOL,
    pub scode: super::SCODE,
    pub cyVal: super::CY,
    pub date: super::DATE,
    pub bstrVal: super::wireBSTR,
    pub punkVal: *mut core::ffi::c_void,
    pub pdispVal: *mut core::ffi::c_void,
    pub parray: wirePSAFEARRAY,
    pub brecVal: wireBRECORD,
    pub pbVal: *mut super::BYTE,
    pub piVal: *mut super::SHORT,
    pub plVal: *mut super::LONG,
    pub pllVal: *mut super::LONGLONG,
    pub pfltVal: *mut super::FLOAT,
    pub pdblVal: *mut super::DOUBLE,
    pub pboolVal: *mut super::VARIANT_BOOL,
    pub pscode: *mut super::SCODE,
    pub pcyVal: *mut super::CY,
    pub pdate: *mut super::DATE,
    pub pbstrVal: *mut super::wireBSTR,
    pub ppunkVal: *mut *mut core::ffi::c_void,
    pub ppdispVal: *mut *mut core::ffi::c_void,
    pub pparray: *mut wirePSAFEARRAY,
    pub pvarVal: *mut wireVARIANT,
    pub cVal: super::CHAR,
    pub uiVal: super::USHORT,
    pub ulVal: super::ULONG,
    pub ullVal: super::ULONGLONG,
    pub intVal: super::INT,
    pub uintVal: super::UINT,
    pub decVal: super::DECIMAL,
    pub pdecVal: *mut super::DECIMAL,
    pub pcVal: *mut super::CHAR,
    pub puiVal: *mut super::USHORT,
    pub pulVal: *mut super::ULONG,
    pub pullVal: *mut super::ULONGLONG,
    pub pintVal: *mut super::INT,
    pub puintVal: *mut super::UINT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "rpc", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
impl Default for _wireVARIANT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct tagVARIANT {
    pub Anonymous: tagVARIANT_0,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Default for tagVARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub union tagVARIANT_0 {
    pub Anonymous: tagVARIANT_0_0,
    pub decVal: super::DECIMAL,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Default for tagVARIANT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct tagVARIANT_0_0 {
    pub vt: super::VARTYPE,
    pub wReserved1: super::WORD,
    pub wReserved2: super::WORD,
    pub wReserved3: super::WORD,
    pub Anonymous: tagVARIANT_0_0_0,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Default for tagVARIANT_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub union tagVARIANT_0_0_0 {
    pub llVal: super::LONGLONG,
    pub lVal: super::LONG,
    pub bVal: super::BYTE,
    pub iVal: super::SHORT,
    pub fltVal: super::FLOAT,
    pub dblVal: super::DOUBLE,
    pub boolVal: super::VARIANT_BOOL,
    pub __OBSOLETE__VARIANT_BOOL: super::VARIANT_BOOL,
    pub scode: super::SCODE,
    pub cyVal: super::CY,
    pub date: super::DATE,
    pub bstrVal: windows_sys::core::BSTR,
    pub punkVal: *mut core::ffi::c_void,
    pub pdispVal: *mut core::ffi::c_void,
    pub parray: *mut SAFEARRAY,
    pub pbVal: *mut super::BYTE,
    pub piVal: *mut super::SHORT,
    pub plVal: *mut super::LONG,
    pub pllVal: *mut super::LONGLONG,
    pub pfltVal: *mut super::FLOAT,
    pub pdblVal: *mut super::DOUBLE,
    pub pboolVal: *mut super::VARIANT_BOOL,
    pub __OBSOLETE__VARIANT_PBOOL: *mut super::VARIANT_BOOL,
    pub pscode: *mut super::SCODE,
    pub pcyVal: *mut super::CY,
    pub pdate: *mut super::DATE,
    pub pbstrVal: *mut windows_sys::core::BSTR,
    pub ppunkVal: *mut *mut core::ffi::c_void,
    pub ppdispVal: *mut *mut core::ffi::c_void,
    pub pparray: *mut *mut SAFEARRAY,
    pub pvarVal: *mut VARIANT,
    pub byref: super::PVOID,
    pub cVal: super::CHAR,
    pub uiVal: super::USHORT,
    pub ulVal: super::ULONG,
    pub ullVal: super::ULONGLONG,
    pub intVal: super::INT,
    pub uintVal: super::UINT,
    pub pdecVal: *mut super::DECIMAL,
    pub pcVal: *mut super::CHAR,
    pub puiVal: *mut super::USHORT,
    pub pulVal: *mut super::ULONG,
    pub pullVal: *mut super::ULONGLONG,
    pub pintVal: *mut super::INT,
    pub puintVal: *mut super::UINT,
    pub Anonymous: tagVARIANT_0_0_0_0,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Default for tagVARIANT_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct tagVARIANT_0_0_0_0 {
    pub pvRecord: super::PVOID,
    pub pRecInfo: *mut core::ffi::c_void,
}
#[cfg(all(feature = "rpc", feature = "shared"))]
pub type wireBRECORD = *mut _wireBRECORD;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "rpc", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
pub type wirePSAFEARRAY = *mut *mut _wireSAFEARRAY;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "rpc", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
pub type wireSAFEARRAY = *mut _wireSAFEARRAY;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "rpc", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
pub type wireVARIANT = *mut _wireVARIANT;
