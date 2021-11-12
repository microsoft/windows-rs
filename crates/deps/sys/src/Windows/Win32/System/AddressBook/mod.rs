#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn BuildDisplayTable(lpallocatebuffer: LPALLOCATEBUFFER, lpallocatemore: LPALLOCATEMORE, lpfreebuffer: LPFREEBUFFER, lpmalloc: super::Com::IMalloc, hinstance: super::super::Foundation::HINSTANCE, cpages: u32, lppage: *mut DTPAGE, ulflags: u32, lpptable: *mut IMAPITable, lpptbldata: *mut ITableData) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeIdleRoutine(ftg: *mut ::core::ffi::c_void, lpfnidle: PFNIDLE, lpvidleparam: *mut ::core::ffi::c_void, priidle: i16, csecidle: u32, iroidle: u16, ircidle: u16);
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn CreateIProp(lpinterface: *mut ::windows_sys::core::GUID, lpallocatebuffer: LPALLOCATEBUFFER, lpallocatemore: LPALLOCATEMORE, lpfreebuffer: LPFREEBUFFER, lpvreserved: *mut ::core::ffi::c_void, lpppropdata: *mut IPropData) -> i32;
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn CreateTable(lpinterface: *mut ::windows_sys::core::GUID, lpallocatebuffer: LPALLOCATEBUFFER, lpallocatemore: LPALLOCATEMORE, lpfreebuffer: LPFREEBUFFER, lpvreserved: *mut ::core::ffi::c_void, ultabletype: u32, ulproptagindexcolumn: u32, lpsproptagarraycolumns: *mut SPropTagArray, lpptabledata: *mut ITableData) -> i32;
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn DeinitMapiUtil();
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn DeregisterIdleRoutine(ftg: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnableIdleRoutine(ftg: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL);
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FEqualNames(lpname1: *mut MAPINAMEID, lpname2: *mut MAPINAMEID) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn FPropCompareProp(lpspropvalue1: *mut SPropValue, ulrelop: u32, lpspropvalue2: *mut SPropValue) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn FPropContainsProp(lpspropvaluedst: *mut SPropValue, lpspropvaluesrc: *mut SPropValue, ulfuzzylevel: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FPropExists(lpmapiprop: IMAPIProp, ulproptag: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn FreePadrlist(lpadrlist: *mut ADRLIST);
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn FreeProws(lprows: *mut SRowSet);
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtAddFt(ftaddend1: super::super::Foundation::FILETIME, ftaddend2: super::super::Foundation::FILETIME) -> super::super::Foundation::FILETIME;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtMulDw(ftmultiplier: u32, ftmultiplicand: super::super::Foundation::FILETIME) -> super::super::Foundation::FILETIME;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtMulDwDw(ftmultiplicand: u32, ftmultiplier: u32) -> super::super::Foundation::FILETIME;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtNegFt(ft: super::super::Foundation::FILETIME) -> super::super::Foundation::FILETIME;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtSubFt(ftminuend: super::super::Foundation::FILETIME, ftsubtrahend: super::super::Foundation::FILETIME) -> super::super::Foundation::FILETIME;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtgRegisterIdleRoutine(lpfnidle: PFNIDLE, lpvidleparam: *mut ::core::ffi::c_void, priidle: i16, csecidle: u32, iroidle: u16) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn HrAddColumns(lptbl: IMAPITable, lpproptagcolumnsnew: *mut SPropTagArray, lpallocatebuffer: LPALLOCATEBUFFER, lpfreebuffer: LPFREEBUFFER) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn HrAddColumnsEx(lptbl: IMAPITable, lpproptagcolumnsnew: *mut SPropTagArray, lpallocatebuffer: LPALLOCATEBUFFER, lpfreebuffer: LPFREEBUFFER, lpfnfiltercolumns: isize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HrAllocAdviseSink(lpfncallback: LPNOTIFCALLBACK, lpvcontext: *mut ::core::ffi::c_void, lppadvisesink: *mut IMAPIAdviseSink) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn HrDispatchNotifications(ulflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HrGetOneProp(lpmapiprop: IMAPIProp, ulproptag: u32, lppprop: *mut *mut SPropValue) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn HrIStorageFromStream(lpunkin: ::windows_sys::core::IUnknown, lpinterface: *mut ::windows_sys::core::GUID, ulflags: u32, lppstorageout: *mut super::Com::StructuredStorage::IStorage) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HrQueryAllRows(lptable: IMAPITable, lpproptags: *mut SPropTagArray, lprestriction: *mut SRestriction, lpsortorderset: *mut SSortOrderSet, crowsmax: i32, lpprows: *mut *mut SRowSet) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HrSetOneProp(lpmapiprop: IMAPIProp, lpprop: *mut SPropValue) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn HrThisThreadAdviseSink(lpadvisesink: IMAPIAdviseSink, lppadvisesink: *mut IMAPIAdviseSink) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn LPropCompareProp(lpspropvaluea: *mut SPropValue, lpspropvalueb: *mut SPropValue) -> i32;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn LpValFindProp(ulproptag: u32, cvalues: u32, lpproparray: *mut SPropValue) -> *mut SPropValue;
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn MAPIDeinitIdle();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn MAPIGetDefaultMalloc() -> ::core::option::Option<super::Com::IMalloc>;
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn MAPIInitIdle(lpvreserved: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OpenStreamOnFile(lpallocatebuffer: LPALLOCATEBUFFER, lpfreebuffer: LPFREEBUFFER, ulflags: u32, lpszfilename: *const i8, lpszprefix: *const i8, lppstream: *mut super::Com::IStream) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn PpropFindProp(lpproparray: *mut SPropValue, cvalues: u32, ulproptag: u32) -> *mut SPropValue;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn PropCopyMore(lpspropvaluedest: *mut SPropValue, lpspropvaluesrc: *mut SPropValue, lpfallocmore: LPALLOCATEMORE, lpvobject: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RTFSync(lpmessage: IMessage, ulflags: u32, lpfmessageupdated: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ScCopyNotifications(cnotification: i32, lpnotifications: *mut NOTIFICATION, lpvdst: *mut ::core::ffi::c_void, lpcb: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ScCopyProps(cvalues: i32, lpproparray: *mut SPropValue, lpvdst: *mut ::core::ffi::c_void, lpcb: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ScCountNotifications(cnotifications: i32, lpnotifications: *mut NOTIFICATION, lpcb: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ScCountProps(cvalues: i32, lpproparray: *mut SPropValue, lpcb: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn ScCreateConversationIndex(cbparent: u32, lpbparent: *mut u8, lpcbconvindex: *mut u32, lppbconvindex: *mut *mut u8) -> i32;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ScDupPropset(cvalues: i32, lpproparray: *mut SPropValue, lpallocatebuffer: LPALLOCATEBUFFER, lppproparray: *mut *mut SPropValue) -> i32;
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn ScInitMapiUtil(ulflags: u32) -> i32;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScLocalPathFromUNC(lpszunc: super::super::Foundation::PSTR, lpszlocal: super::super::Foundation::PSTR, cchlocal: u32) -> i32;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ScRelocNotifications(cnotification: i32, lpnotifications: *mut NOTIFICATION, lpvbaseold: *mut ::core::ffi::c_void, lpvbasenew: *mut ::core::ffi::c_void, lpcb: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ScRelocProps(cvalues: i32, lpproparray: *mut SPropValue, lpvbaseold: *mut ::core::ffi::c_void, lpvbasenew: *mut ::core::ffi::c_void, lpcb: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScUNCFromLocalPath(lpszlocal: super::super::Foundation::PSTR, lpszunc: super::super::Foundation::PSTR, cchunc: u32) -> i32;
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn SzFindCh(lpsz: *mut i8, ch: u16) -> *mut i8;
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn SzFindLastCh(lpsz: *mut i8, ch: u16) -> *mut i8;
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn SzFindSz(lpsz: *mut i8, lpszkey: *mut i8) -> *mut i8;
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn UFromSz(lpsz: *mut i8) -> u32;
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn UlAddRef(lpunk: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn UlPropSize(lpspropvalue: *mut SPropValue) -> u32;
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn UlRelease(lpunk: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn WrapCompressedRTFStream(lpcompressedrtfstream: super::Com::IStream, ulflags: u32, lpuncompressedrtfstream: *mut super::Com::IStream) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn WrapStoreEntryID(ulflags: u32, lpszdllname: *const i8, cborigentry: u32, lporigentry: *const ENTRYID, lpcbwrappedentry: *mut u32, lppwrappedentry: *mut *mut ENTRYID) -> ::windows_sys::core::HRESULT;
}
pub struct ADRENTRY(i32);
pub struct ADRLIST(i32);
pub struct ADRPARM(i32);
pub struct CALLERRELEASE(i32);
pub struct DTBLBUTTON(i32);
pub struct DTBLCHECKBOX(i32);
pub struct DTBLCOMBOBOX(i32);
pub struct DTBLDDLBX(i32);
pub struct DTBLEDIT(i32);
pub struct DTBLGROUPBOX(i32);
pub struct DTBLLABEL(i32);
pub struct DTBLLBX(i32);
pub struct DTBLMVDDLBX(i32);
pub struct DTBLMVLISTBOX(i32);
pub struct DTBLPAGE(i32);
pub struct DTBLRADIOBUTTON(i32);
pub struct DTCTL(i32);
pub struct DTPAGE(i32);
pub struct ENTRYID(i32);
pub struct ERROR_NOTIFICATION(i32);
pub struct EXTENDED_NOTIFICATION(i32);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_BURN_VERIFICATION_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600697i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2DATA_CLIENT_NAME_IS_NOT_VALID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599672i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2DATA_INVALID_MEDIA_STATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599678i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2DATA_MEDIA_IS_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599674i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2DATA_MEDIA_NOT_BLANK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599675i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2DATA_RECORDER_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599673i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2DATA_STREAM_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599677i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2DATA_STREAM_TOO_LARGE_FOR_CURRENT_MEDIA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599676i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2DATA_WRITE_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599680i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2DATA_WRITE_NOT_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599679i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2RAW_CLIENT_NAME_IS_NOT_VALID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599164i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2RAW_DATA_BLOCK_TYPE_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599154i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2RAW_MEDIA_IS_NOT_BLANK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599162i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2RAW_MEDIA_IS_NOT_PREPARED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599166i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2RAW_MEDIA_IS_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599161i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2RAW_MEDIA_IS_PREPARED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599165i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2RAW_NOT_ENOUGH_SPACE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599159i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2RAW_NO_RECORDER_SPECIFIED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599158i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2RAW_RECORDER_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599152i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2RAW_STREAM_LEADIN_TOO_SHORT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599153i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2RAW_STREAM_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599155i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2RAW_WRITE_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599168i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2RAW_WRITE_NOT_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599167i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2TAO_CLIENT_NAME_IS_NOT_VALID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599409i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2TAO_INVALID_ISRC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599413i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2TAO_INVALID_MCN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599412i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2TAO_MEDIA_IS_NOT_BLANK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599418i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2TAO_MEDIA_IS_NOT_PREPARED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599422i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2TAO_MEDIA_IS_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599417i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2TAO_MEDIA_IS_PREPARED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599421i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2TAO_NOT_ENOUGH_SPACE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599415i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2TAO_NO_RECORDER_SPECIFIED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599414i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2TAO_PROPERTY_FOR_BLANK_MEDIA_ONLY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599420i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2TAO_RECORDER_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599410i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2TAO_STREAM_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599411i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2TAO_TABLE_OF_CONTENTS_EMPTY_DISC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599419i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2TAO_TRACK_LIMIT_REACHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599416i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2TAO_WRITE_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599424i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_DF2TAO_WRITE_NOT_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599423i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_ERASE_CLIENT_NAME_IS_NOT_VALID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062598389i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_ERASE_DISC_INFORMATION_TOO_SMALL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136340222i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_ERASE_DRIVE_FAILED_ERASE_COMMAND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136340219i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_ERASE_DRIVE_FAILED_SPINUP_COMMAND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136340216i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_ERASE_MEDIA_IS_NOT_ERASABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136340220i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_ERASE_MEDIA_IS_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062598391i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_ERASE_MODE_PAGE_2A_TOO_SMALL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136340221i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_ERASE_ONLY_ONE_RECORDER_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136340223i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_ERASE_RECORDER_IN_USE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136340224i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_ERASE_RECORDER_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062598390i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_ERASE_TOOK_LONGER_THAN_ONE_HOUR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136340218i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_ERASE_UNEXPECTED_DRIVE_RESPONSE_DURING_ERASE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136340217i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_LOSS_OF_STREAMING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599936i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_RAW_IMAGE_INSUFFICIENT_SPACE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136339963i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_RAW_IMAGE_IS_READ_ONLY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136339968i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_RAW_IMAGE_NO_TRACKS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136339965i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_RAW_IMAGE_SECTOR_TYPE_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136339966i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_RAW_IMAGE_TOO_MANY_TRACKS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136339967i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_RAW_IMAGE_TOO_MANY_TRACK_INDEXES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136339962i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_RAW_IMAGE_TRACKS_ALREADY_ADDED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136339964i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_RAW_IMAGE_TRACK_INDEX_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136339961i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_RAW_IMAGE_TRACK_INDEX_OFFSET_ZERO_CANNOT_BE_CLEARED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136339959i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_RAW_IMAGE_TRACK_INDEX_TOO_CLOSE_TO_OTHER_INDEX: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136339958i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_RECORDER_CLIENT_NAME_IS_NOT_VALID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600175i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_RECORDER_COMMAND_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600179i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_RECORDER_DVD_STRUCTURE_NOT_PRESENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600178i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_RECORDER_FEATURE_IS_NOT_CURRENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600181i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_RECORDER_GET_CONFIGURATION_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600180i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_RECORDER_INVALID_MODE_PARAMETERS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600184i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_RECORDER_INVALID_RESPONSE_FROM_DEVICE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599937i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_RECORDER_LOCKED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600176i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_RECORDER_MEDIA_BECOMING_READY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600187i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_RECORDER_MEDIA_BUSY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600185i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_RECORDER_MEDIA_FORMAT_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600186i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_RECORDER_MEDIA_INCOMPATIBLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600189i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_RECORDER_MEDIA_NOT_FORMATTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600174i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_RECORDER_MEDIA_NO_MEDIA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600190i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_RECORDER_MEDIA_SPEED_MISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600177i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_RECORDER_MEDIA_UPSIDE_DOWN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600188i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_RECORDER_MEDIA_WRITE_PROTECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600183i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_RECORDER_NO_SUCH_FEATURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600182i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_RECORDER_NO_SUCH_MODE_PAGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600191i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_RECORDER_REQUIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600701i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_REQUEST_CANCELLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600702i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const E_IMAPI_UNEXPECTED_RESPONSE_FROM_DEVICE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599935i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const FACILITY_IMAPI2: u32 = 170u32;
pub struct FLATENTRY(i32);
pub struct FLATENTRYLIST(i32);
pub struct FLATMTSIDLIST(i32);
pub struct FNIDLE(i32);
pub struct Gender(i32);
pub struct IABContainer(i32);
pub struct IAddrBook(i32);
pub struct IAttach(i32);
pub struct IDistList(i32);
pub struct IMAPIAdviseSink(i32);
pub struct IMAPIContainer(i32);
pub struct IMAPIControl(i32);
pub struct IMAPIFolder(i32);
pub struct IMAPIProgress(i32);
pub struct IMAPIProp(i32);
pub struct IMAPIStatus(i32);
pub struct IMAPITable(i32);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_BAD_MULTISESSION_PARAMETER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555294i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_BOOT_EMULATION_IMAGE_SIZE_MISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555318i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_BOOT_IMAGE_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555320i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_BOOT_OBJECT_CONFLICT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555319i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_DATA_STREAM_CREATE_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555350i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_DATA_STREAM_INCONSISTENCY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555352i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_DATA_STREAM_READ_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555351i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_DATA_TOO_BIG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555342i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_DIRECTORY_READ_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555349i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_DIR_NOT_EMPTY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555382i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_DIR_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555366i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_DISC_MISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555304i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_DUP_NAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555374i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_EMPTY_DISC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555312i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_FILE_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555367i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_FILE_SYSTEM_CHANGE_NOT_ALLOWED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555293i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_FILE_SYSTEM_FEATURE_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555308i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_FILE_SYSTEM_NOT_EMPTY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555386i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_FILE_SYSTEM_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555310i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_FILE_SYSTEM_READ_CONSISTENCY_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555309i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_FSI_INTERNAL_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555392i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_IMAGEMANAGER_IMAGE_NOT_ALIGNED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555136i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_IMAGEMANAGER_IMAGE_TOO_BIG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555133i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_IMAGEMANAGER_NO_IMAGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555134i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_IMAGEMANAGER_NO_VALID_VD_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555135i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_IMAGE_SIZE_LIMIT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555360i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_IMAGE_TOO_BIG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555359i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_IMPORT_MEDIA_NOT_ALLOWED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555303i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_IMPORT_READ_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555305i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_IMPORT_SEEK_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555306i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_IMPORT_TYPE_COLLISION_DIRECTORY_EXISTS_AS_FILE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555298i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_IMPORT_TYPE_COLLISION_FILE_EXISTS_AS_DIRECTORY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555307i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_INCOMPATIBLE_MULTISESSION_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555301i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_INCOMPATIBLE_PREVIOUS_SESSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555341i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_INVALID_DATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555387i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_INVALID_PARAM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555391i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_INVALID_PATH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555376i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_INVALID_VOLUME_NAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555388i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_INVALID_WORKING_DIRECTORY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555328i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_ISO9660_LEVELS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555343i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_ITEM_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555368i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_MULTISESSION_NOT_SET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555299i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_NOT_DIR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555383i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_NOT_FILE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555384i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_NOT_IN_FILE_SYSTEM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555381i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_NO_COMPATIBLE_MULTISESSION_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555300i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_NO_OUTPUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555389i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_NO_SUPPORTED_FILE_SYSTEM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555311i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_NO_UNIQUE_NAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555373i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_PROPERTY_NOT_ACCESSIBLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555296i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_READONLY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555390i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_RESTRICTED_NAME_VIOLATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555375i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_STASHFILE_MOVE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555326i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_STASHFILE_OPEN_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555336i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_STASHFILE_READ_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555333i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_STASHFILE_SEEK_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555335i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_STASHFILE_WRITE_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555334i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_TOO_MANY_DIRS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555344i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_UDF_NOT_WRITE_COMPATIBLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555302i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_UDF_REVISION_CHANGE_NOT_ALLOWED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555295i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_E_WORKING_DIRECTORY_SPACE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555327i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const IMAPI_S_IMAGE_FEATURE_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(11186527i32 as _);
pub struct IMailUser(i32);
pub struct IMessage(i32);
pub struct IMsgStore(i32);
pub struct IProfSect(i32);
pub struct IPropData(i32);
pub struct IProviderAdmin(i32);
pub struct ITableData(i32);
pub struct IWABExtInit(i32);
pub struct IWABOBJECT_(i32);
pub struct IWABOBJECT_AddRef_METHOD(i32);
pub struct IWABOBJECT_AllocateBuffer_METHOD(i32);
pub struct IWABOBJECT_AllocateMore_METHOD(i32);
pub struct IWABOBJECT_Backup_METHOD(i32);
pub struct IWABOBJECT_Find_METHOD(i32);
pub struct IWABOBJECT_FreeBuffer_METHOD(i32);
pub struct IWABOBJECT_GetLastError_METHOD(i32);
pub struct IWABOBJECT_GetMe_METHOD(i32);
pub struct IWABOBJECT_Import_METHOD(i32);
pub struct IWABOBJECT_LDAPUrl_METHOD(i32);
pub struct IWABOBJECT_QueryInterface_METHOD(i32);
pub struct IWABOBJECT_Release_METHOD(i32);
pub struct IWABOBJECT_SetMe_METHOD(i32);
pub struct IWABOBJECT_VCardCreate_METHOD(i32);
pub struct IWABOBJECT_VCardDisplay_METHOD(i32);
pub struct IWABOBJECT_VCardRetrieve_METHOD(i32);
pub struct IWABObject(i32);
pub struct LPALLOCATEBUFFER(i32);
pub struct LPALLOCATEMORE(i32);
pub struct LPCREATECONVERSATIONINDEX(i32);
pub struct LPDISPATCHNOTIFICATIONS(i32);
pub struct LPFNABSDI(i32);
pub struct LPFNBUTTON(i32);
pub struct LPFNDISMISS(i32);
pub struct LPFREEBUFFER(i32);
pub struct LPNOTIFCALLBACK(i32);
pub struct LPOPENSTREAMONFILE(i32);
pub struct LPWABALLOCATEBUFFER(i32);
pub struct LPWABALLOCATEMORE(i32);
pub struct LPWABFREEBUFFER(i32);
pub struct LPWABOPEN(i32);
pub struct LPWABOPENEX(i32);
pub struct MAPIERROR(i32);
pub struct MAPINAMEID(i32);
pub struct MAPIUID(i32);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const MAPI_COMPOUND: u32 = 128u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const MAPI_DIM: u32 = 1u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const MAPI_ERROR_VERSION: i32 = 0i32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const MAPI_E_CALL_FAILED: i32 = -2147467259i32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const MAPI_E_INTERFACE_NOT_SUPPORTED: i32 = -2147467262i32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const MAPI_E_INVALID_PARAMETER: i32 = -2147024809i32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const MAPI_E_NOT_ENOUGH_MEMORY: i32 = -2147024882i32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const MAPI_E_NO_ACCESS: i32 = -2147024891i32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const MAPI_NOTRECIP: u32 = 64u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const MAPI_NOTRESERVED: u32 = 8u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const MAPI_NOW: u32 = 16u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const MAPI_ONE_OFF_NO_RICH_INFO: u32 = 1u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const MAPI_P1: u32 = 268435456u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const MAPI_SHORTTERM: u32 = 128u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const MAPI_SUBMITTED: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const MAPI_THISSESSION: u32 = 32u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const MAPI_USE_DEFAULT: u32 = 64u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const MNID_ID: u32 = 0u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const MNID_STRING: u32 = 1u32;
pub struct MTSID(i32);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const MV_FLAG: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const MV_INSTANCE: u32 = 8192u32;
pub struct NEWMAIL_NOTIFICATION(i32);
pub struct NOTIFICATION(i32);
pub struct NOTIFKEY(i32);
pub struct OBJECT_NOTIFICATION(i32);
pub struct PFNIDLE(i32);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const PRIHIGHEST: u32 = 32767u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const PRILOWEST: i32 = -32768i32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const PRIUSER: u32 = 0u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const PROP_ID_INVALID: u32 = 65535u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const PROP_ID_NULL: u32 = 0u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const PROP_ID_SECURE_MAX: u32 = 26623u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const PROP_ID_SECURE_MIN: u32 = 26608u32;
pub struct SAndRestriction(i32);
pub struct SAppTimeArray(i32);
pub struct SBinary(i32);
pub struct SBinaryArray(i32);
pub struct SBitMaskRestriction(i32);
pub struct SCommentRestriction(i32);
pub struct SComparePropsRestriction(i32);
pub struct SContentRestriction(i32);
pub struct SCurrencyArray(i32);
pub struct SDateTimeArray(i32);
pub struct SDoubleArray(i32);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const SERVICE_UI_ALLOWED: u32 = 16u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const SERVICE_UI_ALWAYS: u32 = 2u32;
pub struct SExistRestriction(i32);
pub struct SGuidArray(i32);
pub struct SLPSTRArray(i32);
pub struct SLargeIntegerArray(i32);
pub struct SLongArray(i32);
pub struct SNotRestriction(i32);
pub struct SOrRestriction(i32);
pub struct SPropProblem(i32);
pub struct SPropProblemArray(i32);
pub struct SPropTagArray(i32);
pub struct SPropValue(i32);
pub struct SPropertyRestriction(i32);
pub struct SRealArray(i32);
pub struct SRestriction(i32);
pub struct SRow(i32);
pub struct SRowSet(i32);
pub struct SShortArray(i32);
pub struct SSizeRestriction(i32);
pub struct SSortOrder(i32);
pub struct SSortOrderSet(i32);
pub struct SSubRestriction(i32);
pub struct STATUS_OBJECT_NOTIFICATION(i32);
pub struct SWStringArray(i32);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const S_IMAPI_BOTHADJUSTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(11141126i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const S_IMAPI_COMMAND_HAS_SENSE_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(11141632i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const S_IMAPI_RAW_IMAGE_TRACK_INDEX_ALREADY_EXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(11143688i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const S_IMAPI_ROTATIONADJUSTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(11141125i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const S_IMAPI_SPEEDADJUSTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(11141124i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const S_IMAPI_WRITE_NOT_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(11141890i32 as _);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const TABLE_CHANGED: u32 = 1u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const TABLE_ERROR: u32 = 2u32;
pub struct TABLE_NOTIFICATION(i32);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const TABLE_RELOAD: u32 = 9u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const TABLE_RESTRICT_DONE: u32 = 7u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const TABLE_ROW_ADDED: u32 = 3u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const TABLE_ROW_DELETED: u32 = 4u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const TABLE_ROW_MODIFIED: u32 = 5u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const TABLE_SETCOL_DONE: u32 = 8u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const TABLE_SORT_DONE: u32 = 6u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const TAD_ALL_ROWS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const UI_CURRENT_PROVIDER_FIRST: u32 = 4u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const UI_SERVICE: u32 = 2u32;
pub struct WABEXTDISPLAY(i32);
pub struct WABIMPORTPARAM(i32);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const WABOBJECT_LDAPURL_RETURN_MAILUSER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const WABOBJECT_ME_NEW: u32 = 1u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const WABOBJECT_ME_NOCREATE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const WAB_CONTEXT_ADRLIST: u32 = 2u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const WAB_DISPLAY_ISNTDS: u32 = 4u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const WAB_DISPLAY_LDAPURL: u32 = 1u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const WAB_ENABLE_PROFILES: u32 = 4194304u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const WAB_IGNORE_PROFILES: u32 = 8388608u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const WAB_LOCAL_CONTAINERS: u32 = 1048576u32;
pub struct WAB_PARAM(i32);
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const WAB_PROFILE_CONTENTS: u32 = 2097152u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const WAB_USE_OE_SENDMAIL: u32 = 1u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const WAB_VCARD_FILE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_AddressBook`*"]
pub const WAB_VCARD_STREAM: u32 = 1u32;
pub struct _PV(i32);
pub struct _WABACTIONITEM(i32);
pub struct _flaglist(i32);
