#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn CoGetInstanceFromFile<P0, P1>(pserverinfo: ::core::option::Option<*const super::COSERVERINFO>, pclsid: ::core::option::Option<*const ::windows::core::GUID>, punkouter: P0, dwclsctx: super::CLSCTX, grfmode: u32, pwszname: P1, presults: &mut [super::MULTI_QI]) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn CoGetInstanceFromFile ( pserverinfo : *const super:: COSERVERINFO , pclsid : *const :: windows::core::GUID , punkouter : * mut::core::ffi::c_void , dwclsctx : super:: CLSCTX , grfmode : u32 , pwszname : :: windows::core::PCWSTR , dwcount : u32 , presults : *mut super:: MULTI_QI ) -> :: windows::core::HRESULT );
    CoGetInstanceFromFile(::core::mem::transmute(pserverinfo.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pclsid.unwrap_or(::std::ptr::null())), punkouter.into_param().abi(), dwclsctx, grfmode, pwszname.into_param().abi(), presults.len() as _, ::core::mem::transmute(presults.as_ptr())).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn CoGetInstanceFromIStorage<P0, P1>(pserverinfo: ::core::option::Option<*const super::COSERVERINFO>, pclsid: ::core::option::Option<*const ::windows::core::GUID>, punkouter: P0, dwclsctx: super::CLSCTX, pstg: P1, presults: &mut [super::MULTI_QI]) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    P1: ::windows::core::IntoParam<IStorage>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn CoGetInstanceFromIStorage ( pserverinfo : *const super:: COSERVERINFO , pclsid : *const :: windows::core::GUID , punkouter : * mut::core::ffi::c_void , dwclsctx : super:: CLSCTX , pstg : * mut::core::ffi::c_void , dwcount : u32 , presults : *mut super:: MULTI_QI ) -> :: windows::core::HRESULT );
    CoGetInstanceFromIStorage(::core::mem::transmute(pserverinfo.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pclsid.unwrap_or(::std::ptr::null())), punkouter.into_param().abi(), dwclsctx, pstg.into_param().abi(), presults.len() as _, ::core::mem::transmute(presults.as_ptr())).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn CoGetInterfaceAndReleaseStream<P0, T>(pstm: P0) -> ::windows::core::Result<T>
where
    P0: ::windows::core::IntoParam<super::IStream>,
    T: ::windows::core::ComInterface,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn CoGetInterfaceAndReleaseStream ( pstm : * mut::core::ffi::c_void , iid : *const :: windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::std::ptr::null_mut();
    CoGetInterfaceAndReleaseStream(pstm.into_param().abi(), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateILockBytesOnHGlobal<P0, P1>(hglobal: P0, fdeleteonrelease: P1) -> ::windows::core::Result<ILockBytes>
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HGLOBAL>,
    P1: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn CreateILockBytesOnHGlobal ( hglobal : super::super::super::Foundation:: HGLOBAL , fdeleteonrelease : super::super::super::Foundation:: BOOL , pplkbyt : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<ILockBytes>();
    CreateILockBytesOnHGlobal(hglobal.into_param().abi(), fdeleteonrelease.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateStreamOnHGlobal<P0, P1>(hglobal: P0, fdeleteonrelease: P1) -> ::windows::core::Result<super::IStream>
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HGLOBAL>,
    P1: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn CreateStreamOnHGlobal ( hglobal : super::super::super::Foundation:: HGLOBAL , fdeleteonrelease : super::super::super::Foundation:: BOOL , ppstm : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::IStream>();
    CreateStreamOnHGlobal(hglobal.into_param().abi(), fdeleteonrelease.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn FmtIdToPropStgName(pfmtid: *const ::windows::core::GUID, oszname: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "ole32.dll""system" fn FmtIdToPropStgName ( pfmtid : *const :: windows::core::GUID , oszname : :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    FmtIdToPropStgName(pfmtid, ::core::mem::transmute(oszname)).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreePropVariantArray(rgvars: &mut [PROPVARIANT]) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "ole32.dll""system" fn FreePropVariantArray ( cvariants : u32 , rgvars : *mut PROPVARIANT ) -> :: windows::core::HRESULT );
    FreePropVariantArray(rgvars.len() as _, ::core::mem::transmute(rgvars.as_ptr())).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn GetConvertStg<P0>(pstg: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<IStorage>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn GetConvertStg ( pstg : * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    GetConvertStg(pstg.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetHGlobalFromILockBytes<P0>(plkbyt: P0) -> ::windows::core::Result<super::super::super::Foundation::HGLOBAL>
where
    P0: ::windows::core::IntoParam<ILockBytes>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn GetHGlobalFromILockBytes ( plkbyt : * mut::core::ffi::c_void , phglobal : *mut super::super::super::Foundation:: HGLOBAL ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::HGLOBAL>();
    GetHGlobalFromILockBytes(plkbyt.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetHGlobalFromStream<P0>(pstm: P0) -> ::windows::core::Result<super::super::super::Foundation::HGLOBAL>
where
    P0: ::windows::core::IntoParam<super::IStream>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn GetHGlobalFromStream ( pstm : * mut::core::ffi::c_void , phglobal : *mut super::super::super::Foundation:: HGLOBAL ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::HGLOBAL>();
    GetHGlobalFromStream(pstm.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn OleConvertIStorageToOLESTREAM<P0>(pstg: P0) -> ::windows::core::Result<OLESTREAM>
where
    P0: ::windows::core::IntoParam<IStorage>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn OleConvertIStorageToOLESTREAM ( pstg : * mut::core::ffi::c_void , lpolestream : *mut OLESTREAM ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<OLESTREAM>();
    OleConvertIStorageToOLESTREAM(pstg.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn OleConvertIStorageToOLESTREAMEx<P0>(pstg: P0, cfformat: u16, lwidth: i32, lheight: i32, dwsize: u32, pmedium: *const super::STGMEDIUM) -> ::windows::core::Result<OLESTREAM>
where
    P0: ::windows::core::IntoParam<IStorage>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn OleConvertIStorageToOLESTREAMEx ( pstg : * mut::core::ffi::c_void , cfformat : u16 , lwidth : i32 , lheight : i32 , dwsize : u32 , pmedium : *const super:: STGMEDIUM , polestm : *mut OLESTREAM ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<OLESTREAM>();
    OleConvertIStorageToOLESTREAMEx(pstg.into_param().abi(), cfformat, lwidth, lheight, dwsize, pmedium, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn OleConvertOLESTREAMToIStorage<P0>(lpolestream: *const OLESTREAM, pstg: P0, ptd: *const super::DVTARGETDEVICE) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<IStorage>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn OleConvertOLESTREAMToIStorage ( lpolestream : *const OLESTREAM , pstg : * mut::core::ffi::c_void , ptd : *const super:: DVTARGETDEVICE ) -> :: windows::core::HRESULT );
    OleConvertOLESTREAMToIStorage(lpolestream, pstg.into_param().abi(), ptd).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn OleConvertOLESTREAMToIStorageEx<P0>(polestm: *const OLESTREAM, pstg: P0, pcfformat: *mut u16, plwwidth: *mut i32, plheight: *mut i32, pdwsize: *mut u32, pmedium: *mut super::STGMEDIUM) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<IStorage>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn OleConvertOLESTREAMToIStorageEx ( polestm : *const OLESTREAM , pstg : * mut::core::ffi::c_void , pcfformat : *mut u16 , plwwidth : *mut i32 , plheight : *mut i32 , pdwsize : *mut u32 , pmedium : *mut super:: STGMEDIUM ) -> :: windows::core::HRESULT );
    OleConvertOLESTREAMToIStorageEx(polestm, pstg.into_param().abi(), pcfformat, plwwidth, plheight, pdwsize, pmedium).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn PropStgNameToFmtId<P0>(oszname: P0) -> ::windows::core::Result<::windows::core::GUID>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn PropStgNameToFmtId ( oszname : :: windows::core::PCWSTR , pfmtid : *mut :: windows::core::GUID ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
    PropStgNameToFmtId(oszname.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PropVariantClear(pvar: *mut PROPVARIANT) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "ole32.dll""system" fn PropVariantClear ( pvar : *mut PROPVARIANT ) -> :: windows::core::HRESULT );
    PropVariantClear(pvar).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PropVariantCopy(pvardest: *mut PROPVARIANT, pvarsrc: *const PROPVARIANT) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "ole32.dll""system" fn PropVariantCopy ( pvardest : *mut PROPVARIANT , pvarsrc : *const PROPVARIANT ) -> :: windows::core::HRESULT );
    PropVariantCopy(pvardest, pvarsrc).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn ReadClassStg<P0>(pstg: P0) -> ::windows::core::Result<::windows::core::GUID>
where
    P0: ::windows::core::IntoParam<IStorage>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn ReadClassStg ( pstg : * mut::core::ffi::c_void , pclsid : *mut :: windows::core::GUID ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
    ReadClassStg(pstg.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn ReadClassStm<P0>(pstm: P0) -> ::windows::core::Result<::windows::core::GUID>
where
    P0: ::windows::core::IntoParam<super::IStream>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn ReadClassStm ( pstm : * mut::core::ffi::c_void , pclsid : *mut :: windows::core::GUID ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
    ReadClassStm(pstm.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn ReadFmtUserTypeStg<P0>(pstg: P0, pcf: *mut u16, lplpszusertype: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<IStorage>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn ReadFmtUserTypeStg ( pstg : * mut::core::ffi::c_void , pcf : *mut u16 , lplpszusertype : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    ReadFmtUserTypeStg(pstg.into_param().abi(), pcf, ::core::mem::transmute(lplpszusertype.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConvertStg<P0, P1>(pstg: P0, fconvert: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<IStorage>,
    P1: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn SetConvertStg ( pstg : * mut::core::ffi::c_void , fconvert : super::super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    SetConvertStg(pstg.into_param().abi(), fconvert.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StgConvertPropertyToVariant(pprop: *const SERIALIZEDPROPERTYVALUE, codepage: u16, pvar: *mut PROPVARIANT, pma: *const PMemoryAllocator) -> super::super::super::Foundation::BOOLEAN {
    ::windows::imp::link ! ( "ole32.dll""system" fn StgConvertPropertyToVariant ( pprop : *const SERIALIZEDPROPERTYVALUE , codepage : u16 , pvar : *mut PROPVARIANT , pma : *const PMemoryAllocator ) -> super::super::super::Foundation:: BOOLEAN );
    StgConvertPropertyToVariant(pprop, codepage, pvar, pma)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StgConvertVariantToProperty<P0>(pvar: *const PROPVARIANT, codepage: u16, pprop: ::core::option::Option<*mut SERIALIZEDPROPERTYVALUE>, pcb: *mut u32, pid: u32, freserved: P0, pcindirect: ::core::option::Option<*mut u32>) -> *mut SERIALIZEDPROPERTYVALUE
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOLEAN>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn StgConvertVariantToProperty ( pvar : *const PROPVARIANT , codepage : u16 , pprop : *mut SERIALIZEDPROPERTYVALUE , pcb : *mut u32 , pid : u32 , freserved : super::super::super::Foundation:: BOOLEAN , pcindirect : *mut u32 ) -> *mut SERIALIZEDPROPERTYVALUE );
    StgConvertVariantToProperty(pvar, codepage, ::core::mem::transmute(pprop.unwrap_or(::std::ptr::null_mut())), pcb, pid, freserved.into_param().abi(), ::core::mem::transmute(pcindirect.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn StgCreateDocfile<P0>(pwcsname: P0, grfmode: super::STGM, reserved: u32) -> ::windows::core::Result<IStorage>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn StgCreateDocfile ( pwcsname : :: windows::core::PCWSTR , grfmode : super:: STGM , reserved : u32 , ppstgopen : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IStorage>();
    StgCreateDocfile(pwcsname.into_param().abi(), grfmode, reserved, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn StgCreateDocfileOnILockBytes<P0>(plkbyt: P0, grfmode: super::STGM, reserved: u32) -> ::windows::core::Result<IStorage>
where
    P0: ::windows::core::IntoParam<ILockBytes>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn StgCreateDocfileOnILockBytes ( plkbyt : * mut::core::ffi::c_void , grfmode : super:: STGM , reserved : u32 , ppstgopen : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IStorage>();
    StgCreateDocfileOnILockBytes(plkbyt.into_param().abi(), grfmode, reserved, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn StgCreatePropSetStg<P0>(pstorage: P0, dwreserved: u32) -> ::windows::core::Result<IPropertySetStorage>
where
    P0: ::windows::core::IntoParam<IStorage>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn StgCreatePropSetStg ( pstorage : * mut::core::ffi::c_void , dwreserved : u32 , pppropsetstg : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IPropertySetStorage>();
    StgCreatePropSetStg(pstorage.into_param().abi(), dwreserved, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn StgCreatePropStg<P0>(punk: P0, fmtid: *const ::windows::core::GUID, pclsid: *const ::windows::core::GUID, grfflags: u32, dwreserved: u32) -> ::windows::core::Result<IPropertyStorage>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn StgCreatePropStg ( punk : * mut::core::ffi::c_void , fmtid : *const :: windows::core::GUID , pclsid : *const :: windows::core::GUID , grfflags : u32 , dwreserved : u32 , pppropstg : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IPropertyStorage>();
    StgCreatePropStg(punk.into_param().abi(), fmtid, pclsid, grfflags, dwreserved, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn StgCreateStorageEx<P0, P1>(pwcsname: P0, grfmode: super::STGM, stgfmt: STGFMT, grfattrs: u32, pstgoptions: ::core::option::Option<*mut STGOPTIONS>, psecuritydescriptor: P1, riid: *const ::windows::core::GUID, ppobjectopen: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::super::Security::PSECURITY_DESCRIPTOR>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn StgCreateStorageEx ( pwcsname : :: windows::core::PCWSTR , grfmode : super:: STGM , stgfmt : STGFMT , grfattrs : u32 , pstgoptions : *mut STGOPTIONS , psecuritydescriptor : super::super::super::Security:: PSECURITY_DESCRIPTOR , riid : *const :: windows::core::GUID , ppobjectopen : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    StgCreateStorageEx(pwcsname.into_param().abi(), grfmode, stgfmt, grfattrs, ::core::mem::transmute(pstgoptions.unwrap_or(::std::ptr::null_mut())), psecuritydescriptor.into_param().abi(), riid, ppobjectopen).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StgDeserializePropVariant(pprop: *const SERIALIZEDPROPERTYVALUE, cbmax: u32) -> ::windows::core::Result<PROPVARIANT> {
    ::windows::imp::link ! ( "propsys.dll""system" fn StgDeserializePropVariant ( pprop : *const SERIALIZEDPROPERTYVALUE , cbmax : u32 , ppropvar : *mut PROPVARIANT ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<PROPVARIANT>();
    StgDeserializePropVariant(pprop, cbmax, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn StgGetIFillLockBytesOnFile<P0>(pwcsname: P0) -> ::windows::core::Result<IFillLockBytes>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn StgGetIFillLockBytesOnFile ( pwcsname : :: windows::core::PCWSTR , ppflb : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IFillLockBytes>();
    StgGetIFillLockBytesOnFile(pwcsname.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn StgGetIFillLockBytesOnILockBytes<P0>(pilb: P0) -> ::windows::core::Result<IFillLockBytes>
where
    P0: ::windows::core::IntoParam<ILockBytes>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn StgGetIFillLockBytesOnILockBytes ( pilb : * mut::core::ffi::c_void , ppflb : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IFillLockBytes>();
    StgGetIFillLockBytesOnILockBytes(pilb.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn StgIsStorageFile<P0>(pwcsname: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn StgIsStorageFile ( pwcsname : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    StgIsStorageFile(pwcsname.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn StgIsStorageILockBytes<P0>(plkbyt: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<ILockBytes>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn StgIsStorageILockBytes ( plkbyt : * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    StgIsStorageILockBytes(plkbyt.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn StgOpenAsyncDocfileOnIFillLockBytes<P0>(pflb: P0, grfmode: u32, asyncflags: u32) -> ::windows::core::Result<IStorage>
where
    P0: ::windows::core::IntoParam<IFillLockBytes>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn StgOpenAsyncDocfileOnIFillLockBytes ( pflb : * mut::core::ffi::c_void , grfmode : u32 , asyncflags : u32 , ppstgopen : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IStorage>();
    StgOpenAsyncDocfileOnIFillLockBytes(pflb.into_param().abi(), grfmode, asyncflags, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn StgOpenLayoutDocfile<P0>(pwcsdfname: P0, grfmode: u32, reserved: u32) -> ::windows::core::Result<IStorage>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "dflayout.dll""system" fn StgOpenLayoutDocfile ( pwcsdfname : :: windows::core::PCWSTR , grfmode : u32 , reserved : u32 , ppstgopen : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IStorage>();
    StgOpenLayoutDocfile(pwcsdfname.into_param().abi(), grfmode, reserved, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn StgOpenPropStg<P0>(punk: P0, fmtid: *const ::windows::core::GUID, grfflags: u32, dwreserved: u32) -> ::windows::core::Result<IPropertyStorage>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn StgOpenPropStg ( punk : * mut::core::ffi::c_void , fmtid : *const :: windows::core::GUID , grfflags : u32 , dwreserved : u32 , pppropstg : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IPropertyStorage>();
    StgOpenPropStg(punk.into_param().abi(), fmtid, grfflags, dwreserved, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn StgOpenStorage<P0, P1>(pwcsname: P0, pstgpriority: P1, grfmode: super::STGM, snbexclude: ::core::option::Option<*const *const u16>, reserved: u32) -> ::windows::core::Result<IStorage>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<IStorage>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn StgOpenStorage ( pwcsname : :: windows::core::PCWSTR , pstgpriority : * mut::core::ffi::c_void , grfmode : super:: STGM , snbexclude : *const *const u16 , reserved : u32 , ppstgopen : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IStorage>();
    StgOpenStorage(pwcsname.into_param().abi(), pstgpriority.into_param().abi(), grfmode, ::core::mem::transmute(snbexclude.unwrap_or(::std::ptr::null())), reserved, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn StgOpenStorageEx<P0, P1>(pwcsname: P0, grfmode: super::STGM, stgfmt: STGFMT, grfattrs: u32, pstgoptions: ::core::option::Option<*mut STGOPTIONS>, psecuritydescriptor: P1, riid: *const ::windows::core::GUID, ppobjectopen: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::super::Security::PSECURITY_DESCRIPTOR>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn StgOpenStorageEx ( pwcsname : :: windows::core::PCWSTR , grfmode : super:: STGM , stgfmt : STGFMT , grfattrs : u32 , pstgoptions : *mut STGOPTIONS , psecuritydescriptor : super::super::super::Security:: PSECURITY_DESCRIPTOR , riid : *const :: windows::core::GUID , ppobjectopen : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    StgOpenStorageEx(pwcsname.into_param().abi(), grfmode, stgfmt, grfattrs, ::core::mem::transmute(pstgoptions.unwrap_or(::std::ptr::null_mut())), psecuritydescriptor.into_param().abi(), riid, ppobjectopen).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn StgOpenStorageOnILockBytes<P0, P1>(plkbyt: P0, pstgpriority: P1, grfmode: super::STGM, snbexclude: ::core::option::Option<*const *const u16>, reserved: u32) -> ::windows::core::Result<IStorage>
where
    P0: ::windows::core::IntoParam<ILockBytes>,
    P1: ::windows::core::IntoParam<IStorage>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn StgOpenStorageOnILockBytes ( plkbyt : * mut::core::ffi::c_void , pstgpriority : * mut::core::ffi::c_void , grfmode : super:: STGM , snbexclude : *const *const u16 , reserved : u32 , ppstgopen : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IStorage>();
    StgOpenStorageOnILockBytes(plkbyt.into_param().abi(), pstgpriority.into_param().abi(), grfmode, ::core::mem::transmute(snbexclude.unwrap_or(::std::ptr::null())), reserved, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn StgPropertyLengthAsVariant(pprop: *const SERIALIZEDPROPERTYVALUE, cbprop: u32, codepage: u16, breserved: u8) -> u32 {
    ::windows::imp::link ! ( "ole32.dll""system" fn StgPropertyLengthAsVariant ( pprop : *const SERIALIZEDPROPERTYVALUE , cbprop : u32 , codepage : u16 , breserved : u8 ) -> u32 );
    StgPropertyLengthAsVariant(pprop, cbprop, codepage, breserved)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StgSerializePropVariant(ppropvar: *const PROPVARIANT, ppprop: *mut *mut SERIALIZEDPROPERTYVALUE, pcb: *mut u32) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "propsys.dll""system" fn StgSerializePropVariant ( ppropvar : *const PROPVARIANT , ppprop : *mut *mut SERIALIZEDPROPERTYVALUE , pcb : *mut u32 ) -> :: windows::core::HRESULT );
    StgSerializePropVariant(ppropvar, ppprop, pcb).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StgSetTimes<P0>(lpszname: P0, pctime: ::core::option::Option<*const super::super::super::Foundation::FILETIME>, patime: ::core::option::Option<*const super::super::super::Foundation::FILETIME>, pmtime: ::core::option::Option<*const super::super::super::Foundation::FILETIME>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn StgSetTimes ( lpszname : :: windows::core::PCWSTR , pctime : *const super::super::super::Foundation:: FILETIME , patime : *const super::super::super::Foundation:: FILETIME , pmtime : *const super::super::super::Foundation:: FILETIME ) -> :: windows::core::HRESULT );
    StgSetTimes(lpszname.into_param().abi(), ::core::mem::transmute(pctime.unwrap_or(::std::ptr::null())), ::core::mem::transmute(patime.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pmtime.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn WriteClassStg<P0>(pstg: P0, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<IStorage>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn WriteClassStg ( pstg : * mut::core::ffi::c_void , rclsid : *const :: windows::core::GUID ) -> :: windows::core::HRESULT );
    WriteClassStg(pstg.into_param().abi(), rclsid).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn WriteClassStm<P0>(pstm: P0, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::IStream>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn WriteClassStm ( pstm : * mut::core::ffi::c_void , rclsid : *const :: windows::core::GUID ) -> :: windows::core::HRESULT );
    WriteClassStm(pstm.into_param().abi(), rclsid).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn WriteFmtUserTypeStg<P0, P1>(pstg: P0, cf: u16, lpszusertype: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<IStorage>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn WriteFmtUserTypeStg ( pstg : * mut::core::ffi::c_void , cf : u16 , lpszusertype : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    WriteFmtUserTypeStg(pstg.into_param().abi(), cf, lpszusertype.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
pub struct IDirectWriterLock(::windows::core::IUnknown);
impl IDirectWriterLock {
    pub unsafe fn WaitForWriteAccess(&self, dwtimeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WaitForWriteAccess)(::windows::core::Interface::as_raw(self), dwtimeout).ok()
    }
    pub unsafe fn ReleaseWriteAccess(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseWriteAccess)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn HaveWriteAccess(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HaveWriteAccess)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IDirectWriterLock, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDirectWriterLock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectWriterLock {}
impl ::core::fmt::Debug for IDirectWriterLock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectWriterLock").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectWriterLock {
    type Vtable = IDirectWriterLock_Vtbl;
}
impl ::core::clone::Clone for IDirectWriterLock {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDirectWriterLock {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e6d4d92_6738_11cf_9608_00aa00680db4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectWriterLock_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub WaitForWriteAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwtimeout: u32) -> ::windows::core::HRESULT,
    pub ReleaseWriteAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HaveWriteAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
pub struct IEnumSTATPROPSETSTG(::windows::core::IUnknown);
impl IEnumSTATPROPSETSTG {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Next(&self, rgelt: &mut [STATPROPSETSTG], pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(rgelt.as_ptr()), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt)
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumSTATPROPSETSTG> {
        let mut result__ = ::windows::core::zeroed::<IEnumSTATPROPSETSTG>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IEnumSTATPROPSETSTG, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumSTATPROPSETSTG {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumSTATPROPSETSTG {}
impl ::core::fmt::Debug for IEnumSTATPROPSETSTG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumSTATPROPSETSTG").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumSTATPROPSETSTG {
    type Vtable = IEnumSTATPROPSETSTG_Vtbl;
}
impl ::core::clone::Clone for IEnumSTATPROPSETSTG {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumSTATPROPSETSTG {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000013b_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSTATPROPSETSTG_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut STATPROPSETSTG, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
pub struct IEnumSTATPROPSTG(::windows::core::IUnknown);
impl IEnumSTATPROPSTG {
    pub unsafe fn Next(&self, rgelt: &mut [STATPROPSTG], pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(rgelt.as_ptr()), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt)
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumSTATPROPSTG> {
        let mut result__ = ::windows::core::zeroed::<IEnumSTATPROPSTG>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IEnumSTATPROPSTG, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumSTATPROPSTG {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumSTATPROPSTG {}
impl ::core::fmt::Debug for IEnumSTATPROPSTG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumSTATPROPSTG").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumSTATPROPSTG {
    type Vtable = IEnumSTATPROPSTG_Vtbl;
}
impl ::core::clone::Clone for IEnumSTATPROPSTG {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumSTATPROPSTG {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000139_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSTATPROPSTG_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut STATPROPSTG, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
pub struct IEnumSTATSTG(::windows::core::IUnknown);
impl IEnumSTATSTG {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Next(&self, rgelt: &mut [super::STATSTG], pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(rgelt.as_ptr()), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumSTATSTG> {
        let mut result__ = ::windows::core::zeroed::<IEnumSTATSTG>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IEnumSTATSTG, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumSTATSTG {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumSTATSTG {}
impl ::core::fmt::Debug for IEnumSTATSTG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumSTATSTG").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumSTATSTG {
    type Vtable = IEnumSTATSTG_Vtbl;
}
impl ::core::clone::Clone for IEnumSTATSTG {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumSTATSTG {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000000d_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSTATSTG_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut super::STATSTG, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
pub struct IFillLockBytes(::windows::core::IUnknown);
impl IFillLockBytes {
    pub unsafe fn FillAppend(&self, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).FillAppend)(::windows::core::Interface::as_raw(self), pv, cb, &mut result__).from_abi(result__)
    }
    pub unsafe fn FillAt(&self, uloffset: u64, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).FillAt)(::windows::core::Interface::as_raw(self), uloffset, pv, cb, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFillSize(&self, ulsize: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFillSize)(::windows::core::Interface::as_raw(self), ulsize).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Terminate<P0>(&self, bcanceled: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Terminate)(::windows::core::Interface::as_raw(self), bcanceled.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IFillLockBytes, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IFillLockBytes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFillLockBytes {}
impl ::core::fmt::Debug for IFillLockBytes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFillLockBytes").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IFillLockBytes {
    type Vtable = IFillLockBytes_Vtbl;
}
impl ::core::clone::Clone for IFillLockBytes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IFillLockBytes {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99caf010_415e_11cf_8814_00aa00b569f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFillLockBytes_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub FillAppend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT,
    pub FillAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uloffset: u64, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT,
    pub SetFillSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulsize: u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Terminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bcanceled: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Terminate: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
pub struct ILayoutStorage(::windows::core::IUnknown);
impl ILayoutStorage {
    pub unsafe fn LayoutScript(&self, pstoragelayout: &[super::StorageLayout], glfinterleavedflag: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LayoutScript)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstoragelayout.as_ptr()), pstoragelayout.len() as _, glfinterleavedflag).ok()
    }
    pub unsafe fn BeginMonitor(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginMonitor)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EndMonitor(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndMonitor)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ReLayoutDocfile<P0>(&self, pwcsnewdfname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).ReLayoutDocfile)(::windows::core::Interface::as_raw(self), pwcsnewdfname.into_param().abi()).ok()
    }
    pub unsafe fn ReLayoutDocfileOnILockBytes<P0>(&self, pilockbytes: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ILockBytes>,
    {
        (::windows::core::Interface::vtable(self).ReLayoutDocfileOnILockBytes)(::windows::core::Interface::as_raw(self), pilockbytes.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ILayoutStorage, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ILayoutStorage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILayoutStorage {}
impl ::core::fmt::Debug for ILayoutStorage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILayoutStorage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ILayoutStorage {
    type Vtable = ILayoutStorage_Vtbl;
}
impl ::core::clone::Clone for ILayoutStorage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILayoutStorage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e6d4d90_6738_11cf_9608_00aa00680db4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILayoutStorage_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub LayoutScript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstoragelayout: *const super::StorageLayout, nentries: u32, glfinterleavedflag: u32) -> ::windows::core::HRESULT,
    pub BeginMonitor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EndMonitor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReLayoutDocfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsnewdfname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub ReLayoutDocfileOnILockBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pilockbytes: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
pub struct ILockBytes(::windows::core::IUnknown);
impl ILockBytes {
    pub unsafe fn ReadAt(&self, uloffset: u64, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReadAt)(::windows::core::Interface::as_raw(self), uloffset, pv, cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn WriteAt(&self, uloffset: u64, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteAt)(::windows::core::Interface::as_raw(self), uloffset, pv, cb, ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Flush)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetSize(&self, cb: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSize)(::windows::core::Interface::as_raw(self), cb).ok()
    }
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: super::LOCKTYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LockRegion)(::windows::core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnlockRegion)(::windows::core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Stat(&self, pstatstg: *mut super::STATSTG, grfstatflag: super::STATFLAG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Stat)(::windows::core::Interface::as_raw(self), pstatstg, grfstatflag).ok()
    }
}
::windows::imp::interface_hierarchy!(ILockBytes, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ILockBytes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILockBytes {}
impl ::core::fmt::Debug for ILockBytes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILockBytes").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ILockBytes {
    type Vtable = ILockBytes_Vtbl;
}
impl ::core::clone::Clone for ILockBytes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILockBytes {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000000a_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockBytes_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ReadAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uloffset: u64, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::HRESULT,
    pub WriteAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uloffset: u64, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT,
    pub Flush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cb: u64) -> ::windows::core::HRESULT,
    pub LockRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: super::LOCKTYPE) -> ::windows::core::HRESULT,
    pub UnlockRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Stat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatstg: *mut super::STATSTG, grfstatflag: super::STATFLAG) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Stat: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
pub struct IPersistStorage(::windows::core::IUnknown);
impl IPersistStorage {
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetClassID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn IsDirty(&self) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).IsDirty)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn InitNew<P0>(&self, pstg: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IStorage>,
    {
        (::windows::core::Interface::vtable(self).InitNew)(::windows::core::Interface::as_raw(self), pstg.into_param().abi()).ok()
    }
    pub unsafe fn Load<P0>(&self, pstg: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IStorage>,
    {
        (::windows::core::Interface::vtable(self).Load)(::windows::core::Interface::as_raw(self), pstg.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<P0, P1>(&self, pstgsave: P0, fsameasload: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IStorage>,
        P1: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Save)(::windows::core::Interface::as_raw(self), pstgsave.into_param().abi(), fsameasload.into_param().abi()).ok()
    }
    pub unsafe fn SaveCompleted<P0>(&self, pstgnew: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IStorage>,
    {
        (::windows::core::Interface::vtable(self).SaveCompleted)(::windows::core::Interface::as_raw(self), pstgnew.into_param().abi()).ok()
    }
    pub unsafe fn HandsOffStorage(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HandsOffStorage)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IPersistStorage, ::windows::core::IUnknown, super::IPersist);
impl ::core::cmp::PartialEq for IPersistStorage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPersistStorage {}
impl ::core::fmt::Debug for IPersistStorage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistStorage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPersistStorage {
    type Vtable = IPersistStorage_Vtbl;
}
impl ::core::clone::Clone for IPersistStorage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPersistStorage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000010a_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistStorage_Vtbl {
    pub base__: super::IPersist_Vtbl,
    pub IsDirty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InitNew: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstg: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstg: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstgsave: *mut ::core::ffi::c_void, fsameasload: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Save: usize,
    pub SaveCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstgnew: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HandsOffStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
pub struct IPropertyBag(::windows::core::IUnknown);
impl IPropertyBag {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Read<P0, P1>(&self, pszpropname: P0, pvar: *mut super::VARIANT, perrorlog: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::IErrorLog>,
    {
        (::windows::core::Interface::vtable(self).Read)(::windows::core::Interface::as_raw(self), pszpropname.into_param().abi(), pvar, perrorlog.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Write<P0>(&self, pszpropname: P0, pvar: *const super::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Write)(::windows::core::Interface::as_raw(self), pszpropname.into_param().abi(), pvar).ok()
    }
}
::windows::imp::interface_hierarchy!(IPropertyBag, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPropertyBag {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyBag {}
impl ::core::fmt::Debug for IPropertyBag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyBag").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPropertyBag {
    type Vtable = IPropertyBag_Vtbl;
}
impl ::core::clone::Clone for IPropertyBag {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPropertyBag {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55272a00_42cb_11ce_8135_00aa004bb851);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyBag_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpropname: ::windows::core::PCWSTR, pvar: *mut super::VARIANT, perrorlog: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    Read: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpropname: ::windows::core::PCWSTR, pvar: *const super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    Write: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
pub struct IPropertyBag2(::windows::core::IUnknown);
impl IPropertyBag2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Read<P0>(&self, cproperties: u32, ppropbag: *const PROPBAG2, perrlog: P0, pvarvalue: *mut super::VARIANT, phrerror: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::IErrorLog>,
    {
        (::windows::core::Interface::vtable(self).Read)(::windows::core::Interface::as_raw(self), cproperties, ppropbag, perrlog.into_param().abi(), pvarvalue, phrerror).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Write(&self, cproperties: u32, ppropbag: *const PROPBAG2, pvarvalue: *const super::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Write)(::windows::core::Interface::as_raw(self), cproperties, ppropbag, pvarvalue).ok()
    }
    pub unsafe fn CountProperties(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).CountProperties)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPropertyInfo(&self, iproperty: u32, ppropbag: &mut [PROPBAG2], pcproperties: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPropertyInfo)(::windows::core::Interface::as_raw(self), iproperty, ppropbag.len() as _, ::core::mem::transmute(ppropbag.as_ptr()), pcproperties).ok()
    }
    pub unsafe fn LoadObject<P0, P1, P2>(&self, pstrname: P0, dwhint: u32, punkobject: P1, perrlog: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P2: ::windows::core::IntoParam<super::IErrorLog>,
    {
        (::windows::core::Interface::vtable(self).LoadObject)(::windows::core::Interface::as_raw(self), pstrname.into_param().abi(), dwhint, punkobject.into_param().abi(), perrlog.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IPropertyBag2, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPropertyBag2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyBag2 {}
impl ::core::fmt::Debug for IPropertyBag2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyBag2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPropertyBag2 {
    type Vtable = IPropertyBag2_Vtbl;
}
impl ::core::clone::Clone for IPropertyBag2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPropertyBag2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22f55882_280b_11d0_a8a9_00a0c90c2004);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyBag2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cproperties: u32, ppropbag: *const PROPBAG2, perrlog: *mut ::core::ffi::c_void, pvarvalue: *mut super::VARIANT, phrerror: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    Read: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cproperties: u32, ppropbag: *const PROPBAG2, pvarvalue: *const super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    Write: usize,
    pub CountProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcproperties: *mut u32) -> ::windows::core::HRESULT,
    pub GetPropertyInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iproperty: u32, cproperties: u32, ppropbag: *mut PROPBAG2, pcproperties: *mut u32) -> ::windows::core::HRESULT,
    pub LoadObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrname: ::windows::core::PCWSTR, dwhint: u32, punkobject: *mut ::core::ffi::c_void, perrlog: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
pub struct IPropertySetStorage(::windows::core::IUnknown);
impl IPropertySetStorage {
    pub unsafe fn Create(&self, rfmtid: *const ::windows::core::GUID, pclsid: *const ::windows::core::GUID, grfflags: u32, grfmode: u32) -> ::windows::core::Result<IPropertyStorage> {
        let mut result__ = ::windows::core::zeroed::<IPropertyStorage>();
        (::windows::core::Interface::vtable(self).Create)(::windows::core::Interface::as_raw(self), rfmtid, pclsid, grfflags, grfmode, &mut result__).from_abi(result__)
    }
    pub unsafe fn Open(&self, rfmtid: *const ::windows::core::GUID, grfmode: u32) -> ::windows::core::Result<IPropertyStorage> {
        let mut result__ = ::windows::core::zeroed::<IPropertyStorage>();
        (::windows::core::Interface::vtable(self).Open)(::windows::core::Interface::as_raw(self), rfmtid, grfmode, &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete(&self, rfmtid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delete)(::windows::core::Interface::as_raw(self), rfmtid).ok()
    }
    pub unsafe fn Enum(&self) -> ::windows::core::Result<IEnumSTATPROPSETSTG> {
        let mut result__ = ::windows::core::zeroed::<IEnumSTATPROPSETSTG>();
        (::windows::core::Interface::vtable(self).Enum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IPropertySetStorage, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPropertySetStorage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertySetStorage {}
impl ::core::fmt::Debug for IPropertySetStorage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertySetStorage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPropertySetStorage {
    type Vtable = IPropertySetStorage_Vtbl;
}
impl ::core::clone::Clone for IPropertySetStorage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPropertySetStorage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000013a_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertySetStorage_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rfmtid: *const ::windows::core::GUID, pclsid: *const ::windows::core::GUID, grfflags: u32, grfmode: u32, ppprstg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rfmtid: *const ::windows::core::GUID, grfmode: u32, ppprstg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rfmtid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Enum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
pub struct IPropertyStorage(::windows::core::IUnknown);
impl IPropertyStorage {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReadMultiple(&self, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *mut PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReadMultiple)(::windows::core::Interface::as_raw(self), cpspec, rgpspec, rgpropvar).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteMultiple(&self, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *const PROPVARIANT, propidnamefirst: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteMultiple)(::windows::core::Interface::as_raw(self), cpspec, rgpspec, rgpropvar, propidnamefirst).ok()
    }
    pub unsafe fn DeleteMultiple(&self, rgpspec: &[PROPSPEC]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteMultiple)(::windows::core::Interface::as_raw(self), rgpspec.len() as _, ::core::mem::transmute(rgpspec.as_ptr())).ok()
    }
    pub unsafe fn ReadPropertyNames(&self, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReadPropertyNames)(::windows::core::Interface::as_raw(self), cpropid, rgpropid, rglpwstrname).ok()
    }
    pub unsafe fn WritePropertyNames(&self, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const ::windows::core::PCWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WritePropertyNames)(::windows::core::Interface::as_raw(self), cpropid, rgpropid, rglpwstrname).ok()
    }
    pub unsafe fn DeletePropertyNames(&self, rgpropid: &[u32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeletePropertyNames)(::windows::core::Interface::as_raw(self), rgpropid.len() as _, ::core::mem::transmute(rgpropid.as_ptr())).ok()
    }
    pub unsafe fn Commit(&self, grfcommitflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Commit)(::windows::core::Interface::as_raw(self), grfcommitflags).ok()
    }
    pub unsafe fn Revert(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Revert)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Enum(&self) -> ::windows::core::Result<IEnumSTATPROPSTG> {
        let mut result__ = ::windows::core::zeroed::<IEnumSTATPROPSTG>();
        (::windows::core::Interface::vtable(self).Enum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTimes(&self, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTimes)(::windows::core::Interface::as_raw(self), pctime, patime, pmtime).ok()
    }
    pub unsafe fn SetClass(&self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetClass)(::windows::core::Interface::as_raw(self), clsid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Stat(&self, pstatpsstg: *mut STATPROPSETSTG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Stat)(::windows::core::Interface::as_raw(self), pstatpsstg).ok()
    }
}
::windows::imp::interface_hierarchy!(IPropertyStorage, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPropertyStorage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyStorage {}
impl ::core::fmt::Debug for IPropertyStorage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyStorage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPropertyStorage {
    type Vtable = IPropertyStorage_Vtbl;
}
impl ::core::clone::Clone for IPropertyStorage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPropertyStorage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000138_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStorage_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ReadMultiple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *mut PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReadMultiple: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub WriteMultiple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *const PROPVARIANT, propidnamefirst: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WriteMultiple: usize,
    pub DeleteMultiple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const PROPSPEC) -> ::windows::core::HRESULT,
    pub ReadPropertyNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub WritePropertyNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub DeletePropertyNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32) -> ::windows::core::HRESULT,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfcommitflags: u32) -> ::windows::core::HRESULT,
    pub Revert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Enum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTimes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTimes: usize,
    pub SetClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Stat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatpsstg: *mut STATPROPSETSTG) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Stat: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
pub struct IRootStorage(::windows::core::IUnknown);
impl IRootStorage {
    pub unsafe fn SwitchToFile<P0>(&self, pszfile: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SwitchToFile)(::windows::core::Interface::as_raw(self), pszfile.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IRootStorage, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRootStorage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRootStorage {}
impl ::core::fmt::Debug for IRootStorage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRootStorage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRootStorage {
    type Vtable = IRootStorage_Vtbl;
}
impl ::core::clone::Clone for IRootStorage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRootStorage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000012_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRootStorage_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SwitchToFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfile: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
pub struct IStorage(::windows::core::IUnknown);
impl IStorage {
    pub unsafe fn CreateStream<P0>(&self, pwcsname: P0, grfmode: super::STGM, reserved1: u32, reserved2: u32) -> ::windows::core::Result<super::IStream>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::IStream>();
        (::windows::core::Interface::vtable(self).CreateStream)(::windows::core::Interface::as_raw(self), pwcsname.into_param().abi(), grfmode, reserved1, reserved2, &mut result__).from_abi(result__)
    }
    pub unsafe fn OpenStream<P0>(&self, pwcsname: P0, reserved1: ::core::option::Option<*const ::core::ffi::c_void>, grfmode: super::STGM, reserved2: u32) -> ::windows::core::Result<super::IStream>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::IStream>();
        (::windows::core::Interface::vtable(self).OpenStream)(::windows::core::Interface::as_raw(self), pwcsname.into_param().abi(), ::core::mem::transmute(reserved1.unwrap_or(::std::ptr::null())), grfmode, reserved2, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateStorage<P0>(&self, pwcsname: P0, grfmode: super::STGM, reserved1: u32, reserved2: u32) -> ::windows::core::Result<IStorage>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IStorage>();
        (::windows::core::Interface::vtable(self).CreateStorage)(::windows::core::Interface::as_raw(self), pwcsname.into_param().abi(), grfmode, reserved1, reserved2, &mut result__).from_abi(result__)
    }
    pub unsafe fn OpenStorage<P0, P1>(&self, pwcsname: P0, pstgpriority: P1, grfmode: super::STGM, snbexclude: *const *const u16, reserved: u32) -> ::windows::core::Result<IStorage>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IStorage>,
    {
        let mut result__ = ::windows::core::zeroed::<IStorage>();
        (::windows::core::Interface::vtable(self).OpenStorage)(::windows::core::Interface::as_raw(self), pwcsname.into_param().abi(), pstgpriority.into_param().abi(), grfmode, snbexclude, reserved, &mut result__).from_abi(result__)
    }
    pub unsafe fn CopyTo<P0>(&self, rgiidexclude: ::core::option::Option<&[::windows::core::GUID]>, snbexclude: ::core::option::Option<*const *const u16>, pstgdest: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IStorage>,
    {
        (::windows::core::Interface::vtable(self).CopyTo)(::windows::core::Interface::as_raw(self), rgiidexclude.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(rgiidexclude.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(snbexclude.unwrap_or(::std::ptr::null())), pstgdest.into_param().abi()).ok()
    }
    pub unsafe fn MoveElementTo<P0, P1, P2>(&self, pwcsname: P0, pstgdest: P1, pwcsnewname: P2, grfflags: STGMOVE) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IStorage>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).MoveElementTo)(::windows::core::Interface::as_raw(self), pwcsname.into_param().abi(), pstgdest.into_param().abi(), pwcsnewname.into_param().abi(), grfflags).ok()
    }
    pub unsafe fn Commit(&self, grfcommitflags: super::STGC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Commit)(::windows::core::Interface::as_raw(self), grfcommitflags).ok()
    }
    pub unsafe fn Revert(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Revert)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnumElements(&self, reserved1: u32, reserved2: ::core::option::Option<*const ::core::ffi::c_void>, reserved3: u32) -> ::windows::core::Result<IEnumSTATSTG> {
        let mut result__ = ::windows::core::zeroed::<IEnumSTATSTG>();
        (::windows::core::Interface::vtable(self).EnumElements)(::windows::core::Interface::as_raw(self), reserved1, ::core::mem::transmute(reserved2.unwrap_or(::std::ptr::null())), reserved3, &mut result__).from_abi(result__)
    }
    pub unsafe fn DestroyElement<P0>(&self, pwcsname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DestroyElement)(::windows::core::Interface::as_raw(self), pwcsname.into_param().abi()).ok()
    }
    pub unsafe fn RenameElement<P0, P1>(&self, pwcsoldname: P0, pwcsnewname: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).RenameElement)(::windows::core::Interface::as_raw(self), pwcsoldname.into_param().abi(), pwcsnewname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetElementTimes<P0>(&self, pwcsname: P0, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetElementTimes)(::windows::core::Interface::as_raw(self), pwcsname.into_param().abi(), pctime, patime, pmtime).ok()
    }
    pub unsafe fn SetClass(&self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetClass)(::windows::core::Interface::as_raw(self), clsid).ok()
    }
    pub unsafe fn SetStateBits(&self, grfstatebits: u32, grfmask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStateBits)(::windows::core::Interface::as_raw(self), grfstatebits, grfmask).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Stat(&self, pstatstg: *mut super::STATSTG, grfstatflag: super::STATFLAG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Stat)(::windows::core::Interface::as_raw(self), pstatstg, grfstatflag).ok()
    }
}
::windows::imp::interface_hierarchy!(IStorage, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IStorage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorage {}
impl ::core::fmt::Debug for IStorage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IStorage {
    type Vtable = IStorage_Vtbl;
}
impl ::core::clone::Clone for IStorage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStorage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000000b_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorage_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsname: ::windows::core::PCWSTR, grfmode: super::STGM, reserved1: u32, reserved2: u32, ppstm: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OpenStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsname: ::windows::core::PCWSTR, reserved1: *const ::core::ffi::c_void, grfmode: super::STGM, reserved2: u32, ppstm: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsname: ::windows::core::PCWSTR, grfmode: super::STGM, reserved1: u32, reserved2: u32, ppstg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OpenStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsname: ::windows::core::PCWSTR, pstgpriority: *mut ::core::ffi::c_void, grfmode: super::STGM, snbexclude: *const *const u16, reserved: u32, ppstg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CopyTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ciidexclude: u32, rgiidexclude: *const ::windows::core::GUID, snbexclude: *const *const u16, pstgdest: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MoveElementTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsname: ::windows::core::PCWSTR, pstgdest: *mut ::core::ffi::c_void, pwcsnewname: ::windows::core::PCWSTR, grfflags: STGMOVE) -> ::windows::core::HRESULT,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfcommitflags: super::STGC) -> ::windows::core::HRESULT,
    pub Revert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reserved1: u32, reserved2: *const ::core::ffi::c_void, reserved3: u32, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DestroyElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub RenameElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsoldname: ::windows::core::PCWSTR, pwcsnewname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetElementTimes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsname: ::windows::core::PCWSTR, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetElementTimes: usize,
    pub SetClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetStateBits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfstatebits: u32, grfmask: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Stat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatstg: *mut super::STATSTG, grfstatflag: super::STATFLAG) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Stat: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const CCH_MAX_PROPSTG_NAME: u32 = 31u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const CWCSTORAGENAME: u32 = 32u32;
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
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PRSPEC_INVALID: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGOPTIONS_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PIDMSI_STATUS_VALUE(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_NORMAL: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(0i32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_NEW: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(1i32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_PRELIM: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(2i32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_DRAFT: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(3i32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_INPROGRESS: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(4i32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_EDIT: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(5i32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_REVIEW: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(6i32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_PROOF: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(7i32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_FINAL: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(8i32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_OTHER: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(32767i32);
impl ::core::marker::Copy for PIDMSI_STATUS_VALUE {}
impl ::core::clone::Clone for PIDMSI_STATUS_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PIDMSI_STATUS_VALUE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PIDMSI_STATUS_VALUE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PIDMSI_STATUS_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PIDMSI_STATUS_VALUE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPSPEC_KIND(pub u32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PRSPEC_LPWSTR: PROPSPEC_KIND = PROPSPEC_KIND(0u32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PRSPEC_PROPID: PROPSPEC_KIND = PROPSPEC_KIND(1u32);
impl ::core::marker::Copy for PROPSPEC_KIND {}
impl ::core::clone::Clone for PROPSPEC_KIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPSPEC_KIND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROPSPEC_KIND {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROPSPEC_KIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPSPEC_KIND").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct STGFMT(pub u32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGFMT_STORAGE: STGFMT = STGFMT(0u32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGFMT_NATIVE: STGFMT = STGFMT(1u32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGFMT_FILE: STGFMT = STGFMT(3u32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGFMT_ANY: STGFMT = STGFMT(4u32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGFMT_DOCFILE: STGFMT = STGFMT(5u32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGFMT_DOCUMENT: STGFMT = STGFMT(0u32);
impl ::core::marker::Copy for STGFMT {}
impl ::core::clone::Clone for STGFMT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STGFMT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for STGFMT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for STGFMT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STGFMT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct STGMOVE(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGMOVE_MOVE: STGMOVE = STGMOVE(0i32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGMOVE_COPY: STGMOVE = STGMOVE(1i32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGMOVE_SHALLOWCOPY: STGMOVE = STGMOVE(2i32);
impl ::core::marker::Copy for STGMOVE {}
impl ::core::clone::Clone for STGMOVE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STGMOVE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for STGMOVE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for STGMOVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STGMOVE").field(&self.0).finish()
    }
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
impl ::core::fmt::Debug for BSTRBLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BSTRBLOB").field("cbSize", &self.cbSize).field("pData", &self.pData).finish()
    }
}
impl ::windows::core::TypeKind for BSTRBLOB {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for BSTRBLOB {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pData == other.pData
    }
}
impl ::core::cmp::Eq for BSTRBLOB {}
impl ::core::default::Default for BSTRBLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CABOOL {
    pub cElems: u32,
    pub pElems: *mut super::super::super::Foundation::VARIANT_BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CABOOL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CABOOL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CABOOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CABOOL").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for CABOOL {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CABOOL {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CABOOL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CABOOL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CABSTR {
    pub cElems: u32,
    pub pElems: *mut ::windows::core::BSTR,
}
impl ::core::marker::Copy for CABSTR {}
impl ::core::clone::Clone for CABSTR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CABSTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CABSTR").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::windows::core::TypeKind for CABSTR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CABSTR {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::core::cmp::Eq for CABSTR {}
impl ::core::default::Default for CABSTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CABSTRBLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CABSTRBLOB").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::windows::core::TypeKind for CABSTRBLOB {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CABSTRBLOB {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::core::cmp::Eq for CABSTRBLOB {}
impl ::core::default::Default for CABSTRBLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CAC {
    pub cElems: u32,
    pub pElems: ::windows::core::PSTR,
}
impl ::core::marker::Copy for CAC {}
impl ::core::clone::Clone for CAC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CAC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAC").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::windows::core::TypeKind for CAC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CAC {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::core::cmp::Eq for CAC {}
impl ::core::default::Default for CAC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CACLIPDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CACLIPDATA").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::windows::core::TypeKind for CACLIPDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CACLIPDATA {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::core::cmp::Eq for CACLIPDATA {}
impl ::core::default::Default for CACLIPDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CACLSID {
    pub cElems: u32,
    pub pElems: *mut ::windows::core::GUID,
}
impl ::core::marker::Copy for CACLSID {}
impl ::core::clone::Clone for CACLSID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CACLSID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CACLSID").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::windows::core::TypeKind for CACLSID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CACLSID {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::core::cmp::Eq for CACLSID {}
impl ::core::default::Default for CACLSID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CACY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CACY").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::windows::core::TypeKind for CACY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CACY {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::core::cmp::Eq for CACY {}
impl ::core::default::Default for CACY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CADATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CADATE").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::windows::core::TypeKind for CADATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CADATE {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::core::cmp::Eq for CADATE {}
impl ::core::default::Default for CADATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CADBL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CADBL").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::windows::core::TypeKind for CADBL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CADBL {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::core::cmp::Eq for CADBL {}
impl ::core::default::Default for CADBL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CAFILETIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAFILETIME").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for CAFILETIME {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CAFILETIME {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CAFILETIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CAFILETIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CAFLT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAFLT").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::windows::core::TypeKind for CAFLT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CAFLT {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::core::cmp::Eq for CAFLT {}
impl ::core::default::Default for CAFLT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CAH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAH").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::windows::core::TypeKind for CAH {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CAH {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::core::cmp::Eq for CAH {}
impl ::core::default::Default for CAH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CAI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAI").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::windows::core::TypeKind for CAI {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CAI {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::core::cmp::Eq for CAI {}
impl ::core::default::Default for CAI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAL").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::windows::core::TypeKind for CAL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CAL {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::core::cmp::Eq for CAL {}
impl ::core::default::Default for CAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CALPSTR {
    pub cElems: u32,
    pub pElems: *mut ::windows::core::PSTR,
}
impl ::core::marker::Copy for CALPSTR {}
impl ::core::clone::Clone for CALPSTR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CALPSTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CALPSTR").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::windows::core::TypeKind for CALPSTR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CALPSTR {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::core::cmp::Eq for CALPSTR {}
impl ::core::default::Default for CALPSTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CALPWSTR {
    pub cElems: u32,
    pub pElems: *mut ::windows::core::PWSTR,
}
impl ::core::marker::Copy for CALPWSTR {}
impl ::core::clone::Clone for CALPWSTR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CALPWSTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CALPWSTR").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::windows::core::TypeKind for CALPWSTR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CALPWSTR {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::core::cmp::Eq for CALPWSTR {}
impl ::core::default::Default for CALPWSTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CAPROPVARIANT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAPROPVARIANT").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for CAPROPVARIANT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CAPROPVARIANT {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CAPROPVARIANT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CAPROPVARIANT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CASCODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CASCODE").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::windows::core::TypeKind for CASCODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CASCODE {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::core::cmp::Eq for CASCODE {}
impl ::core::default::Default for CASCODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CAUB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAUB").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::windows::core::TypeKind for CAUB {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CAUB {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::core::cmp::Eq for CAUB {}
impl ::core::default::Default for CAUB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CAUH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAUH").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::windows::core::TypeKind for CAUH {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CAUH {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::core::cmp::Eq for CAUH {}
impl ::core::default::Default for CAUH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CAUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAUI").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::windows::core::TypeKind for CAUI {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CAUI {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::core::cmp::Eq for CAUI {}
impl ::core::default::Default for CAUI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CAUL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAUL").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::windows::core::TypeKind for CAUL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CAUL {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::core::cmp::Eq for CAUL {}
impl ::core::default::Default for CAUL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::core::fmt::Debug for CLIPDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLIPDATA").field("cbSize", &self.cbSize).field("ulClipFmt", &self.ulClipFmt).field("pClipData", &self.pClipData).finish()
    }
}
impl ::windows::core::TypeKind for CLIPDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CLIPDATA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.ulClipFmt == other.ulClipFmt && self.pClipData == other.pClipData
    }
}
impl ::core::cmp::Eq for CLIPDATA {}
impl ::core::default::Default for CLIPDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::core::fmt::Debug for OLESTREAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLESTREAM").field("lpstbl", &self.lpstbl).finish()
    }
}
impl ::windows::core::TypeKind for OLESTREAM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for OLESTREAM {
    fn eq(&self, other: &Self) -> bool {
        self.lpstbl == other.lpstbl
    }
}
impl ::core::cmp::Eq for OLESTREAM {}
impl ::core::default::Default for OLESTREAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for OLESTREAMVTBL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLESTREAMVTBL").field("Get", &self.Get).field("Put", &self.Put).finish()
    }
}
impl ::windows::core::TypeKind for OLESTREAMVTBL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for OLESTREAMVTBL {
    fn eq(&self, other: &Self) -> bool {
        self.Get == other.Get && self.Put == other.Put
    }
}
impl ::core::cmp::Eq for OLESTREAMVTBL {}
impl ::core::default::Default for OLESTREAMVTBL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct PMemoryAllocator(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct PROPBAG2 {
    pub dwType: u32,
    pub vt: super::VARENUM,
    pub cfType: u16,
    pub dwHint: u32,
    pub pstrName: ::windows::core::PWSTR,
    pub clsid: ::windows::core::GUID,
}
impl ::core::marker::Copy for PROPBAG2 {}
impl ::core::clone::Clone for PROPBAG2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROPBAG2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROPBAG2").field("dwType", &self.dwType).field("vt", &self.vt).field("cfType", &self.cfType).field("dwHint", &self.dwHint).field("pstrName", &self.pstrName).field("clsid", &self.clsid).finish()
    }
}
impl ::windows::core::TypeKind for PROPBAG2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PROPBAG2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType && self.vt == other.vt && self.cfType == other.cfType && self.dwHint == other.dwHint && self.pstrName == other.pstrName && self.clsid == other.clsid
    }
}
impl ::core::cmp::Eq for PROPBAG2 {}
impl ::core::default::Default for PROPBAG2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::windows::core::TypeKind for PROPSPEC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for PROPSPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub union PROPSPEC_0 {
    pub propid: u32,
    pub lpwstr: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for PROPSPEC_0 {}
impl ::core::clone::Clone for PROPSPEC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for PROPSPEC_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for PROPSPEC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PROPVARIANT {
    pub Anonymous: PROPVARIANT_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROPVARIANT {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for PROPVARIANT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PROPVARIANT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union PROPVARIANT_0 {
    pub Anonymous: ::std::mem::ManuallyDrop<PROPVARIANT_0_0>,
    pub decVal: super::super::super::Foundation::DECIMAL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROPVARIANT_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for PROPVARIANT_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PROPVARIANT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PROPVARIANT_0_0 {
    pub vt: super::VARENUM,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: PROPVARIANT_0_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROPVARIANT_0_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for PROPVARIANT_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PROPVARIANT_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union PROPVARIANT_0_0_0 {
    pub cVal: u8,
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
    pub boolVal: super::super::super::Foundation::VARIANT_BOOL,
    pub __OBSOLETE__VARIANT_BOOL: super::super::super::Foundation::VARIANT_BOOL,
    pub scode: i32,
    pub cyVal: super::CY,
    pub date: f64,
    pub filetime: super::super::super::Foundation::FILETIME,
    pub puuid: *mut ::windows::core::GUID,
    pub pclipdata: *mut CLIPDATA,
    pub bstrVal: ::std::mem::ManuallyDrop<::windows::core::BSTR>,
    pub bstrblobVal: BSTRBLOB,
    pub blob: super::BLOB,
    pub pszVal: ::windows::core::PSTR,
    pub pwszVal: ::windows::core::PWSTR,
    pub punkVal: ::std::mem::ManuallyDrop<::core::option::Option<::windows::core::IUnknown>>,
    pub pdispVal: ::std::mem::ManuallyDrop<::core::option::Option<super::IDispatch>>,
    pub pStream: ::std::mem::ManuallyDrop<::core::option::Option<super::IStream>>,
    pub pStorage: ::std::mem::ManuallyDrop<::core::option::Option<IStorage>>,
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
    pub pcVal: ::windows::core::PSTR,
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub puiVal: *mut u16,
    pub plVal: *mut i32,
    pub pulVal: *mut u32,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut super::super::super::Foundation::VARIANT_BOOL,
    pub pdecVal: *mut super::super::super::Foundation::DECIMAL,
    pub pscode: *mut i32,
    pub pcyVal: *mut super::CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut ::windows::core::BSTR,
    pub ppunkVal: *mut ::core::option::Option<::windows::core::IUnknown>,
    pub ppdispVal: *mut ::core::option::Option<super::IDispatch>,
    pub pparray: *mut *mut super::SAFEARRAY,
    pub pvarVal: *mut PROPVARIANT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROPVARIANT_0_0_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for PROPVARIANT_0_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PROPVARIANT_0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::core::fmt::Debug for RemSNB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RemSNB").field("ulCntStr", &self.ulCntStr).field("ulCntChar", &self.ulCntChar).field("rgString", &self.rgString).finish()
    }
}
impl ::windows::core::TypeKind for RemSNB {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RemSNB {
    fn eq(&self, other: &Self) -> bool {
        self.ulCntStr == other.ulCntStr && self.ulCntChar == other.ulCntChar && self.rgString == other.rgString
    }
}
impl ::core::cmp::Eq for RemSNB {}
impl ::core::default::Default for RemSNB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SERIALIZEDPROPERTYVALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERIALIZEDPROPERTYVALUE").field("dwType", &self.dwType).field("rgb", &self.rgb).finish()
    }
}
impl ::windows::core::TypeKind for SERIALIZEDPROPERTYVALUE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERIALIZEDPROPERTYVALUE {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType && self.rgb == other.rgb
    }
}
impl ::core::cmp::Eq for SERIALIZEDPROPERTYVALUE {}
impl ::core::default::Default for SERIALIZEDPROPERTYVALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STATPROPSETSTG {
    pub fmtid: ::windows::core::GUID,
    pub clsid: ::windows::core::GUID,
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STATPROPSETSTG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STATPROPSETSTG").field("fmtid", &self.fmtid).field("clsid", &self.clsid).field("grfFlags", &self.grfFlags).field("mtime", &self.mtime).field("ctime", &self.ctime).field("atime", &self.atime).field("dwOSVersion", &self.dwOSVersion).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for STATPROPSETSTG {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STATPROPSETSTG {
    fn eq(&self, other: &Self) -> bool {
        self.fmtid == other.fmtid && self.clsid == other.clsid && self.grfFlags == other.grfFlags && self.mtime == other.mtime && self.ctime == other.ctime && self.atime == other.atime && self.dwOSVersion == other.dwOSVersion
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STATPROPSETSTG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STATPROPSETSTG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct STATPROPSTG {
    pub lpwstrName: ::windows::core::PWSTR,
    pub propid: u32,
    pub vt: super::VARENUM,
}
impl ::core::marker::Copy for STATPROPSTG {}
impl ::core::clone::Clone for STATPROPSTG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STATPROPSTG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STATPROPSTG").field("lpwstrName", &self.lpwstrName).field("propid", &self.propid).field("vt", &self.vt).finish()
    }
}
impl ::windows::core::TypeKind for STATPROPSTG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for STATPROPSTG {
    fn eq(&self, other: &Self) -> bool {
        self.lpwstrName == other.lpwstrName && self.propid == other.propid && self.vt == other.vt
    }
}
impl ::core::cmp::Eq for STATPROPSTG {}
impl ::core::default::Default for STATPROPSTG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct STGOPTIONS {
    pub usVersion: u16,
    pub reserved: u16,
    pub ulSectorSize: u32,
    pub pwcsTemplateFile: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for STGOPTIONS {}
impl ::core::clone::Clone for STGOPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STGOPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STGOPTIONS").field("usVersion", &self.usVersion).field("reserved", &self.reserved).field("ulSectorSize", &self.ulSectorSize).field("pwcsTemplateFile", &self.pwcsTemplateFile).finish()
    }
}
impl ::windows::core::TypeKind for STGOPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for STGOPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.usVersion == other.usVersion && self.reserved == other.reserved && self.ulSectorSize == other.ulSectorSize && self.pwcsTemplateFile == other.pwcsTemplateFile
    }
}
impl ::core::cmp::Eq for STGOPTIONS {}
impl ::core::default::Default for STGOPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct VERSIONEDSTREAM {
    pub guidVersion: ::windows::core::GUID,
    pub pStream: ::std::mem::ManuallyDrop<::core::option::Option<super::IStream>>,
}
impl ::core::clone::Clone for VERSIONEDSTREAM {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for VERSIONEDSTREAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VERSIONEDSTREAM").field("guidVersion", &self.guidVersion).field("pStream", &self.pStream).finish()
    }
}
impl ::windows::core::TypeKind for VERSIONEDSTREAM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VERSIONEDSTREAM {
    fn eq(&self, other: &Self) -> bool {
        self.guidVersion == other.guidVersion && self.pStream == other.pStream
    }
}
impl ::core::cmp::Eq for VERSIONEDSTREAM {}
impl ::core::default::Default for VERSIONEDSTREAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
