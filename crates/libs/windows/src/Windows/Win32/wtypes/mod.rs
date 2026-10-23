#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BSTRBLOB {
    pub cbSize: super::ULONG,
    pub pData: *mut super::BYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLIPDATA {
    pub cbSize: super::ULONG,
    pub ulClipFmt: super::LONG,
    pub pClipData: *mut super::BYTE,
}
pub type CLIPFORMAT = u16;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CSPLATFORM {
    pub dwPlatformId: super::DWORD,
    pub dwVersionHi: super::DWORD,
    pub dwVersionLo: super::DWORD,
    pub dwProcessorArch: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union CY {
    pub Anonymous: CY_0,
    pub int64: super::LONGLONG,
}
#[cfg(feature = "shared")]
impl Default for CY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CY_0 {
    pub Lo: super::ULONG,
    pub Hi: super::LONG,
}
pub type DATE = f64;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DECIMAL {
    pub wReserved: super::USHORT,
    pub Anonymous: DECIMAL_0,
    pub Hi32: super::ULONG,
    pub Anonymous2: DECIMAL_1,
}
#[cfg(feature = "shared")]
impl Default for DECIMAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DECIMAL_0 {
    pub Anonymous: DECIMAL_0_0,
    pub signscale: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for DECIMAL_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DECIMAL_0_0 {
    pub scale: super::BYTE,
    pub sign: super::BYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DECIMAL_1 {
    pub Anonymous: DECIMAL_1_0,
    pub Lo64: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for DECIMAL_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DECIMAL_1_0 {
    pub Lo32: super::ULONG,
    pub Mid32: super::ULONG,
}
#[cfg(feature = "shared")]
pub const DECIMAL_NEG: super::BYTE = 128;
pub type DVASPECT = i32;
pub const DVASPECT_CONTENT: DVASPECT = 1;
pub const DVASPECT_DOCPRINT: DVASPECT = 8;
pub const DVASPECT_ICON: DVASPECT = 4;
pub const DVASPECT_THUMBNAIL: DVASPECT = 2;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct GDI_NONREMOTE {
    pub fContext: super::LONG,
    pub u: GDI_NONREMOTE_0,
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
impl Default for GDI_NONREMOTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub union GDI_NONREMOTE_0 {
    pub hInproc: super::LONG,
    pub hRemote: *mut super::DWORD_BLOB,
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
impl Default for GDI_NONREMOTE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type HCONTEXT = *mut core::ffi::c_void;
pub type HMETAFILEPICT = *mut core::ffi::c_void;
pub type LPBSTR = *mut *mut u16;
#[cfg(feature = "shared")]
pub type LPBSTRBLOB = *mut BSTRBLOB;
#[cfg(feature = "shared")]
pub type LPCY = *mut CY;
#[cfg(feature = "shared")]
pub type LPDECIMAL = *mut DECIMAL;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROPERTYKEY {
    pub fmtid: windows_core::GUID,
    pub pid: super::DWORD,
}
pub type PROPID = u32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QUERYCONTEXT {
    pub dwContext: super::DWORD,
    pub Platform: CSPLATFORM,
    pub Locale: super::LCID,
    pub dwVersionHi: super::DWORD,
    pub dwVersionLo: super::DWORD,
}
pub const ROTFLAGS_ALLOWANYCLIENT: i32 = 2;
pub const ROTFLAGS_REGISTRATIONKEEPSALIVE: i32 = 1;
pub const ROT_COMPARE_MAX: i32 = 2048;
#[repr(C)]
#[cfg(all(feature = "rpc", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RemHBITMAP {
    pub cbData: super::ULONG,
    pub data: [super::byte; 1],
}
#[cfg(all(feature = "rpc", feature = "shared"))]
impl Default for RemHBITMAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "rpc", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RemHBRUSH {
    pub cbData: super::ULONG,
    pub data: [super::byte; 1],
}
#[cfg(all(feature = "rpc", feature = "shared"))]
impl Default for RemHBRUSH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "rpc", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RemHENHMETAFILE {
    pub cbData: super::ULONG,
    pub data: [super::byte; 1],
}
#[cfg(all(feature = "rpc", feature = "shared"))]
impl Default for RemHENHMETAFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "rpc", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RemHGLOBAL {
    pub fNullHGlobal: super::LONG,
    pub cbData: super::ULONG,
    pub data: [super::byte; 1],
}
#[cfg(all(feature = "rpc", feature = "shared"))]
impl Default for RemHGLOBAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "rpc", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RemHMETAFILEPICT {
    pub mm: super::LONG,
    pub xExt: super::LONG,
    pub yExt: super::LONG,
    pub cbData: super::ULONG,
    pub data: [super::byte; 1],
}
#[cfg(all(feature = "rpc", feature = "shared"))]
impl Default for RemHMETAFILEPICT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "rpc", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RemHPALETTE {
    pub cbData: super::ULONG,
    pub data: [super::byte; 1],
}
#[cfg(all(feature = "rpc", feature = "shared"))]
impl Default for RemHPALETTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct RemotableHandle {
    pub fContext: super::LONG,
    pub u: RemotableHandle_0,
}
#[cfg(feature = "shared")]
impl Default for RemotableHandle {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union RemotableHandle_0 {
    pub hInproc: super::LONG,
    pub hRemote: super::LONG,
}
#[cfg(feature = "shared")]
impl Default for RemotableHandle_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type STATFLAG = i32;
pub const STATFLAG_DEFAULT: STATFLAG = 0;
pub const STATFLAG_NONAME: STATFLAG = 1;
pub const STATFLAG_NOOPEN: STATFLAG = 2;
pub type STGC = i32;
pub const STGC_CONSOLIDATE: STGC = 8;
pub const STGC_DANGEROUSLYCOMMITMERELYTODISKCACHE: STGC = 4;
pub const STGC_DEFAULT: STGC = 0;
pub const STGC_ONLYIFCURRENT: STGC = 2;
pub const STGC_OVERWRITE: STGC = 1;
pub type STGMOVE = i32;
pub const STGMOVE_COPY: STGMOVE = 1;
pub const STGMOVE_MOVE: STGMOVE = 0;
pub const STGMOVE_SHALLOWCOPY: STGMOVE = 2;
pub type TYSPEC = i32;
pub const TYSPEC_CLSID: TYSPEC = 0;
pub const TYSPEC_FILEEXT: TYSPEC = 1;
pub const TYSPEC_FILENAME: TYSPEC = 3;
pub const TYSPEC_MIMETYPE: TYSPEC = 2;
pub const TYSPEC_OBJECTID: TYSPEC = 6;
pub const TYSPEC_PACKAGENAME: TYSPEC = 5;
pub const TYSPEC_PROGID: TYSPEC = 4;
pub type VARENUM = i32;
pub type VARIANT_BOOL = i16;
pub const VARIANT_FALSE: VARIANT_BOOL = 0;
pub const VARIANT_TRUE: VARIANT_BOOL = -1;
pub type VARTYPE = u16;
pub const VT_ARRAY: VARENUM = 8192;
pub const VT_BLOB: VARENUM = 65;
pub const VT_BLOB_OBJECT: VARENUM = 70;
pub const VT_BOOL: VARENUM = 11;
pub const VT_BSTR: VARENUM = 8;
pub const VT_BSTR_BLOB: VARENUM = 4095;
pub const VT_BYREF: VARENUM = 16384;
pub const VT_CARRAY: VARENUM = 28;
pub const VT_CF: VARENUM = 71;
pub const VT_CLSID: VARENUM = 72;
pub const VT_CY: VARENUM = 6;
pub const VT_DATE: VARENUM = 7;
pub const VT_DECIMAL: VARENUM = 14;
pub const VT_DISPATCH: VARENUM = 9;
pub const VT_EMPTY: VARENUM = 0;
pub const VT_ERROR: VARENUM = 10;
pub const VT_FILETIME: VARENUM = 64;
pub const VT_HRESULT: VARENUM = 25;
pub const VT_I1: VARENUM = 16;
pub const VT_I2: VARENUM = 2;
pub const VT_I4: VARENUM = 3;
pub const VT_I8: VARENUM = 20;
pub const VT_ILLEGAL: VARENUM = 65535;
pub const VT_ILLEGALMASKED: VARENUM = 4095;
pub const VT_INT: VARENUM = 22;
pub const VT_INT_PTR: VARENUM = 37;
pub const VT_LPSTR: VARENUM = 30;
pub const VT_LPWSTR: VARENUM = 31;
pub const VT_NULL: VARENUM = 1;
pub const VT_PTR: VARENUM = 26;
pub const VT_R4: VARENUM = 4;
pub const VT_R8: VARENUM = 5;
pub const VT_RECORD: VARENUM = 36;
pub const VT_RESERVED: VARENUM = 32768;
pub const VT_SAFEARRAY: VARENUM = 27;
pub const VT_STORAGE: VARENUM = 67;
pub const VT_STORED_OBJECT: VARENUM = 69;
pub const VT_STREAM: VARENUM = 66;
pub const VT_STREAMED_OBJECT: VARENUM = 68;
pub const VT_TYPEMASK: VARENUM = 4095;
pub const VT_UI1: VARENUM = 17;
pub const VT_UI2: VARENUM = 18;
pub const VT_UI4: VARENUM = 19;
pub const VT_UI8: VARENUM = 21;
pub const VT_UINT: VARENUM = 23;
pub const VT_UINT_PTR: VARENUM = 38;
pub const VT_UNKNOWN: VARENUM = 13;
pub const VT_USERDEFINED: VARENUM = 29;
pub const VT_VARIANT: VARENUM = 12;
pub const VT_VECTOR: VARENUM = 4096;
pub const VT_VERSIONED_STREAM: VARENUM = 73;
pub const VT_VOID: VARENUM = 24;
pub const WDT_INPROC64_CALL: i32 = 1349805143;
pub const WDT_INPROC_CALL: i32 = 1215587415;
pub const WDT_REMOTE_CALL: i32 = 1383359575;
pub const __REQUIRED_RPCNDR_H_VERSION__: i32 = 501;
pub const __REQUIRED_RPCSAL_H_VERSION__: i32 = 100;
#[repr(C)]
#[cfg(all(feature = "rpc", feature = "shared", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct remoteMETAFILEPICT {
    pub mm: super::LONG,
    pub xExt: super::LONG,
    pub yExt: super::LONG,
    pub hMF: *mut userHMETAFILE,
}
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "shared", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct uCLSSPEC {
    pub tyspec: super::DWORD,
    pub tagged_union: uCLSSPEC_0,
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "wtypesbase"))]
impl Default for uCLSSPEC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "shared", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub union uCLSSPEC_0 {
    pub clsid: super::CLSID,
    pub pFileExt: super::LPOLESTR,
    pub pMimeType: super::LPOLESTR,
    pub pProgId: super::LPOLESTR,
    pub pFileName: super::LPOLESTR,
    pub ByName: uCLSSPEC_0_0,
    pub ByObjectId: uCLSSPEC_0_1,
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "wtypesbase"))]
impl Default for uCLSSPEC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "shared", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct uCLSSPEC_0_0 {
    pub pPackageName: super::LPOLESTR,
    pub PolicyId: windows_core::GUID,
}
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "shared", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct uCLSSPEC_0_1 {
    pub ObjectId: windows_core::GUID,
    pub PolicyId: windows_core::GUID,
}
#[repr(C)]
#[cfg(all(feature = "rpc", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct userBITMAP {
    pub bmType: super::LONG,
    pub bmWidth: super::LONG,
    pub bmHeight: super::LONG,
    pub bmWidthBytes: super::LONG,
    pub bmPlanes: super::WORD,
    pub bmBitsPixel: super::WORD,
    pub cbSize: super::ULONG,
    pub pBuffer: [super::byte; 1],
}
#[cfg(all(feature = "rpc", feature = "shared"))]
impl Default for userBITMAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct userCLIPFORMAT {
    pub fContext: super::LONG,
    pub u: userCLIPFORMAT_0,
}
#[cfg(feature = "shared")]
impl Default for userCLIPFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union userCLIPFORMAT_0 {
    pub dwValue: super::DWORD,
    pub pwszName: *mut u16,
}
#[cfg(feature = "shared")]
impl Default for userCLIPFORMAT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "rpc", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct userHBITMAP {
    pub fContext: super::LONG,
    pub u: userHBITMAP_0,
}
#[cfg(all(feature = "rpc", feature = "shared"))]
impl Default for userHBITMAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "rpc", feature = "shared"))]
#[derive(Clone, Copy)]
pub union userHBITMAP_0 {
    pub hInproc: super::LONG,
    pub hRemote: *mut userBITMAP,
    pub hInproc64: i64,
}
#[cfg(all(feature = "rpc", feature = "shared"))]
impl Default for userHBITMAP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "rpc", feature = "shared", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct userHENHMETAFILE {
    pub fContext: super::LONG,
    pub u: userHENHMETAFILE_0,
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "wtypesbase"))]
impl Default for userHENHMETAFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "rpc", feature = "shared", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub union userHENHMETAFILE_0 {
    pub hInproc: super::LONG,
    pub hRemote: *mut super::BYTE_BLOB,
    pub hInproc64: i64,
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "wtypesbase"))]
impl Default for userHENHMETAFILE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "rpc", feature = "shared", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct userHGLOBAL {
    pub fContext: super::LONG,
    pub u: userHGLOBAL_0,
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "wtypesbase"))]
impl Default for userHGLOBAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "rpc", feature = "shared", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub union userHGLOBAL_0 {
    pub hInproc: super::LONG,
    pub hRemote: *mut super::FLAGGED_BYTE_BLOB,
    pub hInproc64: i64,
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "wtypesbase"))]
impl Default for userHGLOBAL_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "rpc", feature = "shared", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct userHMETAFILE {
    pub fContext: super::LONG,
    pub u: userHMETAFILE_0,
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "wtypesbase"))]
impl Default for userHMETAFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "rpc", feature = "shared", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub union userHMETAFILE_0 {
    pub hInproc: super::LONG,
    pub hRemote: *mut super::BYTE_BLOB,
    pub hInproc64: i64,
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "wtypesbase"))]
impl Default for userHMETAFILE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "rpc", feature = "shared", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct userHMETAFILEPICT {
    pub fContext: super::LONG,
    pub u: userHMETAFILEPICT_0,
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "wtypesbase"))]
impl Default for userHMETAFILEPICT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "rpc", feature = "shared", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub union userHMETAFILEPICT_0 {
    pub hInproc: super::LONG,
    pub hRemote: *mut remoteMETAFILEPICT,
    pub hInproc64: i64,
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "wtypesbase"))]
impl Default for userHMETAFILEPICT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wingdi"))]
#[derive(Clone, Copy)]
pub struct userHPALETTE {
    pub fContext: super::LONG,
    pub u: userHPALETTE_0,
}
#[cfg(all(feature = "shared", feature = "wingdi"))]
impl Default for userHPALETTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wingdi"))]
#[derive(Clone, Copy)]
pub union userHPALETTE_0 {
    pub hInproc: super::LONG,
    pub hRemote: *mut super::LOGPALETTE,
    pub hInproc64: i64,
}
#[cfg(all(feature = "shared", feature = "wingdi"))]
impl Default for userHPALETTE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
pub type wireBSTR = *mut super::FLAGGED_WORD_BLOB;
#[cfg(feature = "shared")]
pub type wireCLIPFORMAT = *mut userCLIPFORMAT;
#[cfg(feature = "shared")]
pub type wireHACCEL = *mut RemotableHandle;
#[cfg(all(feature = "rpc", feature = "shared"))]
pub type wireHBITMAP = *mut userHBITMAP;
#[cfg(feature = "shared")]
pub type wireHBRUSH = *mut RemotableHandle;
#[cfg(feature = "shared")]
pub type wireHDC = *mut RemotableHandle;
#[cfg(all(feature = "rpc", feature = "shared", feature = "wtypesbase"))]
pub type wireHENHMETAFILE = *mut userHENHMETAFILE;
#[cfg(feature = "shared")]
pub type wireHFONT = *mut RemotableHandle;
#[cfg(all(feature = "rpc", feature = "shared", feature = "wtypesbase"))]
pub type wireHGLOBAL = *mut userHGLOBAL;
#[cfg(feature = "shared")]
pub type wireHICON = *mut RemotableHandle;
#[cfg(feature = "shared")]
pub type wireHMENU = *mut RemotableHandle;
#[cfg(all(feature = "rpc", feature = "shared", feature = "wtypesbase"))]
pub type wireHMETAFILE = *mut userHMETAFILE;
#[cfg(all(feature = "rpc", feature = "shared", feature = "wtypesbase"))]
pub type wireHMETAFILEPICT = *mut userHMETAFILEPICT;
#[cfg(feature = "shared")]
pub type wireHMONITOR = *mut RemotableHandle;
#[cfg(all(feature = "shared", feature = "wingdi"))]
pub type wireHPALETTE = *mut userHPALETTE;
#[cfg(feature = "shared")]
pub type wireHRGN = *mut RemotableHandle;
#[cfg(feature = "shared")]
pub type wireHWND = *mut RemotableHandle;
