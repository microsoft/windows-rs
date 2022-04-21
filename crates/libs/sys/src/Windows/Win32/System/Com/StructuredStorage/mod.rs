#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn CoGetInstanceFromFile(pserverinfo: *const super::COSERVERINFO, pclsid: *const ::windows_sys::core::GUID, punkouter: ::windows_sys::core::IUnknown, dwclsctx: super::CLSCTX, grfmode: u32, pwszname: ::windows_sys::core::PCWSTR, dwcount: u32, presults: *mut super::MULTI_QI) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn CoGetInstanceFromIStorage(pserverinfo: *const super::COSERVERINFO, pclsid: *const ::windows_sys::core::GUID, punkouter: ::windows_sys::core::IUnknown, dwclsctx: super::CLSCTX, pstg: IStorage, dwcount: u32, presults: *mut super::MULTI_QI) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn CoGetInterfaceAndReleaseStream(pstm: super::IStream, iid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateILockBytesOnHGlobal(hglobal: isize, fdeleteonrelease: super::super::super::Foundation::BOOL, pplkbyt: *mut ILockBytes) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateStreamOnHGlobal(hglobal: isize, fdeleteonrelease: super::super::super::Foundation::BOOL, ppstm: *mut super::IStream) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn FmtIdToPropStgName(pfmtid: *const ::windows_sys::core::GUID, oszname: ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreePropVariantArray(cvariants: u32, rgvars: *mut PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn GetConvertStg(pstg: IStorage) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn GetHGlobalFromILockBytes(plkbyt: ILockBytes, phglobal: *mut isize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn GetHGlobalFromStream(pstm: super::IStream, phglobal: *mut isize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn OleConvertIStorageToOLESTREAM(pstg: IStorage, lpolestream: *mut OLESTREAM) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn OleConvertIStorageToOLESTREAMEx(pstg: IStorage, cfformat: u16, lwidth: i32, lheight: i32, dwsize: u32, pmedium: *mut super::STGMEDIUM, polestm: *mut OLESTREAM) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn OleConvertOLESTREAMToIStorage(lpolestream: *mut OLESTREAM, pstg: IStorage, ptd: *const super::DVTARGETDEVICE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn OleConvertOLESTREAMToIStorageEx(polestm: *mut OLESTREAM, pstg: IStorage, pcfformat: *mut u16, plwwidth: *mut i32, plheight: *mut i32, pdwsize: *mut u32, pmedium: *mut super::STGMEDIUM) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn PropStgNameToFmtId(oszname: ::windows_sys::core::PCWSTR, pfmtid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PropVariantClear(pvar: *mut PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PropVariantCopy(pvardest: *mut PROPVARIANT, pvarsrc: *const PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn ReadClassStg(pstg: IStorage, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn ReadClassStm(pstm: super::IStream, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn ReadFmtUserTypeStg(pstg: IStorage, pcf: *mut u16, lplpszusertype: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConvertStg(pstg: IStorage, fconvert: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgConvertPropertyToVariant(pprop: *const SERIALIZEDPROPERTYVALUE, codepage: u16, pvar: *mut PROPVARIANT, pma: *const PMemoryAllocator) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgConvertVariantToProperty(pvar: *const PROPVARIANT, codepage: u16, pprop: *mut SERIALIZEDPROPERTYVALUE, pcb: *mut u32, pid: u32, freserved: super::super::super::Foundation::BOOLEAN, pcindirect: *mut u32) -> *mut SERIALIZEDPROPERTYVALUE;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn StgCreateDocfile(pwcsname: ::windows_sys::core::PCWSTR, grfmode: STGM, reserved: u32, ppstgopen: *mut IStorage) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn StgCreateDocfileOnILockBytes(plkbyt: ILockBytes, grfmode: STGM, reserved: u32, ppstgopen: *mut IStorage) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn StgCreatePropSetStg(pstorage: IStorage, dwreserved: u32, pppropsetstg: *mut IPropertySetStorage) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn StgCreatePropStg(punk: ::windows_sys::core::IUnknown, fmtid: *const ::windows_sys::core::GUID, pclsid: *const ::windows_sys::core::GUID, grfflags: u32, dwreserved: u32, pppropstg: *mut IPropertyStorage) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Security\"`*"]
    #[cfg(feature = "Win32_Security")]
    pub fn StgCreateStorageEx(pwcsname: ::windows_sys::core::PCWSTR, grfmode: STGM, stgfmt: STGFMT, grfattrs: u32, pstgoptions: *mut STGOPTIONS, psecuritydescriptor: super::super::super::Security::PSECURITY_DESCRIPTOR, riid: *const ::windows_sys::core::GUID, ppobjectopen: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgDeserializePropVariant(pprop: *const SERIALIZEDPROPERTYVALUE, cbmax: u32, ppropvar: *mut PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn StgGetIFillLockBytesOnFile(pwcsname: ::windows_sys::core::PCWSTR, ppflb: *mut IFillLockBytes) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn StgGetIFillLockBytesOnILockBytes(pilb: ILockBytes, ppflb: *mut IFillLockBytes) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn StgIsStorageFile(pwcsname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn StgIsStorageILockBytes(plkbyt: ILockBytes) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn StgOpenAsyncDocfileOnIFillLockBytes(pflb: IFillLockBytes, grfmode: u32, asyncflags: u32, ppstgopen: *mut IStorage) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn StgOpenLayoutDocfile(pwcsdfname: ::windows_sys::core::PCWSTR, grfmode: u32, reserved: u32, ppstgopen: *mut IStorage) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn StgOpenPropStg(punk: ::windows_sys::core::IUnknown, fmtid: *const ::windows_sys::core::GUID, grfflags: u32, dwreserved: u32, pppropstg: *mut IPropertyStorage) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn StgOpenStorage(pwcsname: ::windows_sys::core::PCWSTR, pstgpriority: IStorage, grfmode: STGM, snbexclude: *const *const u16, reserved: u32, ppstgopen: *mut IStorage) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Security\"`*"]
    #[cfg(feature = "Win32_Security")]
    pub fn StgOpenStorageEx(pwcsname: ::windows_sys::core::PCWSTR, grfmode: STGM, stgfmt: STGFMT, grfattrs: u32, pstgoptions: *mut STGOPTIONS, psecuritydescriptor: super::super::super::Security::PSECURITY_DESCRIPTOR, riid: *const ::windows_sys::core::GUID, ppobjectopen: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn StgOpenStorageOnILockBytes(plkbyt: ILockBytes, pstgpriority: IStorage, grfmode: u32, snbexclude: *const *const u16, reserved: u32, ppstgopen: *mut IStorage) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn StgPropertyLengthAsVariant(pprop: *const SERIALIZEDPROPERTYVALUE, cbprop: u32, codepage: u16, breserved: u8) -> u32;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgSerializePropVariant(ppropvar: *const PROPVARIANT, ppprop: *mut *mut SERIALIZEDPROPERTYVALUE, pcb: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgSetTimes(lpszname: ::windows_sys::core::PCWSTR, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn WriteClassStg(pstg: IStorage, rclsid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn WriteClassStm(pstm: super::IStream, rclsid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    pub fn WriteFmtUserTypeStg(pstg: IStorage, cf: u16, lpszusertype: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct BSTRBLOB {
    pub cbSize: u32,
    pub pData: *mut u8,
}
impl ::core::marker::Copy for BSTRBLOB {}
impl ::core::clone::Clone for BSTRBLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CABOOL {
    pub cElems: u32,
    pub pElems: *mut i16,
}
impl ::core::marker::Copy for CABOOL {}
impl ::core::clone::Clone for CABOOL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CABSTR {
    pub cElems: u32,
    pub pElems: *mut super::super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CABSTR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CABSTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CABSTRBLOB {
    pub cElems: u32,
    pub pElems: *mut BSTRBLOB,
}
impl ::core::marker::Copy for CABSTRBLOB {}
impl ::core::clone::Clone for CABSTRBLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CAC {
    pub cElems: u32,
    pub pElems: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for CAC {}
impl ::core::clone::Clone for CAC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CACLIPDATA {
    pub cElems: u32,
    pub pElems: *mut CLIPDATA,
}
impl ::core::marker::Copy for CACLIPDATA {}
impl ::core::clone::Clone for CACLIPDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CACLSID {
    pub cElems: u32,
    pub pElems: *mut ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for CACLSID {}
impl ::core::clone::Clone for CACLSID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CACY {
    pub cElems: u32,
    pub pElems: *mut super::CY,
}
impl ::core::marker::Copy for CACY {}
impl ::core::clone::Clone for CACY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CADATE {
    pub cElems: u32,
    pub pElems: *mut f64,
}
impl ::core::marker::Copy for CADATE {}
impl ::core::clone::Clone for CADATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CADBL {
    pub cElems: u32,
    pub pElems: *mut f64,
}
impl ::core::marker::Copy for CADBL {}
impl ::core::clone::Clone for CADBL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CAFILETIME {
    pub cElems: u32,
    pub pElems: *mut super::super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CAFILETIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CAFILETIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CAFLT {
    pub cElems: u32,
    pub pElems: *mut f32,
}
impl ::core::marker::Copy for CAFLT {}
impl ::core::clone::Clone for CAFLT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CAH {
    pub cElems: u32,
    pub pElems: *mut i64,
}
impl ::core::marker::Copy for CAH {}
impl ::core::clone::Clone for CAH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CAI {
    pub cElems: u32,
    pub pElems: *mut i16,
}
impl ::core::marker::Copy for CAI {}
impl ::core::clone::Clone for CAI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CAL {
    pub cElems: u32,
    pub pElems: *mut i32,
}
impl ::core::marker::Copy for CAL {}
impl ::core::clone::Clone for CAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CALPSTR {
    pub cElems: u32,
    pub pElems: *mut ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for CALPSTR {}
impl ::core::clone::Clone for CALPSTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CALPWSTR {
    pub cElems: u32,
    pub pElems: *mut ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for CALPWSTR {}
impl ::core::clone::Clone for CALPWSTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CAPROPVARIANT {
    pub cElems: u32,
    pub pElems: *mut PROPVARIANT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CAPROPVARIANT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CAPROPVARIANT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CASCODE {
    pub cElems: u32,
    pub pElems: *mut i32,
}
impl ::core::marker::Copy for CASCODE {}
impl ::core::clone::Clone for CASCODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CAUB {
    pub cElems: u32,
    pub pElems: *mut u8,
}
impl ::core::marker::Copy for CAUB {}
impl ::core::clone::Clone for CAUB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CAUH {
    pub cElems: u32,
    pub pElems: *mut u64,
}
impl ::core::marker::Copy for CAUH {}
impl ::core::clone::Clone for CAUH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CAUI {
    pub cElems: u32,
    pub pElems: *mut u16,
}
impl ::core::marker::Copy for CAUI {}
impl ::core::clone::Clone for CAUI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CAUL {
    pub cElems: u32,
    pub pElems: *mut u32,
}
impl ::core::marker::Copy for CAUL {}
impl ::core::clone::Clone for CAUL {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const CCH_MAX_PROPSTG_NAME: u32 = 31u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CLIPDATA {
    pub cbSize: u32,
    pub ulClipFmt: i32,
    pub pClipData: *mut u8,
}
impl ::core::marker::Copy for CLIPDATA {}
impl ::core::clone::Clone for CLIPDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const CWCSTORAGENAME: u32 = 32u32;
pub type IDirectWriterLock = *mut ::core::ffi::c_void;
pub type IEnumSTATPROPSETSTG = *mut ::core::ffi::c_void;
pub type IEnumSTATPROPSTG = *mut ::core::ffi::c_void;
pub type IEnumSTATSTG = *mut ::core::ffi::c_void;
pub type IFillLockBytes = *mut ::core::ffi::c_void;
pub type ILayoutStorage = *mut ::core::ffi::c_void;
pub type ILockBytes = *mut ::core::ffi::c_void;
pub type IPersistStorage = *mut ::core::ffi::c_void;
pub type IPropertyBag = *mut ::core::ffi::c_void;
pub type IPropertyBag2 = *mut ::core::ffi::c_void;
pub type IPropertySetStorage = *mut ::core::ffi::c_void;
pub type IPropertyStorage = *mut ::core::ffi::c_void;
pub type IRootStorage = *mut ::core::ffi::c_void;
pub type IStorage = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub type LOCKTYPE = i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const LOCK_WRITE: LOCKTYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const LOCK_EXCLUSIVE: LOCKTYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const LOCK_ONLYONCE: LOCKTYPE = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct OLESTREAM {
    pub lpstbl: *mut OLESTREAMVTBL,
}
impl ::core::marker::Copy for OLESTREAM {}
impl ::core::clone::Clone for OLESTREAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct OLESTREAMVTBL {
    pub Get: isize,
    pub Put: isize,
}
impl ::core::marker::Copy for OLESTREAMVTBL {}
impl ::core::clone::Clone for OLESTREAMVTBL {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDI_THUMBNAIL: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_BYTECOUNT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_CATEGORY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_COMPANY: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_DOCPARTS: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_HEADINGPAIR: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_HIDDENCOUNT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_LINECOUNT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_LINKSDIRTY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_MANAGER: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_MMCLIPCOUNT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_NOTECOUNT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_PARCOUNT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_PRESFORMAT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_SCALE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_SLIDECOUNT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_COPYRIGHT: i32 = 11i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_EDITOR: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_OWNER: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_PRODUCTION: i32 = 10i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_PROJECT: i32 = 6i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_RATING: i32 = 9i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_SEQUENCE_NO: i32 = 5i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_SOURCE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS: i32 = 7i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub type PIDMSI_STATUS_VALUE = i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_NORMAL: PIDMSI_STATUS_VALUE = 0i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_NEW: PIDMSI_STATUS_VALUE = 1i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_PRELIM: PIDMSI_STATUS_VALUE = 2i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_DRAFT: PIDMSI_STATUS_VALUE = 3i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_INPROGRESS: PIDMSI_STATUS_VALUE = 4i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_EDIT: PIDMSI_STATUS_VALUE = 5i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_REVIEW: PIDMSI_STATUS_VALUE = 6i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_PROOF: PIDMSI_STATUS_VALUE = 7i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_FINAL: PIDMSI_STATUS_VALUE = 8i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_OTHER: PIDMSI_STATUS_VALUE = 32767i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_SUPPLIER: i32 = 3i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_APPNAME: i32 = 18i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_AUTHOR: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_CHARCOUNT: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_COMMENTS: i32 = 6i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_CREATE_DTM: i32 = 12i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_DOC_SECURITY: i32 = 19i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_EDITTIME: i32 = 10i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_KEYWORDS: i32 = 5i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_LASTAUTHOR: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_LASTPRINTED: i32 = 11i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_LASTSAVE_DTM: i32 = 13i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_PAGECOUNT: i32 = 14i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_REVNUMBER: i32 = 9i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_SUBJECT: i32 = 3i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_TEMPLATE: i32 = 7i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_THUMBNAIL: i32 = 17i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_TITLE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_WORDCOUNT: i32 = 15i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PID_BEHAVIOR: u32 = 2147483651u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PID_CODEPAGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PID_DICTIONARY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PID_FIRST_NAME_DEFAULT: u32 = 4095u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PID_FIRST_USABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PID_ILLEGAL: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PID_LOCALE: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PID_MAX_READONLY: u32 = 3221225471u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PID_MIN_READONLY: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PID_MODIFY_TIME: u32 = 2147483649u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PID_SECURITY: u32 = 2147483650u32;
#[repr(C)]
pub struct PMemoryAllocator(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct PROPBAG2 {
    pub dwType: u32,
    pub vt: u16,
    pub cfType: u16,
    pub dwHint: u32,
    pub pstrName: ::windows_sys::core::PWSTR,
    pub clsid: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for PROPBAG2 {}
impl ::core::clone::Clone for PROPBAG2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PROPSETFLAG_ANSI: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PROPSETFLAG_CASE_SENSITIVE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PROPSETFLAG_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PROPSETFLAG_NONSIMPLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PROPSETFLAG_UNBUFFERED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PROPSETHDR_OSVERSION_UNKNOWN: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PROPSET_BEHAVIOR_CASE_SENSITIVE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct PROPSPEC {
    pub ulKind: PROPSPEC_KIND,
    pub Anonymous: PROPSPEC_0,
}
impl ::core::marker::Copy for PROPSPEC {}
impl ::core::clone::Clone for PROPSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub union PROPSPEC_0 {
    pub propid: u32,
    pub lpwstr: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for PROPSPEC_0 {}
impl ::core::clone::Clone for PROPSPEC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub type PROPSPEC_KIND = u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PRSPEC_LPWSTR: PROPSPEC_KIND = 0u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PRSPEC_PROPID: PROPSPEC_KIND = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PROPVARIANT {
    pub Anonymous: PROPVARIANT_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROPVARIANT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROPVARIANT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union PROPVARIANT_0 {
    pub Anonymous: PROPVARIANT_0_0,
    pub decVal: super::super::super::Foundation::DECIMAL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROPVARIANT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROPVARIANT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PROPVARIANT_0_0 {
    pub vt: u16,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: PROPVARIANT_0_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROPVARIANT_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROPVARIANT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union PROPVARIANT_0_0_0 {
    pub cVal: super::super::super::Foundation::CHAR,
    pub bVal: u8,
    pub iVal: i16,
    pub uiVal: u16,
    pub lVal: i32,
    pub ulVal: u32,
    pub intVal: i32,
    pub uintVal: u32,
    pub hVal: i64,
    pub uhVal: u64,
    pub fltVal: f32,
    pub dblVal: f64,
    pub boolVal: i16,
    pub __OBSOLETE__VARIANT_BOOL: i16,
    pub scode: i32,
    pub cyVal: super::CY,
    pub date: f64,
    pub filetime: super::super::super::Foundation::FILETIME,
    pub puuid: *mut ::windows_sys::core::GUID,
    pub pclipdata: *mut CLIPDATA,
    pub bstrVal: super::super::super::Foundation::BSTR,
    pub bstrblobVal: BSTRBLOB,
    pub blob: super::BLOB,
    pub pszVal: ::windows_sys::core::PSTR,
    pub pwszVal: ::windows_sys::core::PWSTR,
    pub punkVal: ::windows_sys::core::IUnknown,
    pub pdispVal: super::IDispatch,
    pub pStream: super::IStream,
    pub pStorage: IStorage,
    pub pVersionedStream: *mut VERSIONEDSTREAM,
    pub parray: *mut super::SAFEARRAY,
    pub cac: CAC,
    pub caub: CAUB,
    pub cai: CAI,
    pub caui: CAUI,
    pub cal: CAL,
    pub caul: CAUL,
    pub cah: CAH,
    pub cauh: CAUH,
    pub caflt: CAFLT,
    pub cadbl: CADBL,
    pub cabool: CABOOL,
    pub cascode: CASCODE,
    pub cacy: CACY,
    pub cadate: CADATE,
    pub cafiletime: CAFILETIME,
    pub cauuid: CACLSID,
    pub caclipdata: CACLIPDATA,
    pub cabstr: CABSTR,
    pub cabstrblob: CABSTRBLOB,
    pub calpstr: CALPSTR,
    pub calpwstr: CALPWSTR,
    pub capropvar: CAPROPVARIANT,
    pub pcVal: ::windows_sys::core::PSTR,
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub puiVal: *mut u16,
    pub plVal: *mut i32,
    pub pulVal: *mut u32,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut i16,
    pub pdecVal: *mut super::super::super::Foundation::DECIMAL,
    pub pscode: *mut i32,
    pub pcyVal: *mut super::CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut super::super::super::Foundation::BSTR,
    pub ppunkVal: *mut ::windows_sys::core::IUnknown,
    pub ppdispVal: *mut super::IDispatch,
    pub pparray: *mut *mut super::SAFEARRAY,
    pub pvarVal: *mut PROPVARIANT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROPVARIANT_0_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROPVARIANT_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PRSPEC_INVALID: u32 = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct RemSNB {
    pub ulCntStr: u32,
    pub ulCntChar: u32,
    pub rgString: [u16; 1],
}
impl ::core::marker::Copy for RemSNB {}
impl ::core::clone::Clone for RemSNB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct SERIALIZEDPROPERTYVALUE {
    pub dwType: u32,
    pub rgb: [u8; 1],
}
impl ::core::marker::Copy for SERIALIZEDPROPERTYVALUE {}
impl ::core::clone::Clone for SERIALIZEDPROPERTYVALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub type STATFLAG = i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STATFLAG_DEFAULT: STATFLAG = 0i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STATFLAG_NONAME: STATFLAG = 1i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STATFLAG_NOOPEN: STATFLAG = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STATPROPSETSTG {
    pub fmtid: ::windows_sys::core::GUID,
    pub clsid: ::windows_sys::core::GUID,
    pub grfFlags: u32,
    pub mtime: super::super::super::Foundation::FILETIME,
    pub ctime: super::super::super::Foundation::FILETIME,
    pub atime: super::super::super::Foundation::FILETIME,
    pub dwOSVersion: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STATPROPSETSTG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STATPROPSETSTG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct STATPROPSTG {
    pub lpwstrName: ::windows_sys::core::PWSTR,
    pub propid: u32,
    pub vt: u16,
}
impl ::core::marker::Copy for STATPROPSTG {}
impl ::core::clone::Clone for STATPROPSTG {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub type STGC = u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGC_DEFAULT: STGC = 0u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGC_OVERWRITE: STGC = 1u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGC_ONLYIFCURRENT: STGC = 2u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGC_DANGEROUSLYCOMMITMERELYTODISKCACHE: STGC = 4u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGC_CONSOLIDATE: STGC = 8u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub type STGFMT = u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGFMT_STORAGE: STGFMT = 0u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGFMT_NATIVE: STGFMT = 1u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGFMT_FILE: STGFMT = 3u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGFMT_ANY: STGFMT = 4u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGFMT_DOCFILE: STGFMT = 5u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGFMT_DOCUMENT: STGFMT = 0u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub type STGM = u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_DIRECT: STGM = 0u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_TRANSACTED: STGM = 65536u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_SIMPLE: STGM = 134217728u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_READ: STGM = 0u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_WRITE: STGM = 1u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_READWRITE: STGM = 2u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_SHARE_DENY_NONE: STGM = 64u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_SHARE_DENY_READ: STGM = 48u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_SHARE_DENY_WRITE: STGM = 32u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_SHARE_EXCLUSIVE: STGM = 16u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_PRIORITY: STGM = 262144u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_DELETEONRELEASE: STGM = 67108864u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_NOSCRATCH: STGM = 1048576u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_CREATE: STGM = 4096u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_CONVERT: STGM = 131072u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_FAILIFTHERE: STGM = 0u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_NOSNAPSHOT: STGM = 2097152u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_DIRECT_SWMR: STGM = 4194304u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub type STGMOVE = i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGMOVE_MOVE: STGMOVE = 0i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGMOVE_COPY: STGMOVE = 1i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGMOVE_SHALLOWCOPY: STGMOVE = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct STGOPTIONS {
    pub usVersion: u16,
    pub reserved: u16,
    pub ulSectorSize: u32,
    pub pwcsTemplateFile: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for STGOPTIONS {}
impl ::core::clone::Clone for STGOPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGOPTIONS_VERSION: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct VERSIONEDSTREAM {
    pub guidVersion: ::windows_sys::core::GUID,
    pub pStream: super::IStream,
}
impl ::core::marker::Copy for VERSIONEDSTREAM {}
impl ::core::clone::Clone for VERSIONEDSTREAM {
    fn clone(&self) -> Self {
        *self
    }
}
