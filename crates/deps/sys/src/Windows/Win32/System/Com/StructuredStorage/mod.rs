#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoGetInstanceFromFile(pserverinfo: *const super::COSERVERINFO, pclsid: *const ::windows::runtime::GUID, punkouter: ::windows::runtime::RawPtr, dwclsctx: super::CLSCTX, grfmode: u32, pwszname: super::super::super::Foundation::PWSTR, dwcount: u32, presults: *mut ::core::mem::ManuallyDrop<super::MULTI_QI>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoGetInstanceFromIStorage(pserverinfo: *const super::COSERVERINFO, pclsid: *const ::windows::runtime::GUID, punkouter: ::windows::runtime::RawPtr, dwclsctx: super::CLSCTX, pstg: ::windows::runtime::RawPtr, dwcount: u32, presults: *mut ::core::mem::ManuallyDrop<super::MULTI_QI>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn CoGetInterfaceAndReleaseStream(pstm: ::windows::runtime::RawPtr, iid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateILockBytesOnHGlobal(hglobal: isize, fdeleteonrelease: super::super::super::Foundation::BOOL, pplkbyt: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateStreamOnHGlobal(hglobal: isize, fdeleteonrelease: super::super::super::Foundation::BOOL, ppstm: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FmtIdToPropStgName(pfmtid: *const ::windows::runtime::GUID, oszname: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreePropVariantArray(cvariants: u32, rgvars: *mut ::core::mem::ManuallyDrop<PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn GetConvertStg(pstg: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn GetHGlobalFromILockBytes(plkbyt: ::windows::runtime::RawPtr, phglobal: *mut isize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn GetHGlobalFromStream(pstm: ::windows::runtime::RawPtr, phglobal: *mut isize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn OleConvertIStorageToOLESTREAM(pstg: ::windows::runtime::RawPtr, lpolestream: *mut OLESTREAM) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn OleConvertIStorageToOLESTREAMEx(pstg: ::windows::runtime::RawPtr, cfformat: u16, lwidth: i32, lheight: i32, dwsize: u32, pmedium: *mut ::core::mem::ManuallyDrop<super::STGMEDIUM>, polestm: *mut OLESTREAM) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn OleConvertOLESTREAMToIStorage(lpolestream: *mut OLESTREAM, pstg: ::windows::runtime::RawPtr, ptd: *const super::DVTARGETDEVICE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn OleConvertOLESTREAMToIStorageEx(polestm: *mut OLESTREAM, pstg: ::windows::runtime::RawPtr, pcfformat: *mut u16, plwwidth: *mut i32, plheight: *mut i32, pdwsize: *mut u32, pmedium: *mut ::core::mem::ManuallyDrop<super::STGMEDIUM>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PropStgNameToFmtId(oszname: super::super::super::Foundation::PWSTR, pfmtid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PropVariantClear(pvar: *mut ::core::mem::ManuallyDrop<PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PropVariantCopy(pvardest: *mut ::core::mem::ManuallyDrop<PROPVARIANT>, pvarsrc: *const ::core::mem::ManuallyDrop<PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn ReadClassStg(pstg: ::windows::runtime::RawPtr, pclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn ReadClassStm(pstm: ::windows::runtime::RawPtr, pclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadFmtUserTypeStg(pstg: ::windows::runtime::RawPtr, pcf: *mut u16, lplpszusertype: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConvertStg(pstg: ::windows::runtime::RawPtr, fconvert: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgConvertPropertyToVariant(pprop: *const SERIALIZEDPROPERTYVALUE, codepage: u16, pvar: *mut ::core::mem::ManuallyDrop<PROPVARIANT>, pma: *const PMemoryAllocator) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgConvertVariantToProperty(pvar: *const ::core::mem::ManuallyDrop<PROPVARIANT>, codepage: u16, pprop: *mut SERIALIZEDPROPERTYVALUE, pcb: *mut u32, pid: u32, freserved: super::super::super::Foundation::BOOLEAN, pcindirect: *mut u32) -> *mut SERIALIZEDPROPERTYVALUE;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgCreateDocfile(pwcsname: super::super::super::Foundation::PWSTR, grfmode: u32, reserved: u32, ppstgopen: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn StgCreateDocfileOnILockBytes(plkbyt: ::windows::runtime::RawPtr, grfmode: u32, reserved: u32, ppstgopen: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn StgCreatePropSetStg(pstorage: ::windows::runtime::RawPtr, dwreserved: u32, pppropsetstg: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn StgCreatePropStg(punk: ::windows::runtime::RawPtr, fmtid: *const ::windows::runtime::GUID, pclsid: *const ::windows::runtime::GUID, grfflags: u32, dwreserved: u32, pppropstg: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn StgCreateStorageEx(pwcsname: super::super::super::Foundation::PWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, pstgoptions: *mut STGOPTIONS, psecuritydescriptor: *const super::super::super::Security::SECURITY_DESCRIPTOR, riid: *const ::windows::runtime::GUID, ppobjectopen: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgDeserializePropVariant(pprop: *const SERIALIZEDPROPERTYVALUE, cbmax: u32, ppropvar: *mut ::core::mem::ManuallyDrop<PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgGetIFillLockBytesOnFile(pwcsname: super::super::super::Foundation::PWSTR, ppflb: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn StgGetIFillLockBytesOnILockBytes(pilb: ::windows::runtime::RawPtr, ppflb: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgIsStorageFile(pwcsname: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn StgIsStorageILockBytes(plkbyt: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn StgOpenAsyncDocfileOnIFillLockBytes(pflb: ::windows::runtime::RawPtr, grfmode: u32, asyncflags: u32, ppstgopen: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgOpenLayoutDocfile(pwcsdfname: super::super::super::Foundation::PWSTR, grfmode: u32, reserved: u32, ppstgopen: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn StgOpenPropStg(punk: ::windows::runtime::RawPtr, fmtid: *const ::windows::runtime::GUID, grfflags: u32, dwreserved: u32, pppropstg: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgOpenStorage(pwcsname: super::super::super::Foundation::PWSTR, pstgpriority: ::windows::runtime::RawPtr, grfmode: u32, snbexclude: *const *const u16, reserved: u32, ppstgopen: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn StgOpenStorageEx(pwcsname: super::super::super::Foundation::PWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, pstgoptions: *mut STGOPTIONS, psecuritydescriptor: *const super::super::super::Security::SECURITY_DESCRIPTOR, riid: *const ::windows::runtime::GUID, ppobjectopen: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn StgOpenStorageOnILockBytes(plkbyt: ::windows::runtime::RawPtr, pstgpriority: ::windows::runtime::RawPtr, grfmode: u32, snbexclude: *const *const u16, reserved: u32, ppstgopen: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn StgPropertyLengthAsVariant(pprop: *const SERIALIZEDPROPERTYVALUE, cbprop: u32, codepage: u16, breserved: u8) -> u32;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgSerializePropVariant(ppropvar: *const ::core::mem::ManuallyDrop<PROPVARIANT>, ppprop: *mut *mut SERIALIZEDPROPERTYVALUE, pcb: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgSetTimes(lpszname: super::super::super::Foundation::PWSTR, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn WriteClassStg(pstg: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn WriteClassStm(pstm: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteFmtUserTypeStg(pstg: ::windows::runtime::RawPtr, cf: u16, lpszusertype: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
}
