#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoGetInstanceFromFile(pserverinfo: *const super::COSERVERINFO, pclsid: *const ::windows_sys::core::GUID, punkouter: ::windows_sys::core::IUnknown, dwclsctx: super::CLSCTX, grfmode: u32, pwszname: super::super::super::Foundation::PWSTR, dwcount: u32, presults: *mut super::MULTI_QI) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoGetInstanceFromIStorage(pserverinfo: *const super::COSERVERINFO, pclsid: *const ::windows_sys::core::GUID, punkouter: ::windows_sys::core::IUnknown, dwclsctx: super::CLSCTX, pstg: IStorage, dwcount: u32, presults: *mut super::MULTI_QI) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn CoGetInterfaceAndReleaseStream(pstm: super::IStream, iid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateILockBytesOnHGlobal(hglobal: isize, fdeleteonrelease: super::super::super::Foundation::BOOL, pplkbyt: *mut ILockBytes) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateStreamOnHGlobal(hglobal: isize, fdeleteonrelease: super::super::super::Foundation::BOOL, ppstm: *mut super::IStream) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FmtIdToPropStgName(pfmtid: *const ::windows_sys::core::GUID, oszname: super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreePropVariantArray(cvariants: u32, rgvars: *mut PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn GetConvertStg(pstg: IStorage) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn GetHGlobalFromILockBytes(plkbyt: ILockBytes, phglobal: *mut isize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn GetHGlobalFromStream(pstm: super::IStream, phglobal: *mut isize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn OleConvertIStorageToOLESTREAM(pstg: IStorage, lpolestream: *mut OLESTREAM) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn OleConvertIStorageToOLESTREAMEx(pstg: IStorage, cfformat: u16, lwidth: i32, lheight: i32, dwsize: u32, pmedium: *mut super::STGMEDIUM, polestm: *mut OLESTREAM) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn OleConvertOLESTREAMToIStorage(lpolestream: *mut OLESTREAM, pstg: IStorage, ptd: *const super::DVTARGETDEVICE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn OleConvertOLESTREAMToIStorageEx(polestm: *mut OLESTREAM, pstg: IStorage, pcfformat: *mut u16, plwwidth: *mut i32, plheight: *mut i32, pdwsize: *mut u32, pmedium: *mut super::STGMEDIUM) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PropStgNameToFmtId(oszname: super::super::super::Foundation::PWSTR, pfmtid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PropVariantClear(pvar: *mut PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PropVariantCopy(pvardest: *mut PROPVARIANT, pvarsrc: *const PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn ReadClassStg(pstg: IStorage, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn ReadClassStm(pstm: super::IStream, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadFmtUserTypeStg(pstg: IStorage, pcf: *mut u16, lplpszusertype: *mut super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConvertStg(pstg: IStorage, fconvert: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgConvertPropertyToVariant(pprop: *const SERIALIZEDPROPERTYVALUE, codepage: u16, pvar: *mut PROPVARIANT, pma: *const PMemoryAllocator) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgConvertVariantToProperty(pvar: *const PROPVARIANT, codepage: u16, pprop: *mut SERIALIZEDPROPERTYVALUE, pcb: *mut u32, pid: u32, freserved: super::super::super::Foundation::BOOLEAN, pcindirect: *mut u32) -> *mut SERIALIZEDPROPERTYVALUE;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgCreateDocfile(pwcsname: super::super::super::Foundation::PWSTR, grfmode: u32, reserved: u32, ppstgopen: *mut IStorage) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn StgCreateDocfileOnILockBytes(plkbyt: ILockBytes, grfmode: u32, reserved: u32, ppstgopen: *mut IStorage) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn StgCreatePropSetStg(pstorage: IStorage, dwreserved: u32, pppropsetstg: *mut IPropertySetStorage) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn StgCreatePropStg(punk: ::windows_sys::core::IUnknown, fmtid: *const ::windows_sys::core::GUID, pclsid: *const ::windows_sys::core::GUID, grfflags: u32, dwreserved: u32, pppropstg: *mut IPropertyStorage) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn StgCreateStorageEx(pwcsname: super::super::super::Foundation::PWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, pstgoptions: *mut STGOPTIONS, psecuritydescriptor: *const super::super::super::Security::SECURITY_DESCRIPTOR, riid: *const ::windows_sys::core::GUID, ppobjectopen: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgDeserializePropVariant(pprop: *const SERIALIZEDPROPERTYVALUE, cbmax: u32, ppropvar: *mut PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgGetIFillLockBytesOnFile(pwcsname: super::super::super::Foundation::PWSTR, ppflb: *mut IFillLockBytes) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn StgGetIFillLockBytesOnILockBytes(pilb: ILockBytes, ppflb: *mut IFillLockBytes) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgIsStorageFile(pwcsname: super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn StgIsStorageILockBytes(plkbyt: ILockBytes) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn StgOpenAsyncDocfileOnIFillLockBytes(pflb: IFillLockBytes, grfmode: u32, asyncflags: u32, ppstgopen: *mut IStorage) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgOpenLayoutDocfile(pwcsdfname: super::super::super::Foundation::PWSTR, grfmode: u32, reserved: u32, ppstgopen: *mut IStorage) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn StgOpenPropStg(punk: ::windows_sys::core::IUnknown, fmtid: *const ::windows_sys::core::GUID, grfflags: u32, dwreserved: u32, pppropstg: *mut IPropertyStorage) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgOpenStorage(pwcsname: super::super::super::Foundation::PWSTR, pstgpriority: IStorage, grfmode: u32, snbexclude: *const *const u16, reserved: u32, ppstgopen: *mut IStorage) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn StgOpenStorageEx(pwcsname: super::super::super::Foundation::PWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, pstgoptions: *mut STGOPTIONS, psecuritydescriptor: *const super::super::super::Security::SECURITY_DESCRIPTOR, riid: *const ::windows_sys::core::GUID, ppobjectopen: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn StgOpenStorageOnILockBytes(plkbyt: ILockBytes, pstgpriority: IStorage, grfmode: u32, snbexclude: *const *const u16, reserved: u32, ppstgopen: *mut IStorage) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn StgPropertyLengthAsVariant(pprop: *const SERIALIZEDPROPERTYVALUE, cbprop: u32, codepage: u16, breserved: u8) -> u32;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgSerializePropVariant(ppropvar: *const PROPVARIANT, ppprop: *mut *mut SERIALIZEDPROPERTYVALUE, pcb: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgSetTimes(lpszname: super::super::super::Foundation::PWSTR, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn WriteClassStg(pstg: IStorage, rclsid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn WriteClassStm(pstm: super::IStream, rclsid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteFmtUserTypeStg(pstg: IStorage, cf: u16, lpszusertype: super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
}
pub struct BSTRBLOB(i32);
pub struct CABOOL(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CABSTR(i32);
pub struct CABSTRBLOB(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CAC(i32);
pub struct CACLIPDATA(i32);
pub struct CACLSID(i32);
pub struct CACY(i32);
pub struct CADATE(i32);
pub struct CADBL(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CAFILETIME(i32);
pub struct CAFLT(i32);
pub struct CAH(i32);
pub struct CAI(i32);
pub struct CAL(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CALPSTR(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CALPWSTR(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CAPROPVARIANT(i32);
pub struct CASCODE(i32);
pub struct CAUB(i32);
pub struct CAUH(i32);
pub struct CAUI(i32);
pub struct CAUL(i32);
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const CCH_MAX_PROPSTG_NAME: u32 = 31u32;
pub struct CLIPDATA(i32);
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const CWCSTORAGENAME: u32 = 32u32;
pub struct IDirectWriterLock(pub *mut ::core::ffi::c_void);
pub struct IEnumSTATPROPSETSTG(pub *mut ::core::ffi::c_void);
pub struct IEnumSTATPROPSTG(pub *mut ::core::ffi::c_void);
pub struct IEnumSTATSTG(pub *mut ::core::ffi::c_void);
pub struct IFillLockBytes(pub *mut ::core::ffi::c_void);
pub struct ILayoutStorage(pub *mut ::core::ffi::c_void);
pub struct ILockBytes(pub *mut ::core::ffi::c_void);
pub struct IPersistStorage(pub *mut ::core::ffi::c_void);
pub struct IPropertyBag(pub *mut ::core::ffi::c_void);
pub struct IPropertyBag2(pub *mut ::core::ffi::c_void);
pub struct IPropertySetStorage(pub *mut ::core::ffi::c_void);
pub struct IPropertyStorage(pub *mut ::core::ffi::c_void);
pub struct IRootStorage(pub *mut ::core::ffi::c_void);
pub struct IStorage(pub *mut ::core::ffi::c_void);
pub struct LOCKTYPE(i32);
pub struct OLESTREAM(i32);
pub struct OLESTREAMVTBL(i32);
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDDI_THUMBNAIL: i32 = 2i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDDSI_BYTECOUNT: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDDSI_CATEGORY: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDDSI_COMPANY: u32 = 15u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDDSI_DOCPARTS: u32 = 13u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDDSI_HEADINGPAIR: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDDSI_HIDDENCOUNT: u32 = 9u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDDSI_LINECOUNT: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDDSI_LINKSDIRTY: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDDSI_MANAGER: u32 = 14u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDDSI_MMCLIPCOUNT: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDDSI_NOTECOUNT: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDDSI_PARCOUNT: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDDSI_PRESFORMAT: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDDSI_SCALE: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDDSI_SLIDECOUNT: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDMSI_COPYRIGHT: i32 = 11i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDMSI_EDITOR: i32 = 2i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDMSI_OWNER: i32 = 8i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDMSI_PRODUCTION: i32 = 10i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDMSI_PROJECT: i32 = 6i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDMSI_RATING: i32 = 9i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDMSI_SEQUENCE_NO: i32 = 5i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDMSI_SOURCE: i32 = 4i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDMSI_STATUS: i32 = 7i32;
pub struct PIDMSI_STATUS_VALUE(i32);
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDMSI_SUPPLIER: i32 = 3i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_APPNAME: i32 = 18i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_AUTHOR: i32 = 4i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_CHARCOUNT: i32 = 16i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_COMMENTS: i32 = 6i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_CREATE_DTM: i32 = 12i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_DOC_SECURITY: i32 = 19i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_EDITTIME: i32 = 10i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_KEYWORDS: i32 = 5i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_LASTAUTHOR: i32 = 8i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_LASTPRINTED: i32 = 11i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_LASTSAVE_DTM: i32 = 13i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_PAGECOUNT: i32 = 14i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_REVNUMBER: i32 = 9i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_SUBJECT: i32 = 3i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_TEMPLATE: i32 = 7i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_THUMBNAIL: i32 = 17i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_TITLE: i32 = 2i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_WORDCOUNT: i32 = 15i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PID_BEHAVIOR: u32 = 2147483651u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PID_CODEPAGE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PID_DICTIONARY: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PID_FIRST_NAME_DEFAULT: u32 = 4095u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PID_FIRST_USABLE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PID_ILLEGAL: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PID_LOCALE: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PID_MAX_READONLY: u32 = 3221225471u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PID_MIN_READONLY: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PID_MODIFY_TIME: u32 = 2147483649u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PID_SECURITY: u32 = 2147483650u32;
pub struct PMemoryAllocator(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PROPBAG2(i32);
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PROPSETFLAG_ANSI: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PROPSETFLAG_CASE_SENSITIVE: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PROPSETFLAG_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PROPSETFLAG_NONSIMPLE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PROPSETFLAG_UNBUFFERED: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PROPSETHDR_OSVERSION_UNKNOWN: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PROPSET_BEHAVIOR_CASE_SENSITIVE: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct PROPSPEC(i32);
pub struct PROPSPEC_KIND(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PROPVARIANT(i32);
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PRSPEC_INVALID: u32 = 4294967295u32;
pub struct RemSNB(i32);
pub struct SERIALIZEDPROPERTYVALUE(i32);
pub struct STATFLAG(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct STATPROPSETSTG(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct STATPROPSTG(i32);
pub struct STGC(i32);
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGFMT_ANY: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGFMT_DOCFILE: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGFMT_DOCUMENT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGFMT_FILE: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGFMT_NATIVE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGFMT_STORAGE: u32 = 0u32;
pub struct STGMOVE(i32);
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_CONVERT: i32 = 131072i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_CREATE: i32 = 4096i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_DELETEONRELEASE: i32 = 67108864i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_DIRECT: i32 = 0i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_DIRECT_SWMR: i32 = 4194304i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_FAILIFTHERE: i32 = 0i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_NOSCRATCH: i32 = 1048576i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_NOSNAPSHOT: i32 = 2097152i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_PRIORITY: i32 = 262144i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_READ: i32 = 0i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_READWRITE: i32 = 2i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_SHARE_DENY_NONE: i32 = 64i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_SHARE_DENY_READ: i32 = 48i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_SHARE_DENY_WRITE: i32 = 32i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_SHARE_EXCLUSIVE: i32 = 16i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_SIMPLE: i32 = 134217728i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_TRANSACTED: i32 = 65536i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_WRITE: i32 = 1i32;
#[cfg(feature = "Win32_Foundation")]
pub struct STGOPTIONS(i32);
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGOPTIONS_VERSION: u32 = 1u32;
pub struct VERSIONEDSTREAM(i32);
