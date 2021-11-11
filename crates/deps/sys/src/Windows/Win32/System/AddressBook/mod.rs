#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn BuildDisplayTable(lpallocatebuffer: ::windows::runtime::RawPtr, lpallocatemore: ::windows::runtime::RawPtr, lpfreebuffer: ::windows::runtime::RawPtr, lpmalloc: ::windows::runtime::RawPtr, hinstance: super::super::Foundation::HINSTANCE, cpages: u32, lppage: *mut DTPAGE, ulflags: u32, lpptable: *mut ::windows::runtime::RawPtr, lpptbldata: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeIdleRoutine(ftg: *mut ::core::ffi::c_void, lpfnidle: ::windows::runtime::RawPtr, lpvidleparam: *mut ::core::ffi::c_void, priidle: i16, csecidle: u32, iroidle: u16, ircidle: u16);
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn CreateIProp(lpinterface: *mut ::windows::runtime::GUID, lpallocatebuffer: ::windows::runtime::RawPtr, lpallocatemore: ::windows::runtime::RawPtr, lpfreebuffer: ::windows::runtime::RawPtr, lpvreserved: *mut ::core::ffi::c_void, lpppropdata: *mut ::windows::runtime::RawPtr) -> i32;
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn CreateTable(lpinterface: *mut ::windows::runtime::GUID, lpallocatebuffer: ::windows::runtime::RawPtr, lpallocatemore: ::windows::runtime::RawPtr, lpfreebuffer: ::windows::runtime::RawPtr, lpvreserved: *mut ::core::ffi::c_void, ultabletype: u32, ulproptagindexcolumn: u32, lpsproptagarraycolumns: *mut SPropTagArray, lpptabledata: *mut ::windows::runtime::RawPtr) -> i32;
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
    pub fn FPropExists(lpmapiprop: ::windows::runtime::RawPtr, ulproptag: u32) -> super::super::Foundation::BOOL;
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
    pub fn FtgRegisterIdleRoutine(lpfnidle: ::windows::runtime::RawPtr, lpvidleparam: *mut ::core::ffi::c_void, priidle: i16, csecidle: u32, iroidle: u16) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn HrAddColumns(lptbl: ::windows::runtime::RawPtr, lpproptagcolumnsnew: *mut SPropTagArray, lpallocatebuffer: ::windows::runtime::RawPtr, lpfreebuffer: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn HrAddColumnsEx(lptbl: ::windows::runtime::RawPtr, lpproptagcolumnsnew: *mut SPropTagArray, lpallocatebuffer: ::windows::runtime::RawPtr, lpfreebuffer: ::windows::runtime::RawPtr, lpfnfiltercolumns: isize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HrAllocAdviseSink(lpfncallback: ::windows::runtime::RawPtr, lpvcontext: *mut ::core::ffi::c_void, lppadvisesink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn HrDispatchNotifications(ulflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HrGetOneProp(lpmapiprop: ::windows::runtime::RawPtr, ulproptag: u32, lppprop: *mut *mut SPropValue) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn HrIStorageFromStream(lpunkin: ::windows::runtime::RawPtr, lpinterface: *mut ::windows::runtime::GUID, ulflags: u32, lppstorageout: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HrQueryAllRows(lptable: ::windows::runtime::RawPtr, lpproptags: *mut SPropTagArray, lprestriction: *mut SRestriction, lpsortorderset: *mut SSortOrderSet, crowsmax: i32, lpprows: *mut *mut SRowSet) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HrSetOneProp(lpmapiprop: ::windows::runtime::RawPtr, lpprop: *mut SPropValue) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn HrThisThreadAdviseSink(lpadvisesink: ::windows::runtime::RawPtr, lppadvisesink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
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
    pub fn OpenStreamOnFile(lpallocatebuffer: ::windows::runtime::RawPtr, lpfreebuffer: ::windows::runtime::RawPtr, ulflags: u32, lpszfilename: *const i8, lpszprefix: *const i8, lppstream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn PpropFindProp(lpproparray: *mut SPropValue, cvalues: u32, ulproptag: u32) -> *mut SPropValue;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn PropCopyMore(lpspropvaluedest: *mut SPropValue, lpspropvaluesrc: *mut SPropValue, lpfallocmore: ::windows::runtime::RawPtr, lpvobject: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RTFSync(lpmessage: ::windows::runtime::RawPtr, ulflags: u32, lpfmessageupdated: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
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
    pub fn ScDupPropset(cvalues: i32, lpproparray: *mut SPropValue, lpallocatebuffer: ::windows::runtime::RawPtr, lppproparray: *mut *mut SPropValue) -> i32;
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
    pub fn WrapCompressedRTFStream(lpcompressedrtfstream: ::windows::runtime::RawPtr, ulflags: u32, lpuncompressedrtfstream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn WrapStoreEntryID(ulflags: u32, lpszdllname: *const i8, cborigentry: u32, lporigentry: *const ENTRYID, lpcbwrappedentry: *mut u32, lppwrappedentry: *mut *mut ENTRYID) -> ::windows::runtime::HRESULT;
}
