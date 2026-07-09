#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BSTRBLOB {
    pub cbSize: u32,
    pub pData: *mut u8,
}
impl Default for BSTRBLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLIPDATA {
    pub cbSize: u32,
    pub ulClipFmt: i32,
    pub pClipData: *mut u8,
}
impl Default for CLIPDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CLIPFORMAT(pub u16);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CSPLATFORM {
    pub dwPlatformId: u32,
    pub dwVersionHi: u32,
    pub dwVersionLo: u32,
    pub dwProcessorArch: u32,
}
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CY_0 {
    pub Lo: u32,
    pub Hi: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DECIMAL {
    pub wReserved: u16,
    pub Anonymous: DECIMAL_0,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DECIMAL_1_0 {
    pub Lo32: u32,
    pub Mid32: u32,
}
pub const DECIMAL_NEG: u8 = 128;
pub type DVASPECT = i32;
pub const DVASPECT_CONTENT: DVASPECT = 1;
pub const DVASPECT_DOCPRINT: DVASPECT = 8;
pub const DVASPECT_ICON: DVASPECT = 4;
pub const DVASPECT_THUMBNAIL: DVASPECT = 2;
#[repr(C)]
#[cfg(feature = "Win32_wtypesbase")]
#[derive(Clone, Copy)]
pub struct GDI_NONREMOTE {
    pub fContext: i32,
    pub u: GDI_NONREMOTE_0,
}
#[cfg(feature = "Win32_wtypesbase")]
impl Default for GDI_NONREMOTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wtypesbase")]
#[derive(Clone, Copy)]
pub union GDI_NONREMOTE_0 {
    pub hInproc: i32,
    pub hRemote: *mut super::wtypesbase::DWORD_BLOB,
}
#[cfg(feature = "Win32_wtypesbase")]
impl Default for GDI_NONREMOTE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HCONTEXT(pub *mut core::ffi::c_void);
impl HCONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HCONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HMETAFILEPICT(pub *mut core::ffi::c_void);
impl HMETAFILEPICT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HMETAFILEPICT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPBSTR(pub *mut windows_core::BSTR);
impl LPBSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPBSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPBSTRBLOB(pub *mut BSTRBLOB);
impl LPBSTRBLOB {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPBSTRBLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCY(pub *mut CY);
impl LPCY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDECIMAL(pub *mut DECIMAL);
impl LPDECIMAL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPDECIMAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROPERTYKEY {
    pub fmtid: windows_core::GUID,
    pub pid: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PROPID(pub u32);
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QUERYCONTEXT {
    pub dwContext: u32,
    pub Platform: CSPLATFORM,
    pub Locale: super::winnt::LCID,
    pub dwVersionHi: u32,
    pub dwVersionLo: u32,
}
pub const ROTFLAGS_ALLOWANYCLIENT: u32 = 2;
pub const ROTFLAGS_REGISTRATIONKEEPSALIVE: u32 = 1;
pub const ROT_COMPARE_MAX: u32 = 2048;
#[repr(C)]
#[cfg(feature = "Win32_rpc")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RemHBITMAP {
    pub cbData: u32,
    pub data: [super::rpc::byte; 1],
}
#[cfg(feature = "Win32_rpc")]
impl Default for RemHBITMAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_rpc")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RemHBRUSH {
    pub cbData: u32,
    pub data: [super::rpc::byte; 1],
}
#[cfg(feature = "Win32_rpc")]
impl Default for RemHBRUSH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_rpc")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RemHENHMETAFILE {
    pub cbData: u32,
    pub data: [super::rpc::byte; 1],
}
#[cfg(feature = "Win32_rpc")]
impl Default for RemHENHMETAFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_rpc")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RemHGLOBAL {
    pub fNullHGlobal: i32,
    pub cbData: u32,
    pub data: [super::rpc::byte; 1],
}
#[cfg(feature = "Win32_rpc")]
impl Default for RemHGLOBAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_rpc")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RemHMETAFILEPICT {
    pub mm: i32,
    pub xExt: i32,
    pub yExt: i32,
    pub cbData: u32,
    pub data: [super::rpc::byte; 1],
}
#[cfg(feature = "Win32_rpc")]
impl Default for RemHMETAFILEPICT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_rpc")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RemHPALETTE {
    pub cbData: u32,
    pub data: [super::rpc::byte; 1],
}
#[cfg(feature = "Win32_rpc")]
impl Default for RemHPALETTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RemotableHandle {
    pub fContext: i32,
    pub u: RemotableHandle_0,
}
impl Default for RemotableHandle {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RemotableHandle_0 {
    pub hInproc: i32,
    pub hRemote: i32,
}
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct VARIANT_BOOL(pub i16);
pub const VARIANT_FALSE: VARIANT_BOOL = VARIANT_BOOL(0);
pub const VARIANT_TRUE: VARIANT_BOOL = VARIANT_BOOL(-1);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct VARTYPE(pub u16);
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
pub const WDT_INPROC64_CALL: u32 = 1349805143;
pub const WDT_INPROC_CALL: u32 = 1215587415;
pub const WDT_REMOTE_CALL: u32 = 1383359575;
#[repr(C)]
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct remoteMETAFILEPICT {
    pub mm: i32,
    pub xExt: i32,
    pub yExt: i32,
    pub hMF: *mut userHMETAFILE,
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wtypesbase"))]
impl Default for remoteMETAFILEPICT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct uCLSSPEC {
    pub tyspec: u32,
    pub tagged_union: uCLSSPEC_0,
}
impl Default for uCLSSPEC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union uCLSSPEC_0 {
    pub clsid: windows_core::GUID,
    pub pFileExt: windows_core::PWSTR,
    pub pMimeType: windows_core::PWSTR,
    pub pProgId: windows_core::PWSTR,
    pub pFileName: windows_core::PWSTR,
    pub ByName: uCLSSPEC_0_0,
    pub ByObjectId: uCLSSPEC_0_1,
}
impl Default for uCLSSPEC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct uCLSSPEC_0_0 {
    pub pPackageName: windows_core::PWSTR,
    pub PolicyId: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct uCLSSPEC_0_1 {
    pub ObjectId: windows_core::GUID,
    pub PolicyId: windows_core::GUID,
}
#[repr(C)]
#[cfg(feature = "Win32_rpc")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct userBITMAP {
    pub bmType: i32,
    pub bmWidth: i32,
    pub bmHeight: i32,
    pub bmWidthBytes: i32,
    pub bmPlanes: u16,
    pub bmBitsPixel: u16,
    pub cbSize: u32,
    pub pBuffer: [super::rpc::byte; 1],
}
#[cfg(feature = "Win32_rpc")]
impl Default for userBITMAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct userCLIPFORMAT {
    pub fContext: i32,
    pub u: userCLIPFORMAT_0,
}
impl Default for userCLIPFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union userCLIPFORMAT_0 {
    pub dwValue: u32,
    pub pwszName: *mut u16,
}
impl Default for userCLIPFORMAT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_rpc")]
#[derive(Clone, Copy)]
pub struct userHBITMAP {
    pub fContext: i32,
    pub u: userHBITMAP_0,
}
#[cfg(feature = "Win32_rpc")]
impl Default for userHBITMAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_rpc")]
#[derive(Clone, Copy)]
pub union userHBITMAP_0 {
    pub hInproc: i32,
    pub hRemote: *mut userBITMAP,
    pub hInproc64: i64,
}
#[cfg(feature = "Win32_rpc")]
impl Default for userHBITMAP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub struct userHENHMETAFILE {
    pub fContext: i32,
    pub u: userHENHMETAFILE_0,
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wtypesbase"))]
impl Default for userHENHMETAFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub union userHENHMETAFILE_0 {
    pub hInproc: i32,
    pub hRemote: *mut super::wtypesbase::BYTE_BLOB,
    pub hInproc64: i64,
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wtypesbase"))]
impl Default for userHENHMETAFILE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub struct userHGLOBAL {
    pub fContext: i32,
    pub u: userHGLOBAL_0,
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wtypesbase"))]
impl Default for userHGLOBAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub union userHGLOBAL_0 {
    pub hInproc: i32,
    pub hRemote: *mut super::wtypesbase::FLAGGED_BYTE_BLOB,
    pub hInproc64: i64,
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wtypesbase"))]
impl Default for userHGLOBAL_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub struct userHMETAFILE {
    pub fContext: i32,
    pub u: userHMETAFILE_0,
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wtypesbase"))]
impl Default for userHMETAFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub struct userHMETAFILEPICT {
    pub fContext: i32,
    pub u: userHMETAFILEPICT_0,
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wtypesbase"))]
impl Default for userHMETAFILEPICT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub union userHMETAFILEPICT_0 {
    pub hInproc: i32,
    pub hRemote: *mut remoteMETAFILEPICT,
    pub hInproc64: i64,
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wtypesbase"))]
impl Default for userHMETAFILEPICT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub union userHMETAFILE_0 {
    pub hInproc: i32,
    pub hRemote: *mut super::wtypesbase::BYTE_BLOB,
    pub hInproc64: i64,
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wtypesbase"))]
impl Default for userHMETAFILE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wingdi")]
#[derive(Clone, Copy)]
pub struct userHPALETTE {
    pub fContext: i32,
    pub u: userHPALETTE_0,
}
#[cfg(feature = "Win32_wingdi")]
impl Default for userHPALETTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wingdi")]
#[derive(Clone, Copy)]
pub union userHPALETTE_0 {
    pub hInproc: i32,
    pub hRemote: *mut super::wingdi::LOGPALETTE,
    pub hInproc64: i64,
}
#[cfg(feature = "Win32_wingdi")]
impl Default for userHPALETTE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_wtypesbase")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct wireBSTR(pub *mut super::wtypesbase::FLAGGED_WORD_BLOB);
#[cfg(feature = "Win32_wtypesbase")]
impl wireBSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_wtypesbase")]
impl Default for wireBSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct wireCLIPFORMAT(pub *mut userCLIPFORMAT);
impl wireCLIPFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for wireCLIPFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct wireHACCEL(pub *mut RemotableHandle);
impl wireHACCEL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for wireHACCEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_rpc")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct wireHBITMAP(pub *mut userHBITMAP);
#[cfg(feature = "Win32_rpc")]
impl wireHBITMAP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_rpc")]
impl Default for wireHBITMAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct wireHBRUSH(pub *mut RemotableHandle);
impl wireHBRUSH {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for wireHBRUSH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct wireHDC(pub *mut RemotableHandle);
impl wireHDC {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for wireHDC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wtypesbase"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct wireHENHMETAFILE(pub *mut userHENHMETAFILE);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wtypesbase"))]
impl wireHENHMETAFILE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wtypesbase"))]
impl Default for wireHENHMETAFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct wireHFONT(pub *mut RemotableHandle);
impl wireHFONT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for wireHFONT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wtypesbase"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct wireHGLOBAL(pub *mut userHGLOBAL);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wtypesbase"))]
impl wireHGLOBAL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wtypesbase"))]
impl Default for wireHGLOBAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct wireHICON(pub *mut RemotableHandle);
impl wireHICON {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for wireHICON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct wireHMENU(pub *mut RemotableHandle);
impl wireHMENU {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for wireHMENU {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wtypesbase"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct wireHMETAFILE(pub *mut userHMETAFILE);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wtypesbase"))]
impl wireHMETAFILE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wtypesbase"))]
impl Default for wireHMETAFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wtypesbase"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct wireHMETAFILEPICT(pub *mut userHMETAFILEPICT);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wtypesbase"))]
impl wireHMETAFILEPICT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wtypesbase"))]
impl Default for wireHMETAFILEPICT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct wireHMONITOR(pub *mut RemotableHandle);
impl wireHMONITOR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for wireHMONITOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_wingdi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct wireHPALETTE(pub *mut userHPALETTE);
#[cfg(feature = "Win32_wingdi")]
impl wireHPALETTE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_wingdi")]
impl Default for wireHPALETTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct wireHRGN(pub *mut RemotableHandle);
impl wireHRGN {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for wireHRGN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct wireHWND(pub *mut RemotableHandle);
impl wireHWND {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for wireHWND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
